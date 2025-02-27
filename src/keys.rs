use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

const fn no_mod(code: KeyCode) -> KeyEvent {
    KeyEvent {
        code,
        modifiers: KeyModifiers::empty(),
    }
}

const fn with_mod(
    code: KeyCode,
    modifiers: KeyModifiers,
) -> KeyEvent {
    KeyEvent { code, modifiers }
}

pub const TAB_1: KeyEvent = no_mod(KeyCode::Char('1'));
pub const TAB_2: KeyEvent = no_mod(KeyCode::Char('2'));
pub const TAB_3: KeyEvent = no_mod(KeyCode::Char('3'));
pub const TAB_4: KeyEvent = no_mod(KeyCode::Char('4'));
pub const TAB_TOGGLE: KeyEvent = no_mod(KeyCode::Tab);
pub const TAB_TOGGLE_REVERSE: KeyEvent = no_mod(KeyCode::BackTab);
//TODO: https://github.com/extrawurst/gitui/issues/112
pub const TAB_TOGGLE_REVERSE_WINDOWS: KeyEvent =
    with_mod(KeyCode::BackTab, KeyModifiers::SHIFT);
pub const FOCUS_WORKDIR: KeyEvent = no_mod(KeyCode::Char('w'));
pub const FOCUS_STAGE: KeyEvent = no_mod(KeyCode::Char('s'));
pub const FOCUS_RIGHT: KeyEvent = no_mod(KeyCode::Right);
pub const FOCUS_LEFT: KeyEvent = no_mod(KeyCode::Left);
pub const FOCUS_ABOVE: KeyEvent = no_mod(KeyCode::Up);
pub const FOCUS_BELOW: KeyEvent = no_mod(KeyCode::Down);
pub const EXIT: KeyEvent =
    with_mod(KeyCode::Char('c'), KeyModifiers::CONTROL);
pub const EXIT_POPUP: KeyEvent = no_mod(KeyCode::Esc);
pub const CLOSE_MSG: KeyEvent = no_mod(KeyCode::Enter);
pub const OPEN_COMMIT: KeyEvent = no_mod(KeyCode::Char('c'));
pub const OPEN_COMMIT_EDITOR: KeyEvent =
    with_mod(KeyCode::Char('e'), KeyModifiers::CONTROL);
pub const OPEN_HELP: KeyEvent = no_mod(KeyCode::Char('h'));
pub const MOVE_LEFT: KeyEvent = no_mod(KeyCode::Left);
pub const MOVE_RIGHT: KeyEvent = no_mod(KeyCode::Right);
pub const HOME: KeyEvent = no_mod(KeyCode::Home);
pub const END: KeyEvent = no_mod(KeyCode::End);
pub const MOVE_UP: KeyEvent = no_mod(KeyCode::Up);
pub const MOVE_DOWN: KeyEvent = no_mod(KeyCode::Down);
pub const PAGE_DOWN: KeyEvent = no_mod(KeyCode::PageDown);
pub const PAGE_UP: KeyEvent = no_mod(KeyCode::PageUp);
pub const SHIFT_UP: KeyEvent =
    with_mod(KeyCode::Up, KeyModifiers::SHIFT);
pub const SHIFT_DOWN: KeyEvent =
    with_mod(KeyCode::Down, KeyModifiers::SHIFT);
pub const ENTER: KeyEvent = no_mod(KeyCode::Enter);
pub const EDIT_FILE: KeyEvent = no_mod(KeyCode::Char('e'));
pub const STATUS_STAGE_FILE: KeyEvent = no_mod(KeyCode::Enter);
pub const STATUS_STAGE_ALL: KeyEvent = no_mod(KeyCode::Char('a'));
pub const STATUS_RESET_FILE: KeyEvent =
    with_mod(KeyCode::Char('D'), KeyModifiers::SHIFT);
pub const DIFF_RESET_HUNK: KeyEvent = STATUS_RESET_FILE;
pub const STATUS_IGNORE_FILE: KeyEvent = no_mod(KeyCode::Char('i'));
pub const STASHING_SAVE: KeyEvent = no_mod(KeyCode::Char('s'));
pub const STASHING_TOGGLE_UNTRACKED: KeyEvent =
    no_mod(KeyCode::Char('u'));
pub const STASHING_TOGGLE_INDEX: KeyEvent =
    no_mod(KeyCode::Char('i'));
pub const STASH_APPLY: KeyEvent = no_mod(KeyCode::Enter);
pub const STASH_OPEN: KeyEvent = no_mod(KeyCode::Right);
pub const STASH_DROP: KeyEvent =
    with_mod(KeyCode::Char('D'), KeyModifiers::SHIFT);
pub const CMD_BAR_TOGGLE: KeyEvent = no_mod(KeyCode::Char('.'));
pub const LOG_COMMIT_DETAILS: KeyEvent = no_mod(KeyCode::Enter);
pub const LOG_TAG_COMMIT: KeyEvent = no_mod(KeyCode::Char('t'));
pub const COMMIT_AMEND: KeyEvent =
    with_mod(KeyCode::Char('a'), KeyModifiers::CONTROL);
