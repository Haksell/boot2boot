use super::super::{Error, KeyCode, KeyEvent, KeyState};
use super::ScancodeSet;

const EXTENDED_KEY_CODE: u8 = 0xE0;
const EXTENDED2_KEY_CODE: u8 = 0xE1;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum DecodeState {
    Start,
    Extended,
    Extended2,
}

pub struct ScancodeSet1 {
    state: DecodeState,
}

impl ScancodeSet1 {
    pub const fn new() -> ScancodeSet1 {
        ScancodeSet1 {
            state: DecodeState::Start,
        }
    }

    fn map_scancode(code: u8) -> Result<KeyCode, Error> {
        match code {
            0x01 => Ok(KeyCode::Escape),
            0x02 => Ok(KeyCode::Key1),
            0x03 => Ok(KeyCode::Key2),
            0x04 => Ok(KeyCode::Key3),
            0x05 => Ok(KeyCode::Key4),
            0x06 => Ok(KeyCode::Key5),
            0x07 => Ok(KeyCode::Key6),
            0x08 => Ok(KeyCode::Key7),
            0x09 => Ok(KeyCode::Key8),
            0x0A => Ok(KeyCode::Key9),
            0x0B => Ok(KeyCode::Key0),
            0x0C => Ok(KeyCode::OemMinus),
            0x0D => Ok(KeyCode::OemPlus),
            0x0E => Ok(KeyCode::Backspace),
            0x0F => Ok(KeyCode::Tab),
            0x10 => Ok(KeyCode::Q),
            0x11 => Ok(KeyCode::W),
            0x12 => Ok(KeyCode::E),
            0x13 => Ok(KeyCode::R),
            0x14 => Ok(KeyCode::T),
            0x15 => Ok(KeyCode::Y),
            0x16 => Ok(KeyCode::U),
            0x17 => Ok(KeyCode::I),
            0x18 => Ok(KeyCode::O),
            0x19 => Ok(KeyCode::P),
            0x1A => Ok(KeyCode::Oem4),
            0x1B => Ok(KeyCode::Oem6),
            0x1C => Ok(KeyCode::Return),
            0x1D => Ok(KeyCode::LControl),
            0x1E => Ok(KeyCode::A),
            0x1F => Ok(KeyCode::S),
            0x20 => Ok(KeyCode::D),
            0x21 => Ok(KeyCode::F),
            0x22 => Ok(KeyCode::G),
            0x23 => Ok(KeyCode::H),
            0x24 => Ok(KeyCode::J),
            0x25 => Ok(KeyCode::K),
            0x26 => Ok(KeyCode::L),
            0x27 => Ok(KeyCode::Oem1),
            0x28 => Ok(KeyCode::Oem3),
            0x29 => Ok(KeyCode::Oem8),
            0x2A => Ok(KeyCode::LShift),
            0x2B => Ok(KeyCode::Oem7),
            0x2C => Ok(KeyCode::Z),
            0x2D => Ok(KeyCode::X),
            0x2E => Ok(KeyCode::C),
            0x2F => Ok(KeyCode::V),
            0x30 => Ok(KeyCode::B),
            0x31 => Ok(KeyCode::N),
            0x32 => Ok(KeyCode::M),
            0x33 => Ok(KeyCode::OemComma),
            0x34 => Ok(KeyCode::OemPeriod),
            0x35 => Ok(KeyCode::Oem2),
            0x36 => Ok(KeyCode::RShift),
            0x37 => Ok(KeyCode::NumpadMultiply),
            0x38 => Ok(KeyCode::LAlt),
            0x39 => Ok(KeyCode::Spacebar),
            0x3A => Ok(KeyCode::CapsLock),
            0x3B => Ok(KeyCode::F1),
            0x3C => Ok(KeyCode::F2),
            0x3D => Ok(KeyCode::F3),
            0x3E => Ok(KeyCode::F4),
            0x45 => Ok(KeyCode::NumpadLock),
            0x46 => Ok(KeyCode::ScrollLock),
            0x47 => Ok(KeyCode::Numpad7),
            0x48 => Ok(KeyCode::Numpad8),
            0x49 => Ok(KeyCode::Numpad9),
            0x4A => Ok(KeyCode::NumpadSubtract),
            0x4B => Ok(KeyCode::Numpad4),
            0x4C => Ok(KeyCode::Numpad5),
            0x4D => Ok(KeyCode::Numpad6),
            0x4E => Ok(KeyCode::NumpadAdd),
            0x4F => Ok(KeyCode::Numpad1),
            0x50 => Ok(KeyCode::Numpad2),
            0x51 => Ok(KeyCode::Numpad3),
            0x52 => Ok(KeyCode::Numpad0),
            0x53 => Ok(KeyCode::NumpadPeriod),
            0x54 => Ok(KeyCode::SysRq),
            0x56 => Ok(KeyCode::Oem5),
            _ => Err(Error::UnknownKeyCode),
        }
    }

