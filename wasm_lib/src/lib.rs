wit_bindgen::generate!(in "../wit");

use exports::foo::bar::export_iface;

pub struct WorldImpl;

impl export_iface::ExportIface for WorldImpl {
    fn print(args1: export_iface::Arg1) {
        let utc = time::OffsetDateTime::now_utc();
        println!("Hello, world!, {:?}", utc);
        println!("args1.a: {}, args1.b: {}", args1.a, args1.b)
    }
}

export_test_world!(WorldImpl);
