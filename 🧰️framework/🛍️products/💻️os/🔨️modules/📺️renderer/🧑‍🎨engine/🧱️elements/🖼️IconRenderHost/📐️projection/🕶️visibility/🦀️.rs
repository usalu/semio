//! 📐️ Projected depth planes and bounded line classification for SVG export.

use std::{cmp::Ordering, collections::BTreeMap};

type Point = [f32; 3];
type Line = [Point; 2];

const DEPTH_TOLERANCE: f64 = 4.0 * f32::EPSILON as f64;
const PARAMETER_TOLERANCE: f64 = 1e-12;
const TILE_LEVELS: usize = 6;
const STEP_TRIANGLES: usize = 32;

#[derive(Clone, Copy)]
struct Bounds {
    min: [f32; 2],
    max: [f32; 2],
}

impl Bounds {
    fn points(points: &[Point]) -> Self {
        Self {
            min: [0, 1].map(|axis| points.iter().map(|point| point[axis]).fold(f32::INFINITY, f32::min)),
            max: [0, 1].map(|axis| points.iter().map(|point| point[axis]).fold(f32::NEG_INFINITY, f32::max)),
        }
    }

    fn overlaps(self, other: Self) -> bool {
        (0..2).all(|axis| self.min[axis] <= other.max[axis] && other.min[axis] <= self.max[axis])
    }

    fn contains(self, other: Self) -> bool {
        (0..2).all(|axis| self.min[axis] <= other.min[axis] && other.max[axis] <= self.max[axis])
    }

    fn tiles(self) -> [Self; 4] {
        let center = [0, 1].map(|axis| (self.min[axis] + self.max[axis]) * 0.5);
        std::array::from_fn(|tile| Self {
            min: [0, 1].map(|axis| if tile & (1 << axis) == 0 { self.min[axis] } else { center[axis] }),
            max: [0, 1].map(|axis| if tile & (1 << axis) == 0 { center[axis] } else { self.max[axis] }),
        })
    }
}

struct Occluder {
    points: [[f64; 3]; 3],
    bounds: Bounds,
    area: f64,
}

impl Occluder {
    fn new(points: [Point; 3]) -> Option<Self> {
        let bounds = Bounds::points(&points);
        let points = points.map(|point| point.map(f64::from));
        let area = cross(points[0], points[1], points[2]);
        (area.is_finite() && area != 0.0).then_some(Self { points, bounds, area })
    }

    fn hidden_interval(&self, line: &ProjectedLine) -> Option<[f64; 2]> {
        if !self.bounds.overlaps(line.bounds) {
            return None;
        }
        let weights = line.points.map(|point| {
            [0, 1, 2].map(|index| cross(self.points[(index + 1) % 3], self.points[(index + 2) % 3], point) / self.area)
        });
        let mut interval = [0.0, 1.0];
        for index in 0..3 {
            intersect_positive(&mut interval, [weights[0][index], weights[1][index]])?;
        }
        let nearer_than_edge = [0, 1].map(|endpoint| {
            line.points[endpoint][2] - (0..3).map(|corner| weights[endpoint][corner] * self.points[corner][2]).sum::<f64>()
        });
        let max_gap = interval.map(|t| nearer_than_edge[0] + t * (nearer_than_edge[1] - nearer_than_edge[0])).into_iter().fold(f64::NEG_INFINITY, f64::max);
        if max_gap <= DEPTH_TOLERANCE {
            return None;
        }
        intersect_positive(&mut interval, nearer_than_edge)?;
        Some(interval)
    }
}

fn cross(a: [f64; 3], b: [f64; 3], point: [f64; 3]) -> f64 {
    (b[0] - a[0]) * (point[1] - a[1]) - (b[1] - a[1]) * (point[0] - a[0])
}

fn intersect_positive(interval: &mut [f64; 2], values: [f64; 2]) -> Option<()> {
    match values.map(|value| value < 0.0) {
        [true, true] => return None,
        [true, false] => interval[0] = interval[0].max(values[0] / (values[0] - values[1])),
        [false, true] => interval[1] = interval[1].min(values[0] / (values[0] - values[1])),
        [false, false] => {}
    }
    (interval[1] - interval[0] > PARAMETER_TOLERANCE).then_some(())
}

pub struct Occluders {
    bounds: Bounds,
    buckets: BTreeMap<usize, Vec<Occluder>>,
}

impl Occluders {
    pub fn new(dimensions: (u32, u32)) -> Self {
        let max = [dimensions.0 as f32 * 0.5, dimensions.1 as f32 * 0.5];
        Self { bounds: Bounds { min: max.map(|value| -value), max }, buckets: BTreeMap::new() }
    }

