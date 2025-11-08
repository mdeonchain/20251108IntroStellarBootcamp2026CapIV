// Cargo.toml necesario:
// [package]
// name = "rust"
// version = "0.1.0"
// edition = "2021"
//
// [dependencies]
// stellar-rpc-client = "21.0.0"
// stellar-xdr = { version = "21.0.0", features = ["curr"] }
// stellar-strkey = "0.0.8"
// tokio = { version = "1", features = ["full"] }
// hex = "0.4"
// sha2 = "0.10"
// ed25519-dalek = { version = "2.0", features = ["rand_core"] }

use stellar_rpc_client::Client as RpcClient;
use stellar_xdr::curr::{
    ScVal, ScString, ScSymbol, VecM, InvokeContractArgs, HostFunction,
    Limits, ReadXdr, Hash, Transaction, TransactionEnvelope, Memo, MuxedAccount,
    SequenceNumber, Preconditions, Operation, OperationBody, InvokeHostFunctionOp,
    TransactionExt, WriteXdr,
};
use stellar_strkey::ed25519::PrivateKey;
use std::error::Error;
use std::str::FromStr;
use sha2::{Sha256, Digest};
use ed25519_dalek::Signer;

// ============================================
// CLIENTE DEL CONTRATO MESSAGE
// ============================================

pub struct MessageContractClient {
    rpc_client: RpcClient,
    contract_id: String,
    secret_key: String,
    network_passphrase: String,
}

impl MessageContractClient {
    /// Crea un nuevo cliente para el contrato MessageContract
    pub fn new(
        contract_id: &str, 
        rpc_url: &str, 
        secret_key: &str,
        network_passphrase: &str
    ) -> Result<Self, Box<dyn Error>> {
        let rpc_client = RpcClient::new(rpc_url)?;
        
        Ok(Self {
            rpc_client,
            contract_id: contract_id.to_string(),
            secret_key: secret_key.to_string(),
            network_passphrase: network_passphrase.to_string(),
        })
    }

    /// Obtiene el mensaje actual almacenado en el contrato
    pub async fn get_message(&self) -> Result<String, Box<dyn Error>> {
        println!("📖 Obteniendo mensaje del contrato...");
        
        let contract_address = self.parse_contract_address()?;
        let function_name = ScSymbol::from(
            stellar_xdr::curr::StringM::from_str("get_message")?
        );
        
        let args: VecM<ScVal, { u32::MAX }> = VecM::default();
        
        let invoke_args = InvokeContractArgs {
            contract_address,
            function_name,
            args,
        };

        let host_function = HostFunction::InvokeContract(invoke_args);
        
        let tx_envelope = self.build_transaction_envelope_for_simulation(host_function)?;
        
        let response = self.rpc_client
            .simulate_transaction_envelope(&tx_envelope)
            .await?;

        let result = self.extract_string_result(&response)?;
        println!("✅ Mensaje obtenido: '{}'", result);
        
        Ok(result)
    }

    /// Establece un nuevo mensaje en el contrato
    pub async fn set_message(&self, message: &str) -> Result<String, Box<dyn Error>> {
        println!("✍️  Estableciendo mensaje: '{}'", message);
        
        let contract_address = self.parse_contract_address()?;
        let function_name = ScSymbol::from(
            stellar_xdr::curr::StringM::from_str("set_message")?
        );
        
        let message_val = ScVal::String(ScString::from(
            stellar_xdr::curr::StringM::from_str(message)?
        ));
        
        let mut args_vec = Vec::new();
        args_vec.push(message_val);
        
        let args: VecM<ScVal, { u32::MAX }> = args_vec.try_into()?;
        
        let invoke_args = InvokeContractArgs {
            contract_address,
            function_name,
            args,
        };

        let host_function = HostFunction::InvokeContract(invoke_args);
        
        // Primero simular para obtener los recursos necesarios
        let sim_tx = self.build_transaction_envelope_for_simulation(host_function.clone())?;
        let sim_response = self.rpc_client
            .simulate_transaction_envelope(&sim_tx)
            .await?;
        
        // Construir la transacción real con los datos de la simulación
        let tx_envelope = self.build_transaction_envelope_with_simulation(
            host_function,
            &sim_response
        ).await?;
        
        // Enviar la transacción
        let response = self.rpc_client
            .send_transaction(&tx_envelope)
            .await?;
        
        let tx_hash = hex::encode(response.0);
        
        println!("✅ Mensaje establecido exitosamente");
        println!("🔗 Hash de transacción: {}", tx_hash);
        
        Ok(tx_hash)
    }

