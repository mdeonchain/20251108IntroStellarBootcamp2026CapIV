# message_python_clean.py — Cliente para un Contrato de Mensajes en Soroban
#
# Este script demuestra cómo interactuar con un contrato inteligente de Soroban
# en la red de prueba de Stellar usando el SDK de Python v13.1.0.
#
# Flujo de la demostración:
# 1. Lee el mensaje actual del contrato (operación de solo lectura).
# 2. Escribe un nuevo mensaje en el contrato (operación de escritura).
# 3. Vuelve a leer el mensaje para confirmar el cambio.

from time import sleep
import os
from typing import Optional

from stellar_sdk import (
    Keypair,
    Network,
    TransactionBuilder,
    SorobanServer,   # Cliente RPC para comunicarse con la red Soroban
    StrKey,          # Utilidad para decodificar IDs de contrato (formato 'C...')
    scval,           # Utilidades para convertir entre tipos de Python y XDR
    xdr as Xdr,      # Tipos de datos XDR de Stellar, el formato de serialización
)
from stellar_sdk.operation import InvokeHostFunction

# ==============================================================================
# CONFIGURACIÓN
# ==============================================================================
# URL del nodo RPC de la red de prueba de Soroban.
RPC_URL = "https://soroban-testnet.stellar.org"
# ID del contrato desplegado (debe empezar por 'C...').
CONTRACT_ID = "CAJN25XAZLTZEVS7ZFLNZ3HWREJRQHKUU265CK67ED2ASJ22TDQ5Y4PL"
# Clave secreta de la cuenta que pagará las tarifas y firmará las transacciones.
# Se recomienda usar una variable de entorno para mayor seguridad.
USER_SECRET = os.getenv("USER_SECRET", "SDQK5C2WQ67VM4HQ3S3JAQ4XIJED7SJVTGKMDAVS7R4YCT7NJ34TLLKJ")
# Frase de paso de la red para asegurar que la transacción se ejecuta en la red correcta.
NETWORK_PASSPHRASE = Network.TESTNET_NETWORK_PASSPHRASE
# Tarifa base para la transacción en stroops (1 stroop = 0.0000001 XLM).
BASE_FEE = 100

# Inicialización del cliente RPC y del par de claves del usuario.
server = SorobanServer(RPC_URL)
user = Keypair.from_secret(USER_SECRET)

# ==============================================================================
# FUNCIONES UTILITARIAS
# ==============================================================================

def wait_tx(tx_hash: str):
    """
    Espera de forma activa (polling) a que una transacción sea confirmada en la red.
    La función sondea el estado de la transacción cada segundo hasta que
    el estado es 'SUCCESS' (éxito) o 'FAILED' (fracaso). Esto es crucial para
    las operaciones de escritura para saber cuándo se ha completado.
    """
    print(f"⏳ Esperando confirmación de la transacción {tx_hash[:8]}...")
    while True:
        tx = server.get_transaction(tx_hash)
        if tx.status.name == "SUCCESS":
            print("✅ Transacción confirmada con éxito.")
            return tx
        if tx.status.name == "FAILED":
            raise RuntimeError(f"La transacción falló en la red: {tx.result_meta_xdr}")
        sleep(1.0)

def scval_to_string(val: Optional[Xdr.SCVal]) -> Optional[str]:
    """
    Decodifica manualmente un objeto Xdr.SCVal a un string de Python.
    Esta función es necesaria porque el helper `scval.to_python()` fue eliminado
    en las versiones recientes del SDK, por lo que debemos hacer la conversión
    manualmente.
    """
    if val is None:
        return None
    
    # Un SCVal de tipo String contiene un objeto SCString en el atributo .str.
    if hasattr(val, "str") and val.str is not None:
        try:
            # El objeto SCString tiene un atributo .sc_string que contiene los bytes reales.
            # Decodificamos esos bytes de UTF-8 a un string de Python.
            return val.str.sc_string.decode("utf-8")
        except Exception:
            return None
            
    # Un SCVal de tipo Symbol contiene un objeto SCSymbol en el atributo .sym.
    if hasattr(val, "sym") and val.sym is not None:
        try:
            # De forma similar, el objeto SCSymbol tiene un atributo .sc_symbol con los bytes.
            return val.sym.sc_symbol.decode("utf-8")
        except Exception:
            return None
            
    return None

