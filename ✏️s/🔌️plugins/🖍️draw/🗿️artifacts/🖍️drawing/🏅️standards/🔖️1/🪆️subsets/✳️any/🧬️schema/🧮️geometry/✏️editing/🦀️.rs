//! ✏️ Pure path-local edits shared by numeric controls and canvas gestures.
use crate::PathSegment;
#[path="🎛️node/🦀️.rs"]
mod node;
#[path="📉️simplify/🦀️.rs"]
pub mod simplify;

/// 📐️ Borrows ordinal path geometry from persisted pages or computed geometry buffers.
pub trait PathGeometrySource {
    fn path_len(&self) -> usize;
    fn path_segment(&self, index: usize) -> Option<&PathSegment>;
    fn path_segments(&self) -> impl Iterator<Item = &PathSegment> {
        (0..self.path_len()).map(|index| self.path_segment(index).expect("path ordinal remains present"))
    }
}

impl PathGeometrySource for [PathSegment] {
    fn path_len(&self) -> usize { self.len() }
    fn path_segment(&self, index: usize) -> Option<&PathSegment> { self.get(index) }
}

impl PathGeometrySource for Vec<PathSegment> {
    fn path_len(&self) -> usize { self.len() }
    fn path_segment(&self, index: usize) -> Option<&PathSegment> { self.get(index) }
}

impl<const N: usize> PathGeometrySource for semio_framework_value::list::PagedList<PathSegment, N> {
    fn path_len(&self) -> usize { self.len() }
    fn path_segment(&self, index: usize) -> Option<&PathSegment> { self.get(index) }
}

impl PathGeometrySource for std::borrow::Cow<'_, [PathSegment]> {
    fn path_len(&self) -> usize { self.len() }
    fn path_segment(&self, index: usize) -> Option<&PathSegment> { self.get(index) }
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum)]
#[cfg_attr(test, derive(serde::Deserialize, serde::Serialize))]
#[value(tag = "kind", rename_all = "camelCase")]
#[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase"))]
pub enum PathEdit {
    Node { index: usize, mode: NodeMode },
    Simplify { tolerance: f64 },
    DeletePoints { points: Vec<PathPointRef> },
    Translate { points: Vec<PathPointRef>, delta: [f64;2] },
    Position { index: usize, point: PathPoint, to: [f64;2] },
    Coordinate { index: usize, point: PathPoint, axis: PathAxis, value: f64 },
    Split { index: usize, t: f64 },
    Delete { index: usize },
    Reverse,
    Close { index: usize },
    Open { index: usize },
    Convert { index: usize, target: SegmentType },
    Join { index: usize, other: usize },
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Deserialize, serde::Serialize))]
#[dsl(keyword = "path-point")]
pub struct PathPointRef {
    pub index: usize,
    pub point: PathPoint,
}

#[derive(Clone, Copy, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslScalar)]
#[cfg_attr(test, derive(serde::Deserialize, serde::Serialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub enum SegmentType { Line, Cubic }

#[derive(Clone, Copy, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslScalar)]
#[cfg_attr(test, derive(serde::Deserialize, serde::Serialize))]
#[value(rename_all="camelCase")]
#[cfg_attr(test, serde(rename_all="camelCase"))]
pub enum NodeMode { Corner, Smooth, Symmetric }

#[derive(Clone, Copy, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Deserialize, serde::Serialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub enum PathPoint { Anchor, Control1, Control2 }

#[derive(Clone, Copy, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslScalar)]
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

fn contours(segments: &(impl PathGeometrySource + ?Sized)) -> Result<Vec<(usize, usize)>, &'static str> {
    let mut ranges = Vec::new();
    let mut start = None;
    for (index, segment) in segments.path_segments().enumerate() {
        match segment {
            PathSegment::Move { .. } => { if let Some(start) = start { ranges.push((start, index)); } start = Some(index); }
            _ => {
                let current = start.ok_or("A contour must start with a move")?;
                if matches!(segment, PathSegment::Close) { ranges.push((current, index + 1)); start = None; }
            }
        }
    }
    if let Some(start) = start { ranges.push((start, segments.path_len())); }
    Ok(ranges)
}