    // ============================================
    // FUNCIONES AUXILIARES
    // ============================================

    /// Construye un sobre de transacción simple para simulación
    fn build_transaction_envelope_for_simulation(
        &self,
        host_function: HostFunction
    ) -> Result<TransactionEnvelope, Box<dyn Error>> {
        let private_key = PrivateKey::from_string(&self.secret_key)?;
        
        // En ed25519-dalek 2.x usamos SigningKey directamente
        let signing_key = ed25519_dalek::SigningKey::from_bytes(&private_key.0);
        let public_bytes = signing_key.verifying_key().to_bytes();
        
        let source_account = MuxedAccount::Ed25519(stellar_xdr::curr::Uint256(public_bytes));

        let operation = Operation {
            source_account: None,
            body: OperationBody::InvokeHostFunction(InvokeHostFunctionOp {
                host_function,
                auth: VecM::default(),
            }),
        };

        let mut operations = Vec::new();
        operations.push(operation);
        let operations: VecM<Operation, 100> = operations.try_into()?;

        let transaction = Transaction {
            source_account,
            fee: 100,
            seq_num: SequenceNumber(0),
            cond: Preconditions::None,
            memo: Memo::None,
            operations,
            ext: TransactionExt::V0,
        };

        let tx_envelope = TransactionEnvelope::Tx(stellar_xdr::curr::TransactionV1Envelope {
            tx: transaction,
            signatures: VecM::default(),
        });

        Ok(tx_envelope)
    }

    /// Construye un sobre de transacción completo usando datos de simulación
    async fn build_transaction_envelope_with_simulation(
        &self,
        host_function: HostFunction,
        sim_response: &stellar_rpc_client::SimulateTransactionResponse
    ) -> Result<TransactionEnvelope, Box<dyn Error>> {
        let private_key = PrivateKey::from_string(&self.secret_key)?;
        
        // En ed25519-dalek 2.x usamos SigningKey directamente
        let signing_key = ed25519_dalek::SigningKey::from_bytes(&private_key.0);
        let public_bytes = signing_key.verifying_key().to_bytes();
        
        let source_account = MuxedAccount::Ed25519(stellar_xdr::curr::Uint256(public_bytes));

        // Obtener el account info para el sequence number usando la clave pública correcta
        let account_id_str = stellar_strkey::ed25519::PublicKey(public_bytes).to_string();
        
        println!("🔑 Usando cuenta: {}", account_id_str);
        
        let account_info = self.rpc_client
            .get_account(&account_id_str)
            .await?;

        let seq_num = account_info.seq_num.0;
        
        println!("📊 Sequence number: {}", seq_num);

        // Usar los recursos de la simulación si están disponibles
        let soroban_data = if !sim_response.transaction_data.is_empty() {
            println!("📦 Usando SorobanTransactionData de la simulación");
            Some(stellar_xdr::curr::SorobanTransactionData::from_xdr_base64(
                &sim_response.transaction_data,
                Limits::none()
            )?)
        } else {
            println!("⚠️  No hay transaction_data en la simulación");
            None
        };

        // Parsear las autorizaciones de la simulación
        let auth = if let Some(first_result) = sim_response.results.first() {
            if !first_result.auth.is_empty() {
                println!("🔐 Usando {} autorizaciones de la simulación", first_result.auth.len());
                let mut auth_vec = Vec::new();
                for auth_str in &first_result.auth {
                    let soroban_auth = stellar_xdr::curr::SorobanAuthorizationEntry::from_xdr_base64(
                        auth_str,
                        Limits::none()
                    )?;
                    auth_vec.push(soroban_auth);
                }
                auth_vec.try_into()?
            } else {
                VecM::default()
            }
        } else {
            VecM::default()
        };

        let operation = Operation {
            source_account: None,
            body: OperationBody::InvokeHostFunction(InvokeHostFunctionOp {
                host_function,
                auth,
            }),
        };

        let mut operations = Vec::new();
        operations.push(operation);
        let operations: VecM<Operation, 100> = operations.try_into()?;

        // Construir la extensión con SorobanTransactionData si está disponible
        let ext = if let Some(soroban_data) = soroban_data {
            TransactionExt::V1(stellar_xdr::curr::SorobanTransactionData {
                ext: soroban_data.ext,
                resources: soroban_data.resources,
                resource_fee: soroban_data.resource_fee,
            })
        } else {
            TransactionExt::V0
        };

        // Calcular el fee total: base fee + resource fee de la simulación
        let base_fee = 100u32;
        let resource_fee = if let TransactionExt::V1(ref data) = ext {
            data.resource_fee as u32
        } else {
            0
        };
        let total_fee = base_fee + resource_fee + 100000; // Un poco extra por seguridad
        
        println!("💰 Fee total: {} stroops (base: {}, resource: {})", total_fee, base_fee, resource_fee);

        let transaction = Transaction {
            source_account,
            fee: total_fee,
            seq_num: SequenceNumber(seq_num + 1),
            cond: Preconditions::None,
            memo: Memo::None,
            operations,
            ext,
        };

        // Firmar la transacción
        let tx_hash = self.get_transaction_hash(&transaction)?;
        let signature = signing_key.sign(&tx_hash);

        let decorated_signature = stellar_xdr::curr::DecoratedSignature {
            hint: stellar_xdr::curr::SignatureHint(public_bytes[28..32].try_into()?),
            signature: stellar_xdr::curr::Signature::try_from(signature.to_bytes().to_vec())?,
        };

        let mut signatures = Vec::new();
        signatures.push(decorated_signature);
        let signatures: VecM<stellar_xdr::curr::DecoratedSignature, 20> = signatures.try_into()?;

        let tx_envelope = TransactionEnvelope::Tx(stellar_xdr::curr::TransactionV1Envelope {
            tx: transaction,
            signatures,
        });

        Ok(tx_envelope)
    }

