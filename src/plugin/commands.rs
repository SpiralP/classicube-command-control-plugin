use std::sync::{mpsc, Arc, Mutex, OnceLock};

use anyhow::Result;
use classicube_sys::{Chat_Send, OwnedString};

use crate::cli::Action;

type OnceQueueType = (
    mpsc::Sender<Action>,
    Arc<Mutex<Option<mpsc::Receiver<Action>>>>,
);
fn get_queue() -> &'static OnceQueueType {
    static ONCE: OnceLock<OnceQueueType> = OnceLock::new();
    ONCE.get_or_init(|| {
        // Arc::new(Mutex::new(Vec::new()))
        let (tx, rx) = mpsc::channel();

        let rx = Arc::new(Mutex::new(Some(rx)));
        (tx, rx)
    })
}

pub fn tick() {
    let queue = get_queue().1.lock().unwrap();
    let queue = queue.as_ref().expect("queue None");
    match queue.try_recv() {
        Ok(action) => handle_cli_action(action),
        Err(mpsc::TryRecvError::Empty) => {}
        Err(mpsc::TryRecvError::Disconnected) => panic!("queue sender Disconnected"),
    }
}

pub fn queue_cli_action(action: Action) -> Result<()> {
    let queue = get_queue().0.clone();
    queue.send(action)?;

    Ok(())
}

pub fn on_new_map_loaded() {
    // TODO wake up waiting commands
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
    }
}
