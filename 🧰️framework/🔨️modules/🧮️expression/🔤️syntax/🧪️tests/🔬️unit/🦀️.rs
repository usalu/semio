use super::*;
use crate::tree::{AngleUnit, AreaUnit, BinaryOp, CompareOp, Expr, Function, LengthUnit, UnaryOp, Value, VolumeUnit};
use std::collections::BTreeMap;

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }

    fn pick<'a, T>(&mut self, items: &'a [T]) -> &'a T {
        &items[self.below(items.len())]
    }
}

const NAMES: &[&str] = &["w", "h", "a", "n", "f", "Höhe", "frame width", "if", "true", "m", "min", "sqrt", "x²", "2x", "a`b", "tab\tname", "and", "e1", "_u"];
const NUMBERS: &[f64] = &[0.0, 1.0, 2.0, 0.5, 3.14159, 1e-7, 1e20, 90.0, 0.1 + 0.2, 123456.789, 7.0, 1e300];
const TEXTS: &[&str] = &["", "a b", "q\"uote", "back\\slash", "nl\nx", "tab\t", "°²"];

fn generate(rng: &mut Rng, depth: usize) -> Expr {
    let leaf = depth == 0 || rng.below(4) == 0;
    if leaf {
        return match rng.below(9) {
            0 => Expr::Number(*rng.pick(NUMBERS)),
            1 => Expr::Length(*rng.pick(NUMBERS), *rng.pick(LengthUnit::ALL)),
            2 => Expr::Angle(*rng.pick(NUMBERS), *rng.pick(AngleUnit::ALL)),
            3 => Expr::Area(*rng.pick(NUMBERS), *rng.pick(AreaUnit::ALL)),
            4 => Expr::Volume(*rng.pick(NUMBERS), *rng.pick(VolumeUnit::ALL)),
            5 => Expr::Bool(rng.below(2) == 0),
            6 => Expr::Text(rng.pick(TEXTS).to_string()),
            _ => Expr::param(rng.pick(NAMES)),
        };
    }
    let d = depth - 1;
    match rng.below(8) {
        0 => Expr::unary(*rng.pick(&[UnaryOp::Negate, UnaryOp::Not]), generate(rng, d)),
        1 | 2 => Expr::binary(*rng.pick(BinaryOp::ALL), generate(rng, d), generate(rng, d)),
        3 => Expr::compare(*rng.pick(CompareOp::ALL), generate(rng, d), generate(rng, d)),
        4 => Expr::And(Box::new(generate(rng, d)), Box::new(generate(rng, d))),
        5 => Expr::Or(Box::new(generate(rng, d)), Box::new(generate(rng, d))),
        6 => Expr::conditional(generate(rng, d), generate(rng, d), generate(rng, d)),
        _ => {
            let f = *rng.pick(Function::ALL);
            Expr::Call(f, (0..f.arity()).map(|_| generate(rng, d)).collect())
        }
    }
}

fn corpus(count: usize) -> Vec<Expr> {
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    (0..count).map(|i| generate(&mut rng, 1 + i % 6)).collect()
}

#[test]
fn print_then_parse_returns_the_same_tree_for_a_generated_corpus() {
    for e in corpus(20_000) {
        assert!(e.is_canonical());
        let text = print(&e);
        assert_eq!(parse(&text), Ok(e.clone()), "{text}");
    }
}

#[test]
fn printing_is_a_fixed_point() {
    for e in corpus(5_000) {
        let once = print(&e);
        assert_eq!(print(&parse(&once).unwrap()), once);
    }
}

#[test]
fn the_round_trip_preserves_evaluation_results_and_error_paths() {
    let env: BTreeMap<String, Value> = [("w", Value::Length(2.4)), ("h", Value::Length(0.9)), ("a", Value::Angle(0.5)), ("n", Value::Number(3.0)), ("f", Value::Bool(true)), ("m", Value::Number(2.0))]
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect();
    let mut ok = 0;
    for e in corpus(10_000) {
        let back = parse(&print(&e)).unwrap();
        let (x, y) = (crate::evaluate(&e, &env), crate::evaluate(&back, &env));
        assert_eq!(x, y);
        ok += usize::from(x.is_ok());
    }
    assert!(ok > 200, "only {ok} generated expressions evaluate; the corpus is too ill-kinded to be meaningful");
}

#[test]
fn spans_of_a_generated_corpus_are_nested_and_cover_every_node() {
    for e in corpus(2_000) {
        let (text, spans) = print_with_spans(&e);
        assert_eq!(spans.len(), e.size());
        let total = text.chars().count();
        let by_path: BTreeMap<Vec<usize>, Span> = spans.into_iter().collect();
        for (path, span) in &by_path {
            assert!(span.start < span.end && span.end <= total);
            if !path.is_empty() {
                let outer = by_path[&path[..path.len() - 1]];
                assert!(outer.start <= span.start && span.end <= outer.end, "{text}");
            }
        }
    }
}

#[test]
fn the_grammar_in_the_module_docs_accepts_every_documented_unit() {
    for unit in ["mm", "cm", "m", "km", "in", "ft", "deg", "°", "rad", "mm2", "cm2", "m2", "mm3", "cm3", "m3", "l", "m²", "m³"] {
        assert!(parse(&format!("1 {unit}")).is_ok(), "{unit}");
    }
}

#[test]
fn error_codes_are_stable_and_unique() {
    let samples = ["#", "1e999", "\"a", "`a", "``", "\"\\q\"", "1 +", "1 2", "2 foo", "frob(1)", "sqrt()", "a < b < c"];
    let codes: Vec<&str> = samples.iter().map(|s| parse(s).unwrap_err().code()).collect();
    assert_eq!(codes, ["unexpected-character", "invalid-number", "unterminated-text", "unterminated-name", "empty-name", "invalid-escape", "unexpected-end", "unexpected-token", "unknown-unit", "unknown-function", "argument-count", "chained-comparison"]);
}
