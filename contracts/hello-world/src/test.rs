#![cfg(test)]

use super::*;
use soroban_sdk::{Env, String, Symbol};

#[test]
fn test_user_operations() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    // Insertar usuario
    client.insert_user(
        &String::from_str(&env, "Alice"),
        &String::from_str(&env, "alice@example.com"),
        &String::from_str(&env, "1234"),
    );

    // Verificar inserción
    let user = client.get_user(&String::from_str(&env, "Alice"));
    assert!(user.is_some());
    let user_data = user.unwrap();
    assert_eq!(
        user_data.get(Symbol::short("email")),
        Some(String::from_str(&env, "alice@example.com"))
    );
    assert_eq!(
        user_data.get(Symbol::short("password")),
        Some(String::from_str(&env, "1234"))
    );

    // Editar usuario
    client.edit_user(
        &String::from_str(&env, "Alice"),
        &String::from_str(&env, "alice_new@example.com"),
        &String::from_str(&env, "5678"),
    );

    // Verificar edición
    let updated_user = client.get_user(&String::from_str(&env, "Alice"));
    assert!(updated_user.is_some());
    let updated_data = updated_user.unwrap();
    assert_eq!(
        updated_data.get(Symbol::short("email")),
        Some(String::from_str(&env, "alice_new@example.com"))
    );
    assert_eq!(
        updated_data.get(Symbol::short("password")),
        Some(String::from_str(&env, "5678"))
    );

    // Eliminar usuario
    client.delete_user(&String::from_str(&env, "Alice"));

    // Verificar eliminación
    let deleted_user = client.get_user(&String::from_str(&env, "Alice"));
    assert!(deleted_user.is_none());
}
