use super::*;
use crate::domain::error::MyError;
use chrono::{DateTime, Local};

// 選考条件
#[derive(Debug, Clone)]
pub struct Screening {
    screening_id: ScreeningId,        // 採用選考ID
    position_id: PositionId,          // 採用ポジションID
    applicant: Applicant,             // 応募者
    apply_date_time: DateTime<Local>, // 応募日時
    interviews: Interviews,           // 面接
    status: ScreeningStatus,          // 選考ステータス
}

// EntityはIDで同一性を担保する
impl PartialEq for Screening {
    fn eq(&self, other: &Self) -> bool {
        self.screening_id == other.screening_id
    }
}
impl Eq for Screening {}

impl Screening {
    // 新規採用を作成します
    pub fn create(position_id: PositionId, applicant: Applicant) -> Self {
        Self {
            // 以下の値は初期値を設定
            screening_id: ScreeningId::new(),    // IDは新規作成
            interviews: Interviews::empty(),     // 面接の集合は初期値として0件
            status: ScreeningStatus::InProgress, // ステータスは選考中から始まる
            apply_date_time: Local::now(),       // 応募日はインスタンス生成タイミング

            // 以下の値は引数の値を設定
            position_id: position_id,
            applicant: applicant,
        }
    }

    // DBなどの値からインスタンスを再構成します
    pub fn reconstruct(
        screening_id: ScreeningId,
        position_id: PositionId,
        applicant: Applicant,
        apply_date_time: DateTime<Local>,
        interviews: Interviews,
        status: ScreeningStatus,
    ) -> Self {
        Self {
            screening_id: screening_id,
            position_id: position_id,
            applicant: applicant,
            apply_date_time: apply_date_time,
            interviews: interviews,
            status: status,
        }
    }

    // 面接を追加します
    pub fn add_interview(&mut self, interview_date_time: DateTime<Local>) -> Result<(), MyError> {
        if !self.status.can_add_interview() {
            return Err(MyError::BadRequestError(
                "面接が選考中ではありません".into(),
            ));
        }
        self.interviews = self.interviews.add_interview(interview_date_time);
        Ok(())
    }

    // ステータスを「採用」にします
    pub fn adopt(&mut self) {
        self.status = ScreeningStatus::Adopted
    }

    // ステータスを「不採用」にします
    pub fn reject(&mut self) {
        self.status = ScreeningStatus::Rejected
    }
}
