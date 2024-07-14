mod commands;
mod server;

use std::cell::RefCell;

use classicube_helpers::tick::TickEventHandler;

thread_local!(
    static TICK_HANDLER: RefCell<Option<TickEventHandler>> = RefCell::default();
);

pub fn init() {
    server::start().unwrap();

    TICK_HANDLER.with(|cell| {
        let mut tick_handler = TickEventHandler::new();

        tick_handler.on(move |_task| {
            commands::tick();
        });

        *cell.borrow_mut() = Some(tick_handler);
    });
}

pub fn free() {
    TICK_HANDLER.with(|cell| drop(cell.borrow_mut().take()));
}

pub fn reset() {
    // TODO clear tick command queue?
    commands::reset();
}

pub fn on_new_map() {
    commands::on_new_map();
}

pub fn on_new_map_loaded() {
    commands::on_new_map_loaded();
}
