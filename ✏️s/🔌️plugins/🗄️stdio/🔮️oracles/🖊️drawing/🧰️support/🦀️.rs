//! 🧰️ Private shared DXF library operations behind owned byte and JSON entry points.

use semio_repo_test_host::Json;
use dxf::{Drawing, Point};

pub(super) fn load(bytes: &[u8]) -> Result<Drawing, String> {
        Drawing::load(&mut &bytes[..]).map_err(|error| format!("dxf oracle: load failed: {error:?}"))
    }

pub(super) fn point_json(p: &Point) -> Json {
        Json::Array(vec![Json::Number(p.x), Json::Number(p.y), Json::Number(p.z)])
    }

pub(super) fn obj(entries: Vec<(&str, Json)>) -> Json {
        Json::Object(entries.into_iter().map(|(k, v)| (k.to_string(), v)).collect())
    }
