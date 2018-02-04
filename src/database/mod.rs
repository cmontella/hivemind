use spin::Mutex;
use alloc::{BTreeMap, Vec, String};
use alloc::arc::Arc;
use database::transaction::{Transaction, Change, ChangeType};
use drivers::vga::{SCREEN_WRITER};
use x86_64::instructions::rdtsc;
use interrupts::event;
use core::fmt;

pub mod transaction;

// ## Entities

#[derive(Debug)]
pub struct Entity {
  pub id: u64,
  pub attributes: Vec<Attribute>
}

impl Entity {
  pub fn new() -> Entity {
    Entity {
      id: 0,
      attributes: Vec::new(),
    }
  }
}

// ## Attributes

#[derive(Debug)]
pub struct Attribute {
  pub name: String,
  pub display: String,
  pub value: Vec<(u64, u64)>,
}

impl Attribute {
  pub fn new() -> Attribute {
    Attribute {
    name: String::new(),
    display: String::new(),
    value: Vec::new(),
    }
  }
}

// ## Values

#[derive(Clone)]
pub enum Value {
  Null,
  Number(u64),
  Bool(bool),
  String(String),
}

impl Value {

  pub fn from_string(string: String) -> Value {
    Value::String(string)
  }

  pub fn from_str(string: &str) -> Value {
    Value::String(String::from(string))
  }

  pub fn from_int(int: u64) -> Value {
    Value::Number(int)
  }

}

impl fmt::Debug for Value {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
        &Value::Number(ref x) => write!(f, "{}", x),
        &Value::String(ref x) => write!(f, "{}", x),
        &Value::Bool(ref x) => write!(f, "{}", x),
        &Value::Null => write!(f, "Null"),
      }
    }
}

// ## Interner

#[derive(Debug)]
pub struct Interner {
  pub store: Vec<Change>,
}

impl Interner {

  pub fn new() -> Interner {
    Interner {
      store: Vec::new(),
    }
  }

  pub fn intern_change(&mut self, change: &Change) {
    self.store.push(change.clone());
  }
}

// ## Database

#[derive(Debug)]
pub struct Database {
  epoch: u64,
  round: u64,
  transactions: Vec<Transaction>,
  entity_index: BTreeMap<String, Entity>,
  attribute_index: BTreeMap<String, Attribute>,
  store: Interner,
  scanned: usize,
  txn_pointer: usize,
}

impl Database {

  pub fn new() -> Database {
    Database {
      epoch: 0,
      round: 0,
      transactions: Vec::new(),
      entity_index: BTreeMap::new(),
      attribute_index: BTreeMap::new(),
      store: Interner::new(),
      scanned: 0,
      txn_pointer: usize,
    }
  }

  pub fn init(&self) {
    
  }

  pub fn register_transaction(&mut self, transaction: Transaction) {
    self.transactions.push(transaction);
    self.process_transactions();
    self.update_indices();
    self.txn_pointer = self.transactions.len();
    self.epoch = self.epoch + 1;
  }

  fn process_transactions(&mut self) {   
    for txn in self.transactions.iter_mut().skip(self.txn_pointer) {
      if !txn.is_complete() {
        // Handle the adds
        for add in txn.adds.iter() {
            self.store.intern_change(add);
        }
        // Handle the removes
        for remove in txn.removes.iter() {
            self.store.intern_change(remove);
        }
        txn.process();
        txn.epoch = self.epoch;
        txn.round = self.round;
        self.round = self.round + 1;
      }
    }
  }

  fn update_indices(&mut self) {
    for change in self.store.store.iter().skip(self.scanned) {
      match change.kind {
        ChangeType::Add => {
          self.entity_index.insert(change.entity.clone(), Entity::new());          
          self.attribute_index.insert(change.attribute.clone(), Attribute::new());
        },
        ChangeType::Remove => {
          self.entity_index.remove(&change.entity);
        },
      }
    }
    self.scanned = self.store.store.len();
  }

}
 
impl fmt::Debug for Database {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Database:\n--------------------\nEpoch: {:?}\nTransactions: {:?}\nChanges: {:?}\nScanned: {:?}\n--------------------\n", self.epoch, self.transactions.len(), self.store.store.len(), self.scanned)
    }
}


/*
    let entity: &mut Entity = match self.entity_index.get_mut(&change.entity) {
      None => {
        let mut new_entity = Entity::new();
        new_entity.id = change.entity;
        &mut new_entity
      },
      Some(entity) => entity,
    };*/

      
      
      //let attribute = self.attribute_index.get(change.attribute);



  

/*

*/


lazy_static! {
  pub static ref database: Mutex<Database> = Mutex::new(Database::new());
}