use super::state::MultitaskingState;
use core::sync::atomic::{AtomicU64, Ordering};
use spin::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]

pub struct ThreadId(u64);

impl ThreadId {
    pub fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(1);
        ThreadId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

pub struct Thread {
    pub id: ThreadId,
    pub stack_pointer: usize,
    pub stack_bottom: usize,
    pub stack_size: usize,
    pub ms: Mutex<MultitaskingState>,
}