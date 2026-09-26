//! ✏️ Pure path-local edits shared by numeric controls and canvas gestures.
use crate::PathSegment;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslEnum)]
#[cfg_attr(test, derive(serde::Deserialize, serde::Serialize))]
#[value(tag = "kind", rename_all = "camelCase")]
#[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase"))]
pub enum PathEdit {
    Coordinate { index: usize, point: PathPoint, axis: PathAxis, value: f64 },
    Split { index: usize, t: f64 },
    Delete { index: usize },
    Reverse,
    Close { index: usize },
    Open { index: usize },
    Convert { index: usize, target: SegmentType },
    Join { index: usize, other: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslScalar)]
#[cfg_attr(test, derive(serde::Deserialize, serde::Serialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub enum SegmentType { Line, Cubic }

#[derive(Clone, Copy, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslScalar)]
#[cfg_attr(test, derive(serde::Deserialize, serde::Serialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub enum PathPoint { Anchor, Control1, Control2 }

#[derive(Clone, Copy, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslScalar)]
#[cfg_attr(test, derive(serde::Deserialize, serde::Serialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub enum PathAxis { X, Y }

fn endpoint(segment: &PathSegment) -> Option<[f64; 2]> {
    match segment {
        PathSegment::Move { to } | PathSegment::Line { to } | PathSegment::Quad { to, .. } | PathSegment::Cubic { to, .. } | PathSegment::Arc { to, .. } => Some(*to),
        PathSegment::Close => None,
    }
}

fn contours(segments: &[PathSegment]) -> Result<Vec<(usize, usize)>, &'static str> {
    let mut ranges = Vec::new();
    let mut start = None;
    for (index, segment) in segments.iter().enumerate() {
        match segment {
            PathSegment::Move { .. } => { if let Some(start) = start { ranges.push((start, index)); } start = Some(index); }
            _ => {
                let current = start.ok_or("A contour must start with a move")?;
                if matches!(segment, PathSegment::Close) { ranges.push((current, index + 1)); start = None; }
            }
        }
    }
    if let Some(start) = start { ranges.push((start, segments.len())); }
    Ok(ranges)
}

