#[derive(Debug, Clone, PartialEQ)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, PartialEQ)]
pub enum Status {
    Pending,
    InProgress,
    Completed,
}

#[derive(Debug, Clone, PartialEQ)]
pub enum Category {
    Work,
    Personal,
    Health,
    Learning,
    Other(String),
}
