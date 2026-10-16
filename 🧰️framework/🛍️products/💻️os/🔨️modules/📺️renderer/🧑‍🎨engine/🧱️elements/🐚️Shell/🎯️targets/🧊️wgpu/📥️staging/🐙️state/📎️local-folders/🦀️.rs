//! 🐙️ `shell-state` staging of `local-folders`: RENDERER-INTEGRATION-2, the items the direction graph homes in this crate (they were in a higher crate or in the old chunk file).

use super::*;

//#region 🔖️Reconnect
/// 🚪️ Whether this build reopens a remembered folder by itself: the native shell owns a folder transport; the browser
/// reaches folders only through the dev host's route and, like React's browser shell, waits for the person's gesture.
pub const LOCAL_FOLDER_DIRECT_REATTACH: bool = cfg!(not(target_arch = "wasm32"));
//#endregion 🔖️Reconnect
