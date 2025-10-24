# 🌟 Código Futura - Clase 4: Soroban Rust SDK 🌟
# Hello Tiburona - Contrato Soroban 🦈

¡Bienvenidas a mi proyecto Soroban! Este es mi contrato `hello-tiburona`, creado para aprender sobre blockchain y contratos inteligentes en la red Stellar. Este contrato saluda con **"Hola Tiburona"** y gestiona contadores de saludos con un toque personal. 🚀

![Rust](https://img.shields.io/badge/Rust-000000?style=flat-square&logo=rust&logoColor=white)
![Soroban](https://img.shields.io/badge/Soroban-Stellar-blue?style=flat-square)
![Código Futura](https://img.shields.io/badge/Código_Futura-2025-green?style=flat-square)

## 📖 Descripción

Este contrato es parte de mis proyectos en Código Futura. Permite:
- Inicializar un administrador.
- Saludar con "Hola Tiburona" usando un nombre con límite de caracteres configurable.
- Contar saludos totales y por usuario.
- Restringir el reseteo del contador y la configuración del límite al administrador.
- Transferir la administración a una nueva dirección.

  ## 🛠️ Tecnologías Usadas
- **Rust** + **Soroban SDK 23.0.3**
- **Stellar CLI**
- **Stellar Testnet**
- **Cargo** para gestión de dependencias
- **WASM** para compilación de contratos

## 📚 Recursos Consultados
- [Soroban Rust SDK](https://soroban.stellar.org/docs/sdks/rust)
- [Stellar CLI](https://soroban.stellar.org/docs/reference/cli)
- [Soroban Docs](https://soroban.stellar.org/docs)
- [Stellar Laboratory](https://laboratory.stellar.org)
- [StellarExpert](https://stellar.expert)
- [Friendbot](https://friendbot.stellar.org) - Mi amigo para pruebas en *testnet*
- [Documentación oficial de Rust](https://www.rust-lang.org)
- Guía de la Clase 4: `04-hello-tiburona.md`, `02-result-option.md`, `03-storage.md`

## 🛠️ Instalación

1. Clona el repositorio:
   ```bash
   git clone https://github.com/tu-usuario/proyectos-soroban.git
   cd proyectos-soroban/hello-tiburona
   ```
2. Compila el contrato:
   ```bash
   soroban contract build
   ```
3. Ejecuta los tests:
   ```bash
   cargo test -- --lib
   ```

## 🚀 Uso

El contrato está implementado en Rust con Soroban SDK 23.0.3. Genera un archivo WASM en `target/wasm32-unknown-unknown/release/hello-tiburona.wasm` para desplegar en la red Stellar.

## 📋 Requisitos Cumplidos

- **Implementación**: 5 errores personalizados, 5 claves de almacenamiento, funciones `initialize`, `hello`, `get_contador`, `get_ultimo_saludo`, `get_contador_usuario`, `reset_contador`, `transfer_admin`, `set_limite`.
- **Tests**: 11 tests para inicialización, no reinicialización, saludos exitosos, validaciones, control de acceso, transferencia de admin, y límite configurable.
- **Build**: Compila sin warnings, tests pasan, WASM generado y optimizado.
- **Retos adicionales**:
  - Estadísticas por usuario con `ContadorPorUsuario` y `get_contador_usuario`.
  - Transferencia de admin con `transfer_admin`.
  - Límite de caracteres configurable con `set_limite` y `LimiteCaracteres`.
  

## 🎯 Lo que Aprendí

### Rust + Soroban ✨
- Crear contratos inteligentes profesionales con **Soroban SDK**.
- Manejar almacenamiento **instance** y **persistent** con `DataKey`.
- Implementar **errores personalizados** y validaciones robustas.
- Gestionar **TTL** para optimizar el almacenamiento.
- Escribir **tests comprehensivos** para casos exitosos y de error.

### Stellar CLI 🖥️
- Compilar y optimizar archivos **WASM** para despliegue.
- Invocar contratos localmente en *testnet*.
- Automatizar tareas con comandos de **Soroban CLI**.
- Navegar la documentación como una detective.

### Pensamiento Crítico 🧠
- Diseñar contratos seguros con **control de acceso**.
- Depurar errores en el código (¡gracias por la corrección del bug de `Symbol`, Karen! 🦈).
- Escribir documentación clara y profesional (¡como este README!).

### Confianza 💪
- Creer en mi capacidad para dominar **Rust** y **blockchain**.
- Construir un contrato *production-ready* desde cero.
- Visualizar mi futuro como **developer blockchain**.

---

## 📂 ¿Qué vas a encontrar?
- **`src/lib.rs`** 🦀: Contrato Soroban que saluda con "Hola Tiburona", gestiona contadores totales y por usuario, permite transferir admin, y configura un límite de caracteres. Incluye **5 errores personalizados** y **5 claves de almacenamiento**.
- **`Cargo.toml`** ⚙️: Configuración del proyecto para generar `hello-tiburona.wasm`.
- **`README.md`** 📖: Esta documentación con todos los detalles del proyecto.

---

## 🚀 Instalación y Uso

1. **Clona el repositorio**:
   ```bash
   git clone https://github.com/Valortmon/Tarea-de-CODIGO-FUTURA.git
   cd Tarea-de-CODIGO-FUTURA/hello-tiburona
   ```

2. **Compila el contrato**:
   ```bash
   soroban contract build
   ```

3. **Ejecuta los tests**:
   ```bash
   cargo test -- --lib
   ```

4. **Optimiza el WASM**:
   ```bash
   soroban contract optimize --wasm target/wasm32-unknown-unknown/release/hello-tiburona.wasm
   ```

5. **Prueba localmente**:
   ```bash
   soroban contract invoke \
     --wasm target/wasm32-unknown-unknown/release/hello-tiburona.wasm \
     --id test \
     -- initialize \
     --admin CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAD2KM
   ```

   ```bash
   soroban contract invoke \
     --wasm target/wasm32-unknown-unknown/release/hello-tiburona.wasm \
     --id test \
     -- hello \
     --usuario CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAHK3M \
     --nombre "Test"
   ```

---

## 📋 Requisitos Cumplidos

### Implementación
- **5 errores personalizados**: `NombreVacio`, `NombreMuyLargo`, `NoAutorizado`, `NoInicializado`, `YaInicializado`.
- **5 claves de almacenamiento**: `Admin`, `ContadorSaludos`, `UltimoSaludo(Address)`, `ContadorPorUsuario(Address)`, `LimiteCaracteres`.
- **Funciones**:
  - `initialize`: Inicializa el contrato.
  - `hello`: Retorna "Hola Tiburona".
  - `get_contador`: Retorna el contador total de saludos.
  - `get_ultimo_saludo`: Retorna el último saludo por usuario.
  - `get_contador_usuario`: Retorna el contador de saludos por usuario.
  - `reset_contador`: Reinicia el contador total.
  - `transfer_admin`: Transfiere el rol de admin.
  - `set_limite`: Configura el límite de caracteres.
- **Validaciones**:
  - Nombre no vacío.
  - Límite de caracteres configurable.
  - Control de acceso para funciones restringidas.
- **Manejo de TTL** en almacenamiento *instance* y *persistent*.

### Retos Adicionales
- **Reto 1**: Estadísticas por usuario con `ContadorPorUsuario` y `get_contador_usuario`.
- **Reto 2**: Transferencia de admin con `transfer_admin`.
- **Reto 3**: Límite de caracteres configurable con `set_limite` y `LimiteCaracteres`.

### Tests
- **11 tests** que verifican:
  - Inicialización exitosa y no reinicialización.
  - Saludos exitosos y validaciones de nombre.
  - Control de acceso para funciones restringidas.
  - Transferencia de admin.
  - Límite de caracteres configurable.

### Build
- Compila sin *warnings*.
- Genera `hello-tiburona.wasm`.
- Optimiza el archivo WASM para despliegue.

### Corrección de Bug
- Uso de `String` en lugar de `Symbol` para el parámetro `nombre` en `hello()` y tests, solucionando el problema con `.len()` y `.to_string()`.

---

## 🦈 Manténganse atentas!
Más proyectos blockchain vendrán pronto! ✨