    pub fn insert(&mut self, points: [Point; 3]) {
        let Some(triangle) = Occluder::new(points) else { return };
        let mut bounds = self.bounds;
        let mut node = 0;
        for _ in 0..TILE_LEVELS {
            let Some((index, tile)) = bounds.tiles().into_iter().enumerate().find(|(_, tile)| tile.contains(triangle.bounds)) else { break };
            self.buckets.entry(node).or_default();
            bounds = tile;
            node = child_node(node, index);
        }
        self.buckets.entry(node).or_default().push(triangle);
    }

    pub fn line(&self, points: Line) -> ProjectedLine {
        ProjectedLine {
            points: points.map(|point| point.map(f64::from)),
            bounds: Bounds::points(&points),
            visit: vec![TileCursor { node: 0, bounds: self.bounds, index: 0 }],
            events: BTreeMap::from([(Parameter(1.0), 0)]),
            start: 0.0,
            covering: 0,
        }
    }

    pub fn scan_step(&self, line: &mut ProjectedLine) -> bool {
        for _ in 0..STEP_TRIANGLES {
            let Some(cursor) = line.visit.last_mut() else { return true };
            let Some(bucket) = self.buckets.get(&cursor.node) else {
                line.visit.pop();
                continue;
            };
            if let Some(triangle) = bucket.get(cursor.index) {
                cursor.index += 1;
                if let Some([start, end]) = triangle.hidden_interval(line) {
                    *line.events.entry(Parameter(start)).or_default() += 1;
                    *line.events.entry(Parameter(end)).or_default() -= 1;
                }
                continue;
            }
            let cursor = line.visit.pop().expect("active visibility cursor");
            for (index, bounds) in cursor.bounds.tiles().into_iter().enumerate() {
                let node = child_node(cursor.node, index);
                if bounds.overlaps(line.bounds) && self.buckets.contains_key(&node) {
                    line.visit.push(TileCursor { node, bounds, index: 0 });
                }
            }
        }
        line.visit.is_empty()
    }

    pub fn close_step(&mut self) -> bool {
        if let Some(mut bucket) = self.buckets.first_entry() {
            let triangles = bucket.get_mut();
            triangles.truncate(triangles.len().saturating_sub(STEP_TRIANGLES));
            if triangles.is_empty() {
                bucket.remove();
            }
        }
        self.buckets.is_empty()
    }

    pub fn is_empty(&self) -> bool {
        self.buckets.is_empty()
    }
}

fn child_node(node: usize, index: usize) -> usize {
    node * 4 + index + 1
}

struct TileCursor {
    node: usize,
    bounds: Bounds,
    index: usize,
}

#[derive(Clone, Copy, PartialEq)]
struct Parameter(f64);

impl Eq for Parameter {}

impl Ord for Parameter {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).expect("finite clipping parameter")
    }
}

impl PartialOrd for Parameter {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct ProjectedLine {
    points: [[f64; 3]; 2],
    bounds: Bounds,
    visit: Vec<TileCursor>,
    events: BTreeMap<Parameter, i64>,
    start: f64,
    covering: i64,
}

pub enum LineStep {
    Pending,
    Segment { points: Line, hidden: bool },
    Complete,
}

impl ProjectedLine {
    fn interval_step(&mut self) -> Option<([f64; 2], bool)> {
        let (Parameter(end), delta) = self.events.pop_first()?;
        let hidden = self.covering > 0;
        self.covering += delta;
        if end != 1.0 && hidden == (self.covering > 0) {
            return None;
        }
        let start = std::mem::replace(&mut self.start, end);
        (end - start > PARAMETER_TOLERANCE).then_some(([start, end], hidden))
    }

    pub fn next(&mut self, occluders: &Occluders) -> LineStep {
        if !occluders.scan_step(self) {
            return LineStep::Pending;
        }
        for _ in 0..STEP_TRIANGLES {
            if self.events.is_empty() {
                return LineStep::Complete;
            }
            if let Some((interval, hidden)) = self.interval_step() {
                let points = interval.map(|t| {
                    std::array::from_fn(|axis| (self.points[0][axis] + t * (self.points[1][axis] - self.points[0][axis])) as f32)
                });
                return LineStep::Segment { points, hidden };
            }
        }
        LineStep::Pending
    }

    pub fn close_step(&mut self) -> bool {
        for _ in 0..STEP_TRIANGLES {
            if self.events.pop_first().is_none() {
                return true;
            }
        }
        false
    }
}
