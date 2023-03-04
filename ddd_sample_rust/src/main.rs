mod domain;
mod infra;

use crate::domain::screening::ScreeningId;

fn main() {
    let id = ScreeningId::new();
    println!("ScreeningId: {}", id);
}
