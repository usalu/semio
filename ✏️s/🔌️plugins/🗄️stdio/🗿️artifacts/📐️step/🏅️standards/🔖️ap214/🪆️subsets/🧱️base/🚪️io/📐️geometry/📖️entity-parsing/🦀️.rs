//! 📖️ Entity parsing of the STEP geometry codec: the entity table, the slot, reference and number parsers and the B-spline attribute readers. See [`super`].

use super::*;

// #region 🔖️Parse
#[derive(Debug, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub(super) struct StepEntity {
    pub(super) entity_type: String,
    pub(super) attrs: String,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn parse_step_entities(input: &str) -> Result<HashMap<u64, StepEntity>, StepError> {
    let mut entities = HashMap::new();
    let data_start = input.find("DATA;").ok_or_else(|| StepError::Syntax("no DATA section found".to_string()))?;
    let data_end = input[data_start..].find("ENDSEC;").ok_or_else(|| StepError::Syntax("no ENDSEC after DATA".to_string()))?;
    let data_section = &input[data_start + 5..data_start + data_end];
    let joined = data_section.replace(['\n', '\r'], " ");
    for statement in joined.split(';') {
        let stmt = statement.trim();
        if stmt.is_empty() {
            continue;
        }
        if let Some(eq_pos) = stmt.find('=') {
            let id_part = stmt[..eq_pos].trim();
            let rest = stmt[eq_pos + 1..].trim();
            if let Some(id) = parse_entity_id(id_part) {
                if let Some(paren_pos) = rest.find('(') {
                    let entity_type = rest[..paren_pos].trim().to_uppercase();
                    let attrs = rest[paren_pos + 1..].trim().to_string();
                    entities.insert(id, StepEntity { entity_type, attrs });
                }
            }
        }
    }
    Ok(entities)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn parse_entity_id(s: &str) -> Option<u64> {
    s.trim().strip_prefix('#')?.parse().ok()
}

/// 🧭️ ISO 10303-42 `first_proj_axis(z_axis, ?)`: the default `ref_direction` of a placement that
/// leaves it unset — `+X` (or `+Y` when the axis is parallel to `X`) projected onto the axis plane.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn first_proj_axis(axis: Vec3) -> Vec3 {
    let z = axis.normalized().unwrap_or(Vec3::Z);
    let candidate = if z.cross(Vec3::X).norm() > 1e-9 { Vec3::X } else { Vec3::Y };
    (candidate - z * candidate.dot(z)).normalized().unwrap_or(Vec3::X)
}

/// 🧩️ Splits an entity's attribute list into its top-level slots (`'', #1, $, (#2, #3)` → four
/// slots), so optional (`$`) attributes keep their position instead of collapsing away.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn parse_slots(attrs: &str) -> Vec<&str> {
    let attrs = attrs.trim_end().strip_suffix(')').unwrap_or(attrs);
    let mut slots = Vec::new();
    let mut depth = 0usize;
    let mut in_string = false;
    let mut start = 0usize;
    for (index, byte) in attrs.bytes().enumerate() {
        match byte {
            b'\'' => in_string = !in_string,
            b'(' if !in_string => depth += 1,
            b')' if !in_string => depth = depth.saturating_sub(1),
            b',' if !in_string && depth == 0 => {
                slots.push(attrs[start..index].trim());
                start = index + 1;
            }
            _ => {}
        }
    }
    slots.push(attrs[start..].trim());
    slots
}

