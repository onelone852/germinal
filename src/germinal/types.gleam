//// A module containing types in germinal.

import gleam/dict
import gleam/dynamic
import gleam/erlang/atom
import gleam/int

/// The kind of key event.
pub type KeyEventKind {
  Press
  Repeat
  Release
}

/// Key code.
pub type KeyCode {
  Backspace
  Enter
  Left
  Right
  Up
  Down
  Home
  End
  PageUp
  PageDown
  Tab
  BackTab
  Delete
  Insert
  F(Int)
  Char(String)
  Null
  Esc
  CapsLock
  ScrollLock
  NumLock
  PrintScreen
  Pause
  Menu
  KeypadBegin
}

/// Key event.
pub type KeyEvent

// The key modifier of key event.
pub type KeyModifiers

/// The state of key event.
pub type KeyEventState

/// Represent any event in terminal. 
/// 
/// Currently, this library does not support mouse.
pub type Event {
  FocusGained
  FocusLost
  Key(KeyEvent)
  Paste(String)
  Resize(Int, Int)
}

fn expect(msg: String) -> fn(Result(a, b)) -> a {
  fn(res) {
    case res {
      Ok(res) -> res
      Error(_) -> panic(msg)
    }
  }
}

fn unsafe_coerce(anything: a) -> b {
  anything
  |> dynamic.from
  |> dynamic.unsafe_coerce
}

/// Return the key code of the key event.
pub fn get_keycode(event: KeyEvent) -> KeyCode {
  dict.get(unsafe_coerce(event), atom.create_from_string("code"))
  |> expect("Should have code")
}

/// Return the kind of the key event.
pub fn get_keyevent_kind(event: KeyEvent) -> KeyEventKind {
  dict.get(unsafe_coerce(event), atom.create_from_string("kind"))
  |> expect("Should have kind")
}

/// Return the key modifiers of the key event.
pub fn get_key_modifiers(event: KeyEvent) -> KeyModifiers {
  dict.get(unsafe_coerce(event), atom.create_from_string("modifiers"))
  |> expect("Should have modifiers")
}

/// Return the state of the key event.
pub fn get_keyevent_state(event: KeyEvent) -> KeyEventState {
  dict.get(unsafe_coerce(event), atom.create_from_string("state"))
  |> expect("Should have state")
}

/// Represent no state.
pub fn none_state() -> KeyEventState {
  unsafe_coerce(0b0000)
}

/// Represent the KeyPad state.
pub fn keypad_state() -> KeyEventState {
  unsafe_coerce(0b0001)
}

/// Represent the Num Lock state.
pub fn num_lock_state() -> KeyEventState {
  unsafe_coerce(0b1000)
}

/// Represent the Caps Lock state.
pub fn caps_lock_state() -> KeyEventState {
  unsafe_coerce(0b1000)
}

/// Combine two state.
pub fn or_state(state1: KeyEventState, state2: KeyEventState) -> KeyEventState {
  int.bitwise_or(unsafe_coerce(state1), unsafe_coerce(state2))
  |> unsafe_coerce
}

/// Return True if parent_state contains child_state. Return False if not.
pub fn contains_state(
  parent_state: KeyEventState,
  child_state: KeyEventState,
) -> Bool {
  int.bitwise_and(unsafe_coerce(parent_state), unsafe_coerce(child_state))
  == unsafe_coerce(child_state)
}

/// Represent no modifier.
pub fn none_modifier() -> KeyModifiers {
  unsafe_coerce(0b0000)
}

/// Represent the SHIFT modifier.
pub fn shift_modifier() -> KeyModifiers {
  unsafe_coerce(0b0001)
}

// Represent the CONTORL (CTRL) modifier.
pub fn control_modifier() -> KeyModifiers {
  unsafe_coerce(0b0010)
}

/// Represent the ALT modifier.
pub fn alt_modifier() -> KeyModifiers {
  unsafe_coerce(0b0100)
}

/// Combine two modifiers.
pub fn or_modifiers(modifiers1: KeyModifiers, modifiers2: KeyModifiers) -> KeyModifiers {
  int.bitwise_or(unsafe_coerce(modifiers1), unsafe_coerce(modifiers2))
  |> unsafe_coerce
}

/// Return True if parent_modifiers contains child_modifiers. Return False if not.
pub fn contains_modifiers(
  parent_modifiers: KeyModifiers,
  child_modifiers: KeyModifiers,
) -> Bool {
  int.bitwise_and(unsafe_coerce(parent_modifiers), unsafe_coerce(child_modifiers))
  == unsafe_coerce(child_modifiers)
}
