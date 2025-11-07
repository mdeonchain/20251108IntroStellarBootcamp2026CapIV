#!/bin/bash
set -e

echo "=== Script de DESINSTALACIÓN de Rust y Stellar CLI en Ubuntu ==="
echo "Este proceso eliminará Rust, Stellar CLI y sus configuraciones."
read -p "¿Deseas continuar? (s/N): " confirm
if [[ ! "$confirm" =~ ^[sS]$ ]]; then
    echo "Operación cancelada."
    exit 0
fi

# 1. Eliminar Stellar CLI
if command -v stellar &>/dev/null; then
    echo "Eliminando Stellar CLI..."
    sudo rm -f /usr/local/bin/stellar
    echo "Stellar CLI eliminado."
else
    echo "Stellar CLI no está instalado."
fi

# 2. Eliminar Rust y cargo
if command -v rustup &>/dev/null; then
    echo "Eliminando Rust y cargo..."
    rustup self uninstall -y
    echo "Rust y cargo eliminados."
else
    echo "Rust no está instalado."
fi

# 3. Eliminar dependencias opcionales instaladas (curl y build-essential)
read -p "¿Deseas eliminar también las dependencias (curl, build-essential)? (s/N): " deps
if [[ "$deps" =~ ^[sS]$ ]]; then
    sudo apt-get remove --purge -y curl build-essential
    sudo apt-get autoremove -y
    sudo apt-get clean
    echo "Dependencias eliminadas."
else
    echo "Dependencias conservadas."
fi

# 4. Eliminar posibles restos de configuraciones
echo "Eliminando rutas de Rust del .bashrc..."
sed -i '/\.cargo\/bin/d' ~/.bashrc

echo "=== Desinstalación completada ===" 
