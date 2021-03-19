// //! Convert Wayland keys to winit keys.

use crate::keyboard::{Key, KeyCode, NativeKeyCode};

pub fn rawkey_to_keycode(rawkey: u32) -> KeyCode {
    // keycodes are  taken from linux/include/uapi/linux/input-event-codes.h
    match rawkey {
        0 => todo!("What should be done in this case? Return `NativeKeyCode`, or perhaps `None`?"),
        1 => KeyCode::Escape,
        2 => KeyCode::Digit1,
        3 => KeyCode::Digit2,
        4 => KeyCode::Digit3,
        5 => KeyCode::Digit4,
        6 => KeyCode::Digit5,
        7 => KeyCode::Digit6,
        8 => KeyCode::Digit7,
        9 => KeyCode::Digit8,
        10 => KeyCode::Digit9,
        11 => KeyCode::Digit0,
        12 => KeyCode::Minus,
        13 => KeyCode::Equal,
        14 => KeyCode::Backspace,
        15 => KeyCode::Tab,
        16 => KeyCode::KeyQ,
        17 => KeyCode::KeyW,
        18 => KeyCode::KeyE,
        19 => KeyCode::KeyR,
        20 => KeyCode::KeyT,
        21 => KeyCode::KeyY,
        22 => KeyCode::KeyU,
        23 => KeyCode::KeyI,
        24 => KeyCode::KeyO,
        25 => KeyCode::KeyP,
        26 => KeyCode::BracketLeft,
        27 => KeyCode::BracketRight,
        28 => KeyCode::Enter,
        29 => KeyCode::ControlLeft,
        30 => KeyCode::KeyA,
        31 => KeyCode::KeyS,
        32 => KeyCode::KeyD,
        33 => KeyCode::KeyF,
        34 => KeyCode::KeyG,
        35 => KeyCode::KeyH,
        36 => KeyCode::KeyJ,
        37 => KeyCode::KeyK,
        38 => KeyCode::KeyL,
        39 => KeyCode::Semicolon,
        40 => KeyCode::Quote,
        41 => KeyCode::Backquote,
        42 => KeyCode::ShiftLeft,
        43 => KeyCode::Backslash,
        44 => KeyCode::KeyZ,
        45 => KeyCode::KeyX,
        46 => KeyCode::KeyC,
        47 => KeyCode::KeyV,
        48 => KeyCode::KeyB,
        49 => KeyCode::KeyN,
        50 => KeyCode::KeyM,
        51 => KeyCode::Comma,
        52 => KeyCode::Period,
        53 => KeyCode::Slash,
        54 => KeyCode::ShiftRight,
        55 => KeyCode::NumpadMultiply,
        56 => KeyCode::AltLeft,
        57 => KeyCode::Space,
        58 => KeyCode::CapsLock,
        59 => KeyCode::F1,
        60 => KeyCode::F2,
        61 => KeyCode::F3,
        62 => KeyCode::F4,
        63 => KeyCode::F5,
        64 => KeyCode::F6,
        65 => KeyCode::F7,
        66 => KeyCode::F8,
        67 => KeyCode::F9,
        68 => KeyCode::F10,
        69 => KeyCode::NumLock,
        70 => KeyCode::ScrollLock,
        71 => KeyCode::Numpad7,
        72 => KeyCode::Numpad8,
        73 => KeyCode::Numpad9,
        74 => KeyCode::NumpadSubtract,
        75 => KeyCode::Numpad4,
        76 => KeyCode::Numpad4,
        77 => KeyCode::Numpad6,
        78 => KeyCode::NumpadAdd,
        79 => KeyCode::Numpad1,
        80 => KeyCode::Numpad2,
        81 => KeyCode::Numpad3,
        82 => KeyCode::Numpad0,
        83 => KeyCode::NumpadDecimal,
        85 => KeyCode::Lang5,
        86 => KeyCode::IntlBackslash, // TODO: Verify.
        87 => KeyCode::F11,
        88 => KeyCode::F12,
        89 => KeyCode::IntlRo,
        90 => KeyCode::Lang3,
        91 => KeyCode::Lang4,
        92 => KeyCode::Convert,
        93 => KeyCode::KanaMode,
        94 => KeyCode::NonConvert,
        // 95 => KeyCode::KPJPCOMMA, // TODO: What the heck is this supposed to be?
        96 => KeyCode::NumpadEnter,
        97 => KeyCode::ControlRight,
        98 => KeyCode::NumpadDivide,
        99 => KeyCode::PrintScreen, // TODO: Verify.
        100 => KeyCode::AltRight,
        // 101 => KeyCode::LINEFEED, // TODO: What the heck is this supposed to be?
        102 => KeyCode::Home,
        103 => KeyCode::ArrowUp,
        104 => KeyCode::PageUp,
        105 => KeyCode::ArrowLeft,
        106 => KeyCode::ArrowRight,
        107 => KeyCode::End,
        108 => KeyCode::ArrowDown,
        109 => KeyCode::PageDown,
        110 => KeyCode::Insert,
        111 => KeyCode::Delete,
        // 112 => KeyCode::MACRO, // TODO: What the heck is this supposed to be?
        113 => KeyCode::AudioVolumeMute,
        114 => KeyCode::AudioVolumeDown,
        115 => KeyCode::AudioVolumeUp,
        // TODO: I have no idea if this should be mapped to `KeyCode::Power`
        //       Neither the Linux header or the uievents-code document disambigues this.
        // 116 => KeyCode::POWER,
        117 => KeyCode::NumpadEqual,
        // 118 => KeyCode::KPPLUSMINUS, // TODO: What the heck is this supposed to be?
        119 => KeyCode::Pause,
        // 120 => KeyCode::SCALE, // TODO: What the heck is this supposed to be?
        121 => KeyCode::NumpadComma,
        122 => KeyCode::Lang1,
        123 => KeyCode::Lang2,
        124 => KeyCode::IntlYen,
        125 => KeyCode::SuperLeft,
        126 => KeyCode::SuperRight,
        127 => KeyCode::ContextMenu,
        // 128 => KeyCode::STOP,
        // 129 => KeyCode::AGAIN,
        // 130 => KeyCode::PROPS,
        // 131 => KeyCode::UNDO,
        // 132 => KeyCode::FRONT,
        // 133 => KeyCode::COPY,
        // 134 => KeyCode::OPEN,
        // 135 => KeyCode::PASTE,
        // 136 => KeyCode::FIND,
        // 137 => KeyCode::CUT,
        // 138 => KeyCode::HELP,
        // 139 => KeyCode::MENU,
        // 140 => KeyCode::CALC,
        // 141 => KeyCode::SETUP,
        // 142 => KeyCode::SLEEP,
        // 143 => KeyCode::WAKEUP,
        // 144 => KeyCode::FILE,
        // 145 => KeyCode::SENDFILE,
        // 146 => KeyCode::DELETEFILE,
        // 147 => KeyCode::XFER,
        // 148 => KeyCode::PROG1,
        // 149 => KeyCode::PROG2,
        // 150 => KeyCode::WWW,
        // 151 => KeyCode::MSDOS,
        // 152 => KeyCode::COFFEE,
        // 153 => KeyCode::ROTATE_DISPLAY,
        // 154 => KeyCode::CYCLEWINDOWS,
        // 155 => KeyCode::MAIL,
        // 156 => KeyCode::BOOKMARKS,
        // 157 => KeyCode::COMPUTER,
        // 158 => KeyCode::BACK,
        // 159 => KeyCode::FORWARD,
        // 160 => KeyCode::CLOSECD,
        // 161 => KeyCode::EJECTCD,
        // 162 => KeyCode::EJECTCLOSECD,
        163 => KeyCode::MediaTrackNext,
        164 => KeyCode::MediaPlayPause,
        165 => KeyCode::MediaTrackPrevious,
        166 => KeyCode::MediaStop,
        // 167 => KeyCode::RECORD,
        // 168 => KeyCode::REWIND,
        // 169 => KeyCode::PHONE,
        // 170 => KeyCode::ISO,
        // 171 => KeyCode::CONFIG,
        // 172 => KeyCode::HOMEPAGE,
        // 173 => KeyCode::REFRESH,
        // 174 => KeyCode::EXIT,
        // 175 => KeyCode::MOVE,
        // 176 => KeyCode::EDIT,
        // 177 => KeyCode::SCROLLUP,
        // 178 => KeyCode::SCROLLDOWN,
        // 179 => KeyCode::KPLEFTPAREN,
        // 180 => KeyCode::KPRIGHTPAREN,
        // 181 => KeyCode::NEW,
        // 182 => KeyCode::REDO,
        183 => KeyCode::F13,
        184 => KeyCode::F14,
        185 => KeyCode::F15,
        186 => KeyCode::F16,
        187 => KeyCode::F17,
        188 => KeyCode::F18,
        189 => KeyCode::F19,
        190 => KeyCode::F20,
        191 => KeyCode::F21,
        192 => KeyCode::F22,
        193 => KeyCode::F23,
        194 => KeyCode::F24,
        // 200 => KeyCode::PLAYCD,
        // 201 => KeyCode::PAUSECD,
        // 202 => KeyCode::PROG3,
        // 203 => KeyCode::PROG4,
        // 204 => KeyCode::DASHBOARD,
        // 205 => KeyCode::SUSPEND,
        // 206 => KeyCode::CLOSE,
        // 207 => KeyCode::PLAY,
        // 208 => KeyCode::FASTFORWARD,
        // 209 => KeyCode::BASSBOOST,
        // 210 => KeyCode::PRINT,
        // 211 => KeyCode::HP,
        // 212 => KeyCode::CAMERA,
        // 213 => KeyCode::SOUND,
        // 214 => KeyCode::QUESTION,
        // 215 => KeyCode::EMAIL,
        // 216 => KeyCode::CHAT,
        // 217 => KeyCode::SEARCH,
        // 218 => KeyCode::CONNECT,
        // 219 => KeyCode::FINANCE,
        // 220 => KeyCode::SPORT,
        // 221 => KeyCode::SHOP,
        // 222 => KeyCode::ALTERASE,
        // 223 => KeyCode::CANCEL,
        // 224 => KeyCode::BRIGHTNESSDOW,
        // 225 => KeyCode::BRIGHTNESSU,
        // 226 => KeyCode::MEDIA,
        // 227 => KeyCode::SWITCHVIDEOMODE,
        // 228 => KeyCode::KBDILLUMTOGGLE,
        // 229 => KeyCode::KBDILLUMDOWN,
        // 230 => KeyCode::KBDILLUMUP,
        // 231 => KeyCode::SEND,
        // 232 => KeyCode::REPLY,
        // 233 => KeyCode::FORWARDMAIL,
        // 234 => KeyCode::SAVE,
        // 235 => KeyCode::DOCUMENTS,
        // 236 => KeyCode::BATTERY,
        // 237 => KeyCode::BLUETOOTH,
        // 238 => KeyCode::WLAN,
        // 239 => KeyCode::UWB,
        240 => KeyCode::Unidentified(NativeKeyCode::Unidentified),
        // 241 => KeyCode::VIDEO_NEXT,
        // 242 => KeyCode::VIDEO_PREV,
        // 243 => KeyCode::BRIGHTNESS_CYCLE,
        // 244 => KeyCode::BRIGHTNESS_AUTO,
        _ => KeyCode::Unidentified(NativeKeyCode::XKB(rawkey)),
    }
}

