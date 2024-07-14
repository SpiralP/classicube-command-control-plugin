use std::{
    cell::Cell,
    time::{Duration, Instant},
};

use classicube_sys::Game;
use tracing::debug;

use super::IN_MAP;
use crate::{cli::WaitEvent, plugin::commands::waiter::GlobalWaiterManager};

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
            let chunk_updates = unsafe { Game.ChunkUpdates };
            debug!("{:?}", chunk_updates);

            if chunk_updates == 0 {
                GlobalWaiterManager::wake(&WaitEvent::ChunksLoaded);
            }
        }
    }
}

pub fn on_new_map_loaded() {
    const DELAY: Duration = Duration::from_secs(3);

    let now = Instant::now();
    NEXT_CHECK.set(now + DELAY);
}
