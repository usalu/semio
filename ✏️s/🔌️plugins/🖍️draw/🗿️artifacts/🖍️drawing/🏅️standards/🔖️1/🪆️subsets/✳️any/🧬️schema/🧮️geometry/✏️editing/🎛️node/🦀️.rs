//! 🎛️ Authored tangent modes keep node positions and unrelated controls stable.
use super::{PathGeometrySource,NodeMode,endpoint};
use crate::PathSegment;
fn cubic(segment:&PathSegment,from:[f64;2])->Result<PathSegment,&'static str> {
    let mix=|a:[f64;2],b:[f64;2],t:f64|[a[0]*(1.0-t)+b[0]*t,a[1]*(1.0-t)+b[1]*t];
    Ok(match *segment {
        PathSegment::Cubic {..}=>segment.clone(),
        PathSegment::Line {to}=>PathSegment::Cubic {ctrl1:mix(from,to,1.0/3.0),ctrl2:mix(from,to,2.0/3.0),to},
        PathSegment::Quad {ctrl,to}=>PathSegment::Cubic {ctrl1:mix(from,ctrl,2.0/3.0),ctrl2:mix(to,ctrl,2.0/3.0),to},
        PathSegment::Arc {..}=>return Err("Convert adjacent arcs to cubic curves before editing tangents"),
        _=>return Err("Select an anchor with an adjacent segment"),
    })
}
pub fn edit(source:&(impl PathGeometrySource+?Sized),index:usize,mode:NodeMode)->Result<Vec<PathSegment>,&'static str> {
    let segment=source.path_segment(index).ok_or("Missing path node")?;
    let anchor=endpoint(segment).ok_or("Select an anchor")?;
    let mut output=source.path_segments().cloned().collect::<Vec<_>>();
    let (start,end)=super::contours(source)?.into_iter().find(|(start,end)|index>=*start&&index<*end).ok_or("Missing contour")?;
    let closed=matches!(source.path_segment(end-1),Some(PathSegment::Close));
    let mut incoming_index=index;let mut outgoing_index=index+1;
    let incoming=if index==start {
        if closed&&end-start>2 {
            let previous=endpoint(source.path_segment(end-2).unwrap()).ok_or("Missing previous anchor")?;
            incoming_index=if previous==anchor {end-2}else {end-1};
            let from=if previous==anchor {endpoint(source.path_segment(end-3).unwrap()).ok_or("Missing previous anchor")?}else {previous};
            let curve=if previous==anchor {cubic(source.path_segment(end-2).unwrap(),from)?}else {cubic(&PathSegment::Line {to:anchor},from)?};
            let PathSegment::Cubic {ctrl2,..}=curve else {unreachable!()};
            if incoming_index==end-1 {output.insert(incoming_index,curve);}else {output[incoming_index]=curve;}
            Some(ctrl2)
        }else {None}
    }else {
        let from=index.checked_sub(1).and_then(|index|source.path_segment(index)).and_then(endpoint).ok_or("Missing previous anchor")?;
        output[index]=cubic(segment,from)?;
        let PathSegment::Cubic {ctrl2,..}=output[index] else {unreachable!()};Some(ctrl2)
    };
    let outgoing_segment=if closed&&index==end-2 {
        let first=endpoint(source.path_segment(start).unwrap()).ok_or("Missing first anchor")?;
        if anchor==first {outgoing_index=start+1;source.path_segment(outgoing_index).cloned()}else {Some(PathSegment::Line {to:first})}
    }else {source.path_segment(index+1).cloned()};
    let outgoing=match outgoing_segment.as_ref() {
        Some(next) if !matches!(next,PathSegment::Move {..}|PathSegment::Close)=>{
            let curve=cubic(next,anchor)?;let PathSegment::Cubic {ctrl1,..}=curve else {unreachable!()};
            if closed&&index==end-2&&outgoing_index==end-1 {output.insert(outgoing_index,curve);}else {output[outgoing_index]=curve;}
            Some(ctrl1)
        }
        _=>None,
    };
    if incoming.is_none()&&outgoing.is_none() {return Err("Select an anchor with an adjacent segment");}
    let length=|p:[f64;2]|(p[0]-anchor[0]).hypot(p[1]-anchor[1]);
    let mut left=incoming.map(length).unwrap_or(0.0);let mut right=outgoing.map(length).unwrap_or(0.0);
    let a=incoming.filter(|_|left>0.0).map(|p|[(anchor[0]-p[0])/left,(anchor[1]-p[1])/left]).unwrap_or([0.0;2]);
    let b=outgoing.filter(|_|right>0.0).map(|p|[(p[0]-anchor[0])/right,(p[1]-anchor[1])/right]).unwrap_or([0.0;2]);
    let direction=[a[0]+b[0],a[1]+b[1]];let norm=direction[0].hypot(direction[1]);
    let direction=if norm>1e-12 {[direction[0]/norm,direction[1]/norm]}else if right>0.0 {b}else {a};
    if mode==NodeMode::Corner {left=0.0;right=0.0;}
    if mode==NodeMode::Symmetric && incoming.is_some() && outgoing.is_some() {let size=left/2.0+right/2.0;left=size;right=size;}
    if incoming.is_some() {let PathSegment::Cubic {ctrl2,..}=&mut output[incoming_index] else {unreachable!()};*ctrl2=[anchor[0]-direction[0]*left,anchor[1]-direction[1]*left];}
    if outgoing.is_some() {let PathSegment::Cubic {ctrl1,..}=&mut output[outgoing_index] else {unreachable!()};*ctrl1=[anchor[0]+direction[0]*right,anchor[1]+direction[1]*right];}
    if !output.iter().all(crate::schema::valid_path_segment) {return Err("The edit exceeds finite coordinates");}
    Ok(output)
}