fn reverse_path_ranges(source: &(impl PathGeometrySource + ?Sized), ranges: &[(usize,usize)]) -> Result<Vec<PathSegment>, &'static str> {
        let mut output = Vec::with_capacity(ranges.iter().map(|(start,end)| end-start).sum());
        for &(start, end) in ranges {
            let closed = matches!(*source.path_segment(end - 1).expect("validated path ordinal"), PathSegment::Close);
            let last = end - if closed { 2 } else { 1 };
            output.push(PathSegment::Move { to: endpoint(&*source.path_segment(last).expect("validated path ordinal")).ok_or("Missing endpoint")? });
            for index in (start + 1..=last).rev() {
                let to = endpoint(&*source.path_segment(index - 1).expect("validated path ordinal")).ok_or("Invalid contour")?;
                output.push(match *source.path_segment(index).expect("validated path ordinal") {
                    PathSegment::Line { .. } => PathSegment::Line { to },
                    PathSegment::Quad { ctrl, .. } => PathSegment::Quad { ctrl, to },
                    PathSegment::Cubic { ctrl1, ctrl2, .. } => PathSegment::Cubic { ctrl1: ctrl2, ctrl2: ctrl1, to },
                    PathSegment::Arc { rx, ry, rotation, large_arc, sweep, .. } => PathSegment::Arc { rx, ry, rotation, large_arc, sweep: !sweep, to },
                    _ => return Err("Invalid contour"),
                });
            }
            if closed { output.push(PathSegment::Close); }
        }
    Ok(output)
}

