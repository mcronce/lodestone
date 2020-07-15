use failure::Fail;
use std::collections::HashMap;
use std::collections::hash_map::Iter;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Attribute {
    pub level: u16
}

/// Holds information about a profiles level in a particular class.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Attributes(HashMap<String, Attribute>);

impl Attributes {
    pub fn new() -> Self {
        Self(HashMap::with_capacity(18))
    }
    /// Adds or updates a given entry.
    pub fn insert(&mut self, name: String, value: Attribute) {
        self.0.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&Attribute> {
        self.0.get(name)
    }

	pub fn iter(&self) -> Iter<String, Attribute> {
		self.0.iter()
	}

	pub fn len(&self) -> usize {
		self.0.len()
	}
}

