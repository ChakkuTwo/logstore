use crate::error::StoreError;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl Priority {
    pub fn escalate(self) -> Self {
        match self {
            Priority::Low => Priority::Medium,
            Priority::Medium | Priority::High => Priority::High,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Priority::Low => "LOW",
            Priority::Medium => "MEDIUM",
            Priority::High => "HIGH",
        }
    }

    fn parse(line: usize, s: &str) -> Result<Self, StoreError> {
        match s.trim() {
            "LOW" => Ok(Priority::Low),
            "MEDIUM" => Ok(Priority::Medium),
            "HIGH" => Ok(Priority::High),
            other => Err(StoreError::Parse {
                line,
                reason: format!("unknown priority '{other}'"),
            }),
        }
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub id: u32,
    pub title: String,
    pub tags: Vec<String>,
    pub priority: Priority,
}

impl Entry {
    pub fn new(
        id: u32,
        title: impl Into<String>,
        tags: Vec<String>,
        priority: Priority,
    ) -> Result<Self, StoreError> {
        let title = title.into();
        if title.trim().is_empty() {
            return Err(StoreError::EmptyField("title"));
        }
        Ok(Self {
            id,
            title,
            tags,
            priority,
        })
    }

    pub fn to_record(&self) -> String {
        let joined_tags = self.tags.join(",");
        format!("{}|{}|{}|{}", self.id, self.priority, self.title, joined_tags)
    }

    pub fn from_record(line: usize, raw: &str) -> Result<Self, StoreError> {
        let parts: Vec<&str> = raw.splitn(4, '|').collect();
        if parts.len() != 4 {
            return Err(StoreError::Parse {
                line,
                reason: "expected 4 pipe-delimited fields".to_string(),
            });
        }

        let id: u32 = parts[0].trim().parse().map_err(|_| StoreError::Parse {
            line,
            reason: format!("invalid numeric ID '{}'", parts[0]),
        })?;

        let priority = Priority::parse(line, parts[1])?;
        let title = parts[2].trim();
        let tags: Vec<String> = parts[3]
            .split(',')
            .map(str::trim)
            .filter(|t| !t.is_empty())
            .map(String::from)
            .collect();

        Self::new(id, title, tags, priority)
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[#{} | {}] {} (tags: {})",
            self.id,
            self.priority,
            self.title,
            if self.tags.is_empty() {
                "none".to_string()
            } else {
                self.tags.join(", ")
            }
        )
    }
}