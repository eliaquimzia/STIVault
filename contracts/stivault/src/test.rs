#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{Env, Symbol, Address};

    #[test]
    fn test_register_success() {
        let env = Env::default();
        let contract_id = env.register_contract(None, PaperVault);
        let client = PaperVaultClient::new(&env, &contract_id);

        let hash = Symbol::short("paper1");
        let owner = Address::random(&env);

        client.register_paper(&hash, &owner);
        assert!(client.verify_paper(&hash));
    }

    #[test]
    #[should_panic]
    fn test_duplicate_fail() {
        let env = Env::default();
        let contract_id = env.register_contract(None, PaperVault);
        let client = PaperVaultClient::new(&env, &contract_id);

        let hash = Symbol::short("paper1");
        let owner = Address::random(&env);

        client.register_paper(&hash, &owner);
        client.register_paper(&hash, &owner);
    }

    #[test]
    fn test_state_correct() {
        let env = Env::default();
        let contract_id = env.register_contract(None, PaperVault);
        let client = PaperVaultClient::new(&env, &contract_id);

        let hash = Symbol::short("paper1");
        let owner = Address::random(&env);

        client.register_paper(&hash, &owner);
        assert_eq!(client.get_owner(&hash), owner);
    }

    #[test]
    fn test_verify_false() {
        let env = Env::default();
        let contract_id = env.register_contract(None, PaperVault);
        let client = PaperVaultClient::new(&env, &contract_id);

        let hash = Symbol::short("unknown");
        assert!(!client.verify_paper(&hash));
    }

    #[test]
    fn test_multiple_papers() {
        let env = Env::default();
        let contract_id = env.register_contract(None, PaperVault);
        let client = PaperVaultClient::new(&env, &contract_id);

        let h1 = Symbol::short("p1");
        let h2 = Symbol::short("p2");
        let owner = Address::random(&env);

        client.register_paper(&h1, &owner);
        client.register_paper(&h2, &owner);

        assert!(client.verify_paper(&h1));
        assert!(client.verify_paper(&h2));
    }
}