    /// Parsea la dirección del contrato desde el formato Stellar
    fn parse_contract_address(&self) -> Result<stellar_xdr::curr::ScAddress, Box<dyn Error>> {
        let decoded = stellar_strkey::Contract::from_string(&self.contract_id)?;
        let hash = Hash(decoded.0);
        Ok(stellar_xdr::curr::ScAddress::Contract(hash))
    }

    /// Extrae un String del resultado de la simulación
    fn extract_string_result(
        &self, 
        response: &stellar_rpc_client::SimulateTransactionResponse
    ) -> Result<String, Box<dyn Error>> {
        if let Some(first_result) = response.results.first() {
            let sc_val = ScVal::from_xdr_base64(&first_result.xdr, Limits::none())?;
            
            if let ScVal::String(sc_string) = sc_val {
                return Ok(sc_string.to_string());
            }
        }
        
        Err("No se pudo extraer el resultado".into())
    }

    /// Calcula el hash de la transacción para firmar
    fn get_transaction_hash(&self, transaction: &Transaction) -> Result<[u8; 32], Box<dyn Error>> {
        let network_id = self.get_network_id();
        let tx_xdr = transaction.to_xdr(Limits::none())?;
        
        let mut hasher = Sha256::new();
        hasher.update(&network_id);
        hasher.update([0, 0, 0, 2]); // Envelope type for TransactionV1
        hasher.update(&tx_xdr);
        
        let hash: [u8; 32] = hasher.finalize().into();
        Ok(hash)
    }

    /// Obtiene el network ID hash
    fn get_network_id(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(self.network_passphrase.as_bytes());
        hasher.finalize().into()
    }
}

