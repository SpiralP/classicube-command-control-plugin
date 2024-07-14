use std::{
    cell::Cell,
    time::{Duration, Instant},
};

use super::{waiter::GlobalWaiterManager, IN_MAP};
use crate::cli::WaitEvent;

const INTERVAL: Duration = Duration::from_secs(1);

thread_local! {
    static NEXT_CHECK: Cell<Instant> = Cell::new(Instant::now() + INTERVAL);
}

pub fn tick() {
    if IN_MAP.get() {
        let now = Instant::now();
        let next_check = NEXT_CHECK.get();

        if now >= next_check {
            NEXT_CHECK.set(now + INTERVAL);
            GlobalWaiterManager::wake(&WaitEvent::Init);
        }
    }
}
