#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultitaskingState {
    Preemptive,
    UserCooperative { ticks_used: usize, max_ticks: usize },
    Cooperative,
    Blocked,
}