// ============================================
// FUNCIÓN PRINCIPAL CON EJEMPLOS
// ============================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🚀 Cliente Rust para MessageContract");
    println!("=====================================\n");

    let contract_id = "CAJN25XAZLTZEVS7ZFLNZ3HWREJRQHKUU265CK67ED2ASJ22TDQ5Y4PL";
    let rpc_url = "https://soroban-testnet.stellar.org:443";
    let secret_key = "SDQK5C2WQ67VM4HQ3S3JAQ4XIJED7SJVTGKMDAVS7R4YCT7NJ34TLLKJ";
    let network_passphrase = "Test SDF Network ; September 2015";

    let client = MessageContractClient::new(
        contract_id, 
        rpc_url, 
        secret_key,
        network_passphrase
    )?;

    // ===== EJEMPLO 1: Obtener mensaje actual =====
    println!("\n┌─────────────────────────────────────┐");
    println!("│  EJEMPLO 1: Obtener Mensaje Actual │");
    println!("└─────────────────────────────────────┘");
    
    match client.get_message().await {
        Ok(message) => {
            println!("📝 Mensaje actual: '{}'", message);
        }
        Err(e) => {
            eprintln!("❌ Error al obtener mensaje: {}", e);
        }
    }

    // ===== EJEMPLO 2: Establecer nuevo mensaje =====
    println!("\n┌─────────────────────────────────────┐");
    println!("│  EJEMPLO 2: Establecer Nuevo Mensaje│");
    println!("└─────────────────────────────────────┘");
    
    let new_message = "¡Hola desde Rust sin CLI! 🦀🚀";
    match client.set_message(new_message).await {
        Ok(tx_hash) => {
            println!("✅ Mensaje actualizado correctamente");
            println!("📋 TX Hash: {}", tx_hash);
        }
        Err(e) => {
            eprintln!("❌ Error al establecer mensaje: {}", e);
        }
    }

    // Esperar un momento para que la transacción se procese
    println!("\n⏳ Esperando confirmación de la transacción...");
    tokio::time::sleep(tokio::time::Duration::from_secs(7)).await;

    // ===== EJEMPLO 3: Verificar actualización =====
    println!("\n┌─────────────────────────────────────┐");
    println!("│  EJEMPLO 3: Verificar Actualización │");
    println!("└─────────────────────────────────────┘");
    
    match client.get_message().await {
        Ok(message) => {
            println!("📝 Mensaje después de actualizar: '{}'", message);
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
        }
    }

    // ===== EJEMPLO 4: Secuencia de actualizaciones =====
    println!("\n┌─────────────────────────────────────┐");
    println!("│  EJEMPLO 4: Múltiples Actualizaciones│");
    println!("└─────────────────────────────────────┘");
    
    let mensajes = vec![
        "Primer mensaje 📝",
        "Segundo mensaje 💬",
        "Mensaje con Stellar ⭐️",
        "Mensaje final 🎉",
    ];

    for (i, msg) in mensajes.iter().enumerate() {
        println!("\n{}. Procesando...", i + 1);
        
        match client.set_message(msg).await {
            Ok(tx_hash) => {
                println!("   ✅ Mensaje enviado: '{}'", msg);
                println!("   📋 TX: {}", &tx_hash[..16]);
            }
            Err(e) => {
                eprintln!("   ❌ Error al establecer: {}", e);
                continue;
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(7)).await;

        match client.get_message().await {
            Ok(retrieved) => {
                println!("   📥 Mensaje verificado: '{}'", retrieved);
                
                if retrieved == *msg {
                    println!("   ✅ ¡Coincide!");
                } else {
                    println!("   ⚠️  No coincide aún, puede que esté procesándose");
                }
            }
            Err(e) => {
                eprintln!("   ❌ Error al verificar: {}", e);
            }
        }
    }

    println!("\n┌─────────────────────────────────────┐");
    println!("│      Operaciones Completadas       │");
    println!("└─────────────────────────────────────┘");
    
    println!("\n📌 Información del Contrato:");
    println!("   • ID: {}", contract_id);
    println!("   • Red: Testnet");
    println!("   • TTL de mensajes: ~10 minutos (120 ledgers)");
    
    println!("\n🔗 Links útiles:");
    println!("   • Explorer: https://stellar.expert/explorer/testnet/contract/{}", contract_id);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_client() {
        let contract_id = "CAJN25XAZLTZEVS7ZFLNZ3HWREJRQHKUU265CK67ED2ASJ22TDQ5Y4PL";
        let rpc_url = "https://soroban-testnet.stellar.org:443";
        let secret_key = "SDQK5C2WQ67VM4HQ3S3JAQ4XIJED7SJVTGKMDAVS7R4YCT7NJ34TLLKJ";
        let network_passphrase = "Test SDF Network ; September 2015";

        let result = MessageContractClient::new(
            contract_id, 
            rpc_url, 
            secret_key,
            network_passphrase
        );
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_message() {
        let contract_id = "CAJN25XAZLTZEVS7ZFLNZ3HWREJRQHKUU265CK67ED2ASJ22TDQ5Y4PL";
        let rpc_url = "https://soroban-testnet.stellar.org:443";
        let secret_key = "SDQK5C2WQ67VM4HQ3S3JAQ4XIJED7SJVTGKMDAVS7R4YCT7NJ34TLLKJ";
        let network_passphrase = "Test SDF Network ; September 2015";

        let client = MessageContractClient::new(
            contract_id, 
            rpc_url, 
            secret_key,
            network_passphrase
        ).unwrap();
        
        let result = client.get_message().await;
        assert!(result.is_ok());
    }
}