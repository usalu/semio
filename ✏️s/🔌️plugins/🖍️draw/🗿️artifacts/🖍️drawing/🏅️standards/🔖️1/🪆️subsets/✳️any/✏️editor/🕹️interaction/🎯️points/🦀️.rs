//! 🎯️ Snapshot-bound point references for framework-owned node selection.
use crate::{PathSegment, schema::geometry::editing::PathPoint};
use semio_framework_hash::Hasher;
use semio_framework_value::paged::Utf8Text;

#[derive(Clone,Debug,PartialEq)]
pub(crate) struct PointSelectionRef<'a> {
    pub layer_id:&'a str,
    pub geometry:&'a str,
    pub index:usize,
    pub point:PathPoint,
}

pub(crate) fn point_slots(segment:&PathSegment)->&'static [PathPoint] {
    match segment {
        PathSegment::Close=>&[],
        PathSegment::Cubic {..}=>&[PathPoint::Anchor,PathPoint::Control1,PathPoint::Control2],
        PathSegment::Quad {..}=>&[PathPoint::Anchor,PathPoint::Control1],
        _=>&[PathPoint::Anchor],
    }
}

pub(crate) fn point_name(point:PathPoint)->&'static str {
    match point {PathPoint::Anchor=>"anchor",PathPoint::Control1=>"control1",PathPoint::Control2=>"control2"}
}

pub(crate) fn parse_point_id(id:&str)->Option<PointSelectionRef<'_>> {
    let (prefix,point)=id.rsplit_once(':')?;
    let (prefix,digits)=prefix.rsplit_once(':')?;
    let (layer_id,geometry)=prefix.rsplit_once(':')?;
    if layer_id.is_empty() || geometry.len()!=64 || !geometry.bytes().all(|byte|byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)) || digits.is_empty() || digits.len()>1 && digits.starts_with('0') || !digits.bytes().all(|byte|byte.is_ascii_digit()) {return None;}
    let index=digits.parse::<u64>().ok()?;
    if index>u32::MAX as u64 {return None;}
    let point=match point {"anchor"=>PathPoint::Anchor,"control1"=>PathPoint::Control1,"control2"=>PathPoint::Control2,_=>return None};
    Some(PointSelectionRef {layer_id,geometry,index:usize::try_from(index).ok()?,point})
}

pub(crate) fn point_id(layer_id:&(impl Utf8Text+?Sized),geometry:&str,index:usize,point:PathPoint)->Option<String> {
    let mut id=String::with_capacity(layer_id.text_bytes()+geometry.len()+32);
    for chunk in 0..layer_id.text_chunk_count() {id.push_str(layer_id.text_chunk(chunk)?);}
    let prefix=id.len();
    use std::fmt::Write;
    write!(&mut id,":{geometry}:{index}:{}",point_name(point)).ok()?;
    let parsed=parse_point_id(&id)?;
    if parsed.layer_id!=&id[..prefix] || parsed.geometry!=geometry || parsed.index!=index || parsed.point!=point {return None;}
    Some(id)
}

pub(crate) fn geometry_hasher()->Hasher {
    let mut hasher=Hasher::new();hasher.update(b"draw-points-v1");hasher
}

pub(crate) fn hash_segment(hasher:&mut Hasher,segment:&PathSegment)->Option<()> {
    let number=|hasher:&mut Hasher,value:f64|{hasher.update(&(if value==0.0 {0.0_f64} else {value}).to_le_bytes());};
    let point=|hasher:&mut Hasher,value:[f64;2]|{number(hasher,value[0]);number(hasher,value[1]);};
        if !crate::schema::valid_path_segment(segment) {return None;}
        let tag=match segment {PathSegment::Move {..}=>0,PathSegment::Line {..}=>1,PathSegment::Quad {..}=>2,PathSegment::Cubic {..}=>3,PathSegment::Arc {..}=>4,PathSegment::Close=>5};
        hasher.update(&[tag]);
        match segment {
            PathSegment::Move {to}|PathSegment::Line {to}=>point(hasher,*to),
            PathSegment::Quad {ctrl,to}=>{point(hasher,*ctrl);point(hasher,*to);},
            PathSegment::Cubic {ctrl1,ctrl2,to}=>{point(hasher,*ctrl1);point(hasher,*ctrl2);point(hasher,*to);},
            PathSegment::Arc {rx,ry,rotation,large_arc,sweep,to}=>{number(hasher,*rx);number(hasher,*ry);number(hasher,*rotation);hasher.update(&[u8::from(*large_arc),u8::from(*sweep)]);point(hasher,*to);},
            PathSegment::Close=>{},
        }
    Some(())
}

pub(crate) fn geometry_id<'a>(segments:impl IntoIterator<Item=&'a PathSegment>)->Option<String> {
    let mut hasher=geometry_hasher();
    for segment in segments {hash_segment(&mut hasher,segment)?;}
    Some(hasher.finalize().to_hex())
}

#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[derive(Clone,Copy,Debug,PartialEq,Default)]
#[cfg_attr(test,derive(serde::Deserialize))]
#[cfg_attr(test,serde(rename_all="camelCase"))]
pub(crate) enum PointPickMode {#[default] Replace,Toggle,Add}

/// 🖱️ A press on a selected point retains its drag group; modified presses change membership.
pub(crate) fn pick_point_selection(current:&[String],hit:Option<&str>,mode:PointPickMode)->Vec<String> {
    let Some(hit)=hit else {return if mode==PointPickMode::Replace {Vec::new()} else {current.to_vec()};};
    let selected=current.iter().any(|id|id==hit);
    if mode==PointPickMode::Replace {return if selected {current.to_vec()} else {vec![hit.into()]};}
    if mode==PointPickMode::Toggle && selected {return current.iter().filter(|id|id.as_str()!=hit).cloned().collect();}
    let mut result=current.to_vec();if !selected {result.push(hit.into());}result
}

/// ▧️ Marquees address world-space anchors, including rectangle edges.
pub(crate) fn anchor_in_marquee(segment:&PathSegment,matrix:[f64;6],start:[f64;2],end:[f64;2])->bool {
    if !matrix.iter().chain(start.iter()).chain(end.iter()).all(|value|value.is_finite()) {return false;}
    let to=match segment {PathSegment::Close=>return false,PathSegment::Move {to}|PathSegment::Line {to}|PathSegment::Quad {to,..}|PathSegment::Cubic {to,..}|PathSegment::Arc {to,..}=>to};
    let x=matrix[0]*to[0]+matrix[2]*to[1]+matrix[4];let y=matrix[1]*to[0]+matrix[3]*to[1]+matrix[5];
    x.is_finite() && y.is_finite() && x>=start[0].min(end[0]) && x<=start[0].max(end[0]) && y>=start[1].min(end[1]) && y<=start[1].max(end[1])
}

/// 🧮️ A region merge applies membership once per distinct hit.
pub(crate) fn merge_point_selection(current:&[String],hits:&[String],mode:PointPickMode)->Vec<String> {
    let incoming=hits.iter().collect::<std::collections::BTreeSet<_>>();
    let existing=current.iter().collect::<std::collections::BTreeSet<_>>();
    let mut result=Vec::new();let mut seen=std::collections::BTreeSet::new();
    if mode!=PointPickMode::Replace {for id in current {if (mode!=PointPickMode::Toggle || !incoming.contains(id)) && seen.insert(id) {result.push(id.clone());}}}
    for id in hits {if (mode==PointPickMode::Replace || !existing.contains(id)) && seen.insert(id) {result.push(id.clone());}}
    result
}
