use chrono::{DateTime, Local};

/// 面接
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Interview {
    phase: usize,               // 面接次数 FIXME: u16にする?
    date_time: DateTime<Local>, // 面接日時
}

/// 面接のファーストコレクション
#[derive(Debug, Clone)]
pub struct Interviews(Vec<Interview>);

impl Interviews {
    /// 面接を追加します
    pub fn add_interview(&self, interview_date_time: DateTime<Local>) -> Self {
        let new_interview = Interview {
            phase: self.0.len() + 1, // 既存の面接の1つ後の面接時数を設定
            date_time: interview_date_time,
        };
        let mut list: Vec<Interview> = self.0.clone();
        list.push(new_interview);

        Interviews(list)
    }

    /// 初期状態のインスタンスを生成します
    pub fn empty() -> Self {
        Self(vec![])
    }
}
