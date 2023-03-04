// use crate::domain::error::MyError;
// use regex::Regex;
// use std::fmt;

/// タスク値オブジェクト
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    name: String,
    postpone_count: u16,
    status: TaskStatus,
}

impl Task {
    pub fn postpone(&mut self) {
        self.postpone_count += 1
    }

    /// 新しいタスクを作成します
    pub fn create(name: String) -> Self {
        return Self {
            name: name,
            postpone_count: 0,
            status: TaskStatus::TODO,
        };
    }

    /// タスクを再生成します
    pub fn reconstruct(name: String, postpone_count: u16, status: TaskStatus) -> Self {
        return Self {
            name: name,
            postpone_count: postpone_count,
            status: status,
        };
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskStatus {
    TODO,     // 選考中
    COMPLETE, // 完了
}
