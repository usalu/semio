//! 👁️ Typed visibility flags with authored hidden precedence.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BoardVisibility { pub hidden:Option<bool>, pub visible:Option<bool>, pub locked:Option<bool> }
pub fn board_visible_option(flags:&BoardVisibility)->Option<bool>{flags.hidden.map(|hidden|!hidden).or(flags.visible)}
pub fn board_visible_or_true(flags:&BoardVisibility)->bool{board_visible_option(flags).unwrap_or(true)}
pub fn board_locked_option(flags:&BoardVisibility)->Option<bool>{flags.locked}