pub fn edit_path(source: &(impl PathGeometrySource + ?Sized), operation: &PathEdit) -> Result<Vec<PathSegment>, &'static str> {
    if !source.path_segments().all(crate::schema::valid_path_segment) { return Err("Invalid path geometry"); }
    let ranges = contours(source)?;
    if let PathEdit::Node {index,mode}=*operation {return node::edit(source,index,mode);}
    if let PathEdit::Simplify {tolerance}=*operation {
        let mut job=simplify::PathSimplifyJob::new(source.path_segments().cloned().collect(),tolerance)?;
        if !job.advance(262144)?.done {return Err("Path simplification exceeds interactive planning capacity");}
        return job.into_result();
    }
    if let PathEdit::Translate {points,delta}=operation {return translate_path_points(source,points,*delta);}
    if let PathEdit::DeletePoints {points}=operation {
        if points.is_empty() {return Ok(source.path_segments().cloned().collect());}
        if points.len()>4096 {return Err("Point selection exceeds editing capacity");}
        let mut output: Vec<PathSegment> = source.path_segments().cloned().collect();
        let mut removed=std::collections::BTreeSet::new();
        for point in points {
            let segment=source.path_segment(point.index).ok_or("Missing path node")?;
            if matches!(segment,PathSegment::Close) {return Err("Missing path node");}
            if point.point==PathPoint::Anchor {removed.insert(point.index);continue;}
            match (segment,point.point) {
                (PathSegment::Quad {to,..},PathPoint::Control1)=>output[point.index]=PathSegment::Line {to:*to},
                (PathSegment::Cubic {..},PathPoint::Control1|PathPoint::Control2)=>{
                    let previous=point.index.checked_sub(1).and_then(|index|source.path_segment(index)).and_then(endpoint).ok_or("Missing previous anchor")?;
                    let PathSegment::Cubic {ctrl1,ctrl2,to}=&mut output[point.index] else {unreachable!()};
                    if point.point==PathPoint::Control1 {*ctrl1=previous;}else {*ctrl2=*to;}
                }
                _=>return Err("This node has no selected handle"),
            }
        }
        let mut result=Vec::new();
        for &(start,end) in &ranges {
            let closed=matches!(*source.path_segment(end-1).expect("validated path ordinal"),PathSegment::Close);
            let mut kept=0;
            for (index,segment) in output.iter().enumerate().take(end-usize::from(closed)).skip(start) {
                if removed.contains(&index) {continue;}
                result.push(if kept==0 {PathSegment::Move {to:endpoint(segment).ok_or("Invalid contour")?}}else {segment.clone()});
                kept+=1;
            }
            if closed && kept>1 {result.push(PathSegment::Close);}
        }
        return Ok(result);
    }

    if let PathEdit::Join { index,other } = *operation {
        if index == other { return Err("Choose two different endpoints"); }
        let range = |index| ranges.iter().copied().find(|(start,end)| (index == *start || index == end-1) && !matches!(*source.path_segment(end-1).expect("validated path ordinal"),PathSegment::Close)).ok_or("Choose endpoints of open contours");
        let a = range(index)?;
        let b = range(other)?;
        if a == b { return edit_path(source,&PathEdit::Close { index }); }
        let mut joined = if index == a.0 { reverse_path_ranges(source, &[a])? } else { source.path_segments().take(a.1).skip(a.0).cloned().collect::<Vec<_>>() };
        let target = if other == b.0 { source.path_segments().take(b.1).skip(b.0).cloned().collect::<Vec<_>>() } else { reverse_path_ranges(source, &[b])? };
        let to = endpoint(&target[0]).ok_or("Missing target endpoint")?;
        if joined.last().and_then(endpoint) != Some(to) { joined.push(PathSegment::Line { to }); }
        joined.extend_from_slice(&target[1..]);
        let mut output = Vec::with_capacity(source.path_len()+1);
        for (start,end) in ranges {
            if start == a.0.min(b.0) { output.append(&mut joined); }
            if start != a.0 && start != b.0 { output.extend(source.path_segments().take(end).skip(start).cloned()); }
        }
        return Ok(output);
    }
    if matches!(operation, PathEdit::Reverse) { return reverse_path_ranges(source, &ranges); }
    let index = match operation { PathEdit::Coordinate { index, .. } | PathEdit::Position { index, .. } | PathEdit::Split { index, .. } | PathEdit::Delete { index } | PathEdit::Close { index } | PathEdit::Open { index } | PathEdit::Convert { index, .. } => *index, PathEdit::Node {..}|PathEdit::Simplify {..}|PathEdit::Reverse | PathEdit::Join { .. } | PathEdit::Translate { .. } | PathEdit::DeletePoints { .. } => unreachable!() };
    let item = source.path_segment(index).ok_or("Missing path node")?;
    let (start, end) = ranges.into_iter().find(|(start, end)| index >= *start && index < *end).ok_or("Missing contour")?;
    let mut output: Vec<PathSegment> = source.path_segments().cloned().collect();
    match *operation {
        PathEdit::Convert { target, .. } => {
            if matches!(item, PathSegment::Move { .. } | PathSegment::Close) { return Err("Select a segment to convert"); }
            let from = index.checked_sub(1).and_then(|index| endpoint(&*source.path_segment(index).expect("validated path ordinal"))).ok_or("Missing segment start")?;
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
            let closed = matches!(*source.path_segment(end - 1).expect("validated path ordinal"), PathSegment::Close);
            if matches!(operation, PathEdit::Open { .. }) && closed { output.remove(end - 1); }
            if matches!(operation, PathEdit::Close { .. }) && !closed {
                if end - start < 2 { return Err("A contour needs at least two anchors"); }
                output.insert(end, PathSegment::Close);
            }
        }
        PathEdit::Delete { .. } => match item {
            PathSegment::Close => return Err("Select an anchor to delete"),
            PathSegment::Move { .. } => {
                if let Some(next) = source.path_segment(index + 1).filter(|next| !matches!(next, PathSegment::Move { .. } | PathSegment::Close)) {
                    output[index + 1] = PathSegment::Move { to: endpoint(next).ok_or("Missing next endpoint")? };
                    output.remove(index);
                } else { output.drain(start..end); }
            }
            _ => { output.remove(index); }
        },
        PathEdit::Coordinate { point, .. } | PathEdit::Position { point, .. } => {
            let mut to=drag_path_point(item,point,[1.0,0.0,0.0,1.0,0.0,0.0],[0.0,0.0],[0.0,0.0],false).ok_or("This node has no selected handle")?;
            match *operation {
                PathEdit::Position {to:position,..}=>to=position,
                PathEdit::Coordinate {axis:PathAxis::X,value,..}=>to[0]=value,
                PathEdit::Coordinate {value,..}=>to[1]=value,
                _=>unreachable!(),
            }
            let (segment,next)=patch_path_point(item,source.path_segment(index+1),point,to)?;
            output[index]=segment;
            if let Some(next)=next {output[index+1]=next;}
        }
        PathEdit::Split { t, .. } => {
            if !(t > 0.0 && t < 1.0) { return Err("Split position must be between zero and one"); }
            let previous = index.checked_sub(1).and_then(|index| source.path_segment(index)).and_then(endpoint).ok_or("Select a segment to split")?;
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
        PathEdit::Node {..}|PathEdit::Simplify {..}|PathEdit::Reverse | PathEdit::Join { .. } | PathEdit::Translate { .. } | PathEdit::DeletePoints { .. } => unreachable!(),
    }
    if matches!(operation,PathEdit::Delete {..}) && end-start==3 && matches!(source.path_segment(end-1),Some(PathSegment::Close)) {output.remove(start+1);}
    if !output.iter().all(crate::schema::valid_path_segment) { return Err("The edit exceeds finite coordinates"); }
    Ok(output)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

/// 🖱️ Resolves a world-space drag to an absolute path-local point without changing press offset.
pub fn drag_path_point(segment: &PathSegment,point: PathPoint,matrix: [f64;6],start: [f64;2],end: [f64;2],constrained: bool) -> Option<[f64;2]> {
    let local=match (segment,point) {
        (_,PathPoint::Anchor)=>endpoint(segment),
        (PathSegment::Cubic {ctrl1,..},PathPoint::Control1)=>Some(*ctrl1),
        (PathSegment::Cubic {ctrl2,..},PathPoint::Control2)=>Some(*ctrl2),
        (PathSegment::Quad {ctrl,..},PathPoint::Control1)=>Some(*ctrl),
        _=>None,
    }?;
    if !matrix.iter().chain(start.iter()).chain(end.iter()).chain(local.iter()).all(|value|value.is_finite()) {return None;}
    let inverted=super::inverse([matrix[0],matrix[1],matrix[2],matrix[3],0.0,0.0])?;
    let (mut dx,mut dy)=(end[0]-start[0],end[1]-start[1]);
    if constrained {if dx.abs()>=dy.abs(){dy=0.0;}else{dx=0.0;}}
    let result=[local[0]+inverted[0]*dx+inverted[2]*dy,local[1]+inverted[1]*dx+inverted[3]*dy];
    result.iter().all(|value|value.is_finite()).then_some(result)
}

/// 🩹 Copies only a positioned node and, when needed, its outgoing tangent segment.
pub fn patch_path_point(segment: &PathSegment,next: Option<&PathSegment>,point: PathPoint,to: [f64;2]) -> Result<(PathSegment,Option<PathSegment>),&'static str> {
    if !to.iter().all(|value|value.is_finite()) || !crate::schema::valid_path_segment(segment) {return Err("Invalid coordinate");}
    let mut output=segment.clone();
    let mut following=None;
    if point==PathPoint::Anchor {
        let origin=endpoint(segment).ok_or("Select an anchor")?;
        let delta=[to[0]-origin[0],to[1]-origin[1]];
        let translate=|value:&mut [f64;2]|{value[0]+=delta[0];value[1]+=delta[1];};
        match &mut output {
            PathSegment::Move {to:target}|PathSegment::Line {to:target}|PathSegment::Arc {to:target,..}=>*target=to,
            PathSegment::Quad {to:target,ctrl}=>{*target=to;translate(ctrl);},
            PathSegment::Cubic {to:target,ctrl2,..}=>{*target=to;translate(ctrl2);},
            PathSegment::Close=>return Err("Select an anchor"),
        }
        following=match next {
            Some(PathSegment::Quad {ctrl,to})=>{let mut ctrl=*ctrl;translate(&mut ctrl);Some(PathSegment::Quad {ctrl,to:*to})},
            Some(PathSegment::Cubic {ctrl1,ctrl2,to})=>{let mut ctrl1=*ctrl1;translate(&mut ctrl1);Some(PathSegment::Cubic {ctrl1,ctrl2:*ctrl2,to:*to})},
            _=>None,
        };
    } else {
        match (&mut output,point) {
            (PathSegment::Cubic {ctrl1,..},PathPoint::Control1)=>*ctrl1=to,
            (PathSegment::Cubic {ctrl2,..},PathPoint::Control2)=>*ctrl2=to,
            (PathSegment::Quad {ctrl,..},PathPoint::Control1)=>*ctrl=to,
            _=>return Err("This node has no selected handle"),
        }
    }
    if !crate::schema::valid_path_segment(&output) || following.as_ref().is_some_and(|segment|!crate::schema::valid_path_segment(segment)) {return Err("The edit exceeds finite coordinates");}
    Ok((output,following))
}

