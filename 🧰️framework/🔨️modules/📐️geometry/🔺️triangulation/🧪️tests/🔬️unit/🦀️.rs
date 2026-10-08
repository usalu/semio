use super::*;
use crate::geom_sel::point_in_polygon;
use crate::polygon_area;
use serde_json::Value;

fn fixtures() -> Value {
    serde_json::from_str(include_str!("../../../🧫️fixtures/🔺️triangulation/🔣️.json")).unwrap()
}

fn ring(v: &Value) -> Vec<Point> {
    v.as_array().unwrap().iter().map(|p| Point::new(p[0].as_f64().unwrap(), p[1].as_f64().unwrap())).collect()
}

fn check(name: &str, outer: &[Point], holes: &[Vec<Point>], area: f64, triangles: Option<usize>) {
    let t = triangulate(outer, holes);
    assert!((t.area() - area).abs() <= 1e-9 * area.max(1.0), "{name}: area {} vs {area}", t.area());
    if let Some(count) = triangles {
        assert_eq!(t.triangles.len(), count, "{name}: triangle count");
    }
    for tri in &t.triangles {
        let [a, b, c] = tri.map(|i| t.vertices[i as usize]);
        assert!(cross2(a, b, c) > 0.0, "{name}: clockwise or degenerate triangle");
        let centre = Point::new((a.x + b.x + c.x) / 3.0, (a.y + b.y + c.y) / 3.0);
        assert!(point_in_polygon(centre, outer), "{name}: triangle outside the outer ring");
        assert!(holes.iter().all(|h| !point_in_polygon(centre, h)), "{name}: triangle inside a hole");
    }
}

#[test]
fn fixtures_triangulate_to_the_exact_area_and_count() {
    for case in fixtures().as_array().unwrap() {
        let holes: Vec<Vec<Point>> = case["holes"].as_array().unwrap().iter().map(ring).collect();
        check(case["name"].as_str().unwrap(), &ring(&case["outer"]), &holes, case["area"].as_f64().unwrap(), Some(case["triangles"].as_u64().unwrap() as usize));
    }
}

struct Lcg(u64);

impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.0 >> 11) as f64 / (1_u64 << 53) as f64
    }
}

#[test]
fn random_star_polygons_cover_their_exact_area() {
    let mut rng = Lcg(0x9E3779B97F4A7C15);
    for case in 0..200 {
        let n = 5 + (rng.next() * 25.0) as usize;
        let points: Vec<Point> = (0..n)
            .map(|k| {
                let angle = (k as f64 + rng.next() * 0.8) / n as f64 * std::f64::consts::TAU;
                let r = 1.0 + rng.next() * 4.0;
                Point::new(r * angle.cos(), r * angle.sin())
            })
            .collect();
        let hole: Vec<Point> = (0..6).map(|k| Point::new(0.4 * (k as f64 * std::f64::consts::TAU / 6.0).cos(), 0.4 * (k as f64 * std::f64::consts::TAU / 6.0).sin())).collect();
        let expected = polygon_area(&points).abs() - polygon_area(&hole).abs();
        check(&format!("star {case}"), &points, &[hole], expected, Some(n + 6 + 2 - 2));
    }
}

#[test]
fn degenerate_inputs_produce_no_triangles() {
    assert!(triangulate(&[], &[]).triangles.is_empty());
    assert!(triangulate(&[Point::ZERO, Point::new(1.0, 0.0)], &[]).triangles.is_empty());
    let line = triangulate(&[Point::ZERO, Point::new(1.0, 0.0), Point::new(2.0, 0.0)], &[]);
    assert!(line.triangles.is_empty());
}

fn boundary_edges(t: &Triangulation) -> std::collections::BTreeSet<(u32, u32)> {
    let mut directed = std::collections::BTreeSet::new();
    for tri in &t.triangles {
        for k in 0..3 {
            directed.insert((tri[k], tri[(k + 1) % 3]));
        }
    }
    directed.iter().filter(|(a, b)| !directed.contains(&(*b, *a))).copied().collect()
}

#[test]
fn collinear_boundary_vertices_stay_on_the_boundary() {
    let mut outer = Vec::new();
    for k in 0..6 {
        outer.push(Point::new(f64::from(k), 0.0));
    }
    for k in 0..4 {
        outer.push(Point::new(6.0, f64::from(k)));
    }
    for k in 0..6 {
        outer.push(Point::new(6.0 - f64::from(k), 3.0));
    }
    for k in 0..3 {
        outer.push(Point::new(0.0, 3.0 - f64::from(k)));
    }
    let t = triangulate(&outer, &[]);
    assert!((t.area() - 18.0).abs() < 1e-12);
    let used: std::collections::BTreeSet<u32> = boundary_edges(&t).iter().flat_map(|(a, b)| [*a, *b]).collect();
    assert_eq!(used.len(), outer.len(), "every input vertex, collinear ones included, lies on the triangulation boundary");
    assert_eq!(boundary_edges(&t).len(), outer.len());
}

#[test]
fn subdivided_holes_keep_every_vertex_on_their_boundary() {
    let mut outer = Vec::new();
    let mut hole = Vec::new();
    for k in 0..5 {
        outer.push(Point::new(f64::from(k), 0.0));
    }
    for k in 0..5 {
        outer.push(Point::new(5.0, f64::from(k)));
    }
    for k in 0..5 {
        outer.push(Point::new(5.0 - f64::from(k), 5.0));
    }
    for k in 0..5 {
        outer.push(Point::new(0.0, 5.0 - f64::from(k)));
    }
    for k in 0..3 {
        hole.push(Point::new(1.5 + f64::from(k), 1.5));
    }
    for k in 0..3 {
        hole.push(Point::new(4.5 - 1.0, 1.5 + f64::from(k)));
    }
    for k in 0..3 {
        hole.push(Point::new(3.5 - f64::from(k), 3.5));
    }
    for k in 0..3 {
        hole.push(Point::new(1.5, 3.5 - f64::from(k)));
    }
    hole.dedup();
    if hole.last() == hole.first() {
        hole.pop();
    }
    let t = triangulate(&outer, &[hole.clone()]);
    assert!((t.area() - (25.0 - 4.0)).abs() < 1e-12, "area {}", t.area());
    let used: std::collections::BTreeSet<u32> = boundary_edges(&t).iter().flat_map(|(a, b)| [*a, *b]).collect();
    assert_eq!(used.len(), outer.len() + hole.len());
}

#[test]
fn collinear_vertices_do_not_change_the_area() {
    let outer = vec![Point::new(0.0, 0.0), Point::new(1.0, 0.0), Point::new(2.0, 0.0), Point::new(2.0, 1.0), Point::new(2.0, 2.0), Point::new(0.0, 2.0)];
    assert!((triangulate(&outer, &[]).area() - 4.0).abs() < 1e-12);
}

#[test]
fn triangulation_is_deterministic() {
    let outer = ring(&fixtures()[7]["outer"]);
    let holes: Vec<Vec<Point>> = fixtures()[7]["holes"].as_array().unwrap().iter().map(ring).collect();
    assert_eq!(triangulate(&outer, &holes), triangulate(&outer, &holes));
}
