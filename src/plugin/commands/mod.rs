mod chunks;
mod init;
mod waiter;

use std::{
    cell::Cell,
    clone::Clone,
    sync::{mpsc, Arc, Mutex, OnceLock},
};

use anyhow::Result;
use classicube_sys::{Chat_Send, OwnedString};
use tracing::debug;

use self::waiter::GlobalWaiterManager;
use crate::cli::{Action, WaitEvent};

thread_local! {
    static IN_MAP: Cell<bool> = const { Cell::new(false) };
}

type OnceQueue = (mpsc::Sender<Action>, Arc<Mutex<mpsc::Receiver<Action>>>);
fn get_queue() -> &'static OnceQueue {
    static ONCE: OnceLock<OnceQueue> = OnceLock::new();
    ONCE.get_or_init(|| {
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        (tx, rx)
    })
}

pub fn tick() {
    let queue = get_queue().1.lock().unwrap();
    while let Ok(action) = queue.try_recv() {
        handle_cli_action(action);
    }

    chunks::tick();
    init::tick();
}

/// on `handle_client` thread
pub fn queue_cli_action(action: Action) -> Result<()> {
    match action {
        Action::Wait(args) => {
            let waiter = GlobalWaiterManager::new_waiter(args.event);
            waiter.wait();
        }
        Action::Chat(_) => {
            let queue = get_queue().0.clone();
            queue.send(action)?;
        }
    }

    Ok(())
}

/// on main thread
fn handle_cli_action(action: Action) {
    match action {
        Action::Chat(args) => {
            let owned_string = OwnedString::new(args.message);

            unsafe {
                Chat_Send(owned_string.as_cc_string(), 1);
            }
        }
        Action::Wait(_) => {}
    }
}

pub fn reset() {
    debug!("reset");
    IN_MAP.set(false);

    GlobalWaiterManager::wake(&WaitEvent::Reset);
}

pub fn on_new_map() {
    debug!("on_new_map");
    IN_MAP.set(false);

    GlobalWaiterManager::wake(&WaitEvent::MapLoading);
}

pub fn on_new_map_loaded() {
    debug!("on_new_map_loaded");
    IN_MAP.set(true);

    GlobalWaiterManager::wake(&WaitEvent::MapLoaded);
    chunks::on_new_map_loaded();
}
