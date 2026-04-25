#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, Address, Map};

#[contract]
pub struct PaperVault;

#[contractimpl]
impl PaperVault {

    // Register a research paper (hash + owner)
    pub fn register_paper(env: Env, hash: Symbol, owner: Address) {
        let mut storage: Map<Symbol, Address> = env.storage().instance().get(&Symbol::short("papers")).unwrap_or(Map::new(&env));

        // Prevent duplicate registration
        if storage.contains_key(hash.clone()) {
            panic!("Paper already exists");
        }

        storage.set(hash.clone(), owner.clone());
        env.storage().instance().set(&Symbol::short("papers"), &storage);
    }

    // Verify if paper exists and is valid
    pub fn verify_paper(env: Env, hash: Symbol) -> bool {
        let storage: Map<Symbol, Address> = env.storage().instance().get(&Symbol::short("papers")).unwrap_or(Map::new(&env));

        storage.contains_key(hash)
    }

    // Get owner of paper
    pub fn get_owner(env: Env, hash: Symbol) -> Address {
        let storage: Map<Symbol, Address> = env.storage().instance().get(&Symbol::short("papers")).unwrap();

        storage.get(hash).unwrap()
    }
}

