use std::time::Duration;

mod nif_crossterm;

pub use nif_crossterm::*;

use crossterm;
use rustler;

#[rustler::nif]
pub fn read() -> Result<Event, String> {
    crossterm::event::read()
        .map(|event| event.into())
        .map_err(|err| err.to_string())
}

#[rustler::nif]
pub fn poll(time_ms: u64) -> Result<bool, String> {
    crossterm::event::poll(Duration::from_millis(time_ms)).map_err(|err| err.to_string())
}

#[rustler::nif]
pub fn enable_raw_mode() -> Result<(), String> {
    crossterm::terminal::enable_raw_mode().map_err(|err| err.to_string())
}

#[rustler::nif]
pub fn disable_raw_mode() -> Result<(), String> {
    crossterm::terminal::disable_raw_mode().map_err(|err| err.to_string())
}

rustler::init!("germinal_rs", [read, poll, enable_raw_mode, disable_raw_mode]);