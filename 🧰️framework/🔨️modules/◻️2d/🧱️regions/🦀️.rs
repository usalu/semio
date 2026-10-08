//! 🧱️ Polygon regions (outer ring plus holes): booleans and mitered/bevelled/round offsets on top of the budgeted arrangement job.
//!
//! A [`Region`] has a counter-clockwise outer ring and clockwise holes. Every operation is driven in work grants of [`GRANT`] steps and calls `control` between grants with the current [`BooleanProgress`]; returning `false` cancels the job and yields [`BooleanError::Cancelled`] without a partial result.
//! Offsets are exact for straight edges: a band is built on the grown/shrunk side of every edge, corner wedges (miter, bevel or round) fill the gaps at corners that turn away from that side, and the union (grow) or difference (shrink) with the region removes all self-overlap.

use crate::booleans::{BooleanError, BooleanFillRule, BooleanInput, BooleanJob, BooleanOperand, BooleanOperation, BooleanProgress};
use crate::{PathSegment, Vec2};

/// 🕐️ Work steps granted to the arrangement job between two `control` calls.
pub const GRANT: usize = 4096;

/// 🧱️ Polygon with holes; the outer ring is counter-clockwise, holes are clockwise.
#[derive(Clone, Debug, PartialEq)]
pub struct Region {
    pub outer: Vec<Vec2>,
    pub holes: Vec<Vec<Vec2>>,
}

/// 📐️ Corner treatment of an offset.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OffsetJoin {
    /// Sharp corner; beyond `limit * distance` from the original vertex the corner is bevelled (`limit >= 1`).
    Miter { limit: f64 },
    /// Straight cut between the two offset edge ends.
    Bevel,
    /// Circular arc around the vertex flattened within `tolerance`.
    Round { tolerance: f64 },
}

fn signed_area(ring: &[Vec2]) -> f64 {
    let n = ring.len();
    (0..n).map(|i| ring[i][0] * ring[(i + 1) % n][1] - ring[(i + 1) % n][0] * ring[i][1]).sum::<f64>() / 2.0
}

fn oriented(ring: &[Vec2], ccw: bool) -> Vec<Vec2> {
    let mut out = ring.to_vec();
    if (signed_area(ring) > 0.0) != ccw {
        out.reverse();
    }
    out
}

fn inside(ring: &[Vec2], p: Vec2) -> bool {
    let n = ring.len();
    let mut crossings = false;
    for i in 0..n {
        let (a, b) = (ring[i], ring[(i + 1) % n]);
        if (a[1] > p[1]) != (b[1] > p[1]) && p[0] < a[0] + (p[1] - a[1]) / (b[1] - a[1]) * (b[0] - a[0]) {
            crossings = !crossings;
        }
    }
    crossings
}

impl Region {
    /// 🧱️ Region from rings of any orientation (normalised).
    pub fn new(outer: &[Vec2], holes: &[Vec<Vec2>]) -> Self {
        Self { outer: oriented(outer, true), holes: holes.iter().map(|h| oriented(h, false)).collect() }
    }

    /// 📐️ Area of the outer ring minus the holes.
    pub fn area(&self) -> f64 {
        signed_area(&self.outer) + self.holes.iter().map(|h| signed_area(h)).sum::<f64>()
    }

    fn rings(&self) -> impl Iterator<Item = &Vec<Vec2>> {
        std::iter::once(&self.outer).chain(self.holes.iter())
    }
}

fn operand(regions: &[Region]) -> BooleanOperand {
    BooleanOperand { contours: regions.iter().flat_map(|r| r.rings().cloned()).collect(), fill_rule: BooleanFillRule::Nonzero }
}

fn run(operation: BooleanOperation, operands: Vec<BooleanOperand>, control: &mut dyn FnMut(&BooleanProgress) -> bool) -> Result<Vec<Region>, BooleanError> {
    let magnitude = operands.iter().flat_map(|o| o.contours.iter().flatten()).fold(0.0f64, |m, p| m.max(p[0].abs()).max(p[1].abs()));
    let mut job = BooleanJob::new(BooleanInput { operation, operands, epsilon: 1e-8f64.max(magnitude * f64::EPSILON * 32.0), max_edges: 262144, max_parameters: 1048576, max_atomic_edges: 262144, max_segments: 327680, max_work: 1_000_000_000 })?;
    loop {
        let progress = job.advance(GRANT)?;
        if progress.done {
            break;
        }
        if !control(&progress) {
            job.cancel();
            return Err(BooleanError::Cancelled);
        }
    }
    Ok(regions_from_path(&job.into_result()?))
}