pub fn edit_path(source: &[PathSegment], operation: &PathEdit) -> Result<Vec<PathSegment>, &'static str> {
    if !source.iter().all(crate::schema::valid_path_segment) { return Err("Invalid path geometry"); }
    let ranges = contours(source)?;
    if let PathEdit::Join { index,other } = *operation {
        if index == other { return Err("Choose two different endpoints"); }
        let range = |index| ranges.iter().copied().find(|(start,end)| (index == *start || index == end-1) && !matches!(source[end-1],PathSegment::Close)).ok_or("Choose endpoints of open contours");
        let a = range(index)?;
        let b = range(other)?;
        if a == b { return edit_path(source,&PathEdit::Close { index }); }
        let mut joined = if index == a.0 { edit_path(&source[a.0..a.1],&PathEdit::Reverse)? } else { source[a.0..a.1].to_vec() };
        let target = if other == b.0 { source[b.0..b.1].to_vec() } else { edit_path(&source[b.0..b.1],&PathEdit::Reverse)? };
        let to = endpoint(&target[0]).ok_or("Missing target endpoint")?;
        if joined.last().and_then(endpoint) != Some(to) { joined.push(PathSegment::Line { to }); }
        joined.extend_from_slice(&target[1..]);
        let mut output = Vec::with_capacity(source.len()+1);
        for (start,end) in ranges {
            if start == a.0.min(b.0) { output.append(&mut joined); }
            if start != a.0 && start != b.0 { output.extend_from_slice(&source[start..end]); }
        }
        return Ok(output);
    }
    if matches!(operation, PathEdit::Reverse) {
        let mut output = Vec::with_capacity(source.len());
        for (start, end) in ranges {
            let closed = matches!(source[end - 1], PathSegment::Close);
            let last = end - if closed { 2 } else { 1 };
            output.push(PathSegment::Move { to: endpoint(&source[last]).ok_or("Missing endpoint")? });
            for index in (start + 1..=last).rev() {
                let to = endpoint(&source[index - 1]).ok_or("Invalid contour")?;
                output.push(match source[index] {
                    PathSegment::Line { .. } => PathSegment::Line { to },
                    PathSegment::Quad { ctrl, .. } => PathSegment::Quad { ctrl, to },
                    PathSegment::Cubic { ctrl1, ctrl2, .. } => PathSegment::Cubic { ctrl1: ctrl2, ctrl2: ctrl1, to },
                    PathSegment::Arc { rx, ry, rotation, large_arc, sweep, .. } => PathSegment::Arc { rx, ry, rotation, large_arc, sweep: !sweep, to },
                    _ => return Err("Invalid contour"),
                });
            }
            if closed { output.push(PathSegment::Close); }
        }
        return Ok(output);
    }
    let index = match operation { PathEdit::Coordinate { index, .. } | PathEdit::Split { index, .. } | PathEdit::Delete { index } | PathEdit::Close { index } | PathEdit::Open { index } | PathEdit::Convert { index, .. } => *index, PathEdit::Reverse | PathEdit::Join { .. } => unreachable!() };
    let item = source.get(index).ok_or("Missing path node")?;
    let (start, end) = ranges.into_iter().find(|(start, end)| index >= *start && index < *end).ok_or("Missing contour")?;
    let mut output = source.to_vec();
    match *operation {
        PathEdit::Convert { target, .. } => {
            if matches!(item, PathSegment::Move { .. } | PathSegment::Close) { return Err("Select a segment to convert"); }
            let from = index.checked_sub(1).and_then(|index| endpoint(&source[index])).ok_or("Missing segment start")?;
            let to = endpoint(item).ok_or("Missing segment end")?;
            let mix = |a: [f64;2], b: [f64;2], t: f64| [a[0]*(1.0-t)+b[0]*t,a[1]*(1.0-t)+b[1]*t];
            let line = || PathSegment::Cubic { ctrl1: mix(from,to,1.0/3.0), ctrl2: mix(from,to,2.0/3.0), to };
            let replacements = if target == SegmentType::Line { vec![PathSegment::Line { to }] } else {
                match *item {
                    PathSegment::Line { .. } => vec![line()],
                    PathSegment::Quad { ctrl, .. } => vec![PathSegment::Cubic { ctrl1: mix(from,ctrl,2.0/3.0),ctrl2: mix(to,ctrl,2.0/3.0),to }],
                    PathSegment::Cubic { .. } => vec![item.clone()],
                    PathSegment::Arc { rx,ry,rotation,large_arc,sweep,.. } => {
                        let mut curves = super::super::arc_segment_to_cubics(from,rx,ry,rotation,large_arc,sweep,to).into_iter().map(|(ctrl1,ctrl2,to)| PathSegment::Cubic { ctrl1,ctrl2,to }).collect::<Vec<_>>();
                        if let Some(PathSegment::Cubic { to: end, .. }) = curves.last_mut() { *end=to; }
                        if curves.is_empty() { curves.push(line()); }
                        curves
                    }
                    _ => unreachable!(),
                }
            };
            output.splice(index..index+1,replacements);
        }
        PathEdit::Close { .. } | PathEdit::Open { .. } => {
            let closed = matches!(source[end - 1], PathSegment::Close);
            if matches!(operation, PathEdit::Open { .. }) && closed { output.remove(end - 1); }
            if matches!(operation, PathEdit::Close { .. }) && !closed {
                if end - start < 2 { return Err("A contour needs at least two anchors"); }
                output.insert(end, PathSegment::Close);
            }
        }
        PathEdit::Delete { .. } => match item {
            PathSegment::Close => return Err("Select an anchor to delete"),
            PathSegment::Move { .. } => {
                if let Some(next) = source.get(index + 1).filter(|next| !matches!(next, PathSegment::Move { .. } | PathSegment::Close)) {
                    output[index + 1] = PathSegment::Move { to: endpoint(next).ok_or("Missing next endpoint")? };
                    output.remove(index);
                } else { output.drain(start..end); }
            }
            _ => { output.remove(index); }
        },
        PathEdit::Coordinate { point, axis, value, .. } => {
            if !value.is_finite() { return Err("Invalid coordinate"); }
            let axis = if axis == PathAxis::X { 0 } else { 1 };
            if point == PathPoint::Anchor {
                let delta = value - endpoint(item).ok_or("Select an anchor")?[axis];
                match &mut output[index] {
                    PathSegment::Move { to } | PathSegment::Line { to } | PathSegment::Arc { to, .. } => to[axis] = value,
                    PathSegment::Quad { to, ctrl } => { to[axis] = value; ctrl[axis] += delta; }
                    PathSegment::Cubic { to, ctrl2, .. } => { to[axis] = value; ctrl2[axis] += delta; }
                    PathSegment::Close => return Err("Select an anchor"),
                }
                match output.get_mut(index + 1) {
                    Some(PathSegment::Quad { ctrl, .. }) => ctrl[axis] += delta,
                    Some(PathSegment::Cubic { ctrl1, .. }) => ctrl1[axis] += delta,
                    _ => {}
                }
            } else {
                match (&mut output[index], point) {
                    (PathSegment::Cubic { ctrl1, .. }, PathPoint::Control1) => ctrl1[axis] = value,
                    (PathSegment::Cubic { ctrl2, .. }, PathPoint::Control2) => ctrl2[axis] = value,
                    (PathSegment::Quad { ctrl, .. }, PathPoint::Control1) => ctrl[axis] = value,
                    _ => return Err("This node has no selected handle"),
                }
            }
        }
        PathEdit::Split { t, .. } => {
            if !(t > 0.0 && t < 1.0) { return Err("Split position must be between zero and one"); }
            let previous = index.checked_sub(1).and_then(|index| source.get(index)).and_then(endpoint).ok_or("Select a segment to split")?;
            let mix = |a: [f64;2], b: [f64;2]| [a[0]*(1.0-t)+b[0]*t, a[1]*(1.0-t)+b[1]*t];
            let replacements = match *item {
                PathSegment::Line { to } => vec![PathSegment::Line { to: mix(previous, to) }, item.clone()],
                PathSegment::Quad { ctrl, to } => {
                    let a = mix(previous, ctrl); let b = mix(ctrl, to);
                    vec![PathSegment::Quad { ctrl: a, to: mix(a,b) }, PathSegment::Quad { ctrl: b, to }]
                }
                PathSegment::Cubic { ctrl1, ctrl2, to } => super::split_cubic([previous,ctrl1,ctrl2,to],t).ok_or("Invalid split geometry")?.into_iter().map(|points| PathSegment::Cubic { ctrl1: points[1], ctrl2: points[2], to: points[3] }).collect(),
                PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => {
                    if rx == 0.0 || ry == 0.0 || previous == to { vec![PathSegment::Line { to: mix(previous,to) }, PathSegment::Line { to }] }
                    else {
                        let arc = super::arc_geometry(previous,[rx,ry],rotation,large_arc,sweep,to).ok_or("Invalid arc geometry")?;
                        let [rx,ry] = arc.radii;
                        vec![PathSegment::Arc { rx, ry, rotation, large_arc: (arc.sweep*t).abs()>std::f64::consts::PI, sweep, to: arc.point(t) }, PathSegment::Arc { rx, ry, rotation, large_arc: (arc.sweep*(1.0-t)).abs()>std::f64::consts::PI, sweep, to }]
                    }
                }
                _ => return Err("Select a segment to split"),
            };
            output.splice(index..index + 1, replacements);
        }
        PathEdit::Reverse | PathEdit::Join { .. } => unreachable!(),
    }
    if !output.iter().all(crate::schema::valid_path_segment) { return Err("The edit exceeds finite coordinates"); }
    Ok(output)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