/// 🎯 Returns the nearest anchor or control in world coordinates; anchors win coincident ties.
pub fn path_point_hit(segment:&PathSegment,matrix:[f64;6],world:[f64;2],tolerance:f64)->Option<(PathPoint,f64)> {
    if tolerance<0.0 || !tolerance.is_finite() || !matrix.iter().chain(world.iter()).all(|value|value.is_finite()) {return None;}
    let mut nearest=None;
    for point in [PathPoint::Anchor,PathPoint::Control1,PathPoint::Control2] {
        let Some(local)=drag_path_point(segment,point,[1.0,0.0,0.0,1.0,0.0,0.0],[0.0,0.0],[0.0,0.0],false) else {continue;};
        let [a,b,c,d,e,f]=matrix;
        let distance=(a*local[0]+c*local[1]+e-world[0]).hypot(b*local[0]+d*local[1]+f-world[1]);
        if distance<=tolerance && nearest.is_none_or(|(_,previous)|distance<previous) {nearest=Some((point,distance));}
    }
    nearest
}

/// ↔️ Moves the union of selected coordinates and attached tangents exactly once.
fn translate_path_points(source:&(impl PathGeometrySource + ?Sized),points:&[PathPointRef],delta:[f64;2])->Result<Vec<PathSegment>,&'static str> {
    translate_owned_path_points(source.path_segments().cloned().collect(), points, delta)
}

