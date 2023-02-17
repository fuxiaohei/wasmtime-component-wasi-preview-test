wit_bindgen::generate!("world" in "../wit");

pub struct WorldImpl;

impl exports::Exports for WorldImpl {
    fn print() {
        println!("Hello, world!");
    }
}

export_test_world!(WorldImpl);