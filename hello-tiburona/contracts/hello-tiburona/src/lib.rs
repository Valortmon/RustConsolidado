#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracterror, contracttype,
    Env, Address, String
};

// Definir errores
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    NombreVacio = 1,
    NombreMuyLargo = 2,
    NoAutorizado = 3,
    NoInicializado = 4,
    YaInicializado = 5,
}

// Definir claves de almacenamiento
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    ContadorSaludos,
    UltimoSaludo(Address),
    ContadorPorUsuario(Address),
    LimiteCaracteres,
}

// Definir contrato
#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    // Inicializar
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::YaInicializado);
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::ContadorSaludos, &0u32);
        env.storage().instance().set(&DataKey::LimiteCaracteres, &32u32);
        env.storage().instance().extend_ttl(100, 100);
        Ok(())
    }

    // Saludar
    pub fn hello(
        env: Env,
        usuario: Address,
        nombre: String
    ) -> Result<String, Error> {
        usuario.require_auth();
        if nombre.len() == 0 {
            return Err(Error::NombreVacio);
        }
        let limite: u32 = env.storage().instance().get(&DataKey::LimiteCaracteres).unwrap_or(32);
        if nombre.len() > limite {
            return Err(Error::NombreMuyLargo);
        }
        let key_contador = DataKey::ContadorSaludos;
        let contador: u32 = env.storage().instance().get(&key_contador).unwrap_or(0);
        env.storage().instance().set(&key_contador, &(contador + 1));
        
        let key_usuario = DataKey::ContadorPorUsuario(usuario.clone());
        let contador_usuario: u32 = env.storage().persistent().get(&key_usuario).unwrap_or(0);
        env.storage().persistent().set(&key_usuario, &(contador_usuario + 1));
        
        env.storage().persistent().set(&DataKey::UltimoSaludo(usuario.clone()), &nombre);
        env.storage().persistent().extend_ttl(&DataKey::UltimoSaludo(usuario.clone()), 100, 100);
        env.storage().persistent().extend_ttl(&key_usuario, 100, 100);
        env.storage().instance().extend_ttl(100, 100);
        Ok(String::from_str(&env, "Hola Tiburona"))
    }

    // Obtener contador total
    pub fn get_contador(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::ContadorSaludos).unwrap_or(0)
    }

    // Obtener último saludo
    pub fn get_ultimo_saludo(env: Env, usuario: Address) -> Option<String> {
        env.storage().persistent().get(&DataKey::UltimoSaludo(usuario))
    }

    // Obtener contador por usuario
    pub fn get_contador_usuario(env: Env, usuario: Address) -> u32 {
        env.storage().persistent().get(&DataKey::ContadorPorUsuario(usuario)).unwrap_or(0)
    }

    // Reiniciar contador
    pub fn reset_contador(env: Env, caller: Address) -> Result<(), Error> {
        caller.require_auth();
        let admin: Address = env.storage().instance().get(&DataKey::Admin).ok_or(Error::NoInicializado)?;
        if caller != admin {
            return Err(Error::NoAutorizado);
        }
        env.storage().instance().set(&DataKey::ContadorSaludos, &0u32);
        Ok(())
    }

    // Transferir admin
    pub fn transfer_admin(env: Env, caller: Address, nuevo_admin: Address) -> Result<(), Error> {
        caller.require_auth();
        let admin: Address = env.storage().instance().get(&DataKey::Admin).ok_or(Error::NoInicializado)?;
        if caller != admin {
            return Err(Error::NoAutorizado);
        }
        env.storage().instance().set(&DataKey::Admin, &nuevo_admin);
        env.storage().instance().extend_ttl(100, 100);
        Ok(())
    }

    // Configurar límite de caracteres
    pub fn set_limite(env: Env, caller: Address, limite: u32) -> Result<(), Error> {
        caller.require_auth();
        let admin: Address = env.storage().instance().get(&DataKey::Admin).ok_or(Error::NoInicializado)?;
        if caller != admin {
            return Err(Error::NoAutorizado);
        }
        env.storage().instance().set(&DataKey::LimiteCaracteres, &limite);
        env.storage().instance().extend_ttl(100, 100);
        Ok(())
    }
}

