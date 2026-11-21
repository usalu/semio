//! 🔺️ Bounded material boundary loops for exact rectangular footprint booleans.

type Point = [f64;2];
const MAX_NODES: usize = 16;
const MAX_RINGS: usize = 4;
const ANGULAR: f64 = 64. * f64::EPSILON;

/// 🧮️ Selects the occupied side of each rectangle pair.
#[derive(Clone, Copy)]
pub enum RectangleOperation { Union, Difference, Intersection }

/// 🪟️ Circuits oriented with material on their left. A negative ring encloses a void.
pub struct PolygonRegion {
    points: [Point; MAX_NODES],
    sizes: [usize; MAX_RINGS],
    ring_count: usize,
}

#[derive(Clone, Copy)]
struct Splits {
    values: [(f64,usize); 6],
    count: usize,
}

impl Splits {
    fn new(start: usize, end: usize) -> Self {
        let mut values = [(0.,0);6];
        values[..2].copy_from_slice(&[(0.,start),(1.,end)]);
        Self { values, count: 2 }
    }

    fn insert(&mut self, t: f64, node: usize) -> Option<()> {
        *self.values.get_mut(self.count)? = (t,node);
        self.count += 1;
        Some(())
    }
}

fn difference(a: Point,b: Point) -> Point { [a[0]-b[0],a[1]-b[1]] }
fn cross(a: Point,b: Point) -> f64 { a[0]*b[1]-a[1]*b[0] }
fn dot(a: Point,b: Point) -> f64 { a[0]*b[0]+a[1]*b[1] }
fn length(a: Point) -> f64 { a[0].hypot(a[1]) }

fn unit(a: Point) -> Option<(Point,f64)> {
    let n = length(a);
    (n > 0. && n.is_finite()).then_some((a.map(|v|v/n),n))
}

fn ring_area(ring: &[Point]) -> f64 {
    (1..ring.len().saturating_sub(1)).map(|i| cross(difference(ring[i],ring[0]),difference(ring[i+1],ring[0]))).sum::<f64>() * 0.5
}

fn crossing(a: [Point;2], b: [Point;2], tol: f64) -> Option<Option<(f64,f64,Point)>> {
    let (u,la) = unit(difference(a[1],a[0]))?;
    let (v,lb) = unit(difference(b[1],b[0]))?;
    let delta = difference(b[0],a[0]);
    let det = cross(u,v);
    if det.abs() <= ANGULAR && det.abs()*la.max(lb) <= tol/4. {
        if cross(u,delta).abs() <= tol && cross(u,difference(b[1],a[0])).abs() <= tol {
            let q = dot(u,delta);
            let r = dot(u,difference(b[1],a[0]));
            if q.min(r) <= la+tol && q.max(r) >= -tol { return None; }
        }
        return Some(None);
    }
    let (s,t) = (cross(delta,v)/det,cross(delta,u)/det);
    if !s.is_finite() || !t.is_finite() { return None; }
    if s < -tol || t < -tol || s > la+tol || t > lb+tol { return Some(None); }
    if s <= tol || t <= tol || s >= la-tol || t >= lb-tol { return None; }
    let point = std::array::from_fn(|i|a[0][i]+u[i]*s);
    let check = std::array::from_fn(|i|b[0][i]+v[i]*t);
    (length(difference(point,check)) <= tol / 4.).then_some(Some((s/la,t/lb,point)))
}

fn inside(p: Point, rectangle: &[Point;4], tol: f64) -> Option<bool> {
    let mut boundary = false;
    for i in 0..4 {
        let (u,_) = unit(difference(rectangle[(i+1)%4],rectangle[i]))?;
        let signed = cross(u,difference(p,rectangle[i]));
        if !signed.is_finite() { return None; }
        if signed < -tol { return Some(false); }
        boundary |= signed <= tol;
    }
    (!boundary).then_some(true)
}

