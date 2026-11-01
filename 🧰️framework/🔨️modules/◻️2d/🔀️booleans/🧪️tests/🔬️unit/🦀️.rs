
use super::*;

fn polygon(path:&[PathSegment])->Result<BooleanOperand,BooleanError>{let mut rings=Vec::new();let mut ring=None;for segment in path{match segment{PathSegment::Move{to}=>{if ring.is_some(){return Err(BooleanError::Invalid("Boolean contour must be closed"));}ring=Some(vec![*to]);},PathSegment::Line{to}=>ring.as_mut().ok_or(BooleanError::Invalid("Boolean line requires a contour"))?.push(*to),PathSegment::Close=>rings.push(ring.take().ok_or(BooleanError::Invalid("Boolean close requires a contour"))?),_=>return Err(BooleanError::Invalid("Boolean fixture requires linear contours"))}}if ring.is_some(){return Err(BooleanError::Invalid("Boolean contour must be closed"));}Ok(BooleanOperand{contours:rings,fill_rule:BooleanFillRule::Evenodd})}
fn evaluate(a:&[PathSegment],b:&[PathSegment],operation:&str)->Result<Vec<PathSegment>,BooleanError>{evaluate_many(&[a.to_vec(),b.to_vec()],operation)}
fn evaluate_many(paths:&[Vec<PathSegment>],operation:&str)->Result<Vec<PathSegment>,BooleanError>{evaluate_many_with(paths,operation,1e-9)}
fn evaluate_with(a:&[PathSegment],b:&[PathSegment],operation:&str,epsilon:f64)->Result<Vec<PathSegment>,BooleanError>{evaluate_many_with(&[a.to_vec(),b.to_vec()],operation,epsilon)}
fn evaluate_many_with(paths:&[Vec<PathSegment>],operation:&str,epsilon:f64)->Result<Vec<PathSegment>,BooleanError>{let operation=BooleanOperation::parse(operation)?;if paths.is_empty(){return Err(BooleanError::Invalid("Boolean operands are required"));}let operands=paths.iter().map(|path|polygon(path)).collect::<Result<Vec<_>,_>>()?;let input=BooleanInput{operation,operands,epsilon,max_edges:65536,max_parameters:262144,max_atomic_edges:262144,max_segments:65536,max_work:100000000};Ok(super::bounded_tests::finish(input,7).0)}


fn square(origin: [f64; 2], size: f64) -> Vec<PathSegment> {
    vec![PathSegment::Move { to: origin }, PathSegment::Line { to: [origin[0] + size, origin[1]] }, PathSegment::Line { to: [origin[0] + size, origin[1] + size] }, PathSegment::Line { to: [origin[0], origin[1] + size] }, PathSegment::Close]
}

fn reversed(path: Vec<PathSegment>) -> Vec<PathSegment> {
    let mut points = path
        .into_iter()
        .filter_map(|segment| match segment {
            PathSegment::Move { to } | PathSegment::Line { to } => Some(to),
            _ => None,
        })
        .collect::<Vec<_>>();
    points.reverse();
    let first = points.remove(0);
    let mut result = vec![PathSegment::Move { to: first }];
    result.extend(points.into_iter().map(|to| PathSegment::Line { to }));
    result.push(PathSegment::Close);
    result
}

fn contours(path: &[PathSegment]) -> usize {
    path.iter().filter(|segment| matches!(segment, PathSegment::Move { .. })).count()
}

fn absolute_ring_area(path: &[PathSegment]) -> f64 {
    let operand=polygon(path).expect("valid result");
    operand.contours.iter().map(|ring|{let mut area=0.0;for at in 1..ring.len().saturating_sub(1){area+=cross(ring[0],ring[at],ring[at+1]);}area.abs()/2.0}).sum()
}

#[test]
fn oracle_operations_match_overlapping_rectangles() {
    let a = square([0.0, 0.0], 10.0);
    let b = square([5.0, 5.0], 10.0);
    let union = evaluate(&a, &b, "union").expect("union");
    assert_eq!(absolute_ring_area(&union), 175.0);
    assert_eq!(
        union,
        vec![
            PathSegment::Move { to: [0.0, 0.0] },
            PathSegment::Line { to: [10.0, 0.0] },
            PathSegment::Line { to: [10.0, 5.0] },
            PathSegment::Line { to: [15.0, 5.0] },
            PathSegment::Line { to: [15.0, 15.0] },
            PathSegment::Line { to: [5.0, 15.0] },
            PathSegment::Line { to: [5.0, 10.0] },
            PathSegment::Line { to: [0.0, 10.0] },
            PathSegment::Close,
        ]
    );
    assert_eq!(absolute_ring_area(&evaluate(&a, &b, "intersection").expect("intersection")), 25.0);
    assert_eq!(absolute_ring_area(&evaluate(&a, &b, "difference").expect("difference")), 75.0);
    assert_eq!(contours(&evaluate(&a, &b, "xor").expect("xor")), 2);
}

#[test]
fn oracle_preserves_disjoint_and_touching_topology() {
    let a = square([0.0, 0.0], 5.0);
    assert_eq!(contours(&evaluate(&a, &square([10.0, 0.0], 5.0), "union").expect("disjoint")), 2);
    assert_eq!(contours(&evaluate(&a, &square([5.0, 0.0], 5.0), "union").expect("edge touch")), 1);
    assert_eq!(contours(&evaluate(&a, &square([5.0, 5.0], 5.0), "union").expect("vertex touch")), 2);
    assert!(evaluate(&a, &square([10.0, 0.0], 5.0), "intersection").unwrap().is_empty());
}