pub fn keysym_to_key(keysym: u32) -> Key<'static> {
    use sctk::seat::keyboard::keysyms;
    match keysym {
        //         // Numbers.
        //         // keysyms::XKB_KEY_1 => Some(Key::Character("1")),
        //         // keysyms::XKB_KEY_2 => Some(Key::Character("2")),
        //         // keysyms::XKB_KEY_3 => Some(Key::Character("3")),
        //         // keysyms::XKB_KEY_4 => Some(Key::Character("4")),
        //         // keysyms::XKB_KEY_5 => Some(Key::Character("5")),
        //         // keysyms::XKB_KEY_6 => Some(Key::Character("6")),
        //         // keysyms::XKB_KEY_7 => Some(Key::Character("7")),
        //         // keysyms::XKB_KEY_8 => Some(Key::Character("8")),
        //         // keysyms::XKB_KEY_9 => Some(Key::Character("9")),
        //         // keysyms::XKB_KEY_0 => Some(Key::Character("0")),
        //         // Letters.
        //         // keysyms::XKB_KEY_A => Some(Key::Character("A")),
        //         // keysyms::XKB_KEY_a => Some(Key::Character("a")),
        //         // keysyms::XKB_KEY_B => Some(Key::Character("B")),
        //         // keysyms::XKB_KEY_b => Some(Key::Character("b")),
        //         // keysyms::XKB_KEY_C => Some(Key::Character("C")),
        //         // keysyms::XKB_KEY_c => Some(Key::Character("c")),
        //         // keysyms::XKB_KEY_D => Some(Key::Character("D")),
        //         // keysyms::XKB_KEY_d => Some(Key::Character("d")),
        //         // keysyms::XKB_KEY_E => Some(Key::Character("E")),
        //         // keysyms::XKB_KEY_e => Some(Key::Character("e")),
        //         // keysyms::XKB_KEY_F => Some(Key::Character("F")),
        //         // keysyms::XKB_KEY_f => Some(Key::Character("f")),
        //         // keysyms::XKB_KEY_G => Some(Key::Character("G")),
        //         // keysyms::XKB_KEY_g => Some(Key::Character("g")),
        //         // keysyms::XKB_KEY_H => Some(Key::Character("H")),
        //         // keysyms::XKB_KEY_h => Some(Key::Character("h")),
        //         // keysyms::XKB_KEY_I => Some(Key::Character("I")),
        //         // keysyms::XKB_KEY_i => Some(Key::Character("i")),
        //         // keysyms::XKB_KEY_J => Some(Key::Character("J")),
        //         // keysyms::XKB_KEY_j => Some(Key::Character("j")),
        //         // keysyms::XKB_KEY_K => Some(Key::Character("K")),
        //         // keysyms::XKB_KEY_k => Some(Key::Character("k")),
        //         // keysyms::XKB_KEY_L => Some(Key::Character("L")),
        //         // keysyms::XKB_KEY_l => Some(Key::Character("l")),
        //         // keysyms::XKB_KEY_M => Some(Key::Character("M")),
        //         // keysyms::XKB_KEY_m => Some(Key::Character("m")),
        //         // keysyms::XKB_KEY_N => Some(Key::Character("N")),
        //         // keysyms::XKB_KEY_n => Some(Key::Character("n")),
        //         // keysyms::XKB_KEY_O => Some(Key::Character("O")),
        //         // keysyms::XKB_KEY_o => Some(Key::Character("o")),
        //         // keysyms::XKB_KEY_P => Some(Key::Character("P")),
        //         // keysyms::XKB_KEY_p => Some(Key::Character("p")),
        //         // keysyms::XKB_KEY_Q => Some(Key::Character("Q")),
        //         // keysyms::XKB_KEY_q => Some(Key::Character("q")),
        //         // keysyms::XKB_KEY_R => Some(Key::Character("R")),
        //         // keysyms::XKB_KEY_r => Some(Key::Character("r")),
        //         // keysyms::XKB_KEY_S => Some(Key::Character("S")),
        //         // keysyms::XKB_KEY_s => Some(Key::Character("s")),
        //         // keysyms::XKB_KEY_T => Some(Key::Character("T")),
        //         // keysyms::XKB_KEY_t => Some(Key::Character("t")),
        //         // keysyms::XKB_KEY_U => Some(Key::Character("U")),
        //         // keysyms::XKB_KEY_u => Some(Key::Character("u")),
        //         // keysyms::XKB_KEY_V => Some(Key::Character("V")),
        //         // keysyms::XKB_KEY_v => Some(Key::Character("v")),
        //         // keysyms::XKB_KEY_W => Some(Key::Character("W")),
        //         // keysyms::XKB_KEY_w => Some(Key::Character("w")),
        //         // keysyms::XKB_KEY_X => Some(Key::Character("X")),
        //         // keysyms::XKB_KEY_x => Some(Key::Character("x")),
        //         // keysyms::XKB_KEY_Y => Some(Key::Character("Y")),
        //         // keysyms::XKB_KEY_y => Some(Key::Character("y")),
        //         // keysyms::XKB_KEY_Z => Some(Key::Character("Z")),
        //         // keysyms::XKB_KEY_z => Some(Key::Character("z")),
        //         // Escape.
        //         keysyms::XKB_KEY_Escape => Some(Key::Escape),
        //         // Function keys.
        //         keysyms::XKB_KEY_F1 => Some(Key::F1),
        //         keysyms::XKB_KEY_F2 => Some(Key::F2),
        //         keysyms::XKB_KEY_F3 => Some(Key::F3),
        //         keysyms::XKB_KEY_F4 => Some(Key::F4),
        //         keysyms::XKB_KEY_F5 => Some(Key::F5),
        //         keysyms::XKB_KEY_F6 => Some(Key::F6),
        //         keysyms::XKB_KEY_F7 => Some(Key::F7),
        //         keysyms::XKB_KEY_F8 => Some(Key::F8),
        //         keysyms::XKB_KEY_F9 => Some(Key::F9),
        //         keysyms::XKB_KEY_F10 => Some(Key::F10),
        //         keysyms::XKB_KEY_F11 => Some(Key::F11),
        //         keysyms::XKB_KEY_F12 => Some(Key::F12),
        //         keysyms::XKB_KEY_F13 => Some(Key::F13),
        //         keysyms::XKB_KEY_F14 => Some(Key::F14),
        //         keysyms::XKB_KEY_F15 => Some(Key::F15),
        //         keysyms::XKB_KEY_F16 => Some(Key::F16),
        //         keysyms::XKB_KEY_F17 => Some(Key::F17),
        //         keysyms::XKB_KEY_F18 => Some(Key::F18),
        //         keysyms::XKB_KEY_F19 => Some(Key::F19),
        //         keysyms::XKB_KEY_F20 => Some(Key::F20),
        //         keysyms::XKB_KEY_F21 => Some(Key::F21),
        //         keysyms::XKB_KEY_F22 => Some(Key::F22),
        //         keysyms::XKB_KEY_F23 => Some(Key::F23),
        //         keysyms::XKB_KEY_F24 => Some(Key::F24),
        //         keysyms::XKB_KEY_F25 => Some(Key::F25),
        //         keysyms::XKB_KEY_F26 => Some(Key::F26),
        //         keysyms::XKB_KEY_F27 => Some(Key::F27),
        //         keysyms::XKB_KEY_F28 => Some(Key::F28),
        //         keysyms::XKB_KEY_F29 => Some(Key::F29),
        //         keysyms::XKB_KEY_F30 => Some(Key::F30),
        //         keysyms::XKB_KEY_F31 => Some(Key::F31),
        //         keysyms::XKB_KEY_F32 => Some(Key::F32),
        //         keysyms::XKB_KEY_F33 => Some(Key::F33),
        //         keysyms::XKB_KEY_F34 => Some(Key::F34),
        //         keysyms::XKB_KEY_F35 => Some(Key::F35),
        //         // Flow control.
        //         keysyms::XKB_KEY_Print => Some(Key::PrintScreen),
        //         keysyms::XKB_KEY_Scroll_Lock => Some(Key::ScrollLock),
        //         keysyms::XKB_KEY_Pause => Some(Key::Pause),
        //         keysyms::XKB_KEY_Insert => Some(Key::Insert),
        //         keysyms::XKB_KEY_Home => Some(Key::Home),
        //         keysyms::XKB_KEY_Delete => Some(Key::Delete),
        //         keysyms::XKB_KEY_End => Some(Key::End),
        //         keysyms::XKB_KEY_Page_Down => Some(Key::PageDown),
        //         keysyms::XKB_KEY_Page_Up => Some(Key::PageUp),
        //         // Arrows.
        //         keysyms::XKB_KEY_Left => Some(Key::Left),
        //         keysyms::XKB_KEY_Up => Some(Key::Up),
        //         keysyms::XKB_KEY_Right => Some(Key::Right),
        //         keysyms::XKB_KEY_Down => Some(Key::Down),

        //         keysyms::XKB_KEY_BackSpace => Some(Key::Back),
        //         keysyms::XKB_KEY_Return => Some(Key::Return),
        //         keysyms::XKB_KEY_space => Some(Key::Space),

        //         keysyms::XKB_KEY_Multi_key => Some(Key::Compose),
        //         keysyms::XKB_KEY_caret => Some(Key::Caret),

        //         // Keypad.
        //         keysyms::XKB_KEY_Num_Lock => Some(Key::Numlock),
        //         keysyms::XKB_KEY_KP_0 => Some(Key::Numpad0),
        //         keysyms::XKB_KEY_KP_1 => Some(Key::Numpad1),
        //         keysyms::XKB_KEY_KP_2 => Some(Key::Numpad2),
        //         keysyms::XKB_KEY_KP_3 => Some(Key::Numpad3),
        //         keysyms::XKB_KEY_KP_4 => Some(Key::Numpad4),
        //         keysyms::XKB_KEY_KP_5 => Some(Key::Numpad5),
        //         keysyms::XKB_KEY_KP_6 => Some(Key::Numpad6),
        //         keysyms::XKB_KEY_KP_7 => Some(Key::Numpad7),
        //         keysyms::XKB_KEY_KP_8 => Some(Key::Numpad8),
        //         keysyms::XKB_KEY_KP_9 => Some(Key::Numpad9),
        //         // Misc.
        //         // => Some(Key::AbntC1),
        //         // => Some(Key::AbntC2),
        //         keysyms::XKB_KEY_plus => Some(Key::Plus),
        //         keysyms::XKB_KEY_apostrophe => Some(Key::Apostrophe),
        //         // => Some(Key::Apps),
        //         keysyms::XKB_KEY_at => Some(Key::At),
        //         // => Some(Key::Ax),
        //         keysyms::XKB_KEY_backslash => Some(Key::Backslash),
        //         keysyms::XKB_KEY_XF86Calculator => Some(Key::Calculator),
        //         // => Some(Key::Capital),
        //         keysyms::XKB_KEY_colon => Some(Key::Colon),
        //         keysyms::XKB_KEY_comma => Some(Key::Comma),
        //         // => Some(Key::Convert),
        //         keysyms::XKB_KEY_equal => Some(Key::Equals),
        //         keysyms::XKB_KEY_grave => Some(Key::Grave),
        //         // => Some(Key::Kana),
        //         keysyms::XKB_KEY_Kanji => Some(Key::Kanji),
        //         keysyms::XKB_KEY_Alt_L => Some(Key::LAlt),
        //         keysyms::XKB_KEY_bracketleft => Some(Key::LBracket),
        //         keysyms::XKB_KEY_Control_L => Some(Key::LControl),
        //         keysyms::XKB_KEY_Shift_L => Some(Key::LShift),
        //         keysyms::XKB_KEY_Super_L => Some(Key::LWin),
        //         keysyms::XKB_KEY_XF86Mail => Some(Key::Mail),
        //         // => Some(Key::MediaSelect),
        //         // => Some(Key::MediaStop),
        //         keysyms::XKB_KEY_minus => Some(Key::Minus),
        //         keysyms::XKB_KEY_asterisk => Some(Key::Asterisk),
        //         keysyms::XKB_KEY_XF86AudioMute => Some(Key::Mute),
        //         // => Some(Key::MyComputer),
        //         keysyms::XKB_KEY_XF86AudioNext => Some(Key::NextTrack),
        //         // => Some(Key::NoConvert),
        //         keysyms::XKB_KEY_KP_Separator => Some(Key::NumpadComma),
        //         keysyms::XKB_KEY_KP_Enter => Some(Key::NumpadEnter),
        //         keysyms::XKB_KEY_KP_Equal => Some(Key::NumpadEquals),
        //         keysyms::XKB_KEY_KP_Add => Some(Key::NumpadAdd),
        //         keysyms::XKB_KEY_KP_Subtract => Some(Key::NumpadSubtract),
        //         keysyms::XKB_KEY_KP_Multiply => Some(Key::NumpadMultiply),
        //         keysyms::XKB_KEY_KP_Divide => Some(Key::NumpadDivide),
        //         keysyms::XKB_KEY_KP_Decimal => Some(Key::NumpadDecimal),
        //         keysyms::XKB_KEY_KP_Page_Up => Some(Key::PageUp),
        //         keysyms::XKB_KEY_KP_Page_Down => Some(Key::PageDown),
        //         keysyms::XKB_KEY_KP_Home => Some(Key::Home),
        //         keysyms::XKB_KEY_KP_End => Some(Key::End),
        //         keysyms::XKB_KEY_KP_Left => Some(Key::Left),
        //         keysyms::XKB_KEY_KP_Up => Some(Key::Up),
        //         keysyms::XKB_KEY_KP_Right => Some(Key::Right),
        //         keysyms::XKB_KEY_KP_Down => Some(Key::Down),
        //         // => Some(Key::OEM102),
        //         keysyms::XKB_KEY_period => Some(Key::Period),
        //         // => Some(Key::Playpause),
        //         keysyms::XKB_KEY_XF86PowerOff => Some(Key::Power),
        //         keysyms::XKB_KEY_XF86AudioPrev => Some(Key::PrevTrack),
        //         keysyms::XKB_KEY_Alt_R => Some(Key::RAlt),
        //         keysyms::XKB_KEY_bracketright => Some(Key::RBracket),
        //         keysyms::XKB_KEY_Control_R => Some(Key::RControl),
        //         keysyms::XKB_KEY_Shift_R => Some(Key::RShift),
        //         keysyms::XKB_KEY_Super_R => Some(Key::RWin),
        //         keysyms::XKB_KEY_semicolon => Some(Key::Semicolon),
        //         keysyms::XKB_KEY_slash => Some(Key::Slash),
        //         keysyms::XKB_KEY_XF86Sleep => Some(Key::Sleep),
        //         // => Some(Key::Stop),
        //         // => Some(Key::Sysrq),
        //         keysyms::XKB_KEY_Tab => Some(Key::Tab),
        //         keysyms::XKB_KEY_ISO_Left_Tab => Some(Key::Tab),
        //         keysyms::XKB_KEY_underscore => Some(Key::Underline),
        //         // => Some(Key::Unlabeled),
        //         keysyms::XKB_KEY_XF86AudioLowerVolume => Some(Key::VolumeDown),
        //         keysyms::XKB_KEY_XF86AudioRaiseVolume => Some(Key::VolumeUp),
        //         // => Some(Key::Wake),
        //         // => Some(Key::Webback),
        //         // => Some(Key::WebFavorites),
        //         // => Some(Key::WebForward),
        //         // => Some(Key::WebHome),
        //         // => Some(Key::WebRefresh),
        //         // => Some(Key::WebSearch),
        //         // => Some(Key::WebStop),
        //         keysyms::XKB_KEY_yen => Some(Key::Yen),
        //         keysyms::XKB_KEY_XF86Copy => Some(Key::Copy),
        //         keysyms::XKB_KEY_XF86Paste => Some(Key::Paste),
        //         keysyms::XKB_KEY_XF86Cut => Some(Key::Cut),
        //         // Fallback.
        _ => Key::Unidentified(NativeKeyCode::Unidentified),
    }
}
