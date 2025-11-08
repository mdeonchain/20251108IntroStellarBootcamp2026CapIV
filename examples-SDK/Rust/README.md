# 🦀 Cliente Rust para Stellar Soroban

Cliente nativo en Rust para interactuar con contratos inteligentes de Soroban en la red Stellar, **sin depender del CLI de Stellar**.

## 📋 Características

- ✅ Interacción directa con contratos Soroban vía RPC
- ✅ Operaciones de lectura (view functions)
- ✅ Operaciones de escritura con firma de transacciones
- ✅ Manejo automático de recursos y fees
- ✅ Soporte para simulación de transacciones
- ✅ Tests unitarios incluidos

## 🚀 Inicio Rápido

### Prerrequisitos

- Rust 1.70+ ([Instalar Rust](https://rustup.rs/))
- Una cuenta en Stellar Testnet con fondos
- ID del contrato desplegado en Testnet

### Instalación

1. **Clonar o crear el proyecto:**

```bash
cargo new stellar-client
cd stellar-client
```

2. **Configurar `Cargo.toml`:**

```toml
[package]
name = "stellar-client"
version = "0.1.0"
edition = "2021"

[dependencies]
stellar-rpc-client = "21.0.0"
stellar-xdr = { version = "21.0.0", features = ["curr"] }
stellar-strkey = "0.0.8"
tokio = { version = "1", features = ["full"] }
hex = "0.4"
sha2 = "0.10"
ed25519-dalek = { version = "2.0", features = ["rand_core"] }
```

3. **Copiar el código del cliente** en `src/main.rs`

## 🔧 Configuración

### 1. Obtener una cuenta en Testnet

**Opción A: Usando Stellar Laboratory**
- Ve a [Stellar Laboratory - Account Creator](https://laboratory.stellar.org/#account-creator?network=test)
- Haz clic en "Generate keypair"
- Guarda tu **Secret Key** (S...) y **Public Key** (G...)
- Haz clic en "Get test network lumens" para fondear tu cuenta

**Opción B: Usando Stellar CLI**
```bash
stellar keys generate mi-cuenta --network testnet
stellar keys address mi-cuenta
```

### 2. Configurar las credenciales

Edita las siguientes líneas en `src/main.rs`:

```rust
let contract_id = "TU_CONTRACT_ID_AQUI";  // C...
let secret_key = "TU_SECRET_KEY_AQUI";    // S...
```

Constantes de la red:
```rust
let rpc_url = "https://soroban-testnet.stellar.org:443";
let network_passphrase = "Test SDF Network ; September 2015";
```

## 📖 Uso

### Compilar el proyecto

```bash
cargo build
```

### Ejecutar el cliente

```bash
cargo run
```

### Ejemplo de salida

```
🚀 Cliente Rust para MessageContract
=====================================

┌─────────────────────────────────────┐
│  EJEMPLO 1: Obtener Mensaje Actual │
└─────────────────────────────────────┘
📖 Obteniendo mensaje del contrato...
✅ Mensaje obtenido: 'Hello, World!'
📝 Mensaje actual: 'Hello, World!'

┌─────────────────────────────────────┐
│  EJEMPLO 2: Establecer Nuevo Mensaje│
└─────────────────────────────────────┘
✍️  Estableciendo mensaje: '¡Hola desde Rust sin CLI! 🦀🚀'
🔑 Usando cuenta: GATTQ6RGZSL3BJG6TMLEENNFHCCUHDZGOJYJ2AMWCJD4H3IEI3CCDESB
📊 Sequence number: 12345
📦 Usando SorobanTransactionData de la simulación
💰 Fee total: 150200 stroops (base: 100, resource: 50100)
✅ Mensaje establecido exitosamente
🔗 Hash de transacción: a1b2c3d4...
```

## 🏗️ Arquitectura del Cliente

### Estructura principal

```rust
pub struct MessageContractClient {
    rpc_client: RpcClient,
    contract_id: String,
    secret_key: String,
    network_passphrase: String,
}
```

### Métodos principales

#### `get_message()` - Lectura (View)
```rust
let message = client.get_message().await?;
println!("Mensaje: {}", message);
```

#### `set_message(message)` - Escritura
```rust
let tx_hash = client.set_message("Nuevo mensaje").await?;
println!("TX Hash: {}", tx_hash);
```

## 🔐 Seguridad

### Manejo de claves privadas

⚠️ **NUNCA** subas tu clave secreta a repositorios públicos.

**Mejores prácticas:**

1. **Variables de entorno:**
```rust
use std::env;

let secret_key = env::var("STELLAR_SECRET_KEY")
    .expect("STELLAR_SECRET_KEY debe estar configurada");
```

```bash
export STELLAR_SECRET_KEY="SXXXXXX..."
cargo run
```

2. **Archivo `.env` (con `dotenv`):**

Agregar a `Cargo.toml`:
```toml
dotenv = "0.15"
```

Crear archivo `.env`:
```
STELLAR_SECRET_KEY=SXXXXXX...
STELLAR_CONTRACT_ID=CXXXXXX...
```

Cargar en el código:
```rust
dotenv::dotenv().ok();
let secret_key = env::var("STELLAR_SECRET_KEY")?;
```

**No olvides agregar `.env` a tu `.gitignore`:**
```
.env
```

## 🧪 Testing

### Ejecutar tests

```bash
cargo test
```

### Tests incluidos

- ✅ `test_create_client` - Verificar creación del cliente
- ✅ `test_get_message` - Verificar lectura de mensajes

### Agregar más tests

```rust
#[tokio::test]
async fn test_set_message() {
    let client = MessageContractClient::new(
        "CONTRACT_ID",
        "RPC_URL",
        "SECRET_KEY",
        "NETWORK_PASSPHRASE"
    ).unwrap();
    
    let result = client.set_message("Test").await;
    assert!(result.is_ok());
}
```

## 🐛 Solución de Problemas

### Error: `TxMalformed`

**Causa:** La clave secreta no corresponde a la cuenta esperada.

**Solución:**
```bash
# Verificar qué cuenta genera tu clave secreta
cargo run --example verify-keys
```

### Error: `Account not found`

**Causa:** La cuenta no está fondeada en Testnet.

**Solución:**
1. Ve a [Stellar Laboratory](https://laboratory.stellar.org/#account-creator?network=test)
2. Pega tu Public Key (G...)
3. Haz clic en "Get test network lumens"

### Error: Compilación falla con `ed25519-dalek`

**Causa:** Versión incorrecta de `ed25519-dalek`.

**Solución:** Asegúrate de usar la versión 2.0:
```toml
ed25519-dalek = { version = "2.0", features = ["rand_core"] }
```

### Error: `InsufficientBalance`

**Causa:** No hay suficientes XLM para pagar los fees.

**Solución:** Solicita más XLM del faucet de Testnet.

## 📚 Recursos Adicionales

### Documentación Oficial

- [Stellar Docs](https://developers.stellar.org/)
- [Soroban Docs](https://soroban.stellar.org/)
- [Stellar RPC Client Docs](https://docs.rs/stellar-rpc-client/)
- [Stellar XDR Docs](https://docs.rs/stellar-xdr/)

### Herramientas

- [Stellar Laboratory](https://laboratory.stellar.org/) - Explorar y testear
- [Stellar Expert](https://stellar.expert/) - Block explorer
- [Stellar CLI](https://github.com/stellar/stellar-cli) - Herramienta de línea de comandos

### Comunidad

- [Discord de Stellar](https://discord.gg/stellardev)
- [Stack Exchange](https://stellar.stackexchange.com/)
- [GitHub Discussions](https://github.com/stellar/stellar-protocol/discussions)

## 🤝 Contribuir

Las contribuciones son bienvenidas. Por favor:

1. Fork el proyecto
2. Crea una rama para tu feature (`git checkout -b feature/AmazingFeature`)
3. Commit tus cambios (`git commit -m 'Add some AmazingFeature'`)
4. Push a la rama (`git push origin feature/AmazingFeature`)
5. Abre un Pull Request

## 📝 Licencia

Este proyecto está bajo la licencia MIT. Ver archivo `LICENSE` para más detalles.

## 🙏 Agradecimientos

- Equipo de Stellar Development Foundation
- Comunidad de desarrolladores de Soroban
- Contribuidores del proyecto

## 📞 Contacto

Si tienes preguntas o necesitas ayuda:

- Abre un [Issue](https://github.com/tu-usuario/tu-repo/issues)
- Únete al [Discord de Stellar](https://discord.gg/stellardev)

---

**Hecho con ❤️ y 🦀 Rust**