#[test]
fn oracle_emits_hole_for_contained_difference_and_xor() {
    let outer = square([0.0, 0.0], 10.0);
    let inner = square([2.0, 2.0], 4.0);
    assert_eq!(contours(&evaluate(&outer, &inner, "difference").expect("difference")), 2);
    assert_eq!(contours(&evaluate(&outer, &inner, "xor").expect("xor")), 2);
    let mut two_outers = square([0.0, 0.0], 10.0);
    two_outers.extend(square([20.0, 0.0], 10.0));
    let mut two_inners = square([2.0, 2.0], 4.0);
    two_inners.extend(square([22.0, 2.0], 4.0));
    let result = evaluate(&two_outers, &two_inners, "difference").expect("two polygons with holes");
    let moves = result
        .iter()
        .filter_map(|segment| match segment {
            PathSegment::Move { to } => Some(*to),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(moves, vec![[0.0, 0.0], [2.0, 6.0], [20.0, 0.0], [22.0, 6.0]]);
}

#[test]
fn oracle_is_winding_independent_and_deterministic() {
    let a = square([0.0, 0.0], 10.0);
    let b = square([5.0, 5.0], 10.0);
    let expected = evaluate(&a, &b, "union").expect("union");
    assert_eq!(evaluate(&reversed(a), &reversed(b), "union").expect("reversed"), expected);
    assert_eq!(evaluate(&square([0.0, 0.0], 10.0), &square([5.0, 5.0], 10.0), "union").expect("repeat"), expected);
}

#[test]
fn oracle_discards_degenerate_and_duplicate_edges() {
    let degenerate = vec![PathSegment::Move { to: [0.0, 0.0] }, PathSegment::Line { to: [5.0, 0.0] }, PathSegment::Line { to: [10.0, 0.0] }, PathSegment::Close];
    let result = evaluate(&degenerate, &square([20.0, 0.0], 2.0), "union").expect("union");
    assert_eq!(contours(&result), 1);
    assert_eq!(absolute_ring_area(&result), 4.0);
    let duplicate = vec![PathSegment::Move { to: [0.0, 0.0] }, PathSegment::Line { to: [5.0, 0.0] }, PathSegment::Line { to: [5.0, 0.0] }, PathSegment::Line { to: [5.0, 5.0] }, PathSegment::Line { to: [0.0, 5.0] }, PathSegment::Close];
    assert_eq!(absolute_ring_area(&evaluate(&duplicate, &square([10.0, 0.0], 2.0), "union").expect("duplicate")), 29.0);
    assert_eq!(absolute_ring_area(&evaluate(&square([0.0, 0.0], 10.0), &square([5.0, 0.0], 10.0), "intersection").expect("collinear overlap")), 50.0);
}

#[test]
fn translated_coordinates_preserve_topology() {
    let origin = 1.0e12-100.0;
    let (a, b) = (square([origin, origin], 10.0), square([origin + 5.0, origin + 5.0], 10.0));
    let union = evaluate_with(&a, &b, "union", 1.0e-2).expect("translated union");
    assert_eq!(contours(&union), 1);
    assert_eq!(union.len(), 9);
}

#[test]
fn self_crossing_even_odd_contour_is_regularized() {
    let bow_tie = vec![PathSegment::Move { to: [0.0, 0.0] }, PathSegment::Line { to: [10.0, 10.0] }, PathSegment::Line { to: [0.0, 10.0] }, PathSegment::Line { to: [10.0, 0.0] }, PathSegment::Close];
    let result = evaluate(&bow_tie, &square([20.0, 0.0], 2.0), "union").expect("self-crossing union");
    assert_eq!(contours(&result), 3);
    assert_eq!(absolute_ring_area(&result), 54.0);
}

#[test]
fn oracle_many_operations_keep_intermediate_holes() {
    let inputs = vec![square([0.0, 0.0], 10.0), square([2.0, 2.0], 6.0), square([4.0, -2.0], 2.0)];
    assert_eq!(contours(&evaluate_many(&inputs, "difference").expect("difference")), 2);
    assert!(evaluate_many(&inputs, "intersection").unwrap().is_empty());
    let recovered = evaluate_many(&[square([0.0, 0.0], 10.0), square([0.0, 0.0], 10.0), square([20.0, 0.0], 2.0)], "xor").expect("xor recovers from an empty intermediate result");
    assert_eq!(absolute_ring_area(&recovered), 4.0);
}

#[test]
fn errors_match_public_contract() {
    let open = vec![PathSegment::Move { to: [0.0, 0.0] }, PathSegment::Line { to: [1.0, 1.0] }];
    assert!(matches!(evaluate(&open, &square([0.0, 0.0], 5.0), "union"), Err(BooleanError::Invalid(_))));
    assert!(matches!(evaluate(&square([0.0, 0.0], 5.0), &square([1.0, 1.0], 5.0), "bogus"), Err(BooleanError::Invalid(message)) if message.to_lowercase().contains("unknown boolean operation")));
    assert!(matches!(evaluate_many(&[], "union"), Err(BooleanError::Invalid(_))));
    assert!(evaluate(&square([0.0, 0.0], 5.0), &square([100.0, 100.0], 5.0), "intersection").unwrap().is_empty());
    assert!(evaluate_many(&[square([0.0,0.0],5.0)],"bogus").is_err());
    assert!(evaluate(&[PathSegment::Move{to:[0.0,0.0]},PathSegment::Quad{ctrl:[1.0,1.0],to:[2.0,0.0]},PathSegment::Close],&square([0.0,0.0],1.0),"union").is_err());
}
