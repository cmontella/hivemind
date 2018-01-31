// # Transactions

use alloc::{String, Vec};
use interrupts::event;


/*
Transactions are units of atomic updates to the DB.
*/

#[derive(Debug)]
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

#[derive(Debug)]
pub enum ChangeType {
  Add,
  Remove,
}

#[derive(Debug)]
pub struct Change {
  pub kind: ChangeType,
  pub entity: String,
  pub attribute: String,
  pub value: Value,
}

impl Change {
  pub fn new() -> Change {
    Change {
      kind: ChangeType::Add,
      entity: String::new(),
      attribute: String::new(),
      value: Value::Null,
    }
  }

  pub fn from_eav(entity: &str, attribute: &str, value: Value) -> Change {
    let e = String::from(entity);
    let a = String::from(attribute);
    Change {
      kind: ChangeType::Add,
      entity: e,
      attribute: a,
      value: value,
    }
  }

  pub fn new_add(value: Value) -> Change {
    let mut change = Change::new();
    change.value = value;
    change
  }

  pub fn new_remove(value: Value) -> Change {
    let mut change = Change::new();
    change.kind = ChangeType::Remove;
    change.value = value;
    change
  }
}

#[derive(Debug)]
pub struct Transaction {
  timestamp: u64,
  complete: bool,
  pub adds: Vec<Change>,
  pub removes: Vec<Change>,

}

impl Transaction {
  pub fn new() -> Transaction {
    Transaction {
      timestamp: event::update_time(),
      complete: false,
      adds: Vec::new(),
      removes: Vec::new(),
    }
  }
}