fn translate_owned_path_points(mut source: Vec<PathSegment>,points:&[PathPointRef],delta:[f64;2])->Result<Vec<PathSegment>,&'static str> {
    if points.is_empty() {return Err("Select at least one path point");}
    if !delta.iter().all(|value|value.is_finite()) {return Err("Invalid translation");}
    let mut masks=vec![0_u8;source.len()];
    for target in points {
        let segment=source.get(target.index).ok_or("Missing path node")?;
        let bit=match (segment,target.point) {
            (PathSegment::Close,_)=>return Err("Select a path point"),
            (_,PathPoint::Anchor)=>1,
            (PathSegment::Quad {..}|PathSegment::Cubic {..},PathPoint::Control1)=>2,
            (PathSegment::Cubic {..},PathPoint::Control2)=>4,
            _=>return Err("This node has no selected handle"),
        };
        masks[target.index]|=bit;
        if target.point==PathPoint::Anchor {
            masks[target.index]|=match segment {PathSegment::Quad {..}=>2,PathSegment::Cubic {..}=>4,_=>0};
            if matches!(source.get(target.index+1),Some(PathSegment::Quad {..}|PathSegment::Cubic {..})) {masks[target.index+1]|=2;}
        }
    }
    for (segment,mask) in source.iter_mut().zip(masks) {
        let shift=|point:&mut [f64;2],bit:u8| {if mask&bit!=0 {point[0]+=delta[0];point[1]+=delta[1];}};
        match segment {
            PathSegment::Move {to}|PathSegment::Line {to}|PathSegment::Arc {to,..}=>shift(to,1),
            PathSegment::Quad {to,ctrl}=>{shift(to,1);shift(ctrl,2);},
            PathSegment::Cubic {to,ctrl1,ctrl2}=>{shift(to,1);shift(ctrl1,2);shift(ctrl2,4);},
            PathSegment::Close=>{},
        }
        if !crate::schema::valid_path_segment(segment) {return Err("The edit exceeds finite coordinates");}
    }
    Ok(source)
}

/// 🌍️ Translate selected coordinates in document axes without applying the affine origin.
pub fn translate_world_path_points<'a>(source:impl IntoIterator<Item = &'a PathSegment>,points:&[PathPointRef],matrix:[f64;6],delta:[f64;2])->Result<Vec<PathSegment>,&'static str> {
    let basis=super::inverse([matrix[0],matrix[1],matrix[2],matrix[3],0.0,0.0]).ok_or("Cannot move points through a singular transform")?;
    if !matrix.iter().chain(delta.iter()).all(|value|value.is_finite()) {return Err("Cannot move points by a nonfinite displacement");}
    let source: Vec<PathSegment> = source.into_iter().cloned().collect();
    if !source.iter().all(crate::schema::valid_path_segment) { return Err("Invalid path geometry"); }
    contours(&source)?;
    translate_owned_path_points(source,points,[basis[0]*delta[0]+basis[2]*delta[1],basis[1]*delta[0]+basis[3]*delta[1]])
}
