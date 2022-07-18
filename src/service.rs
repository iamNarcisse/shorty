use crate::hasher::Hasher;
use crate::store::Store;

pub struct Service {
    store: Store,
    hasher: Hasher,
}

impl Service {
    /// Creates a new service.
    pub fn new() -> Service {
        let store = Store::new();
        match store {
            Ok(store) => Service {
                store,
                hasher: Hasher::new(),
            },
            Err(e) => panic!("Failed to initialize store: {}", e),
        }
    }
    pub fn store(&mut self, link: &str) -> String {
        let key = self.hasher.generate(); // Generator generates unique ids
        let result = self.store.save(&key, &link);
        match result {
            Ok(_) => key, // Return key
            Err(e) => panic!("Failed to save link: {}", e),
        }
    }

    pub fn retrieve(&mut self, id: &str) -> Option<String> {
        let result = self.store.retrieve(&id);
        match result {
            Some(value) => Some(value),
            None => None,
        }
    }
}

#[test]
fn test_service() {
    let mut srv = Service::new();
    let id = srv.store("https://www.rust-lang.org");
    let result = srv.retrieve("gY");
    println!("{:?}, {:?}", id, result);
}
