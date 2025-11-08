# Cliente JS para un Contrato de Mensajes en Soroban

Este repositorio incluye un script de ejemplo en **Node.js** que demuestra cómo **leer** y **escribir** un mensaje en un contrato inteligente desplegado en **Soroban (Stellar)** sobre la **red de prueba (Testnet)** usando el **JavaScript SDK**.

## 🧭 Flujo de la demo
1) Lee el mensaje actual del contrato (`get_message`).  
2) Escribe un nuevo mensaje (`set_message`).  
3) Vuelve a leer para confirmar el cambio.

---

## 📦 Requisitos
- **Node.js 18+** (recomendado 20 LTS)  
- **npm** o **pnpm**
- Una **clave secreta** con fondos en **Testnet** para pagar las comisiones de la transacción de escritura
- Un **ID de contrato** válido (formato `C...`) en Testnet

> ⚠️ **Seguridad:** No publiques tu clave secreta en GitHub. Este ejemplo muestra constantes para facilitar la prueba local; cámbialas antes de subir o usa variables de entorno.

---

## 🚀 Instalación rápida

Clona el repo e instala dependencias:

```bash
# 1) Clonar el repositorio
git clone <URL_DEL_REPO>
cd <CARPETA_DEL_REPO>

# 2) Instalar dependencias
npm init -y
npm install @stellar/stellar-sdk@13.1.0
```

---

## ⚙️ Configuración
El script utiliza estas constantes:

- `RPC_URL` → `https://soroban-testnet.stellar.org`
- `CONTRACT_ID` → **reemplaza** si tu contrato es distinto (debe empezar por `C`)
- `USER_SECRET` → clave secreta de **Testnet** con fondos
- `NETWORK_PASSPHRASE` → `Networks.TESTNET`

> **Sugerencia:** en producción usa `process.env.USER_SECRET` y un gestor de secretos (dotenv, Docker secrets, CI vars, etc.).

---

## ▶️ Ejecución
Guarda el archivo como `message-ecmascript.js` y ejecuta:

```bash
node message-ecmascript.js
```

## 🧩 Estructura del archivo

```
.
├─ message-ecmascript.js   # Script principal (este repo)
├─ node_modules            # Librerias del sistema
└─ README.md               # Este documento
```

---

## ✅ Resultado esperado (ejemplo de consola)

```text
--- Iniciando Demo del Contrato de Mensajes ---

🔍 get_message() (leyendo desde el contrato)
📨 Mensaje actual: Hola desde Python ✅

✏️ set_message("Hola desde Python ✅") (escribiendo en el contrato)
⏳ Esperando confirmación de la transacción 363dac87...
✅ Transacción confirmada con éxito.
✅ Confirmada. Hash: 363dac871c021ae2fe722902ed1642ddcc631db7d6e493b740a8014dd904992b

🔍 get_message() (leyendo desde el contrato)
📨 Mensaje actual: Hola desde Python ✅

--- Demo Finalizada ---
```

> **Notas:**
> - La primera lectura usa **simulación** (no paga comisión ni cambia el estado).
> - La escritura firma y envía una transacción; luego espera activamente hasta su **confirmación**.

---

## 🧠 ¿Cómo funciona?

1) **Construcción de operaciones:** se usa `new Contract(CONTRACT_ID)` y `contract.call("get_message"|"set_message", ...)` para crear las operaciones.  
2) **Lecturas:** se construye una transacción, se llama `server.simulateTransaction(tx)` y se extrae el valor de retorno (puede venir en `result.retval`, `result.retVal` o `returnValue`). Se decodifica con `scValToNative(...)`.  
3) **Escrituras:** se prepara y firma la transacción (`server.prepareTransaction(...)` → añade **footprint**), se envía con `server.sendTransaction(...)` y se espera el estado con `server.getTransaction(hash)` hasta `SUCCESS` o `FAILED`.  
4) **Formateo de salida:** el script imprime mensajes con emojis y estructura fija para que la consola coincida exactamente con el formato esperado.

---

## 🛠️ Solución de problemas
- **`FAILED` en la transacción:** Revisa que tu cuenta tenga **XLM de Testnet** y que el `CONTRACT_ID` exista en Testnet.  
- **`CONTRACT_ID` inválido:** Asegúrate de que empieza por `C` y es correcto.  
- **Errores de decodificación:** Algunas respuestas de simulación cambian la propiedad del retorno; por eso se comprueban varias (`retval`, `retVal`, `returnValue`).  
- **Timeouts o latencia:** Aumenta `.setTimeout(...)` o el `sleep` del polling.

---

## 🔐 Buenas prácticas
- Nunca subas llaves privadas.  
- Usa variables de entorno y bloquea versiones en tu `package-lock.json` o `pnpm-lock.yaml`.  
- Loguea hashes y prefijos de transacciones solo para depurar; evita exponer datos sensibles.

---

## 📄 Licencia
Este ejemplo se distribuye bajo la licencia **MIT**. Ajusta a las necesidades de tu proyecto.

