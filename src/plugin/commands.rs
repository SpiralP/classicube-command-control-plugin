use std::sync::{mpsc, Arc, Condvar, Mutex, OnceLock, RwLock};

use anyhow::Result;
use classicube_sys::{Chat_Send, OwnedString};

use crate::cli::Action;

type OnceQueue = (mpsc::Sender<Action>, Arc<Mutex<mpsc::Receiver<Action>>>);
fn get_queue() -> &'static OnceQueue {
    static ONCE: OnceLock<OnceQueue> = OnceLock::new();
    ONCE.get_or_init(|| {
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        (tx, rx)
    })
}

type Waiter = Arc<(Mutex<bool>, Condvar)>;
type OnceWaiter = Arc<RwLock<Waiter>>;
fn get_waiter() -> &'static OnceWaiter {
    static ONCE: OnceLock<OnceWaiter> = OnceLock::new();
    ONCE.get_or_init(|| Arc::new(RwLock::new(Arc::new((Mutex::new(false), Condvar::new())))))
}

pub fn tick() {
    let queue = get_queue().1.lock().unwrap();
    if let Ok(action) = queue.try_recv() {
        handle_cli_action(action);
    }
}

/// on `handle_client` thread
pub fn queue_cli_action(action: Action) -> Result<()> {
    match action {
        Action::Wait(_) => {
            let (lock, cvar) = &*create_map_loaded_waiter();
            let mut started = lock.lock().unwrap();
            while !*started {
                started = cvar.wait(started).unwrap();
            }
        }
        Action::Chat(_) => {
            let queue = get_queue().0.clone();
            queue.send(action)?;
        }
    }

    Ok(())
}

pub fn create_map_loaded_waiter() -> Waiter {
    let waiter_static = get_waiter().read().unwrap();
    waiter_static.clone()
}

pub fn on_new_map_loaded() {
    let mut waiter_static = get_waiter().write().unwrap();
    {
        let (lock, cvar) = &**waiter_static;
        {
            let mut woken = lock.lock().unwrap();
            *woken = true;
        }
        cvar.notify_all();
    }
    *waiter_static = Arc::new((Mutex::new(false), Condvar::new()));
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
