//! 🖊️ DXF R12 carrier — writer AND reader, both the `dxf` 0.6 crate, matching
//! `dxf-crate-note-ink-reader` in `../../../🔮️oracle/🔣️.json`. Reproduces `NoteIntoDxf::serialize`'s
//! body exactly: only `Ink` blocks' raw `points.windows(2)` become `LINE` entities on layer `"0"` —
//! no transform, no visibility filter, no width.

use crate::{flatten_all, obj, Block, Json, NoteDoc};
use dxf::entities::{Entity, EntityType, Line};
use dxf::{Drawing, Point};

fn ink_lines(doc: &NoteDoc) -> Vec<([f64; 2], [f64; 2])> {
    let mut lines = Vec::new();
    for block in flatten_all(doc) {
        if let Block::Ink { points, .. } = block {
            for pair in points.windows(2) {
                lines.push((pair[0], pair[1]));
            }
        }
    }
    lines
}

/// ⏱️ Pins the `dxf` crate's wall-clock header stamps (`$TDCREATE`/`$TDUCREATE`/`$TDUPDATE`/
/// `$TDUUPDATE`, all group code 40) to a fixed Julian day, exactly the trick
/// `…✳️cad/🔬️probes/🦀️oracle-probe/src/main.rs::pin_wall_clock` already proved necessary and
/// correct for this same crate: the edit goes through the LIBRARY'S OWN text, re-parsed by
/// `Drawing::load` to confirm it is still valid DXF, never patched into opaque bytes.
fn pin_wall_clock(drawing: &Drawing) -> Result<Drawing, String> {
    const PINNED: &str = "2451545.0";
    const TIME_VARS: [&str; 4] = ["$TDCREATE", "$TDUCREATE", "$TDUPDATE", "$TDUUPDATE"];
    let mut buffer: Vec<u8> = Vec::new();
    drawing.save(&mut buffer).map_err(|e| format!("dxf save to buffer: {e}"))?;
    let text = String::from_utf8_lossy(&buffer).into_owned();
    let mut lines: Vec<String> = text.split('\n').map(|l| l.trim_end_matches('\r').to_string()).collect();
    let mut index = 0;
    while index + 2 < lines.len() {
        if TIME_VARS.contains(&lines[index].trim()) && lines[index + 1].trim() == "40" {
            lines[index + 2] = PINNED.to_string();
        }
        index += 1;
    }
    let pinned_text = lines.join("\r\n");
    Drawing::load(&mut pinned_text.as_bytes()).map_err(|e| format!("dxf reload pinned text: {e}"))
}

/// ✍️ Writes real DXF R12 bytes with the `dxf` crate's own `Drawing`/`save` — nothing hand-formatted.
pub fn write_dxf(doc: &NoteDoc) -> Result<Vec<u8>, String> {
    let mut drawing = Drawing::new();
    for (start, end) in ink_lines(doc) {
        let mut entity = Entity::new(EntityType::Line(Line { p1: Point::new(start[0], start[1], 0.0), p2: Point::new(end[0], end[1], 0.0), ..Default::default() }));
        entity.common.layer = "0".to_string();
        drawing.add_entity(entity);
    }
    let pinned = pin_wall_clock(&drawing)?;
    let mut bytes = Vec::new();
    pinned.save(&mut bytes).map_err(|e| format!("dxf save: {e}"))?;
    Ok(bytes)
}

/// 📖 Reads DXF bytes with the `dxf` crate's own `Drawing::load` and reduces every `LINE` entity to
/// its (start, end) pair — independent of this file's own writer, the same crate reading back what it
/// (or note's real serializer) wrote.
pub fn project_dxf(bytes: &[u8]) -> Result<Vec<([f64; 2], [f64; 2])>, String> {
    let drawing = Drawing::load(&mut std::io::Cursor::new(bytes)).map_err(|e| format!("dxf load: {e}"))?;
    let mut lines = Vec::new();
    for entity in drawing.entities() {
        if let EntityType::Line(line) = &entity.specific {
            lines.push(([line.p1.x, line.p1.y], [line.p2.x, line.p2.y]));
        }
    }
    Ok(lines)
}

pub fn project_dxf_json(bytes: &[u8]) -> Result<Json, String> {
    let lines = project_dxf(bytes)?;
    Ok(obj(vec![("lineCount", crate::Json::Int(lines.len() as i64)), ("lines", Json::Arr(lines.into_iter().map(|(a, b)| obj(vec![("start", crate::nums(&a)), ("end", crate::nums(&b))])).collect()))]))
}

/// ⚖️ Set-equality over (start,end) pairs — entity ORDER is writer freedom, per
/// `semantic-note-dxf-ink-v1`'s own `"arrays": "set"` declaration in `../../../🔮️oracle/🔣️.json`.
pub fn compare_dxf(expected: &[u8], actual: &[u8]) -> Result<(bool, Vec<String>), String> {
    let e = project_dxf(expected)?;
    let a = project_dxf(actual)?;
    const TOL: f64 = 1e-9;
    let close = |p: &[f64; 2], q: &[f64; 2]| (p[0] - q[0]).abs() < TOL && (p[1] - q[1]).abs() < TOL;
    let mut unmatched_actual: Vec<usize> = (0..a.len()).collect();
    let mut problems = Vec::new();
    for (start, end) in &e {
        let found = unmatched_actual.iter().position(|&i| close(&a[i].0, start) && close(&a[i].1, end));
        match found {
            Some(pos) => {
                unmatched_actual.remove(pos);
            }
            None => problems.push(format!("expected line start={start:?} end={end:?} not found in actual")),
        }
    }
    for &i in &unmatched_actual {
        problems.push(format!("actual has extra line start={:?} end={:?} not present in expected", a[i].0, a[i].1));
    }
    if e.len() != a.len() {
        problems.push(format!("line count differs: expected {} actual {}", e.len(), a.len()));
    }
    Ok((problems.is_empty(), problems))
}
