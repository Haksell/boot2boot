pub mod layouts;

mod scancodes;
pub use scancodes::ScancodeSet1;

#[derive(Debug)]
pub struct Keyboard<L, S>
where
    S: ScancodeSet,
    L: KeyboardLayout,
{
    scancode_set: S,
    event_decoder: EventDecoder<L>,
}

#[derive(Debug)]
pub struct EventDecoder<L: KeyboardLayout> {
    modifiers: Modifiers,
    layout: L,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Error {
    UnknownKeyCode,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
#[repr(u8)]
pub enum KeyCode {
    // ========= Row 1 (the F-keys) =========
    /// Top Left of the Keyboard
    Escape,
    /// Function Key F1
    F1,
    /// Function Key F2
    F2,
    /// Function Key F3
    F3,
    /// Function Key F4
    F4,
    /// Function Key F5
    F5,
    /// Function Key F6
    F6,
    /// Function Key F7
    F7,
    /// Function Key F8
    F8,
    /// Function Key F9
    F9,
    /// Function Key F10
    F10,
    /// Function Key F11
    F11,
    /// Function Key F12
    F12,

    /// The Print Screen Key
    PrintScreen,
    /// The Sys Req key (you get this keycode with Alt + PrintScreen)
    SysRq,
    /// The Scroll Lock key
    ScrollLock,
    /// The Pause/Break key
    PauseBreak,

    // ========= Row 2 (the numbers) =========
    /// Symbol key to the left of `Key1`
    Oem8,
    /// Number Line, Digit 1
    Key1,
    /// Number Line, Digit 2
    Key2,
    /// Number Line, Digit 3
    Key3,
    /// Number Line, Digit 4
    Key4,
    /// Number Line, Digit 5
    Key5,
    /// Number Line, Digit 6
    Key6,
    /// Number Line, Digit 7
    Key7,
    /// Number Line, Digit 8
    Key8,
    /// Number Line, Digit 9
    Key9,
    /// Number Line, Digit 0
    Key0,
    /// US Minus/Underscore Key (right of 'Key0')
    OemMinus,
    /// US Equals/Plus Key (right of 'OemMinus')
    OemPlus,
    /// Backspace
    Backspace,

    /// Top Left of the Extended Block
    Insert,
    /// Top Middle of the Extended Block
    Home,
    /// Top Right of the Extended Block
    PageUp,

    /// The Num Lock key
    NumpadLock,
    /// The Numpad Divide (or Slash) key
    NumpadDivide,
    /// The Numpad Multiple (or Star) key
    NumpadMultiply,
    /// The Numpad Subtract (or Minus) key
    NumpadSubtract,

    // ========= Row 3 (QWERTY) =========
    /// The Tab Key
    Tab,
    /// Letters, Top Row #1
    Q,
    /// Letters, Top Row #2
    W,
    /// Letters, Top Row #3
    E,
    /// Letters, Top Row #4
    R,
    /// Letters, Top Row #5
    T,
    /// Letters, Top Row #6
    Y,
    /// Letters, Top Row #7
    U,
    /// Letters, Top Row #8
    I,
    /// Letters, Top Row #9
    O,
    /// Letters, Top Row #10
    P,
    /// US ANSI Left-Square-Bracket key
    Oem4,
    /// US ANSI Right-Square-Bracket key
    Oem6,
    /// US ANSI Backslash Key / UK ISO Backslash Key
    Oem5,
    /// The UK/ISO Hash/Tilde key (ISO layout only)
    Oem7,

    /// The Delete key - bottom Left of the Extended Block
    Delete,
    /// The End key - bottom Middle of the Extended Block
    End,
    /// The Page Down key - -bottom Right of the Extended Block
    PageDown,

    /// The Numpad 7/Home key
    Numpad7,
    /// The Numpad 8/Up Arrow key
    Numpad8,
    /// The Numpad 9/Page Up key
    Numpad9,
    /// The Numpad Add/Plus key
    NumpadAdd,

    // ========= Row 4 (ASDF) =========
    /// Caps Lock
    CapsLock,
    /// Letters, Middle Row #1
    A,
    /// Letters, Middle Row #2
    S,
    /// Letters, Middle Row #3
    D,
    /// Letters, Middle Row #4
    F,
    /// Letters, Middle Row #5
    G,
    /// Letters, Middle Row #6
    H,
    /// Letters, Middle Row #7
    J,
    /// Letters, Middle Row #8
    K,
    /// Letters, Middle Row #9
    L,
    /// The US ANSI Semicolon/Colon key
    Oem1,
    /// The US ANSI Single-Quote/At key
    Oem3,

    /// The Return Key
    Return,

    /// The Numpad 4/Left Arrow key
    Numpad4,
    /// The Numpad 5 Key
    Numpad5,
    /// The Numpad 6/Right Arrow key
    Numpad6,

    // ========= Row 5 (ZXCV) =========
    /// Left Shift
    LShift,
    /// Letters, Bottom Row #1
    Z,
    /// Letters, Bottom Row #2
    X,
    /// Letters, Bottom Row #3
    C,
    /// Letters, Bottom Row #4
    V,
    /// Letters, Bottom Row #5
    B,
    /// Letters, Bottom Row #6
    N,
    /// Letters, Bottom Row #7
    M,
    /// US ANSI `,<` key
    OemComma,
    /// US ANSI `.>` Key
    OemPeriod,
    /// US ANSI `/?` Key
    Oem2,
    /// Right Shift
    RShift,

    /// The up-arrow in the inverted-T
    ArrowUp,

    /// Numpad 1/End Key
    Numpad1,
    /// Numpad 2/Arrow Down Key
    Numpad2,
    /// Numpad 3/Page Down Key
    Numpad3,
    /// Numpad Enter
    NumpadEnter,

    // ========= Row 6 (modifers and space bar) =========
    /// The left-hand Control key
    LControl,
    /// The left-hand 'Windows' key
    LWin,
    /// The left-hand Alt key
    LAlt,
    /// The Space Bar
    Spacebar,
    /// The right-hand AltGr key
    RAltGr,
    /// The right-hand Win key
    RWin,
    /// The 'Apps' key (aka 'Menu' or 'Right-Click')
    Apps,
    /// The right-hand Control key
    RControl,

    /// The left-arrow in the inverted-T
    ArrowLeft,
    /// The down-arrow in the inverted-T
    ArrowDown,
    /// The right-arrow in the inverted-T
    ArrowRight,

    /// The Numpad 0/Insert Key
    Numpad0,
    /// The Numppad Period/Delete Key
    NumpadPeriod,

    // ========= JIS 109-key extra keys =========
    /// Extra JIS key (0x7B)
    Oem9,
    /// Extra JIS key (0x79)
    Oem10,
    /// Extra JIS key (0x70)
    Oem11,
    /// Extra JIS symbol key (0x73)
    Oem12,
    /// Extra JIS symbol key (0x7D)
    Oem13,

    // ========= Extra Keys =========
    /// Multi-media keys - Previous Track
    PrevTrack,
    /// Multi-media keys - Next Track
    NextTrack,
    /// Multi-media keys - Volume Mute Toggle
    Mute,
    /// Multi-media keys - Open Calculator
    Calculator,
    /// Multi-media keys - Play
    Play,
    /// Multi-media keys - Stop
    Stop,
    /// Multi-media keys - Increase Volume
    VolumeDown,
    /// Multi-media keys - Decrease Volume
    VolumeUp,
    /// Multi-media keys - Open Browser
    WWWHome,
    /// Sent when the keyboard boots
    PowerOnTestOk,
    /// Sent by the keyboard when too many keys are pressed
    TooManyKeys,
    /// Used as a 'hidden' Right Control Key (Pause = RControl2 + Num Lock)
    RControl2,
    /// Used as a 'hidden' Right Alt Key (Print Screen = RAlt2 + PrntScr)
    RAlt2,
}

/// The new state for a key, as part of a key event.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum KeyState {
    Up,
    Down,
}

/// A event describing something happen to a key on your keyboard.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct KeyEvent {
    /// Which key this event is for
    pub code: KeyCode,
    /// The new state for the key
    pub state: KeyState,
}

