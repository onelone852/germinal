//// A module containing functions that used to control terminal and read event. 

import germinal/types

/// Checks if there is an Event available.
/// Returns Ok(true) if an Event is available otherwise it returns Ok(false).
/// Ok(true) guarantees that subsequent call to the read function won’t block.
/// 
/// This function need raw mode to work properly.
/// 
/// It can only be used in a single thread.
@external(erlang, "germinal_rs", "poll")
pub fn poll(timeout_ms: Int) -> Result(Bool, String)

/// Reads a single Event. This function blocks until an Event is recevived. The poll function can be used if blocking is not desired.
/// 
/// This function need raw mode to work properly.
/// 
/// It can only be used in single thread.
@external(erlang, "germinal_rs", "read")
pub fn read() -> Result(types.Event, String)

/// Enables the raw mode.
/// 
/// Not disabling afterwards may cause weird behaviour of the terminal.
@external(erlang, "germinal_rs", "enable_raw_mode")
pub fn enable_raw_mode() -> Result(Nil, String)

/// Disables the raw mode.
@external(erlang, "germinal_rs", "disable_raw_mode")
pub fn disable_raw_mode() -> Result(Nil, String)
