// # Transactions

use alloc::{String, Vec};
use interrupts::event;
use core::fmt;

/*
Transactions are units of atomic updates to the DB.
*/


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
        &Value::Number(ref x) => write!(f, "{:?}", x),
        &Value::String(ref x) => write!(f, "{:?}", x),
        &Value::Bool(ref x) => write!(f, "{:?}", x),
        &Value::Null => write!(f, "Null"),
      }
    }
}

// ## Changes

#[derive(Debug)]
pub enum ChangeType {
  Add,
  Remove,
}

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

impl fmt::Debug for Change {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: [{:?} {:?}: {:?}]", self.kind, self.entity, self.attribute, self.value)
    }
}


// ## Transactions

pub struct Transaction {
  pub timestamp: u64,
  pub complete: u64,
  pub adds: Vec<Change>,
  pub removes: Vec<Change>,
}

impl Transaction {
  pub fn new() -> Transaction {
    Transaction {
      timestamp: event::update_time(),
      complete: 0,
      adds: Vec::new(),
      removes: Vec::new(),
    }
  }

  pub fn process(&mut self) -> u64 {
    if self.complete == 0 {
      self.complete = event::update_time();
    }
    self.complete
  }

  pub fn is_complete(&self) -> bool {
    self.complete != 0
  }
}

impl fmt::Debug for Transaction {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: Add: {:?}  Remove: {:?}", self.timestamp, self.adds.len(), self.removes.len())
    }
}