/// 🧱️ Groups the closed contours of a boolean result into regions (positive rings are outers, negative rings are holes of the smallest enclosing outer).
pub fn regions_from_path(path: &[PathSegment]) -> Vec<Region> {
    let mut rings: Vec<Vec<Vec2>> = Vec::new();
    let mut current: Vec<Vec2> = Vec::new();
    for segment in path {
        match segment {
            PathSegment::Move { to } => current = vec![*to],
            PathSegment::Line { to } => current.push(*to),
            PathSegment::Close => {
                if current.len() >= 3 {
                    rings.push(std::mem::take(&mut current));
                }
            }
            _ => {}
        }
    }
    let mut regions: Vec<Region> = rings.iter().filter(|r| signed_area(r) > 0.0).map(|r| Region { outer: r.clone(), holes: Vec::new() }).collect();
    for hole in rings.iter().filter(|r| signed_area(r) < 0.0) {
        let probe = [(hole[0][0] + hole[1][0]) / 2.0, (hole[0][1] + hole[1][1]) / 2.0];
        let owner = regions.iter_mut().filter(|r| inside(&r.outer, probe) || r.outer.contains(&hole[0])).min_by(|a, b| signed_area(&a.outer).total_cmp(&signed_area(&b.outer)));
        if let Some(region) = owner {
            region.holes.push(hole.clone());
        }
    }
    regions
}

/// 🔀️ Boolean of two region sets (nonzero fill inside each set, so overlapping regions of one set are merged first).
pub fn region_boolean(operation: BooleanOperation, subject: &[Region], clip: &[Region], control: &mut dyn FnMut(&BooleanProgress) -> bool) -> Result<Vec<Region>, BooleanError> {
    run(operation, vec![operand(subject), operand(clip)], control)
}

fn wedge(vertex: Vec2, from: Vec2, to: Vec2, distance: f64, join: OffsetJoin) -> Vec<Vec2> {
    let at = |n: Vec2| [vertex[0] + n[0] * distance, vertex[1] + n[1] * distance];
    let mut polygon = vec![vertex, at(from)];
    match join {
        OffsetJoin::Bevel => {}
        OffsetJoin::Miter { limit } => {
            let dot = from[0] * to[0] + from[1] * to[1];
            if 1.0 + dot > 1e-9 {
                let k = distance / (1.0 + dot);
                let tip = [vertex[0] + (from[0] + to[0]) * k, vertex[1] + (from[1] + to[1]) * k];
                if (tip[0] - vertex[0]).hypot(tip[1] - vertex[1]) <= limit * distance {
                    polygon.push(tip);
                }
            }
        }
        OffsetJoin::Round { tolerance } => {
            let sweep = (from[0] * to[1] - from[1] * to[0]).atan2(from[0] * to[0] + from[1] * to[1]);
            let step = 2.0 * (1.0 - (tolerance / distance).clamp(1e-9, 1.0)).acos();
            let n = ((sweep.abs() / step.max(1e-3)).ceil() as usize).clamp(1, 4096);
            let base = from[1].atan2(from[0]);
            for i in 1..n {
                let a = base + sweep * i as f64 / n as f64;
                polygon.push([vertex[0] + a.cos() * distance, vertex[1] + a.sin() * distance]);
            }
        }
    }
    polygon.push(at(to));
    polygon
}

fn bands(region: &Region, distance: f64, join: OffsetJoin) -> Vec<Vec<Vec2>> {
    let grow = distance > 0.0;
    let d = distance.abs();
    let mut out = Vec::new();
    for ring in region.rings() {
        let n = ring.len();
        let edges: Vec<(Vec2, Vec2, Vec2)> = (0..n)
            .filter_map(|i| {
                let (a, b) = (ring[i], ring[(i + 1) % n]);
                let length = (b[0] - a[0]).hypot(b[1] - a[1]);
                (length > 0.0).then(|| {
                    let u = [(b[0] - a[0]) / length, (b[1] - a[1]) / length];
                    let side = if grow { [u[1], -u[0]] } else { [-u[1], u[0]] };
                    (a, b, side)
                })
            })
            .collect();
        for (a, b, side) in &edges {
            out.push(oriented(&[*a, *b, [b[0] + side[0] * d, b[1] + side[1] * d], [a[0] + side[0] * d, a[1] + side[1] * d]], true));
        }
        let m = edges.len();
        for i in 0..m {
            let (prev, next) = (&edges[(i + m - 1) % m], &edges[i]);
            let (u0, u1) = ([prev.1[0] - prev.0[0], prev.1[1] - prev.0[1]], [next.1[0] - next.0[0], next.1[1] - next.0[1]]);
            let turn = u0[0] * u1[1] - u0[1] * u1[0];
            if (grow && turn > 0.0) || (!grow && turn < 0.0) {
                out.push(oriented(&wedge(next.0, prev.2, next.2, d, join), true));
            }
        }
    }
    out
}

/// ↔️ Offsets every region by `distance` (positive grows, negative shrinks) with the given corner join.
pub fn offset_regions(regions: &[Region], distance: f64, join: OffsetJoin, control: &mut dyn FnMut(&BooleanProgress) -> bool) -> Result<Vec<Region>, BooleanError> {
    if !distance.is_finite() {
        return Err(BooleanError::Invalid("Offset distance must be finite"));
    }
    if distance == 0.0 {
        return Ok(regions.to_vec());
    }
    let strips = BooleanOperand { contours: regions.iter().flat_map(|r| bands(r, distance, join)).collect(), fill_rule: BooleanFillRule::Nonzero };
    let operation = if distance > 0.0 { BooleanOperation::Union } else { BooleanOperation::Difference };
    run(operation, vec![operand(regions), strips], control)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
