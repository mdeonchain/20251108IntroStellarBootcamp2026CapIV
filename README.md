# 🌟 Intro Stellar Bootcamp 2026 — Capítulo IV 🚀

**Repositorio:** ejemplos prácticos de un contrato Soroban (Stellar) y clientes que realizan la operación básica **get / set** desde varios entornos 💫

👉 Este repo contiene un **contrato Soroban** y ejemplos que lo ejecutan desde **JavaScript**, **Python**, **Rust** y una **aplicación React**. Cada uno tiene su propio `README.md` con explicaciones y pasos detallados.

---

## 🧰 Requisitos

- 💻 Visual Studio Code (opcional, pero ideal para Codespaces)
- 🌀 Git
- ☁️ GitHub Codespaces o entorno UNIX-like (bash)
- 🧩 Herramientas: Node.js, npm/yarn, Python, Cargo/Rustup, Soroban CLI

> ⚙️ El script `/scripts/install.sh` instala todo lo necesario, pero cada ejemplo tiene su propio README con instrucciones específicas.

---

## 🚀 Instalación rápida (en GitHub Codespaces)

1. Abre este repositorio en un Codespace (o clónalo localmente 🧑‍💻)
2. Abre la terminal integrada y ejecuta:

```bash
bash /scripts/install.sh
```

3. Cuando termine, revisa los `README.md` dentro de las carpetas de ejemplos para aprender a compilar, desplegar y ejecutar el contrato desde cada cliente 🌈

---

## 🗂️ Estructura del repositorio

```
/ (root)
├─ /contracts/                 # Contrato Soroban ⚙️
├─ /laboratorio-de-stellar/    # Ejemplos get/set desde distintos entornos 🧪
│   ├─ /js/                    # Cliente JavaScript 💛
│   ├─ /python/                # Cliente Python 🐍
│   ├─ /rust/                  # Cliente Rust 🦀
│   └─ /react/                 # App React con UI 💻
├─ /scripts/                   # Scripts útiles (install.sh, etc.) 🔧
└─ README.md                   # Este documento 📖
```

> 📚 Cada subcarpeta tiene su propio `README.md` con instrucciones paso a paso.

---

## 🧪 Cómo ejecutar un ejemplo

1. Ejecuta:
   ```bash
   bash /scripts/install.sh
   ```
2. Entra al ejemplo que quieras probar:
   ```bash
   cd laboratorio-de-stellar/js
   less README.md
   ```
3. Sigue los pasos del `README.md` para:
   - ⚙️ Configurar variables de entorno
   - 🏗️ Compilar y desplegar el contrato Soroban
   - 📦 Instalar dependencias (`npm install`, `pip install`, `cargo build`…)
   - 🧠 Ejecutar el flujo `set` ➡️ `get`

---

## 💡 Qué hace cada ejemplo

- **JavaScript (Node.js):** cliente rápido y práctico 🌐
- **Python:** ejemplo claro y didáctico 🐍
- **Rust:** interacción desde backend de alto rendimiento 🦀
- **React:** interfaz visual para probar el contrato en tiempo real 💻

Cada uno demuestra cómo almacenar y recuperar datos desde el contrato Soroban.

---

## 🪄 Consejos útiles

- 📖 Lee siempre el `README.md` dentro de cada carpeta antes de ejecutar comandos.
- 🪐 Usa la red *testnet* para pruebas seguras.
- 🔄 Asegúrate de tener la versión correcta del Soroban CLI.

---

## 🤝 Contribuir

¿Quieres aportar mejoras o nuevos ejemplos? ¡Perfecto! 💬
- Abre un *pull request* con tu cambio ✨
- O crea un *issue* para discutir ideas 🧭

---

## ⚖️ Licencia

📜 Repositorio de código abierto (ver `LICENSE` si existe).  
Si no hay archivo de licencia, contacta al autor antes de usarlo comercialmente.

---

🌌 ¡Gracias por ser parte del **Stellar Bootcamp 2026**!  
Construyamos juntos el futuro en la red de las estrellas 💫

