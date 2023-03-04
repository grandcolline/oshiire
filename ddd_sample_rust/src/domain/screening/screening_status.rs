#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScreeningStatus {
    InProgress, // 選考中
    Adopted,    // 採用
    Rejected,   // 不採用
}

impl ScreeningStatus {
    /// 面接が追加可能かどうか
    pub fn can_add_interview(&self) -> bool {
        match self {
            ScreeningStatus::InProgress => true,
            ScreeningStatus::Adopted => false,
            ScreeningStatus::Rejected => false,
        }
    }
}
