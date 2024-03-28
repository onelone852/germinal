use crossterm;

#[derive(rustler::NifTaggedEnum)]
pub enum Event {
    FocusGained,
    FocusLost,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Paste(String),
    Resize(u16, u16),
}

impl Into<Event> for crossterm::event::Event {
    fn into(self) -> Event {
        use crossterm::event::Event::*;
        match self {
            FocusGained => Event::FocusGained,
            FocusLost => Event::FocusLost,
            Key(event) => Event::Key(event.into()),
            Mouse(event) => Event::Mouse(event.into()),
            Paste(string) => Event::Paste(string),
            Resize(a, b) => Event::Resize(a, b),
        }
    }
}

#[derive(rustler::NifTaggedEnum)]
pub enum KeyCode {
    Backspace,
    Enter,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Tab,
    BackTab,
    Delete,
    Insert,
    F(u8),
    Char(String),
    Null,
    Esc,
    CapsLock,
    ScrollLock,
    NumLock,
    PrintScreen,
    Pause,
    Menu,
    KeypadBegin,
    Media(MediaKeyCode),
    Modifier(ModifierKeyCode),
}

impl Into<KeyCode> for crossterm::event::KeyCode {
    fn into(self) -> KeyCode {
        use crossterm::event::KeyCode::*;
        match self {
            Backspace => KeyCode::Backspace,
            Enter => KeyCode::Enter,
            Left => KeyCode::Left,
            Right => KeyCode::Right,
            Up => KeyCode::Up,
            Down => KeyCode::Down,
            Home => KeyCode::Home,
            End => KeyCode::End,
            PageUp => KeyCode::PageUp,
            PageDown => KeyCode::PageDown,
            Tab => KeyCode::Tab,
            BackTab => KeyCode::BackTab,
            Delete => KeyCode::Delete,
            Insert => KeyCode::Insert,
            F(f) => KeyCode::F(f),
            Char(ch) => KeyCode::Char(ch.to_string()),
            Null => KeyCode::Null,
            Esc => KeyCode::Esc,
            CapsLock => KeyCode::CapsLock,
            ScrollLock => KeyCode::ScrollLock,
            NumLock => KeyCode::NumLock,
            PrintScreen => KeyCode::PrintScreen,
            Pause => KeyCode::Pause,
            Menu => KeyCode::Menu,
            KeypadBegin => KeyCode::KeypadBegin,
            Media(code) => KeyCode::Media(code.into()),
            Modifier(code) => KeyCode::Modifier(code.into()),
        }
    }
}

#[derive(rustler::NifUnitEnum)]
pub enum MediaKeyCode {
    Play,
    Pause,
    PlayPause,
    Reverse,
    Stop,
    FastForward,
    Rewind,
    TrackNext,
    TrackPrevious,
    Record,
    LowerVolume,
    RaiseVolume,
    MuteVolume,
}

impl Into<MediaKeyCode> for crossterm::event::MediaKeyCode {
    fn into(self) -> MediaKeyCode {
        use crossterm::event::MediaKeyCode::*;
        match self {
            Play => MediaKeyCode::Play,
            Pause => MediaKeyCode::Pause,
            PlayPause => MediaKeyCode::PlayPause,
            Reverse => MediaKeyCode::Reverse,
            Stop => MediaKeyCode::Stop,
            FastForward => MediaKeyCode::FastForward,
            Rewind => MediaKeyCode::Rewind,
            TrackNext => MediaKeyCode::TrackNext,
            TrackPrevious => MediaKeyCode::TrackPrevious,
            Record => MediaKeyCode::Record,
            LowerVolume => MediaKeyCode::LowerVolume,
            RaiseVolume => MediaKeyCode::RaiseVolume,
            MuteVolume => MediaKeyCode::LowerVolume,
        }
    }
}

#[derive(rustler::NifUnitEnum)]
pub enum ModifierKeyCode {
    LeftShift,
    LeftControl,
    LeftAlt,
    LeftSuper,
    LeftHyper,
    LeftMeta,
    RightShift,
    RightControl,
    RightAlt,
    RightSuper,
    RightHyper,
    RightMeta,
    IsoLevel3Shift,
    IsoLevel5Shift,
}