/// Describes a Keyboard Layout.
///
/// Layouts might include "en_US", or "en_GB", or "de_GR".
pub trait KeyboardLayout {
    /// Convert a `KeyCode` enum to a Unicode character, if possible.
    /// `KeyCode::A` maps to `DecodedKey::Unicode('a')` (or
    /// `DecodedKey::Unicode('A')` if shifted), while `KeyCode::LAlt` becomes
    /// `DecodedKey::RawKey(KeyCode::LAlt)` because there's no Unicode equivalent.
    fn map_keycode(&self, keycode: KeyCode, modifiers: &Modifiers) -> DecodedKey;
}

/// A mechanism to convert bytes from a Keyboard into [`KeyCode`] values.
///
/// This conversion is stateful.
pub trait ScancodeSet {
    /// Handles the state logic for the decoding of scan codes into key events.
    fn advance_state(&mut self, code: u8) -> Result<Option<KeyEvent>, Error>;
}

/// The set of modifier keys you have on a keyboard.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Modifiers {
    /// The left shift key is down
    pub lshift: bool,
    /// The right shift key is down
    pub rshift: bool,
    /// The left control key is down
    pub lctrl: bool,
    /// The right control key is down
    pub rctrl: bool,
    /// The Num Lock toggle is on
    pub numlock: bool,
    /// The caps lock toggle is on
    pub capslock: bool,
    /// The left alt key is down
    pub lalt: bool,
    /// The right alt key is down
    pub ralt: bool,
    /// Special 'hidden' control key is down (used when you press Pause)
    pub rctrl2: bool,
}