impl PolygonRegion {
    /// 📐️ Collects the material circuits of two counterclockwise rectangular footprints.
    pub fn rectangles(a: [Point;4], b: [Point;4], operation: RectangleOperation, tol: f64) -> Option<Self> {
        if !tol.is_finite() || tol <= 0. { return None; }
        for rectangle in [a,b] {
            let area = ring_area(&rectangle);
            if !area.is_finite() || area <= tol*tol { return None; }
            for i in 0..4 {
                let (first,a) = unit(difference(rectangle[(i+1)%4],rectangle[i]))?;
                let (second,b) = unit(difference(rectangle[(i+2)%4],rectangle[(i+1)%4]))?;
                if a <= tol || b <= tol || cross(first,second) <= 0. || dot(first,second).abs()*a.max(b) > tol/4. { return None; }
            }
        }
        let mut nodes = [[0.;2];MAX_NODES];
        nodes[..4].copy_from_slice(&a);
        nodes[4..8].copy_from_slice(&b);
        let mut next = 8;
        let mut splits = std::array::from_fn::<_,8,_>(|i| Splits::new(i,4*(i/4)+(i+1)%4));
        for i in 0..4 {
            let edge_a = [a[i],a[(i+1)%4]];
            for j in 0..4 {
                let edge_b = [b[j],b[(j+1)%4]];
                if let Some((u,v,p)) = crossing(edge_a,edge_b,tol)? {
                    *nodes.get_mut(next)? = p;
                    splits[i].insert(u,next)?;
                    splits[4+j].insert(v,next)?;
                    next += 1;
                }
            }
        }
        let mut incoming = [false; MAX_NODES];
        let mut outgoing = [None; MAX_NODES];
        for (i,edge) in splits.iter_mut().enumerate() {
            let active = &mut edge.values[..edge.count];
            active.sort_unstable_by(|a,b| a.0.total_cmp(&b.0));
            for segment in active.windows(2) {
                let (mut from,mut to) = (segment[0].1,segment[1].1);
                let delta = difference(nodes[to],nodes[from]);
                if length(delta) <= tol { return None; }
                let middle = std::array::from_fn(|axis| nodes[from][axis]+0.5*delta[axis]);
                let in_other = inside(middle,if i<4{&b}else{&a},tol)?;
                let keep = match operation {
                    RectangleOperation::Union => !in_other,
                    RectangleOperation::Intersection => in_other,
                    RectangleOperation::Difference => {
                        if i>=4 { std::mem::swap(&mut from,&mut to); }
                        in_other == (i>=4)
                    }
                };
                if keep && (outgoing[from].replace(to).is_some() || std::mem::replace(&mut incoming[to],true)) { return None; }
            }
        }
        if (0..next).any(|i| outgoing[i].is_some()!=incoming[i]) { return None; }
        Self::assemble(nodes,outgoing,tol)
    }

    fn assemble(nodes: [Point; MAX_NODES], mut outgoing: [Option<usize>; MAX_NODES], tol: f64) -> Option<Self> {
        let mut region = Self { points: [[0.;2];MAX_NODES],sizes: [0; MAX_RINGS],ring_count: 0 };
        let mut used = 0;
        while let Some(start) = outgoing.iter().position(Option::is_some) {
            let mut node = start;
            let mut count = 0;
            loop {
                *region.points.get_mut(used+count)? = nodes[node];
                count += 1;
                node = outgoing[node].take()?;
                if node==start { break; }
            }
            let area = ring_area(&region.points[used..used+count]);
            if count<3 || !area.is_finite() || area.abs() <= tol*tol { return None; }
            *region.sizes.get_mut(region.ring_count)? = count;
            region.ring_count += 1;
            used += count;
        }
        Some(region)
    }

    /// 🔄️ Borrows the disjoint material circuits from the fixed-size arrangement.
    pub fn rings(&self) -> impl ExactSizeIterator<Item=&[Point]> {
        let mut start = 0;
        self.sizes[..self.ring_count].iter().map(move |&count| {
            let ring = &self.points[start..start+count];
            start += count;
            ring
        })
    }

    /// 📏️ Returns the signed physical area of one oriented ring.
    pub fn ring_area(ring: &[Point]) -> f64 { ring_area(ring) }
}
