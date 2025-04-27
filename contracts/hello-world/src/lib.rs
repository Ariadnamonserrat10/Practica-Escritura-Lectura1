#![no_std]

use soroban_sdk::{contract, contractimpl, map, Env, String, Map, Symbol};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn insert_user(env: Env, name: String, email: String, password: String) {
        let mut users: Map<String, Map<Symbol, String>> = env
            .storage()
            .persistent()
            .get(&Symbol::short("users"))
            .unwrap_or_else(|| Map::new(&env));

        let user_data = map![
            &env,
            (Symbol::short("email"), email),
            (Symbol::short("password"), password),
        ];

        users.set(name.clone(), user_data);
        env.storage().persistent().set(&Symbol::short("users"), &users);
    }

    pub fn edit_user(env: Env, name: String, new_email: String, new_password: String) {
        let mut users: Map<String, Map<Symbol, String>> = env
            .storage()
            .persistent()
            .get(&Symbol::short("users"))
            .unwrap_or_else(|| Map::new(&env));

        if let Some(mut user_data) = users.get(name.clone()) {
            user_data.set(Symbol::short("email"), new_email);
            user_data.set(Symbol::short("password"), new_password);
            users.set(name.clone(), user_data);
            env.storage().persistent().set(&Symbol::short("users"), &users);
        }
    }

    pub fn delete_user(env: Env, name: String) {
        let mut users: Map<String, Map<Symbol, String>> = env
            .storage()
            .persistent()
            .get(&Symbol::short("users"))
            .unwrap_or_else(|| Map::new(&env));

        users.remove(name.clone());
        env.storage().persistent().set(&Symbol::short("users"), &users);
    }

    pub fn get_user(env: Env, name: String) -> Option<Map<Symbol, String>> {
        let users: Map<String, Map<Symbol, String>> = env
            .storage()
            .persistent()
            .get(&Symbol::short("users"))
            .unwrap_or_else(|| Map::new(&env));

        users.get(name)
    }
}

mod test;