// Pruebas
#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{Env, testutils::Address as _};

    // Inicialización
    #[test]
    fn test_initialize() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        
        client.initialize(&admin);
        assert_eq!(client.get_contador(), 0);
    }

    // No reinicializar
    #[test]
    #[should_panic(expected = "HostError: Error(Contract, #5)")]
    fn test_no_reinicializar() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        
        client.initialize(&admin);
        client.initialize(&admin);
    }

    // Saludo exitoso
    #[test]
    fn test_hello_exitoso() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let usuario = Address::generate(&env);
        
        client.initialize(&admin);
        
        let nombre = String::from_str(&env, "Test");
        let resultado = client.hello(&usuario, &nombre);
        
        assert_eq!(resultado, String::from_str(&env, "Hola Tiburona"));
        assert_eq!(client.get_contador(), 1);
        assert_eq!(client.get_ultimo_saludo(&usuario), Some(nombre));
        assert_eq!(client.get_contador_usuario(&usuario), 1);
    }

    // Nombre vacío
    #[test]
    #[should_panic(expected = "HostError: Error(Contract, #1)")]
    fn test_nombre_vacio() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let usuario = Address::generate(&env);
        
        client.initialize(&admin);
        
        let vacio = String::from_str(&env, "");
        client.hello(&usuario, &vacio);
    }

    // Nombre muy largo
    #[test]
    #[should_panic(expected = "HostError: Error(Contract, #2)")]
    fn test_nombre_muy_largo() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let usuario = Address::generate(&env);
        
        client.initialize(&admin);
        
        let largo = String::from_str(&env, "abcdefghijklmnopqrstuvwxyz0123456789");
        client.hello(&usuario, &largo);
    }

    // Reinicio por admin
    #[test]
    fn test_reset_solo_admin() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let usuario = Address::generate(&env);
        
        client.initialize(&admin);
        
        client.hello(&usuario, &String::from_str(&env, "Test"));
        assert_eq!(client.get_contador(), 1);
        
        client.reset_contador(&admin);
        assert_eq!(client.get_contador(), 0);
    }

    // Reinicio no autorizado
    #[test]
    #[should_panic(expected = "HostError: Error(Contract, #3)")]
    fn test_reset_no_autorizado() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let otro = Address::generate(&env);
        
        client.initialize(&admin);
        
        client.reset_contador(&otro);
    }

    // Contador por usuario
    #[test]
    fn test_contador_usuario() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let usuario = Address::generate(&env);
        
        client.initialize(&admin);
        
        let nombre = String::from_str(&env, "Test");
        client.hello(&usuario, &nombre);
        client.hello(&usuario, &nombre);
        
        assert_eq!(client.get_contador(), 2);
        assert_eq!(client.get_contador_usuario(&usuario), 2);
    }

    // Transferencia de admin
    #[test]
    fn test_transfer_admin() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let nuevo_admin = Address::generate(&env);
        
        client.initialize(&admin);
        client.transfer_admin(&admin, &nuevo_admin);
        
        client.reset_contador(&nuevo_admin);
        assert_eq!(client.get_contador(), 0);
    }

    // Transferencia no autorizada
    #[test]
    #[should_panic(expected = "HostError: Error(Contract, #3)")]
    fn test_transfer_admin_no_autorizado() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let no_admin = Address::generate(&env);
        let nuevo_admin = Address::generate(&env);
        
        client.initialize(&admin);
        client.transfer_admin(&no_admin, &nuevo_admin);
    }

    // Configuración de límite
    #[test]
    fn test_set_limite() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let usuario = Address::generate(&env);
        
        client.initialize(&admin);
        client.set_limite(&admin, &10);
        
        let nombre_largo = String::from_str(&env, "abcdefghijk");
        let result = client.try_hello(&usuario, &nombre_largo);
        assert!(result.is_err());
        
        let nombre_ok = String::from_str(&env, "Test");
        let resultado = client.hello(&usuario, &nombre_ok);
        assert_eq!(resultado, String::from_str(&env, "Hola Tiburona"));
    }

    // Configuración de límite no autorizada
    #[test]
    #[should_panic(expected = "HostError: Error(Contract, #3)")]
    fn test_set_limite_no_autorizado() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let no_admin = Address::generate(&env);
        
        client.initialize(&admin);
        client.set_limite(&no_admin, &10);
    }
}