    fn map_extended_scancode(code: u8) -> Result<KeyCode, Error> {
        match code {
            0x10 => Ok(KeyCode::PrevTrack),
            0x19 => Ok(KeyCode::NextTrack),
            0x1C => Ok(KeyCode::NumpadEnter),
            0x1D => Ok(KeyCode::RControl),
            0x20 => Ok(KeyCode::Mute),
            0x21 => Ok(KeyCode::Calculator),
            0x22 => Ok(KeyCode::Play),
            0x24 => Ok(KeyCode::Stop),
            0x2A => Ok(KeyCode::RAlt2),
            0x2E => Ok(KeyCode::VolumeDown),
            0x30 => Ok(KeyCode::VolumeUp),
            0x32 => Ok(KeyCode::WWWHome),
            0x35 => Ok(KeyCode::NumpadDivide),
            0x37 => Ok(KeyCode::PrintScreen),
            0x38 => Ok(KeyCode::RAltGr),
            0x47 => Ok(KeyCode::Home),
            0x48 => Ok(KeyCode::ArrowUp),
            0x49 => Ok(KeyCode::PageUp),
            0x4B => Ok(KeyCode::ArrowLeft),
            0x4D => Ok(KeyCode::ArrowRight),
            0x4F => Ok(KeyCode::End),
            0x50 => Ok(KeyCode::ArrowDown),
            0x51 => Ok(KeyCode::PageDown),
            0x52 => Ok(KeyCode::Insert),
            0x53 => Ok(KeyCode::Delete),
            0x5B => Ok(KeyCode::LWin),
            0x5C => Ok(KeyCode::RWin),
            0x5D => Ok(KeyCode::Apps),
            0x70 => Ok(KeyCode::Oem11),
            0x73 => Ok(KeyCode::Oem12),
            0x79 => Ok(KeyCode::Oem10),
            0x7B => Ok(KeyCode::Oem9),
            0x7D => Ok(KeyCode::Oem13),
            _ => Err(Error::UnknownKeyCode),
        }
    }

    fn map_extended2_scancode(code: u8) -> Result<KeyCode, Error> {
        match code {
            0x1D => Ok(KeyCode::RControl2),
            _ => Err(Error::UnknownKeyCode),
        }
    }
}

impl ScancodeSet for ScancodeSet1 {
    fn add_byte(&mut self, code: u8) -> Result<Option<KeyEvent>, Error> {
        match self.state {
            DecodeState::Start => match code {
                EXTENDED_KEY_CODE => {
                    self.state = DecodeState::Extended;
                    Ok(None)
                }
                EXTENDED2_KEY_CODE => {
                    self.state = DecodeState::Extended2;
                    Ok(None)
                }
                0x80..=0xFF => Ok(Some(KeyEvent::new(
                    Self::map_scancode(code - 0x80)?,
                    KeyState::Up,
                ))),
                _ => Ok(Some(KeyEvent::new(
                    Self::map_scancode(code)?,
                    KeyState::Down,
                ))),
            },
            DecodeState::Extended => {
                self.state = DecodeState::Start;
                match code {
                    0x80..=0xFF => Ok(Some(KeyEvent::new(
                        Self::map_extended_scancode(code - 0x80)?,
                        KeyState::Up,
                    ))),
                    _ => Ok(Some(KeyEvent::new(
                        Self::map_extended_scancode(code)?,
                        KeyState::Down,
                    ))),
                }
            }
            DecodeState::Extended2 => {
                self.state = DecodeState::Start;
                match code {
                    0x80..=0xFF => Ok(Some(KeyEvent::new(
                        Self::map_extended2_scancode(code - 0x80)?,
                        KeyState::Up,
                    ))),
                    _ => Ok(Some(KeyEvent::new(
                        Self::map_extended2_scancode(code)?,
                        KeyState::Down,
                    ))),
                }
            }
        }
    }
}
