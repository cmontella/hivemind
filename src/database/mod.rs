use spin::Mutex;
use alloc::{BTreeMap, Vec, String};
use alloc::arc::Arc;
use database::transaction::{Transaction, Value, Change, ChangeType};
use drivers::vga::{SCREEN_WRITER};
use x86_64::instructions::rdtsc;
use interrupts::event;

pub mod transaction;

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

#[derive(Debug)]
pub struct Database {
  epoch: u64,
  round: u64,
  transactions: Vec<Transaction>,
  entity_index: BTreeMap<String, Entity>,
  attribute_index: BTreeMap<String, Attribute>,
  store: Interner,
  scanned: usize,
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
    }
  }

  pub fn init(&self) {
    
  }

  pub fn register_transaction(&mut self, transaction: Transaction) {
    self.transactions.push(transaction);
    self.process_transactions();
    self.update_indices();
    self.epoch = self.epoch + 1;
  }

  fn process_transactions(&mut self) {   
    for txn in self.transactions.iter_mut() {
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