/// 📏️ Whether a B-spline's whole control polygon is collinear — such a curve is exactly the segment
/// between its end vertices (a B-spline stays inside its control polygon's convex hull, and a
/// degenerate hull is that segment), so it is read as the `Line` it is, oriented from the edge's
/// own start vertex like every `LINE` edge. Real Rhino exports write every straight edge of a
/// polyhedron as a degree-1 or degree-3 B-spline, and the kernel measures and imprints analytic
/// lines exactly where a NURBS carrier only approximates.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn straight_control_polygon(controls: &[Pnt3]) -> bool {
    let (Some(first), Some(last)) = (controls.first(), controls.last()) else { return false };
    let Some(unit) = (*last - *first).normalized() else { return false };
    controls.iter().all(|control| (*control - *first).cross(unit).norm() <= Tol::DEFAULT.0)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn parse_refs(attrs: &str) -> Vec<u64> {
    let mut refs = Vec::new();
    let bytes = attrs.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'#' {
            i += 1;
            let start = i;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }
            if i > start {
                if let Ok(num) = attrs[start..i].parse::<u64>() {
                    refs.push(num);
                }
            }
        } else {
            i += 1;
        }
    }
    refs
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn parse_list_refs(attrs: &str) -> Vec<u64> {
    if let Some(start) = attrs.find('(') {
        if let Some(end) = attrs[start..].find(')') {
            return parse_refs(&attrs[start + 1..start + end]);
        }
    }
    Vec::new()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn parse_floats(attrs: &str) -> Vec<f64> {
    let mut result = Vec::new();
    if let Some(start) = attrs.find('(') {
        if let Some(end) = attrs[start..].find(')') {
            let inner = &attrs[start + 1..start + end];
            for part in inner.split(',') {
                if let Ok(v) = part.trim().parse::<f64>() {
                    result.push(v);
                }
            }
        }
    }
    if result.is_empty() {
        for part in attrs.split(',') {
            let trimmed = part.trim().trim_matches('\'').trim_end_matches(')');
            if trimmed.starts_with('#') || trimmed.starts_with('.') || trimmed.is_empty() {
                continue;
            }
            if let Ok(v) = trimmed.parse::<f64>() {
                result.push(v);
            }
        }
    }
    result
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_ints_in_parens(s: &str) -> Vec<u32> {
    let mut result = Vec::new();
    for part in s.split(',') {
        let trimmed = part.trim().trim_matches('(').trim_matches(')').trim();
        if let Ok(v) = trimmed.parse::<u32>() {
            result.push(v);
        }
    }
    result
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn expand_knots(mults: &[u32], vals: &[f64]) -> Vec<f64> {
    let mut knots = Vec::new();
    for (&m, &v) in mults.iter().zip(vals.iter()) {
        for _ in 0..m {
            knots.push(v);
        }
    }
    knots
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn find_composite_bspline_attrs<'a>(attrs: &'a str, base_name: &str) -> Option<&'a str> {
    let with_knots = format!("{base_name}_WITH_KNOTS");
    if let Some(pos) = attrs.find(&with_knots) {
        return Some(&attrs[pos + with_knots.len()..]);
    }
    let anchored = format!("{base_name}(");
    if let Some(pos) = attrs.find(&anchored) {
        return Some(&attrs[pos + base_name.len()..]);
    }
    None
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn parse_bspline_curve_attrs(attrs: &str) -> Option<BsplineCurveAttributes> {
    let mut tokens = Vec::new();
    let mut depth = 0i32;
    let mut current = String::new();
    let mut groups: Vec<String> = Vec::new();
    for ch in attrs.chars() {
        match ch {
            '(' => {
                if depth == 0 && !current.trim().is_empty() {
                    tokens.push(current.trim().to_string());
                    current.clear();
                }
                depth += 1;
                current.push(ch);
            }
            ')' => {
                current.push(ch);
                depth -= 1;
                if depth == 0 {
                    groups.push(current.clone());
                    current.clear();
                }
            }
            ',' if depth == 0 => {
                let trimmed = current.trim().to_string();
                if !trimmed.is_empty() {
                    tokens.push(trimmed);
                }
                current.clear();
            }
            _ => current.push(ch),
        }
    }
    if !current.trim().is_empty() {
        tokens.push(current.trim().to_string());
    }
    let mut degree = None;
    for tok in &tokens {
        if tok.starts_with('\'') || tok.starts_with('.') {
            continue;
        }
        if let Ok(d) = tok.parse::<usize>() {
            degree = Some(d);
            break;
        }
    }
    let degree = degree?;
    if groups.len() < 3 {
        return None;
    }
    Some((degree, parse_refs(&groups[0]), parse_ints_in_parens(&groups[1]), parse_floats(&groups[2])))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_nested_refs(s: &str) -> Vec<Vec<u64>> {
    let mut rows: Vec<Vec<u64>> = Vec::new();
    let mut depth = 0i32;
    let mut current = String::new();
    for ch in s.chars() {
        match ch {
            '(' => {
                depth += 1;
                if depth >= 2 {
                    current.push(ch);
                }
            }
            ')' => {
                if depth >= 2 {
                    current.push(ch);
                }
                depth -= 1;
                if depth == 1 && !current.is_empty() {
                    rows.push(parse_refs(&current));
                    current.clear();
                }
            }
            ',' if depth == 1 => {
                if !current.is_empty() {
                    rows.push(parse_refs(&current));
                    current.clear();
                }
            }
            _ => {
                if depth >= 2 {
                    current.push(ch);
                }
            }
        }
    }
    rows
}

#[allow(clippy::type_complexity)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn parse_bspline_surface_attrs(attrs: &str) -> Option<(usize, usize, Vec<Vec<u64>>, Vec<u32>, Vec<u32>, Vec<f64>, Vec<f64>)> {
    let mut tokens = Vec::new();
    let mut depth = 0i32;
    let mut current = String::new();
    let mut groups: Vec<String> = Vec::new();
    for ch in attrs.chars() {
        match ch {
            '(' => {
                if depth == 0 && !current.trim().is_empty() {
                    tokens.push(current.trim().to_string());
                    current.clear();
                }
                depth += 1;
                current.push(ch);
            }
            ')' => {
                current.push(ch);
                depth -= 1;
                if depth == 0 {
                    groups.push(current.clone());
                    current.clear();
                }
            }
            ',' if depth == 0 => {
                let trimmed = current.trim().to_string();
                if !trimmed.is_empty() {
                    tokens.push(trimmed);
                }
                current.clear();
            }
            _ => current.push(ch),
        }
    }
    if !current.trim().is_empty() {
        tokens.push(current.trim().to_string());
    }
    let mut degrees: Vec<usize> = Vec::new();
    for tok in &tokens {
        if tok.starts_with('\'') || tok.starts_with('.') {
            continue;
        }
        if let Ok(d) = tok.parse::<usize>() {
            degrees.push(d);
        }
    }
    if degrees.len() < 2 || groups.len() < 5 {
        return None;
    }
    Some((degrees[0], degrees[1], parse_nested_refs(&groups[0]), parse_ints_in_parens(&groups[1]), parse_ints_in_parens(&groups[2]), parse_floats(&groups[3]), parse_floats(&groups[4])))
}
// #endregion 🔖️Parse
