use harsh::Harsh;

pub struct Hasher {
    generator: Harsh,
    id: u64,
}

impl Hasher {
    #[allow(dead_code)]
    pub fn new() -> Hasher {
        let harsh = Harsh::default();
        Hasher {
            generator: harsh,
            id: 0, // in practice, should be replaced with a unique id Generator
        }
    }

    #[allow(dead_code)]
    pub fn generate(&mut self) -> String {
        let value = self.generator.encode(&[self.id]);
        self.id += 1;
        value
    }
}

#[test]
fn test_hasher() {
    let mut engine = Hasher::new();
    let id = engine.generate();
    println!("{}", id);
}
