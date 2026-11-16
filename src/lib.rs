// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-kerberos: kerberos component model.
//!
//! Real implementation: kerberos_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KerberosItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KerberosError {
    EmptyKerberosItemName,
    DuplicateKerberosItem(String),
    EmptyEventSubject,
}

impl fmt::Display for KerberosError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyKerberosItemName => write!(f, "kerberos_item name must not be empty"),
            Self::DuplicateKerberosItem(id) => write!(f, "duplicate kerberos_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for KerberosError {}

pub struct Kerberos {
    pub name: String,
    kerberos_items: BTreeMap<String, KerberosItem>,
    events: BTreeMap<String, Event>,
}

impl Kerberos {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            kerberos_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_kerberos_item(&mut self, item: KerberosItem) -> Result<(), KerberosError> {
        if item.name.trim().is_empty() {
            return Err(KerberosError::EmptyKerberosItemName);
        }
        let id = item.id.clone();
        if self.kerberos_items.contains_key(&id) {
            return Err(KerberosError::DuplicateKerberosItem(id.clone()));
        }
        self.kerberos_items.insert(id, item);
        Ok(())
    }

    pub fn kerberos_item_count(&self) -> usize {
        self.kerberos_items.len()
    }

    pub fn get_kerberos_item(&self, id: &str) -> Option<&KerberosItem> {
        self.kerberos_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), KerberosError> {
        if event.subject.trim().is_empty() {
            return Err(KerberosError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_kerberos_item(id: &str, name: &str) -> KerberosItem {
        KerberosItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Kerberos::new("Kerberos");
        assert_eq!(m.name, "Kerberos");
        assert_eq!(m.kerberos_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_kerberos_item() {
        let mut m = Kerberos::new("Kerberos");
        m.add_kerberos_item(sample_kerberos_item("a1", "Alpha")).unwrap();
        assert_eq!(m.kerberos_item_count(), 1);
        assert!(m.get_kerberos_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Kerberos::new("Kerberos");
        assert!(m.add_kerberos_item(sample_kerberos_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Kerberos::new("Kerberos");
        m.add_kerberos_item(sample_kerberos_item("x", "One")).unwrap();
        assert!(m.add_kerberos_item(sample_kerberos_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Kerberos::new("Kerberos");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
