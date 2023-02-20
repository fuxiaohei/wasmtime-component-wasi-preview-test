wit_bindgen::generate!("world" in "../wit");

pub struct WorldImpl;

impl exports::Exports for WorldImpl {
    fn print() {
        let utc = time::OffsetDateTime::now_utc();
        println!("Hello, world!, {:?}", utc);
    }
}

export_test_world!(WorldImpl);
