## 📝 Message Contract - Soroban Smart Contract

Contrato inteligente simple desarrollado en Rust para la blockchain de Stellar usando Soroban SDK. Permite almacenar y recuperar mensajes con un tiempo de vida (TTL) de 10 minutos.

## 🚀 Características

* **Constructor automático**: Inicializa el contrato con un mensaje predeterminado al desplegarlo
* **Almacenamiento temporal**: Los mensajes expiran automáticamente después de 10 minutos
* **Instance Storage**: Utiliza almacenamiento eficiente ligado a la instancia del contrato
* **Simple y educativo**: Perfecto para aprender los fundamentos de Soroban

**Compilar el contrato**

bash

```plaintext
soroban contract build
```

El archivo WASM compilado se generará en:

```plaintext
target/wasm32v1-none/release/message_contract.wasm
```

## 📦 Despliegue

### En Testnet (Recomendado para pruebas)

```plaintext
stellar contract deploy \ 
--wasm target/wasm32v1-none/release/et_set_message.wasm \ 
--source <identity> \ 
--network testnet \
--alias get_set_message
```

## 🎯 Uso del Contrato

### 1. Establecer un Mensaje

bash

```plaintext
stellar contract invoke \  
--id get_set_message \   
--source <identity> \
--network testnet \
-- \
set_message \
--message "Hola desde Soroban 🌟"
```

### 2. Obtener el Mensaje

bash

```plaintext
stellar contract invoke \  
--id  get_set_message \
--source <identity> \
--network testnet \
-- \ 
get_message
```

**Respuesta esperada:**

```plaintext
"Hola desde Soroban 🌟"
```

### 3. Probar la Expiración del TTL

Espera aproximadamente 10 minutos y vuelve a consultar:

bash

```plaintext
stellar contract invoke \  
--id get_set_message \  
--source <identity> \ 
--network testnet \   
-- \ 
get_message
```

**Respuesta después de expiración:**

```plaintext
"Default Message"
```
