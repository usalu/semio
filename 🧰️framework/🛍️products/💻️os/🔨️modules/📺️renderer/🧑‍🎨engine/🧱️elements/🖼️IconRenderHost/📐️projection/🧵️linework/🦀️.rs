//! 🧵️ Incremental assembly of projected line paths within a single SVG layer.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;

const POINT_SCALE: f32 = 1024.0;
type Point = [i32; 2];

#[derive(Default)]
struct Node {
    neighbors: BTreeSet<Point>,
    corner: Option<[Point; 2]>,
    degree: usize,
}

#[derive(Default)]
pub struct LineGraph {
    nodes: BTreeMap<Point, Node>,
    ends: BTreeSet<Point>,
    cursor: Option<Point>,
    start: Option<Point>,
    len: usize,
}

impl LineGraph {
    pub fn len(&self) -> usize { self.len }

    pub fn is_empty(&self) -> bool { self.len == 0 }

    pub fn insert(&mut self, points: [[f32; 3]; 2]) {
        let [a, b] = points.map(|point| [0, 1].map(|axis| (point[axis] * POINT_SCALE).round() as i32));
        if a == b || self.nodes.get(&a).is_some_and(|node| node.neighbors.contains(&b)) {
            return;
        }
        for (point, neighbor) in [(a, b), (b, a)] {
            let node = self.nodes.entry(point).or_default();
            node.neighbors.insert(neighbor);
            node.degree = node.neighbors.len();
            if node.degree == 2 {
                self.ends.remove(&point);
                node.corner = node.neighbors.first().copied().zip(node.neighbors.last().copied()).map(|(first, last)| [first, last]);
            } else {
                self.ends.insert(point);
                node.corner = None;
            }
        }
        self.len += 1;
    }

    pub fn next(&mut self, merge: bool, radius: f32) -> Option<String> {
        let continuation = self.cursor.take();
        let from = continuation.or_else(|| self.ends.first().copied()).or_else(|| self.nodes.first_key_value().map(|(&point, _)| point))?;
        let to = *self.nodes[&from].neighbors.first()?;
        let closes = self.start == Some(to);
        if merge && continuation.is_none() {
            self.start = Some(from);
        }
        if merge && self.nodes[&to].degree == 2 && !closes {
            self.cursor = Some(to);
        } else {
            self.start = None;
        }
        let from_corner = self.nodes[&from].corner.and_then(|neighbors| Corner::new(from, to, neighbors, radius));
        let to_corner = self.nodes[&to].corner.and_then(|neighbors| Corner::new(to, from, neighbors, radius));
        let mut path = String::new();
        if continuation.is_none() {
            match (merge, from_corner) {
                (false, Some(corner)) => {
                    command(&mut path, 'M', corner.midpoint());
                    corner.curve(&mut path, corner.half_control(), corner.trim);
                }
                (_, corner) => command(&mut path, 'M', corner.map_or_else(|| position(from), |corner| corner.trim)),
            }
        }
        match (merge, to_corner) {
            (false, Some(corner)) => {
                command(&mut path, 'L', corner.trim);
                corner.curve(&mut path, corner.half_control(), corner.midpoint());
            }
            (true, Some(corner)) => {
                command(&mut path, 'L', corner.trim);
                corner.curve(&mut path, corner.vertex, corner.opposite);
            }
            (_, None) => command(&mut path, 'L', position(to)),
        }
        if merge && closes {
            path.push('Z');
        }
        self.remove_edge(from, to);
        Some(path)
    }

    pub fn remove_one(&mut self) -> bool {
        let Some((&point, node)) = self.nodes.first_key_value() else { return false };
        let neighbor = *node.neighbors.first().expect("line graph stores no isolated nodes");
        self.remove_edge(point, neighbor);
        true
    }

    fn remove_edge(&mut self, a: Point, b: Point) {
        for (point, neighbor) in [(a, b), (b, a)] {
            let node = self.nodes.get_mut(&point).expect("line graph stores both endpoints");
            node.neighbors.remove(&neighbor);
            if node.neighbors.is_empty() {
                self.nodes.remove(&point);
                self.ends.remove(&point);
            }
        }
        self.len -= 1;
    }
}

#[derive(Clone, Copy)]
struct Corner {
    vertex: [f32; 2],
    trim: [f32; 2],
    opposite: [f32; 2],
}

impl Corner {
    fn new(vertex: Point, toward: Point, neighbors: [Point; 2], radius: f32) -> Option<Self> {
        if radius <= 0.0 {
            return None;
        }
        let other = if neighbors[0] == toward { neighbors[1] } else { neighbors[0] };
        let vertex = position(vertex);
        let [a, b] = [toward, other].map(|point| {
            let point = position(point);
            [point[0] - vertex[0], point[1] - vertex[1]]
        });
        let lengths = [a, b].map(|delta| delta[0].hypot(delta[1]));
        if (a[0] * b[1] - a[1] * b[0]).abs() <= 1e-5 * lengths[0] * lengths[1] {
            return None;
        }
        let trim = radius.min(lengths[0].min(lengths[1]) * 0.5);
        let [trim, opposite] = [a, b].map(|delta| {
            let length = delta[0].hypot(delta[1]);
            [0, 1].map(|axis| vertex[axis] + delta[axis] * (trim / length))
        });
        Some(Self { vertex, trim, opposite })
    }

    fn midpoint(self) -> [f32; 2] {
        [0, 1].map(|axis| (self.trim[axis] + 2.0 * self.vertex[axis] + self.opposite[axis]) * 0.25)
    }

    fn half_control(self) -> [f32; 2] {
        [0, 1].map(|axis| (self.trim[axis] + self.vertex[axis]) * 0.5)
    }

    fn curve(self, path: &mut String, control: [f32; 2], end: [f32; 2]) {
        let _ = write!(path, "Q{},{} {},{}", scalar(control[0]), scalar(control[1]), scalar(end[0]), scalar(end[1]));
    }
}

fn position(point: Point) -> [f32; 2] {
    point.map(|coordinate| coordinate as f32 / POINT_SCALE)
}

fn command(path: &mut String, operation: char, point: [f32; 2]) {
    let _ = write!(path, "{operation}{},{}", scalar(point[0]), scalar(point[1]));
}

pub fn scalar(value: f32) -> String {
    if value.abs() < 0.000_000_5 {
        return "0".into();
    }
    let mut value = format!("{value:.6}");
    while value.ends_with('0') {
        value.pop();
    }
    if value.ends_with('.') {
        value.pop();
    }
    value
}
