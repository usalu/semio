//! 📦️ Native IO payload ownership shared by registered directions and physical retirement.
use semio_framework_value::{ToValue,FromValue};
/// 📦️ The one payload envelope the whole io mechanism moves. **Payload law**: the `IoPayload` of
/// dialect D is D's own *native* encoding — `Binary` = its pack, `Text` = its DSL — EXCEPT for the
/// two carrier dialects (`CARRIER_BINARY`, `CARRIER_TEXT`), whose native encoding IS the raw
/// external file content. So: **open a file** = `io_identify(bytes)` → `io_run(io_route(carrier →
/// D))`; **save a file** = `io_run(io_route(D → carrier))`. This is the rule that stops an export
/// writing pack bytes into a `.gif`/`.png` file.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_value::RetireOwned)]
pub enum IoPayload {
    Text(String),
    Binary(Vec<u8>),
}
