use std::fmt::Debug;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents an entry name that can be of different types
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum EntryName {
    Exact(String),
    Any(Vec<String>),
    AnyNamed(Vec<EntryName>),
    #[cfg(feature = "regex")]
    Regex(String),
    #[cfg(feature = "wildmatch")]
    Wildmatch(String),
}

/// Represents a type of the entry such as file, folder or both
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum EntryType {
    File,
    Folder,
    Any,
}

/// Represents a named entry of some type (file/folder/both)
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub(crate) struct EntryNamed {
    entry_name: EntryName,
    entry_type: EntryType,
}

impl EntryNamed {
    pub fn new(entry_name: EntryName, entry_type: EntryType) -> Self {
        Self {
            entry_name,
            entry_type,
        }
    }

    pub fn file(name: EntryName) -> Self {
        Self::new(name, EntryType::File)
    }

    pub fn folder(name: EntryName) -> Self {
        Self::new(name, EntryType::Folder)
    }

    pub fn any(name: EntryName) -> Self {
        Self::new(name, EntryType::Any)
    }

    pub fn entry_name(&self) -> &EntryName {
        &self.entry_name
    }

    pub fn entry_type(&self) -> &EntryType {
        &self.entry_type
    }
}