/// Contains either a Unicode character, or a raw key code.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum DecodedKey {
    RawKey(KeyCode),
    Unicode(char),
}

// ****************************************************************************
//
// Public Data
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Private Types
//
// ****************************************************************************

/// Tracls
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum DecodeState {
    Start,
    Extended,
    Extended2,
}

// ****************************************************************************
//
// Private Data
//
// ****************************************************************************

const EXTENDED_KEY_CODE: u8 = 0xE0;
const EXTENDED2_KEY_CODE: u8 = 0xE1;

// ****************************************************************************
//
// Public Functions and Implementation
//
// ****************************************************************************

impl<L, S> Keyboard<L, S>
where
    L: KeyboardLayout,
    S: ScancodeSet,
{
    /// Make a new Keyboard object with the given layout.
    pub const fn new(scancode_set: S, layout: L) -> Keyboard<L, S> {
        Keyboard {
            scancode_set,
            event_decoder: EventDecoder::new(layout),
        }
    }

    /// Processes an 8-bit byte from the keyboard.
    ///
    /// We assume the start, stop and parity bits have been processed and
    /// verified.
    pub fn add_byte(&mut self, byte: u8) -> Result<Option<KeyEvent>, Error> {
        self.scancode_set.advance_state(byte)
    }

    /// Processes a `KeyEvent` returned from `add_bit`, `add_byte` or `add_word`
    /// and produces a decoded key.
    ///
    /// For example, the KeyEvent for pressing the '5' key on your keyboard
    /// gives a DecodedKey of unicode character '5', unless the shift key is
    /// held in which case you get the unicode character '%'.
    pub fn process_keyevent(&mut self, ev: KeyEvent) -> Option<DecodedKey> {
        self.event_decoder.process_keyevent(ev)
    }
}

