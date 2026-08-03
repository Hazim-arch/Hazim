pub mod scheduler;
pub mod keyboard;
pub mod executor;
pub mod context;
pub mod threed;
pub mod state;

use core::{future::Future, pin::Pin, sync::atomic::{AtomicU64, Ordering}};
use alloc::boxed::Box;

pub struct Task {
    id: TaskId, // new
    future: Pin<Box<dyn Future<Output = ()>>>,
    ms: state::MultitaskingState,
}

impl Task {
    pub fn new(future: impl Future<Output = ()> + 'static) -> Task {
        Task {
            id: TaskId::new(), // new
            future: Box::pin(future),
            ms: state::MultitaskingState::Cooperative,
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct TaskId(u64);

impl TaskId {
    fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        TaskId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

