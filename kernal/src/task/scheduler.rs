use alloc::{sync::Arc, collections::VecDeque};
use spin::Mutex;
use super::threed::Thread;

use crate::task::context::switch_context;

/// Tracks the currently executing thread in the kernel.
static CURRENT_THREAD: Mutex<Option<Arc<Thread>>> = Mutex::new(None);

/// Sets the currently active thread. Called by the scheduler when switching context.
pub fn set_current_running_task(thread: Arc<Thread>) {
    let mut current = CURRENT_THREAD.lock();
    *current = Some(thread);
}

/// Retrieves a clone of the currently running thread.
/// Returns `None` if the scheduler hasn't started yet.
pub fn get_current_running_task() -> Option<Arc<Thread>> {
    CURRENT_THREAD.lock().clone()
}

/// The global queue of threads waiting for CPU time.
static READY_QUEUE: Mutex<Option<VecDeque<Arc<Thread>>>> = Mutex::new(None);

/// Initializes the global scheduler queues. Call this once during kernel startup!
pub fn init() {
    let mut ready = READY_QUEUE.lock();
    if ready.is_none() {
        *ready = Some(VecDeque::new());
    }
}

/// Registers a new thread into the ready queue.
pub fn spawn_thread(thread: Thread) {
    let thread_arc = Arc::new(thread);
    let mut ready = READY_QUEUE.lock();
    
    if let Some(ref mut queue) = *ready {
        queue.push_back(thread_arc);
    } else {
        panic!("Scheduler not initialized! Call scheduler::init() first.");
    }
}

/// Selects the next ready thread and performs a context switch.
/// Safe to call from timer interrupts or explicit yields.
pub fn schedule_next_task() {
    let mut ready_guard = READY_QUEUE.lock();
    let mut current_guard = CURRENT_THREAD.lock();

    let queue = match ready_guard.as_mut() {
        Some(q) => q,
        None => return, // Scheduler not initialized
    };

    // Grab the next thread to execute
    let next_thread = match queue.pop_front() {
        Some(t) => t,
        None => return, // No other threads ready to run!
    };

    // Swap the current running thread
    let old_thread = current_guard.take();
    *current_guard = Some(next_thread.clone());

    // If the old thread was running, put it back at the end of the ready queue
    if let Some(prev) = old_thread {
        queue.push_back(prev.clone());
        
        // Drop locks before switching stacks so we don't leave mutexes locked!
        drop(current_guard);
        drop(ready_guard);

        unsafe {
            let old_rsp_ptr = &prev.stack_pointer as *const usize as *mut usize;
            let new_rsp_ptr = &next_thread.stack_pointer as *const usize;

            switch_context(old_rsp_ptr, new_rsp_ptr);
        }
    } else {
        // First context switch ever (bootstrapping the scheduler)
        drop(current_guard);
        drop(ready_guard);

        unsafe {
            let dummy_rsp: usize = 0;
            let new_rsp_ptr = &next_thread.stack_pointer as *const usize;

            switch_context(&dummy_rsp as *const usize as *mut usize, new_rsp_ptr);
        }
    }
}

/// Yields the CPU voluntarily from code running inside a thread.
pub fn yield_now() {
    schedule_next_task();
}