impl<L> EventDecoder<L>
where
    L: KeyboardLayout,
{
    pub const fn new(layout: L) -> EventDecoder<L> {
        EventDecoder {
            modifiers: Modifiers {
                lshift: false,
                rshift: false,
                lctrl: false,
                rctrl: false,
                numlock: true,
                capslock: false,
                lalt: false,
                ralt: false,
                rctrl2: false,
            },
            layout,
        }
    }

    pub fn process_keyevent(&mut self, ev: KeyEvent) -> Option<DecodedKey> {
        match ev {
            KeyEvent {
                code: KeyCode::LShift,
                state: KeyState::Down,
            } => {
                self.modifiers.lshift = true;
                Some(DecodedKey::RawKey(KeyCode::LShift))
            }
            KeyEvent {
                code: KeyCode::RShift,
                state: KeyState::Down,
            } => {
                self.modifiers.rshift = true;
                Some(DecodedKey::RawKey(KeyCode::RShift))
            }
            KeyEvent {
                code: KeyCode::LShift,
                state: KeyState::Up,
            } => {
                self.modifiers.lshift = false;
                None
            }
            KeyEvent {
                code: KeyCode::RShift,
                state: KeyState::Up,
            } => {
                self.modifiers.rshift = false;
                None
            }
            KeyEvent {
                code: KeyCode::CapsLock,
                state: KeyState::Down,
            } => {
                self.modifiers.capslock = !self.modifiers.capslock;
                Some(DecodedKey::RawKey(KeyCode::CapsLock))
            }
            KeyEvent {
                code: KeyCode::NumpadLock,
                state: KeyState::Down,
            } => {
                if self.modifiers.rctrl2 {
                    // It's a Pause key because we got the 'hidden' rctrl2
                    // sequence first.
                    Some(DecodedKey::RawKey(KeyCode::PauseBreak))
                } else {
                    // It's a numlock toggle
                    self.modifiers.numlock = !self.modifiers.numlock;
                    Some(DecodedKey::RawKey(KeyCode::NumpadLock))
                }
            }
            KeyEvent {
                code: KeyCode::LControl,
                state: KeyState::Down,
            } => {
                self.modifiers.lctrl = true;
                Some(DecodedKey::RawKey(KeyCode::LControl))
            }
            KeyEvent {
                code: KeyCode::LControl,
                state: KeyState::Up,
            } => {
                self.modifiers.lctrl = false;
                None
            }
            KeyEvent {
                code: KeyCode::RControl,
                state: KeyState::Down,
            } => {
                self.modifiers.rctrl = true;
                Some(DecodedKey::RawKey(KeyCode::RControl))
            }
            KeyEvent {
                code: KeyCode::RControl,
                state: KeyState::Up,
            } => {
                self.modifiers.rctrl = false;
                None
            }
            KeyEvent {
                code: KeyCode::LAlt,
                state: KeyState::Down,
            } => {
                self.modifiers.lalt = true;
                Some(DecodedKey::RawKey(KeyCode::LAlt))
            }
            KeyEvent {
                code: KeyCode::LAlt,
                state: KeyState::Up,
            } => {
                self.modifiers.lalt = false;
                None
            }
            KeyEvent {
                code: KeyCode::RAltGr,
                state: KeyState::Down,
            } => {
                self.modifiers.ralt = true;
                Some(DecodedKey::RawKey(KeyCode::RAltGr))
            }
            KeyEvent {
                code: KeyCode::RAltGr,
                state: KeyState::Up,
            } => {
                self.modifiers.ralt = false;
                None
            }
            KeyEvent {
                code: KeyCode::RControl2,
                state: KeyState::Down,
            } => {
                self.modifiers.rctrl2 = true;
                Some(DecodedKey::RawKey(KeyCode::RControl2))
            }
            KeyEvent {
                code: KeyCode::RControl2,
                state: KeyState::Up,
            } => {
                self.modifiers.rctrl2 = false;
                None
            }
            KeyEvent {
                code: c,
                state: KeyState::Down,
            } => Some(self.layout.map_keycode(c, &self.modifiers)),
            _ => None,
        }
    }
}

impl KeyEvent {
    pub const fn new(code: KeyCode, state: KeyState) -> KeyEvent {
        KeyEvent { code, state }
    }
}

// ****************************************************************************
//
// Keyboard Layouts
//
// ****************************************************************************

impl Modifiers {
    pub const fn is_shifted(&self) -> bool {
        self.lshift | self.rshift
    }

    pub const fn is_caps(&self) -> bool {
        self.is_shifted() ^ self.capslock
    }
}
