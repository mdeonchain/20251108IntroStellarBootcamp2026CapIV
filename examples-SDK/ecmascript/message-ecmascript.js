// message_js_clean.js — Cliente para un Contrato de Mensajes en Soroban (Node.js)

const {
  Keypair,
  Contract,
  TransactionBuilder,
  BASE_FEE,
  Networks,
  rpc,                 // Server, Api, etc.
  scValToNative,
  nativeToScVal,
} = require("@stellar/stellar-sdk");

// ====== Ajusta estos valores ======
const RPC_URL = "https://soroban-testnet.stellar.org";
const CONTRACT_ID = "CAJN25XAZLTZEVS7ZFLNZ3HWREJRQHKUU265CK67ED2ASJ22TDQ5Y4PL";
const USER_SECRET = "SDQK5C2WQ67VM4HQ3S3JAQ4XIJED7SJVTGKMDAVS7R4YCT7NJ34TLLKJ";
// ===================================

const server = new rpc.Server(RPC_URL);
const NETWORK_PASSPHRASE = Networks.TESTNET;
const user = Keypair.fromSecret(USER_SECRET);
const contract = new Contract(CONTRACT_ID);

// Utilidad: envía una tx y espera hasta SUCCESS o FAILED
async function sendAndWait(builtTx) {
  const prepared = await server.prepareTransaction(builtTx); // añade footprint
  prepared.sign(user);
  const sent = await server.sendTransaction(prepared);

  if (sent.errorResult) {
    throw new Error("Rechazada al enviar: " + sent.errorResult.toString());
  }

  // Log: prefijo del hash para cumplir el formato
  const shortHash = sent.hash.slice(0, 8);
  console.log(`⏳ Esperando confirmación de la transacción ${shortHash}...`);

  // Polling hasta que la red confirme
  for (;;) {
    const txr = await server.getTransaction(sent.hash);
    if (txr.status === rpc.Api.GetTransactionStatus.SUCCESS) {
      console.log("✅ Transacción confirmada con éxito.");
      console.log(`✅ Confirmada. Hash: ${sent.hash}`);
      return txr;
    }
    if (txr.status === rpc.Api.GetTransactionStatus.FAILED) {
      throw new Error("Falló on-chain: " + JSON.stringify(txr));
    }
    await new Promise((r) => setTimeout(r, 1200));
  }
}

// LECTURA (gratis): simulamos la transacción para obtener el valor de retorno
async function getMessage() {
  console.log("🔍 get_message() (leyendo desde el contrato)");
  const account = await server.getAccount(user.publicKey());

  const tx = new TransactionBuilder(account, {
    fee: BASE_FEE,
    networkPassphrase: NETWORK_PASSPHRASE,
  })
    .addOperation(contract.call("get_message")) // sin argumentos
    .setTimeout(30)
    .build();

  const sim = await server.simulateTransaction(tx);

  // Algunas versiones devuelven retval, otras retVal o returnValue
  const scval = sim.result?.retval ?? sim.result?.retVal ?? sim.returnValue;
  const message = scValToNative(scval);
  console.log("📨 Mensaje actual:", message);
  return message;
}

// ESCRITURA (paga fee): invocamos set_message con un string
async function setMessage(newMessage) {
  console.log(`✏️ set_message("${newMessage}") (escribiendo en el contrato)`);
  const account = await server.getAccount(user.publicKey());

  const arg = nativeToScVal(newMessage, { type: "string" });

  const tx = new TransactionBuilder(account, {
    fee: BASE_FEE,
    networkPassphrase: NETWORK_PASSPHRASE,
  })
    .addOperation(contract.call("set_message", arg))
    .setTimeout(60)
    .build();

  await sendAndWait(tx);
}

(async () => {
  try {
    console.log("--- Iniciando Demo del Contrato de Mensajes ---\n");
    await getMessage();                                 // 1) leer
    await setMessage("Hola desde JS ✅");            // 2) escribir (texto exacto)
    console.log(""); // línea en blanco como en el ejemplo
    await getMessage();                                 // 3) leer de nuevo
    console.log("\n--- Demo Finalizada ---");
  } catch (e) {
    console.error("❌ Error:", e);
    process.exit(1);
  }
})();
