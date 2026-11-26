use crate::error::StoreError;
use crate::models::Entry;
use std::fs;
use std::path::{Path, PathBuf};

pub struct LogStore {
    path: PathBuf,
    entries: Vec<Entry>,
}

impl LogStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            entries: Vec::new(),
        }
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self, StoreError> {
        let path = path.as_ref().to_path_buf();
        let content = fs::read_to_string(&path)?;
        let mut entries = Vec::new();

        for (idx, line) in content.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }
            let entry = Entry::from_record(idx + 1, line)?;
            entries.push(entry);
        }

        Ok(Self { path, entries })
    }

    pub fn add(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    pub fn escalate_entry(&mut self, id: u32) -> Result<&Entry, StoreError> {
        let entry = self
            .entries
            .iter_mut()
            .find(|e| e.id == id)
            .ok_or(StoreError::NotFound(id))?;

        entry.priority = entry.priority.escalate();
        Ok(entry)
    }

    pub fn find_by_tag<'a>(&'a self, tag: &str) -> Vec<&'a Entry> {
        self.entries
            .iter()
            .filter(|e| e.tags.iter().any(|t| t.eq_ignore_ascii_case(tag)))
            .collect()
    }

    pub fn save(&self) -> Result<(), StoreError> {
        let mut buffer = String::new();
        for entry in &self.entries {
            buffer.push_str(&entry.to_record());
            buffer.push('\n');
        }
        fs::write(&self.path, buffer)?;
        Ok(())
    }

    pub fn into_entries(self) -> Vec<Entry> {
        self.entries
    }
}