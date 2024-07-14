use std::{
    collections::HashMap,
    sync::{Arc, Condvar, Mutex, OnceLock, Weak},
};

use crate::cli::WaitEvent;

type OnceWaiter = Arc<Mutex<WaiterManager>>;
fn get_global_waiter_manager() -> &'static OnceWaiter {
    static ONCE: OnceLock<OnceWaiter> = OnceLock::new();
    ONCE.get_or_init(|| Arc::new(Mutex::new(WaiterManager::new())))
}

pub struct GlobalWaiterManager {}

impl GlobalWaiterManager {
    pub fn wake(event: &WaitEvent) {
        let mut manager = get_global_waiter_manager().lock().unwrap();
        manager.wake(event);
    }

    pub fn new_waiter(event: WaitEvent) -> Arc<Waiter> {
        let waiter = Arc::new(Waiter::new());

        let mut manager = get_global_waiter_manager().lock().unwrap();
        manager.add_waiter(event, &waiter);

        waiter
    }
}

struct WaiterManager {
    inner: HashMap<WaitEvent, Weak<Waiter>>,
}

impl WaiterManager {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn add_waiter(&mut self, event: WaitEvent, waiter: &Arc<Waiter>) {
        self.inner.insert(event, Arc::downgrade(waiter));
    }

    fn wake(&mut self, event: &WaitEvent) {
        if let Some(waiter_weak) = self.inner.remove(event) {
            if let Some(waiter) = waiter_weak.upgrade() {
                waiter.wake();
            }
        }
    }
}

pub struct Waiter {
    inner: (Mutex<bool>, Condvar),
}

impl Waiter {
    fn new() -> Self {
        Self {
            inner: (Mutex::new(false), Condvar::new()),
        }
    }

    pub fn wait(&self) {
        let (lock, cvar) = &self.inner;
        let mut started = lock.lock().unwrap();
        while !*started {
            started = cvar.wait(started).unwrap();
        }
    }

    fn wake(&self) {
        let (lock, cvar) = &self.inner;
        {
            let mut woken = lock.lock().unwrap();
            *woken = true;
        }
        cvar.notify_all();
    }
}

#[test]
fn test_waiter() {
    use std::{thread, time::Duration};

    thread::spawn(|| {
        thread::sleep(Duration::from_secs(1));
        GlobalWaiterManager::wake(&WaitEvent::MapLoaded);
    });

    let waiter = GlobalWaiterManager::new_waiter(WaitEvent::MapLoaded);
    waiter.wait();
}
