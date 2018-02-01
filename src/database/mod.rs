use spin::Mutex;
use alloc::{BTreeMap, Vec, String};
use database::transaction::{Transaction, Value};
use drivers::vga::{SCREEN_WRITER};

pub mod transaction;


pub struct Database {
  transactions: Vec<Transaction>,
  pub store: BTreeMap<String, Value>,
}

impl Database {

  pub fn new() -> Database {
    Database {
      transactions: Vec::new(),
      store: BTreeMap::new(),
    }
  }

  pub fn init(&self) {
    
  }

  pub fn insert_transaction(&mut self, transaction: Transaction) {
    self.transactions.push(transaction);
    self.process_transactions();
  }

  fn process_transactions(&mut self) {

    for txn in self.transactions.iter_mut() {
      if !txn.is_complete() {
        // Handle the adds
        for add in txn.adds.iter() {
          let id = format!("{:?}|{:?}|{:?}", add.entity, add.attribute, add.value);
          let value = add.value.clone();
          self.store.insert(String::from(id), value);
        }
        // Handle the removes
        for remove in txn.removes.iter() {
          let id = format!("{:?}|{:?}|{:?}", remove.entity, remove.attribute, remove.value);
          self.store.remove(&String::from(id));
        }
        txn.process();
      }
    }
    SCREEN_WRITER.lock().clear();
    for (key, val) in &self.store {
        println!("{:?}: {:?}", key, val);
    }


    
  }
}

lazy_static! {
  pub static ref database: Mutex<Database> = Mutex::new(Database::new());
}