use spin::Mutex;
use alloc::BTreeMap;

pub struct Database {
  pub store: BTreeMap<u8, u8>,
}

impl Database {

  pub fn new() -> Database {
    Database {
      store: BTreeMap::new(),
    }
  }

  pub fn init(&self) {
    
  }

}

lazy_static! {
  pub static ref database: Mutex<Database> = Mutex::new(Database::new());
}