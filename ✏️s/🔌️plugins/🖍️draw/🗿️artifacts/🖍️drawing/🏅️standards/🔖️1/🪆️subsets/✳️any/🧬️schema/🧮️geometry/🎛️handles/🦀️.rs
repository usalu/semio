//! 🎛️ Absolute world-space transforms for eight resize handles and one rotation handle.
const ANCHORS: [[f64;2];8]=[[0.0,0.0],[0.5,0.0],[1.0,0.0],[1.0,0.5],[1.0,1.0],[0.5,1.0],[0.0,1.0],[0.0,0.5]];

pub fn handle_points([x,y,w,h]: [f64;4],zoom: f64) -> [[f64;2];9] {
    std::array::from_fn(|index| if index==8 {[x+w*0.5,y-28.0/zoom.max(1e-6)]} else {let [u,v]=ANCHORS[index];[x+u*w,y+v*h]})
}

pub fn hit_handle(bounds: [f64;4],point: [f64;2],zoom: f64) -> Option<usize> {
    let mut best=None;
    let mut distance=8.0/zoom.max(1e-6);
    for (index,center) in handle_points(bounds,zoom).iter().enumerate() {
        let next=(point[0]-center[0]).hypot(point[1]-center[1]);
        if next<=distance {distance=next;best=Some(index);}
    }
    best
}

pub fn handle_matrix(handle: usize,bounds: [f64;4],start: [f64;2],end: [f64;2],constrained: bool,centered: bool) -> Option<[f64;6]> {
    if !bounds.iter().chain(start.iter()).chain(end.iter()).all(|value|value.is_finite()) || handle>8 {return None;}
    let [x,y,w,h]=bounds;
    let matrix=if handle==8 {
        let (cx,cy)=(x+w*0.5,y+h*0.5);
        let angle=(end[1]-cy).atan2(end[0]-cx)-(start[1]-cy).atan2(start[0]-cx);
        let mut angle=angle.sin().atan2(angle.cos());
        if constrained {let step=std::f64::consts::PI/12.0;angle=(angle/step).round()*step;}
        let (s,c)=angle.sin_cos();
        [c,s,-s,c,cx-c*cx+s*cy,cy-s*cx-c*cy]
    } else {
        let [u,v]=ANCHORS[handle];
        let (ax,ay)=(x+if centered {0.5*w} else {(1.0-u)*w},y+if centered {0.5*h} else {(1.0-v)*h});
        let factor=if centered {2.0} else {1.0};
        let mut sx=if u==0.5||w==0.0 {1.0} else {1.0+factor*(end[0]-start[0])/((2.0*u-1.0)*w)};
        let mut sy=if v==0.5||h==0.0 {1.0} else {1.0+factor*(end[1]-start[1])/((2.0*v-1.0)*h)};
        if constrained {let ratio=if (sx-1.0).abs()>=(sy-1.0).abs(){sx}else{sy};sx=ratio;sy=ratio;}
        [sx,0.0,0.0,sy,ax*(1.0-sx),ay*(1.0-sy)]
    };
    matrix.iter().all(|value|value.is_finite()).then_some(matrix)
}

#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
