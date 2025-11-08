# Cliente para un Contrato de Mensajes en Soroban

Este repositorio incluye un script de ejemplo en **Python** que demuestra cómo **leer** y **escribir** un mensaje en un contrato inteligente desplegado en **Soroban (Stellar)** sobre la **red de prueba (Testnet)** usando el **SDK de Python v13.1.0**.

## 🧭 Flujo de la demo
1) Lee el mensaje actual del contrato (`get_message`).  
2) Escribe un nuevo mensaje (`set_message`).  
3) Vuelve a leer para confirmar el cambio.

---

## 📦 Requisitos
- Python **3.10+** (recomendado 3.11/3.12)
- pip
- (Opcional) **virtualenv** o `venv`
- Una **clave secreta** (`USER_SECRET`) con fondos en **Testnet** para pagar las comisiones de la transacción de escritura
- Un **ID de contrato** válido (formato `C...`) en Testnet

> ⚠️ **Seguridad:** Nunca publiques tu `USER_SECRET` en GitHub. Usa variables de entorno o un `.env` que NO subas al repositorio.

---

## 🚀 Instalación rápida

Clona el repo y crea un entorno virtual (opcional, pero recomendado):

```bash
# 1) Clonar el repositorio
git clone <URL_DEL_REPO>
cd <CARPETA_DEL_REPO>

# 2) Crear y activar entorno virtual (Linux/macOS)
python3 -m venv .venv
source .venv/bin/activate

#    En Windows (PowerShell)
#    python -m venv .venv
#    .venv\Scripts\Activate.ps1

# 3) Instalar dependencias exactas
pip install -U pip
pip install "stellar-sdk==13.1.0"
```

---

## ⚙️ Configuración
El script utiliza estas variables y constantes:

- `RPC_URL` → `https://soroban-testnet.stellar.org` (nodo RPC de Testnet)
- `CONTRACT_ID` → **reemplaza** si tu contrato es distinto (debe empezar por `C`)
- `USER_SECRET` → se lee desde la **variable de entorno** `USER_SECRET` (recomendado).  
  Si no está presente, el script trae un valor de ejemplo **(cámbialo antes de subir a GitHub)**.
- `NETWORK_PASSPHRASE` → `Network.TESTNET_NETWORK_PASSPHRASE`
- `BASE_FEE` → `100` stroops (0.0000100 XLM)

### Establecer `USER_SECRET`

**Linux / macOS**
```bash
export USER_SECRET="SC...TU_SECRETO..."
```

**Windows (PowerShell)**
```powershell
$env:USER_SECRET = "SC...TU_SECRETO..."
```

> Si necesitas fondos de prueba, usa el **faucet** de Stellar Testnet.

---

## ▶️ Ejecución

Guarda el archivo como `message_python_clean.py` (ya incluido) y ejecuta:

```bash
python message_python.py
```

---

## ✅ Resultado esperado (ejemplo de consola)

```text
--- Iniciando Demo del Contrato de Mensajes ---

🔍 get_message() (leyendo desde el contrato)
📨 Mensaje actual: Hola anterior

✏️ set_message("Hola desde Python ✅") (escribiendo en el contrato)
⏳ Esperando confirmación de la transacción 1a2b3c4d...
✅ Transacción confirmada con éxito.
✅ Confirmada. Hash: 1a2b3c4d5e6f...

🔍 get_message() (leyendo desde el contrato)
📨 Mensaje actual: Hola desde Python ✅

--- Demo Finalizada ---
```

> **Notas:**
> - La primera lectura se hace vía **simulación** (no paga comisión ni cambia el estado).
> - La escritura firma y envía una transacción; luego se espera activamente hasta su **confirmación**.

---

## 🧠 ¿Cómo funciona?

### 1) Llamadas al contrato
Se construye un `HostFunction` de tipo `INVOKE_CONTRACT` con:
- Dirección del contrato (`SCAddress`) a partir de `StrKey.decode_contract(CONTRACT_ID)`
- Nombre de la función (`SCSymbol`)
- Argumentos serializados como `Xdr.SCVal` (para strings, `scval.to_string(...)`)

### 2) Lecturas (solo lectura)
- Se **prepara** y **simula** la transacción (`server.prepare_transaction(...)` y `server.simulate_transaction(...)`).
- El valor de retorno viene como **XDR base64**; se decodifica con `Xdr.SCVal.from_xdr(...)` y se convierte a `str` mediante el helper `scval_to_string(...)` (porque `scval.to_python()` fue retirado en versiones recientes del SDK).

### 3) Escrituras (modifican estado)
- Se **prepara** (añade footprint), **firma** y **envía** la transacción.
- Se realiza **polling** con `wait_tx(hash)` hasta `SUCCESS` o `FAILED`.

---

## 🧩 Estructura del archivo

```
.
├─ message_python.py   # Script principal (este repo)
└─ README.md                 # Este documento
```

---

## 🛠️ Solución de problemas

- **`FAILED` en la transacción:** Revisa el `result_xdr` y que tu cuenta tenga **XLM de Testnet**.  
- **`CONTRACT_ID` inválido:** Debe empezar por `C` y existir en **Testnet**.  
- **`USER_SECRET` sin fondos:** Usa el **faucet** de Testnet.  
- **`scval.to_python()` no existe:** Correcto; por eso el script incluye `scval_to_string(...)` para decodificar `Xdr.SCVal` a `str`.
- **Timeouts:** Aumenta `set_timeout(...)` en las transacciones si tu conexión es lenta.

---

## 🔐 Buenas prácticas
- Nunca subas llaves privadas.  
- Usa `.env` + `python-dotenv` si prefieres cargar variables localmente.  
- Fija versiones (SDK, Python) para reproducibilidad.

---

## 📄 Licencia
Este ejemplo se distribuye bajo la licencia **MIT**. Ajusta a las necesidades de tu proyecto.

