//! 🪟️ Renderer-neutral navigation values shared by exact-window configuration and scene protocols.

use protocol::value::{DslValue, FromValue, ValueError};

#[path = "◻️2d/🧬️schema/🦀️.rs"]
mod planar;
#[path = "🧊️3d/🧬️schema/🦀️.rs"]
mod orbit;
pub use planar::Viewport2d;
pub use orbit::*;

fn fields(value: DslValue, allowed: &[&str]) -> Result<Vec<(String, DslValue)>, ValueError> {
    let entries = value.into_object()?;
    if entries.len() > allowed.len() { return Err(ValueError::new("too many viewport fields")); }
    for (index, (name, _)) in entries.iter().enumerate() {
        if !allowed.contains(&name.as_str()) || entries[..index].iter().any(|(prior, _)| prior == name) {
            return Err(ValueError::new("unknown or duplicate viewport field").under(name));
        }
    }
    Ok(entries)
}

fn take<T: FromValue>(entries: &mut Vec<(String, DslValue)>, name: &str) -> Result<T, ValueError> {
    let index = entries.iter().position(|(key, _)| key == name).ok_or_else(|| ValueError::new("missing viewport field").under(name))?;
    T::from_value(entries.swap_remove(index).1).map_err(|error| error.under(name))
}

fn finish(entries: Vec<(String, DslValue)>) -> Result<(), ValueError> {
    if let Some((name, _)) = entries.into_iter().next() { Err(ValueError::new("field does not belong to selected viewport variant").under(name)) } else { Ok(()) }
}

fn finite(value: f64, name: &str) -> Result<(), ValueError> {
    if value.is_finite() { Ok(()) } else { Err(ValueError::new("expected finite coordinate").under(name)) }
}

fn zoom(value: f64) -> Result<(), ValueError> {
    if value.is_finite() && value > 0.0 { Ok(()) } else { Err(ValueError::new("expected positive finite zoom").under("zoom")) }
}

#[cfg(test)]
#[path = "🧪️tests/🪟️poses/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/📐️projection/🦀️.rs"]
mod projection_tests;
