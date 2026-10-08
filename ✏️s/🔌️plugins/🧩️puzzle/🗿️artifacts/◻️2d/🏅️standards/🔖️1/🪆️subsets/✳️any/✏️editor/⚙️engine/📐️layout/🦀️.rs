//! 📐️ Puzzle layout consumes admitted typed Board records and declared options.
use semio_framework_os_infinite::board::{ports::directed::BoardSnapshot,schema::layout::{RedrawLayoutOptions,LayoutControl,LayoutError}};
/// 🔀️ The shared typed Board owner chooses geometry from the admitted snapshot schema.
pub fn redraw_layout_snapshot(snapshot:&mut BoardSnapshot,options:&RedrawLayoutOptions,control:&mut LayoutControl<'_>)->Result<(),LayoutError>{semio_framework_os_infinite::board::schema::layout_inferences::redraw::redraw_layout(snapshot,options,control)}
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
