wit_bindgen::generate!(in "../wit");

use exports::foo::bar::export_iface;

pub struct WorldImpl;

impl export_iface::ExportIface for WorldImpl {
    fn print() {
        let utc = time::OffsetDateTime::now_utc();
        println!("Hello, world!, {:?}", utc);
    }
}

export_test_world!(WorldImpl);
