//! 📐️ Certifies a bounded simple prism from two translated caps and their reciprocal walls.

use super::{MeshResult, Plane, Soup, difference, dot, invalid, length};

#[path = "🧭️axial-relation/🦀️.rs"]
mod axial_relation;
pub(super) use axial_relation::{AxialAperture,AxialProfileCertificate,AxialRelation,aperture};

pub(super) const MAX_PROFILE: usize = 128;

#[derive(semio_framework_value::RetireOwned)]
pub(super) struct NestedPrism {
    axis: usize,
    caps: [usize; 2],
    epsilon: f64,
    anchor: usize,
    corner: usize,
    peer: usize,
    wall: usize,
    stage: u8,
}

fn projected(point: [f32; 3], axis: usize) -> [f64; 2] {
    [(axis + 1) % 3, (axis + 2) % 3].map(|i| f64::from(point[i]))
}

fn orient(a: [f64; 2], b: [f64; 2], c: [f64; 2]) -> f64 {
    (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
}

fn edges_intersect(a: [f64; 2], b: [f64; 2], c: [f64; 2], d: [f64; 2]) -> bool {
    let overlaps = (0..2).all(|i| a[i].min(b[i]) <= c[i].max(d[i]) && c[i].min(d[i]) <= a[i].max(b[i]));
    let separates = |a, b, c, d| {
        let (u, v) = (orient(a, b, c), orient(a, b, d));
        (u > 0.0 && v > 0.0) || (u < 0.0 && v < 0.0)
    };
    overlaps && !separates(a, b, c, d) && !separates(c, d, a, b)
}

impl NestedPrism {
    pub(super) fn new(source: &Soup, epsilon: f32) -> MeshResult<Self> {
        let n = source.positions.len() / 2;
        if !(3..=MAX_PROFILE).contains(&n) || source.positions.len() != 2 * n || source.faces.len() != n + 2 {
            return Err(invalid("concave boolean requires a bounded closed prism"));
        }
        Ok(Self { axis: 0, caps: [0, 1], epsilon: f64::from(epsilon), anchor: 0, corner: 0, peer: 1, wall: 0, stage: 0 })
    }

    fn match_caps(&self, source: &Soup, planes: &[Plane]) -> Option<(usize, usize)> {
        let n = source.positions.len() / 2;
        let [low, high] = self.caps.map(|id| &source.faces[id]);
        if low.len() != n || high.len() != n { return None; }
        let [base, lid] = self.caps.map(|id| planes[id]);
        let [base_normal, lid_normal] = [base, lid].map(|plane| plane.normal.0.map(f64::from));
        if dot(base_normal, lid_normal) > -1.0 + 1e-6 { return None; }

        let point = |id: u32| source.positions[id as usize];
        let mut offset = [0.0; 3];
        for (&a, &b) in low.iter().zip(high) {
            let (a, b) = (point(a), point(b));
            if dot(difference(a, base.origin.0), base_normal).abs() > self.epsilon
                || dot(difference(b, lid.origin.0), lid_normal).abs() > self.epsilon {
                return None;
            }
            for (sum, delta) in offset.iter_mut().zip(difference(b, a)) { *sum += delta / n as f64; }
        }
        if dot(offset, lid_normal) <= self.epsilon { return None; }
        let matches = |a: u32, b: u32| {
            let mut delta = difference(point(b), point(a));
            for (value, expected) in delta.iter_mut().zip(offset) { *value -= expected; }
            length(delta) <= self.epsilon
        };
        let mut anchors = low.iter().enumerate().filter(|&(_, &a)| matches(a, high[0]));
        let (anchor, _) = anchors.next()?;
        if anchors.next().is_some() || !high.iter().enumerate().all(|(i, &b)| matches(low[(anchor + n - i) % n], b)) {
            return None;
        }
        let axis = (0..3).max_by(|&a, &b| lid_normal[a].abs().total_cmp(&lid_normal[b].abs()))?;
        Some((anchor, axis))
    }

    pub(super) fn caps(&self)->[usize;2]{self.caps}

    pub(super) fn pair(&self, source: &Soup, i: usize) -> [u32; 2] {
        let [low, high] = self.caps.map(|id| &source.faces[id]);
        [low[(self.anchor + low.len() - i) % low.len()], high[i]]
    }

    fn orthogonal_axis(&self, source: &Soup) -> Option<usize> {
        let levels = self.pair(source, 0).map(|id| source.positions[id as usize]);
        let axis = (0..3).find(|&i| levels[0][i] != levels[1][i])?;
        let count = source.positions.len()/2;
        for i in 0..count {
            let [a,b] = self.pair(source, i).map(|id| source.positions[id as usize]);
            if projected(a,axis) != projected(b,axis) || [a[axis],b[axis]] != [levels[0][axis],levels[1][axis]] {
                return None;
            }
        }
        Some(axis)
    }

    pub(super) fn matching_axis(&self, source: &Soup, other: &Self, cutter: &Soup) -> Option<usize> {
        if source.positions.len() != cutter.positions.len() { return None; }
        let axis = self.orthogonal_axis(source)?;
        if other.orthogonal_axis(cutter) != Some(axis) { return None; }
        let levels = self.pair(source,0).map(|id| source.positions[id as usize][axis]);
        let other_levels = other.pair(cutter,0).map(|id| cutter.positions[id as usize][axis]);
        let first = |prism: &Self, soup: &Soup, i| projected(soup.positions[prism.pair(soup,i)[0] as usize],axis);
        let count = source.positions.len()/2;
        let anchor = (0..count).find(|&i| first(other,cutter,i) == first(self,source,0))?;
        let same_direction = (levels[0] < levels[1]) == (other_levels[0] < other_levels[1]);
        (0..count).all(|i| {
            let peer = if same_direction { (anchor+i)%count } else { (anchor+count-i)%count };
            first(self,source,i) == first(other,cutter,peer)
        }).then_some(axis)
    }

    fn uv(&self, source: &Soup, index: usize, side: usize) -> [f64; 2] {
        projected(source.positions[self.pair(source, index)[side] as usize], self.axis)
    }

    pub(super) fn advance(&mut self, source: &Soup, planes: &[Plane]) -> MeshResult<bool> {
        let n = source.positions.len() / 2;
        match self.stage {
            0 => {
                if self.caps[0] + 1 == source.faces.len() { return Err(invalid("concave boolean requires matching translated prism caps")); }
                if let Some((anchor, axis)) = self.match_caps(source, planes) {
                    self.anchor = anchor;
                    self.axis = axis;
                    self.stage = 1;
                } else {
                    self.caps[1] += 1;
                    if self.caps[1] == source.faces.len() { self.caps = [self.caps[0] + 1, self.caps[0] + 2]; }
                }
            }
            1 => {
                for side in 0..2 {
                    let [a, b, c] = [(self.corner + n - 1) % n, self.corner, (self.corner + 1) % n].map(|i| self.uv(source, i, side));
                    if orient(a, b, c) == 0.0 && (0..2).map(|i| (a[i] - b[i]) * (c[i] - b[i])).sum::<f64>() > 0.0 {
                        return Err(invalid("concave boolean prism profile doubles back"));
                    }
                }
                self.corner += 1;
                if self.corner == n { self.corner = 0; self.stage = 2; }
            }
            2 => {
                if let Some(ring) = source.faces.get(self.wall) {
                    if !self.caps.contains(&self.wall) { self.check_wall(source, ring)?; }
                    self.wall += 1;
                } else { self.stage = 3; }
            }
            3 => {
                if self.corner + 1 == n { return Ok(true); }
                if self.peer != self.corner + 1 && (self.corner != 0 || self.peer != n - 1) {
                    for side in 0..2 {
                        let [a, b, c, d] = [self.corner, self.corner + 1, self.peer, (self.peer + 1) % n].map(|i| self.uv(source, i, side));
                        if edges_intersect(a, b, c, d) { return Err(invalid("concave boolean prism profile crosses itself")); }
                    }
                }
                self.peer += 1;
                if self.peer == n { self.corner += 1; self.peer = self.corner + 1; }
            }
            _ => unreachable!(),
        }
        Ok(false)
    }

    fn check_wall(&self, source: &Soup, ring: &[u32]) -> MeshResult<()> {
        let fail = || invalid("concave boolean prism requires one reciprocal quad per cap edge");
        if ring.len() != 4 { return Err(fail()); }
        let top = &source.faces[self.caps[1]];
        let mut edges = ring.iter().copied().zip(ring.iter().copied().cycle().skip(1));
        let at = edges.find_map(|(a, b)| (top.contains(&a) && top.contains(&b)).then_some(b)).ok_or_else(fail)?;
        let i = top.iter().position(|&id| id == at).ok_or_else(fail)?;
        let [lower, upper] = self.pair(source, i);
        let [next_lower, next_upper] = self.pair(source, (i + 1) % top.len());
        let expected = [lower, next_lower, next_upper, upper];
        if (0..4).any(|start| (0..4).all(|j| ring[(start + j) % 4] == expected[j])) { Ok(()) } else { Err(fail()) }
    }
}