def make_host_fn(function_name: str, args_scval: list[Xdr.SCVal]) -> Xdr.HostFunction:
    """
    Construye un objeto Xdr.HostFunction que representa la llamada a una función
    de un contrato. Este es el "payload" principal de la operación Stellar.
    """
    # 1. Decodifica el ID del contrato (StrKey) a su formato de bytes crudos (32 bytes).
    contract_id_bytes = StrKey.decode_contract(CONTRACT_ID)
    
    # 2. Crea una dirección de contrato (SCAddress) a partir de esos bytes.
    sc_addr = Xdr.SCAddress(
        type=Xdr.SCAddressType.SC_ADDRESS_TYPE_CONTRACT,
        contract_id=Xdr.Hash(contract_id_bytes),
    )
    
    # 3. Construye los argumentos de la invocación: qué contrato, qué función y con qué argumentos.
    invoke_args = Xdr.InvokeContractArgs(
        contract_address=sc_addr,
        function_name=Xdr.SCSymbol(function_name.encode("utf-8")),
        args=args_scval,
    )
    
    # 4. Envuelve todo en un HostFunction de tipo INVOKE_CONTRACT.
    return Xdr.HostFunction(
        type=Xdr.HostFunctionType.HOST_FUNCTION_TYPE_INVOKE_CONTRACT,
        invoke_contract=invoke_args,
    )

# ==============================================================================
# FUNCIONES PRINCIPALES DE INTERACCIÓN
# ==============================================================================

def get_message():
    """
    LECTURA: Obtiene el mensaje actual del contrato.
    Esta función simula la transacción para obtener el resultado sin gastar
    comisión ni modificar el estado de la blockchain. Es el método preferido
    para llamar a funciones de solo lectura.
    """
    print("\n🔍 get_message() (leyendo desde el contrato)")
    account = server.load_account(user.public_key)

    # 1. Construir la operación para llamar a la función 'get_message' del contrato.
    host_fn = make_host_fn("get_message", [])
    op = InvokeHostFunction(host_function=host_fn)

    # 2. Construir la transacción que contiene la operación.
    tx = (
        TransactionBuilder(account, NETWORK_PASSPHRASE, BASE_FEE)
        .append_operation(op)
        .set_timeout(30)
        .build()
    )

    # 3. Preparar la transacción para la simulación. Esto ayuda al servidor a
    # determinar qué datos del ledger necesita (el "footprint").
    prepared_tx = server.prepare_transaction(tx)

    # 4. Simular la transacción para obtener el resultado.
    sim = server.simulate_transaction(prepared_tx)

    # 5. Verificar si la simulación fue exitosa.
    if sim.error:
        raise RuntimeError(f"La simulación de la transacción falló: {sim.error}")
    if not sim.results:
        raise RuntimeError("La simulación no devolvió resultados.")

    # 6. Extraer y decodificar el valor de retorno.
    # El resultado está en formato XDR codificado en Base64 dentro de `sim.results[0].xdr`.
    retval_xdr_string = sim.results[0].xdr
    # Decodificamos el string Base64 a un objeto Xdr.SCVal.
    retval_scval = Xdr.SCVal.from_xdr(retval_xdr_string)
    # Usamos nuestra función auxiliar para convertir el SCVal a un string de Python.
    message = scval_to_string(retval_scval)

    print("📨 Mensaje actual:", message)
    return message

def set_message(new_message: str):
    """
    ESCRITURA: Envía una transacción para almacenar un nuevo mensaje en el contrato.
    Este proceso modifica el estado de la blockchain y requiere una tarifa y una firma.
    """
    print(f'\n✏️ set_message("{new_message}") (escribiendo en el contrato)')
    account = server.load_account(user.public_key)

    # 1. Construir la operación para llamar a 'set_message' con el nuevo argumento.
    # Convertimos el string de Python a un SCVal que el contrato pueda entender.
    arg_msg = scval.to_string(new_message)
    host_fn = make_host_fn("set_message", [arg_msg])
    op = InvokeHostFunction(host_function=host_fn)

    # 2. Construir la transacción.
    tx = (
        TransactionBuilder(account, NETWORK_PASSPHRASE, BASE_FEE)
        .append_operation(op)
        .set_timeout(60)
        .build()
    )

    # 3. Preparar la transacción (paso obligatorio para escrituras, añade footprint).
    prepared = server.prepare_transaction(tx)

    # 4. Firmar la transacción con la clave privada del usuario.
    prepared.sign(user)

    # 5. Enviar la transacción a la red.
    sent = server.send_transaction(prepared)

    # 6. Verificar si la transacción fue rechazada al envío.
    if getattr(sent, "error_result_xdr", None):
        raise RuntimeError(f"Transacción rechazada al enviar (XDR): {sent.error_result_xdr}")

    # 7. Esperar a que la transacción sea confirmada en un ledger.
    wait_tx(sent.hash)
    print(f"✅ Confirmada. Hash: {sent.hash}")

# ==============================================================================
# EJECUCIÓN PRINCIPAL
# ==============================================================================
if __name__ == "__main__":
    print("--- Iniciando Demo del Contrato de Mensajes ---")
    
    # 1. Leer el mensaje actual (puede ser el predeterminado o el último guardado).
    get_message()
    
    # 2. Escribir un nuevo mensaje en el contrato.
    set_message("Hola desde Python ✅")
    
    # 3. Leer el mensaje de nuevo para verificar que se ha actualizado correctamente.
    get_message()
    
    print("\n--- Demo Finalizada ---")