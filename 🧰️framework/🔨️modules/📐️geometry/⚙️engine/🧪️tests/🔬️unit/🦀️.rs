
use super::*;

#[test]
fn distance_between_matches_pythagoras() {
    let a = Point::new(0.0, 0.0);
    let b = Point::new(3.0, 4.0);
    assert!((distance_between(a, b) - 5.0).abs() < 1e-9);
}

#[test]
fn normalize_or_zero_handles_zero_vector() {
    let v = normalize_or_zero(Vec2::new(0.0, 0.0));
    assert_eq!(v.x(), 0.0);
    assert_eq!(v.y(), 0.0);
}

#[test]
fn cubic_split_endpoints_match_source_and_split_point() {
    let c = CubicBez::new(Point::new(0.0, 0.0), Point::new(0.0, 10.0), Point::new(10.0, 10.0), Point::new(10.0, 0.0));
    let mid = cubic_point_at(c, 0.5);
    let (left, right) = cubic_split(c, 0.5);
    assert!(distance_between(left.p0(), c.p0()) < 1e-9);
    assert!(distance_between(right.p3(), c.p3()) < 1e-9);
    assert!(distance_between(left.p3(), mid) < 1e-9);
    assert!(distance_between(right.p0(), mid) < 1e-9);
}

#[test]
fn cubic_nearest_t_finds_endpoint_for_endpoint_query() {
    let c = CubicBez::new(Point::new(0.0, 0.0), Point::new(0.0, 10.0), Point::new(10.0, 10.0), Point::new(10.0, 0.0));
    let t = cubic_nearest_t(c.p0(), c, 64);
    assert!(t < 0.05);
}

#[test]
fn segment_intersection_finds_crossing_point() {
    let hit = segment_intersection(Point::new(0.0, 0.0), Point::new(10.0, 10.0), Point::new(0.0, 10.0), Point::new(10.0, 0.0));
    let hit = hit.expect("segments cross");
    assert!(distance_between(hit, Point::new(5.0, 5.0)) < 1e-9);
}

#[test]
fn segment_intersection_none_for_parallel_lines() {
    let hit = segment_intersection(Point::new(0.0, 0.0), Point::new(10.0, 0.0), Point::new(0.0, 5.0), Point::new(10.0, 5.0));
    assert!(hit.is_none());
}

#[test]
fn circle_line_intersections_finds_two_points_through_center() {
    let hits = circle_line_intersections(Point::new(0.0, 0.0), 5.0, Point::new(-10.0, 0.0), Point::new(10.0, 0.0));
    assert_eq!(hits.len(), 2);
    assert!(distance_between(hits[0], Point::new(-5.0, 0.0)) < 1e-9);
    assert!(distance_between(hits[1], Point::new(5.0, 0.0)) < 1e-9);
}

#[test]
fn convex_hull_of_square_with_interior_point_drops_interior() {
    let points = vec![Point::new(0.0, 0.0), Point::new(10.0, 0.0), Point::new(10.0, 10.0), Point::new(0.0, 10.0), Point::new(5.0, 5.0)];
    let hull = convex_hull(&points);
    assert_eq!(hull.len(), 4);
}

#[test]
fn polygon_area_of_unit_square_is_one() {
    let square = vec![Point::new(0.0, 0.0), Point::new(1.0, 0.0), Point::new(1.0, 1.0), Point::new(0.0, 1.0)];
    assert!((polygon_area(&square).abs() - 1.0).abs() < 1e-9);
}

#[test]
fn polygon_centroid_of_square_is_center() {
    let square = vec![Point::new(0.0, 0.0), Point::new(10.0, 0.0), Point::new(10.0, 10.0), Point::new(0.0, 10.0)];
    let centroid = polygon_centroid(&square);
    assert!(distance_between(centroid, Point::new(5.0, 5.0)) < 1e-9);
}

#[test]
fn bounding_box_covers_all_points() {
    let points = vec![Point::new(-2.0, 3.0), Point::new(5.0, -1.0), Point::new(1.0, 8.0)];
    let bb = bounding_box(&points).expect("non-empty");
    assert_eq!(bb.min_x, -2.0);
    assert_eq!(bb.max_x, 5.0);
    assert_eq!(bb.min_y, -1.0);
    assert_eq!(bb.max_y, 8.0);
}

#[test]
fn point_in_polygon_detects_interior_and_exterior() {
    let square = [Point::new(0.0, 0.0), Point::new(10.0, 0.0), Point::new(10.0, 10.0), Point::new(0.0, 10.0)];
    assert!(geom_sel::point_in_polygon(Point::new(5.0, 5.0), &square));
    assert!(!geom_sel::point_in_polygon(Point::new(15.0, 5.0), &square));
}