impl Into<ModifierKeyCode> for crossterm::event::ModifierKeyCode {
    fn into(self) -> ModifierKeyCode {
        use crossterm::event::ModifierKeyCode::*;
        match self {
            LeftShift => ModifierKeyCode::LeftShift,
            LeftControl => ModifierKeyCode::LeftControl,
            LeftAlt => ModifierKeyCode::LeftAlt,
            LeftSuper => ModifierKeyCode::LeftSuper,
            LeftHyper => ModifierKeyCode::LeftHyper,
            RightMeta => ModifierKeyCode::RightMeta,
            RightShift => ModifierKeyCode::RightShift,
            RightControl => ModifierKeyCode::RightControl,
            RightAlt => ModifierKeyCode::RightAlt,
            RightSuper => ModifierKeyCode::RightSuper,
            RightHyper => ModifierKeyCode::RightHyper,
            LeftMeta => ModifierKeyCode::LeftMeta,
            IsoLevel3Shift => ModifierKeyCode::IsoLevel3Shift,
            IsoLevel5Shift => ModifierKeyCode::IsoLevel5Shift,
        }
    }
}

#[derive(rustler::NifUnitEnum)]
pub enum KeyEventKind {
    Press,
    Repeat,
    Release,
}

impl Into<KeyEventKind> for crossterm::event::KeyEventKind {
    fn into(self) -> KeyEventKind {
        use crossterm::event::KeyEventKind::*;
        match self {
            Press => KeyEventKind::Press,
            Repeat => KeyEventKind::Repeat,
            Release => KeyEventKind::Release,
        }
    }
}

#[derive(rustler::NifUnitEnum)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

impl Into<MouseButton> for crossterm::event::MouseButton {
    fn into(self) -> MouseButton {
        use crossterm::event::MouseButton::*;
        match self {
            Left => MouseButton::Left,
            Right => MouseButton::Right,
            Middle => MouseButton::Middle,
        }
    }
}

#[derive(rustler::NifTaggedEnum)]
pub enum MouseEventKind {
    Down(MouseButton),
    Up(MouseButton),
    Drag(MouseButton),
    Moved,
    ScrollDown,
    ScrollUp,
    ScrollLeft,
    ScrollRight,
}

impl Into<MouseEventKind> for crossterm::event::MouseEventKind {
    fn into(self) -> MouseEventKind {
        use crossterm::event::MouseEventKind::*;
        match self {
            Down(button) => MouseEventKind::Down(button.into()),
            Up(button) => MouseEventKind::Up(button.into()),
            Drag(button) => MouseEventKind::Drag(button.into()),
            Moved => MouseEventKind::Moved,
            ScrollDown => MouseEventKind::ScrollDown,
            ScrollUp => MouseEventKind::ScrollUp,
            ScrollLeft => MouseEventKind::ScrollLeft,
            ScrollRight => MouseEventKind::ScrollRight,
        }
    }
}

#[derive(rustler::NifStruct)]
#[module = "KeyEvent"]
pub struct KeyEvent {
    pub code: KeyCode,
    pub modifiers: u8,
    pub kind: KeyEventKind,
    pub state: u8,
}

impl Into<KeyEvent> for crossterm::event::KeyEvent {
    fn into(self) -> KeyEvent {
        KeyEvent { code: self.code.into(), modifiers: self.modifiers.bits(), kind: self.kind.into(), state: self.state.bits() }
    }
}

#[derive(rustler::NifStruct)]
#[module = "MouseEvent"]
pub struct MouseEvent {
    pub kind: MouseEventKind,
    pub column: u16,
    pub row: u16,
    pub modifiers: u8,
}

impl Into<MouseEvent> for crossterm::event::MouseEvent {
    fn into(self) -> MouseEvent {
        MouseEvent {
            kind: self.kind.into(),
            column: self.column,
            row: self.row,
            modifiers: self.modifiers.bits(),
        }
    }
}
