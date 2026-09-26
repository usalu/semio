//! 🎯 Polygon containment used by lasso selection.
pub fn polygon_contains_point(polygon: &[[f64;2]], point: [f64;2]) -> bool {
    if polygon.len()<3 || !point.iter().all(|value| value.is_finite()) { return false; }
    let mut inside=false;
    let mut previous=polygon[polygon.len()-1];
    for &current in polygon {
        let cross=(current[0]-previous[0])*(point[1]-previous[1])-(current[1]-previous[1])*(point[0]-previous[0]);
        if cross.abs()<1e-10 && point[0]>=current[0].min(previous[0]) && point[0]<=current[0].max(previous[0]) && point[1]>=current[1].min(previous[1]) && point[1]<=current[1].max(previous[1]) { return true; }
        if (current[1]>point[1])!=(previous[1]>point[1]) && point[0]<(previous[0]-current[0])*(point[1]-current[1])/(previous[1]-current[1])+current[0] { inside=!inside; }
        previous=current;
    }
    inside
}

pub fn polygon_encloses_bounds(polygon: &[[f64;2]], bounds: [f64;4]) -> bool {
    let [x,y,width,height]=bounds;
    if !bounds.iter().all(|value| value.is_finite()) || width<0.0 || height<0.0 { return false; }
    let corners=[[x,y],[x+width,y],[x+width,y+height],[x,y+height]];
    if !corners.iter().all(|point| polygon_contains_point(polygon,*point)) { return false; }
    if width==0.0 || height==0.0 { return polygon_encloses_segment(polygon,[x,y],[x+width,y+height]); }
    let mut previous=polygon[polygon.len()-1];
    for &current in polygon {
        let mut lower: f64=0.0;
        let mut upper: f64=1.0;
        let mut possible=true;
        for axis in 0..2 {
            let min=bounds[axis]; let max=min+bounds[axis+2]; let delta=current[axis]-previous[axis];
            if delta==0.0 { if previous[axis]<=min || previous[axis]>=max { possible=false; } }
            else {
                let a=(min-previous[axis])/delta; let b=(max-previous[axis])/delta;
                lower=lower.max(a.min(b)); upper=upper.min(a.max(b));
            }
        }
        if possible && lower<upper { return false; }
        previous=current;
    }
    true
}

fn polygon_encloses_segment(polygon: &[[f64;2]], from: [f64;2], to: [f64;2]) -> bool {
    let direction=[to[0]-from[0],to[1]-from[1]];
    let length=direction[0]*direction[0]+direction[1]*direction[1];
    if length==0.0 { return polygon_contains_point(polygon,from); }
    let cross=|a:[f64;2],b:[f64;2]| a[0]*b[1]-a[1]*b[0];
    let mut cuts=vec![0.0,1.0];
    let mut previous=polygon[polygon.len()-1];
    for &current in polygon {
        let edge=[current[0]-previous[0],current[1]-previous[1]];
        let offset=[previous[0]-from[0],previous[1]-from[1]];
        let denominator=cross(direction,edge);
        if denominator!=0.0 {
            let t=cross(offset,edge)/denominator; let u=cross(offset,direction)/denominator;
            if t>0.0 && t<1.0 && (0.0..=1.0).contains(&u) { cuts.push(t); }
        } else if cross(offset,direction).abs()<1e-10 {
            for point in [previous,current] { let t=((point[0]-from[0])*direction[0]+(point[1]-from[1])*direction[1])/length; if t>0.0 && t<1.0 { cuts.push(t); } }
        }
        previous=current;
    }
    cuts.sort_by(f64::total_cmp);
    cuts.windows(2).all(|pair| { let t=(pair[0]+pair[1])/2.0; polygon_contains_point(polygon,[from[0]+direction[0]*t,from[1]+direction[1]*t]) })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
