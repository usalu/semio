//! 👆️ Unit suite of the gesture twin: the press machine edge by edge, the heat of attention against its closed form, circling, stroking and shaking against drawn shapes and their mirrors, the committed vectors the Python oracle admitted, and ordinary travel that must set nothing off — the cases of the TypeScript suite.
//!
//! Shapes are drawn with the sine and cosine of `📐️trigonometry` (in turns), so they are the same on every platform;
//! the laws they check hold for any faithful sine. The committed traces are replayed exactly as the subject adapters
//! of both languages replay them. Levels: the tests outside a submodule are fundamental, `quick` and `exhaustive` draw
//! more shapes and replay the travel in four and in all sixteen variants.
//!
//! @see ../../🦀️.rs — the twin under test
//! @see ../../🟦️.ts, ./🟦️.ts — the TypeScript twin and its suite
//! @see ../../../../🧫️fixtures/👆️gesture-recognition/🔣️.json — the committed vectors (numpy's winding, scipy's peaks)

use super::*;
use crate::schema::tests::{assert_same, entries, fixture, number};
use crate::schema::TIERS;
use crate::trigonometry::{cos_turns, sin_turns};
use serde_json::{json, Value};

const BODY: Rect = Rect { x: 380.0, y: 276.0, width: 40.0, height: 48.0 };
const CENTRE: Point = Point { x: 400.0, y: 300.0 };
const MOUSE: Pointer = Pointer::Mouse;
const SOURCE: &str = include_str!("../../🦀️.rs");

/// 🎰️ A deterministic stream of numbers in `[0, 1)`, the TypeScript suite's 32-bit linear congruential generator.
struct Stream(u32);

impl Stream {
    /// ➡️ The next number.
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        f64::from(self.0) / 4_294_967_296.0
    }
}

/// 🔒️ The guards with one of them up by name, or none.
fn up(name: &str) -> Guards {
    match name {
        "control" => Guards { control: true, ..UNGUARDED },
        "scrolled" => Guards { scrolled: true, ..UNGUARDED },
        "quiet" => Guards { quiet: true, ..UNGUARDED },
        "still" => Guards { still: true, ..UNGUARDED },
        _ => UNGUARDED,
    }
}

/// 🏷️ The wire name of a cue, a signal or a tier.
fn named(value: impl Serialize) -> String {
    serde_json::to_value(value).ok().and_then(|value| value.as_str().map(str::to_string)).unwrap_or_default()
}

/// 📮️ A press input of the TypeScript suite's shape.
fn pressed_at(x: f64, y: f64, pointer: Pointer) -> PressInput {
    PressInput::Pressed(Pressed { x, y, pointer })
}

/// 🧲️ A drag to a point.
fn dragged_to(x: f64, y: f64) -> PressInput {
    PressInput::Dragged(Dragged { x, y })
}

/// 🎈️ A release at a point.
fn released_at(x: f64, y: f64) -> PressInput {
    PressInput::Released(Released { x, y })
}

/// 🕹️ The signals of a sequence of press inputs, one per tick from tick 1, each tick passing first; `guards[index]` is up for the input of tick `index + 1`.
fn signals_of(ticks: Ticks, inputs: &[(Ticks, PressInput)], guards: &[Guards]) -> Vec<(Ticks, String)> {
    let mut signals = Vec::new();
    let mut press = IDLE;
    for tick in 1..=ticks {
        let passed = press_step(press, PressInput::Ticked, tick, UNGUARDED);
        press = passed.state;
        if let Some(signal) = passed.signal {
            signals.push((tick, named(signal)));
        }
        for &(at, input) in inputs {
            if at != tick {
                continue;
            }
            let step = press_step(press, input, tick, guards.get(tick as usize - 1).copied().unwrap_or(UNGUARDED));
            press = step.state;
            if let Some(signal) = step.signal {
                signals.push((tick, named(signal)));
            }
        }
    }
    signals
}

/// 🔖️ Expected signals by tick and name.
fn sighted(entries: &[(Ticks, &str)]) -> Vec<(Ticks, String)> {
    entries.iter().map(|&(tick, name)| (tick, name.to_string())).collect()
}

/// ⭕️ The pointer of every tick on a circle round `centre`: `turns` per second, clockwise on screen for `way` = 1, starting at the angle `start` (in turns), for `ticks` ticks; `radius` may change along the way.
fn circling(centre: Point, radius: impl Fn(usize) -> f64, turns: f64, way: f64, start: f64, ticks: usize) -> Vec<Point> {
    (0..ticks)
        .map(|tick| {
            let angle = start + (way * turns * tick as f64) / 64.0;
            Point { x: centre.x + radius(tick) * cos_turns(angle), y: centre.y + radius(tick) * sin_turns(angle) }
        })
        .collect()
}

/// 🐈️ The pointer of every tick going left and right over `centre`: `swing` pixels to either side, `hertz` times a second, `rise` pixels up and down with each stroke.
fn petting(centre: Point, swing: f64, hertz: f64, ticks: usize, rise: f64, phase: f64) -> Vec<Point> {
    (0..ticks)
        .map(|tick| {
            let wave = sin_turns(phase + (hertz * tick as f64) / 64.0);
            Point { x: centre.x + swing * wave, y: centre.y + rise * wave }
        })
        .collect()
}

/// 📈️ The radius of a spiral that starts at 50 px and grows by the factor `per_tick` every tick (the TypeScript suite's `50 × factor^(tick ÷ 64)`, multiplied up instead of raised to a power).
fn spiral(per_tick: f64) -> impl Fn(usize) -> f64 {
    move |tick| (0..tick).fold(50.0, |radius, _| radius * per_tick)
}

/// 🎬️ Every cue of the hover gestures along points past a body.
fn hover_cues(points: &[Point], body: Rect, guards: impl Fn(Ticks) -> Guards) -> Vec<(Ticks, String)> {
    let mut cues = Vec::new();
    let mut hover = no_hover(0);
    for (tick, point) in points.iter().enumerate() {
        let tick = tick as Ticks;
        let step = hover_step(hover, *point, body, tick, guards(tick));
        hover = step.state;
        if let Some(cue) = step.cue {
            cues.push((tick, named(cue)));
        }
    }
    cues
}

/// 🔁️ Every cue of circling alone along points round a body.
fn circle_cues(points: &[Point], body: Rect, guards: impl Fn(Ticks) -> Guards) -> Vec<(Ticks, String)> {
    let mut cues = Vec::new();
    let mut state = no_circling(0);
    for (tick, point) in points.iter().enumerate() {
        let tick = tick as Ticks;
        let step = circle_step(state, *point, body, tick, guards(tick));
        state = step.state;
        if let Some(cue) = step.cue {
            cues.push((tick, named(cue)));
        }
    }
    cues
}

/// 🥤️ Every cue of the shake along the points of a grip.
fn shake_cues(points: &[Point], height: f64, guards: Guards) -> Vec<(Ticks, String)> {
    let mut cues = Vec::new();
    let mut shaking = no_shaking(0);
    for (tick, point) in points.iter().enumerate() {
        let tick = tick as Ticks;
        let step = shake_step(shaking, *point, height, tick, guards);
        shaking = step.state;
        if let Some(cue) = step.cue {
            cues.push((tick, named(cue)));
        }
    }
    cues
}

/// 🪞️ Points mirrored left and right about `x`.
fn mirrored(points: &[Point], x: f64) -> Vec<Point> {
    points.iter().map(|point| Point { x: 2.0 * x - point.x, y: point.y }).collect()
}

/// ⏱️ The ticks of a list of cues.
fn ticks_of(cues: &[(Ticks, String)]) -> Vec<Ticks> {
    cues.iter().map(|(tick, _)| *tick).collect()
}

/// 🔤️ The names of a list of cues.
fn names_of(cues: &[(Ticks, String)]) -> Vec<String> {
    cues.iter().map(|(_, cue)| cue.clone()).collect()
}

//#region 🔖️Replay of the committed vectors
/// 🧵️ A pointer path decoded into one point per tick, in quanta.
#[derive(Clone, Debug, PartialEq)]
struct Trail {
    xs: Vec<i64>,
    ys: Vec<i64>,
}

/// 🔢️ The whole numbers of a list member.
fn integers(value: &Value) -> Vec<i64> {
    entries(value).iter().map(|entry| number(entry) as i64).collect()
}

/// 🧶️ The points of a path, one per tick, in quanta: the first point, then per tick a step or `hold + n` repeats.
fn trail_of(path: &[i64], hold: i64) -> Trail {
    let (mut x, mut y) = (path.first().copied().unwrap_or(0), path.get(1).copied().unwrap_or(0));
    let mut trail = Trail { xs: vec![x], ys: vec![y] };
    let mut index = 2;
    while index < path.len() {
        let token = path[index];
        if token >= hold {
            for _ in 0..token - hold {
                trail.xs.push(x);
                trail.ys.push(y);
            }
            index += 1;
        } else {
            x += token;
            y += path[index + 1];
            trail.xs.push(x);
            trail.ys.push(y);
            index += 2;
        }
    }
    trail
}

/// 🪟️ A trail in one of its sixteen variants on a page: bit 0 mirrors left and right, bit 1 top and bottom, bit 2 swaps the axes, bit 3 plays it backwards.
fn variant_of(trail: &Trail, page: [i64; 2], variant: u32) -> Trail {
    let mirrored: Vec<i64> = if variant & 1 == 0 { trail.xs.clone() } else { trail.xs.iter().map(|x| page[0] - x).collect() };
    let flipped: Vec<i64> = if variant & 2 == 0 { trail.ys.clone() } else { trail.ys.iter().map(|y| page[1] - y).collect() };
    let (mut xs, mut ys) = if variant & 4 == 0 { (mirrored, flipped) } else { (flipped, mirrored) };
    if variant & 8 != 0 {
        xs.reverse();
        ys.reverse();
    }
    Trail { xs, ys }
}

/// 🔲️ A body (left, top, width, height in quanta) in the same variant.
fn box_variant(body: [i64; 4], page: [i64; 2], variant: u32) -> [i64; 4] {
    let x = if variant & 1 == 0 { body[0] } else { page[0] - body[0] - body[2] };
    let y = if variant & 2 == 0 { body[1] } else { page[1] - body[1] - body[3] };
    if variant & 4 == 0 {
        [x, y, body[2], body[3]]
    } else {
        [y, x, body[3], body[2]]
    }
}

/// 🪶️ Every cue of the hover gestures while the pointer follows a trail past one body, as `[tick, cue]` rows.
fn hovered(trail: &Trail, quantum: f64, body: [i64; 4], guards: Guards) -> Vec<Value> {
    let rect = Rect { x: body[0] as f64 / quantum, y: body[1] as f64 / quantum, width: body[2] as f64 / quantum, height: body[3] as f64 / quantum };
    let mut cues = Vec::new();
    let mut hover = no_hover(0);
    for (tick, (x, y)) in trail.xs.iter().zip(&trail.ys).enumerate() {
        let step = hover_step(hover, Point { x: *x as f64 / quantum, y: *y as f64 / quantum }, rect, tick as Ticks, guards);
        hover = step.state;
        if let Some(cue) = step.cue {
            cues.push(json!([tick, cue]));
        }
    }
    cues
}

/// 🫨️ Every cue of the shake while the grip follows a trail, as `[tick, cue]` rows.
fn held(trail: &Trail, quantum: f64, height: f64, guards: Guards) -> Vec<Value> {
    let mut cues = Vec::new();
    let mut shaking = no_shaking(0);
    for (tick, (x, y)) in trail.xs.iter().zip(&trail.ys).enumerate() {
        let step = shake_step(shaking, Point { x: *x as f64 / quantum, y: *y as f64 / quantum }, height / quantum, tick as Ticks, guards);
        shaking = step.state;
        if let Some(cue) = step.cue {
            cues.push(json!([tick, cue]));
        }
    }
    cues
}

/// 🖲️ Every signal of a committed press: per tick first the passing of the tick, then the events that arrive at it, in order.
fn pressed(vector: &Value, quantum: f64) -> Vec<Value> {
    let mut signals = Vec::new();
    let mut press = IDLE;
    for tick in 1..=number(&vector["ticks"]) as Ticks {
        let passed = press_step(press, PressInput::Ticked, tick, UNGUARDED);
        press = passed.state;
        if let Some(signal) = passed.signal {
            signals.push(json!([tick, signal]));
        }
        for event in entries(&vector["events"]) {
            if number(&event["at"]) as Ticks != tick {
                continue;
            }
            let x = event.get("x").map_or(0.0, number) / quantum;
            let y = event.get("y").map_or(0.0, number) / quantum;
            let input = match event["kind"].as_str() {
                Some("pressed") => pressed_at(x, y, event.get("pointer").map_or(MOUSE, |pointer| serde_json::from_value(pointer.clone()).expect("a pointer"))),
                Some("cancelled") => PressInput::Cancelled,
                Some("released") => released_at(x, y),
                _ => dragged_to(x, y),
            };
            let guards = entries(&event["guards"]).iter().filter_map(Value::as_str).fold(UNGUARDED, |guards, name| Guards {
                control: guards.control || name == "control",
                scrolled: guards.scrolled || name == "scrolled",
                quiet: guards.quiet || name == "quiet",
                still: guards.still || name == "still",
            });
            let step = press_step(press, input, tick, guards);
            press = step.state;
            if let Some(signal) = step.signal {
                signals.push(json!([tick, signal]));
            }
        }
    }
    signals
}

/// 💓️ The answer to every caress of a history: the heat right after it, the tier, the run, the tricks so far and the tick until which the pet has had enough.
fn warmed(caresses: &Value) -> Vec<Value> {
    let mut warmth = COLD;
    let mut answers = Vec::new();
    for caress in entries(caresses) {
        warmth = warmth_after(warmth, number(&caress[0]) as Ticks, serde_json::from_value(caress[1].clone()).expect("a caress"));
        answers.push(json!([warmth.heat, warmth.tier, warmth.run, warmth.tricks, warmth.until]));
    }
    answers
}

/// 🧳️ The committed document's numbers the replays need: quantum, hold and page.
fn frame(document: &Value) -> (f64, i64, [i64; 2]) {
    let page = integers(&document["page"]);
    (number(&document["quantum"]), number(&document["hold"]) as i64, [page[0], page[1]])
}

/// 📦️ A committed body.
fn boxed(value: &Value) -> [i64; 4] {
    let edges = integers(value);
    [edges[0], edges[1], edges[2], edges[3]]
}

/// 🔭️ Every cue ordinary travel sets off — each chosen stretch, in each of the given variants, past every body — and how many pointer ticks were watched.
fn travelled(document: &Value, sample: bool, variants: u32) -> (usize, Vec<Value>) {
    let (quantum, hold, page) = frame(document);
    let bodies: Vec<[i64; 4]> = entries(&document["bodies"]).iter().map(boxed).collect();
    let mut cues = Vec::new();
    let mut pointer_ticks = 0;
    for travel in entries(&document["travels"]) {
        if sample && travel["sample"] != json!(true) {
            continue;
        }
        let trail = trail_of(&integers(&travel["path"]), hold);
        for variant in 0..variants {
            let turned = variant_of(&trail, page, variant);
            pointer_ticks += turned.xs.len();
            for (index, body) in bodies.iter().enumerate() {
                for cue in hovered(&turned, quantum, box_variant(*body, page, variant), UNGUARDED) {
                    cues.push(json!([travel["id"], variant, index, cue[0], cue[1]]));
                }
            }
        }
    }
    (pointer_ticks, cues)
}

/// 🚦️ The guards a committed trace names, or none.
fn guard_of(vector: &Value) -> Guards {
    vector.get("guard").and_then(Value::as_str).map_or(UNGUARDED, up)
}
//#endregion 🔖️Replay of the committed vectors

//#region 🔖️Laws on drawn shapes
/// 🌀️ Any circle is cued at the same ticks, the other way round, in its mirror.
fn mirrored_circles_agree(shapes: usize) {
    let mut draw = Stream(11);
    let mut cued = 0;
    for _ in 0..shapes {
        let radius = 50.0 + draw.next() * 80.0;
        let centre = Point { x: CENTRE.x + (draw.next() - 0.5) * 12.0, y: CENTRE.y + (draw.next() - 0.5) * 12.0 };
        let (turns, way, start) = (0.5 + draw.next() * 2.5, if draw.next() < 0.5 { 1.0 } else { -1.0 }, draw.next());
        let points = circling(centre, |tick| radius * (1.0 + 0.1 * sin_turns(tick as f64 / 9.0 / std::f64::consts::TAU)), turns, way, start, 64 * 3);
        let cues = circle_cues(&points, BODY, |_| UNGUARDED);
        let mirror = circle_cues(&mirrored(&points, CENTRE.x), BODY, |_| UNGUARDED);
        assert_eq!(ticks_of(&mirror), ticks_of(&cues));
        assert_eq!(names_of(&mirror), cues.iter().map(|(_, cue)| if cue == "circle" { "countercircle".to_string() } else { "circle".to_string() }).collect::<Vec<_>>());
        cued += usize::from(!cues.is_empty());
    }
    assert_eq!(cued, shapes);
}

/// 🛤️ No straight pass past a pet, however near, fast or slow, is a gesture.
fn straight_passes_are_silent(passes: usize) {
    let mut draw = Stream(13);
    for _ in 0..passes {
        let angle = draw.next();
        let aside = (draw.next() - 0.5) * 300.0;
        let speed = 30.0 + draw.next() * 3000.0;
        let points: Vec<Point> = (0..((1200.0 / speed) * 64.0).ceil() as usize)
            .map(|tick| {
                let along = -600.0 + (speed * tick as f64) / 64.0;
                Point { x: CENTRE.x + along * cos_turns(angle) - aside * sin_turns(angle), y: CENTRE.y + along * sin_turns(angle) + aside * cos_turns(angle) }
            })
            .collect();
        assert_eq!(hover_cues(&points, BODY, |_| UNGUARDED), Vec::new());
    }
}

/// 🐾️ Any petting is cued, at the same ticks in its mirror.
fn mirrored_pettings_agree(shapes: usize) {
    let mut draw = Stream(17);
    for _ in 0..shapes {
        let centre = Point { x: CENTRE.x + (draw.next() - 0.5) * 8.0, y: CENTRE.y + (draw.next() - 0.5) * 30.0 };
        let (swing, hertz) = (12.0 + draw.next() * 12.0, 1.5 + draw.next() * 2.5);
        let (rise, phase) = ((draw.next() - 0.5) * 6.0, draw.next());
        let points = petting(centre, swing, hertz, 64 * 3, rise, phase);
        let cues = hover_cues(&points, BODY, |_| UNGUARDED);
        assert!(!cues.is_empty());
        assert_eq!(hover_cues(&mirrored(&points, CENTRE.x), BODY, |_| UNGUARDED), cues);
    }
}

/// 🫙️ Any shake is cued, in any direction, at the same ticks in its mirror.
fn mirrored_shakes_agree(shapes: usize) {
    let mut draw = Stream(19);
    for _ in 0..shapes {
        let angle = draw.next();
        let swing = 25.0 + draw.next() * 40.0;
        let hertz = 2.5 + draw.next() * 2.0;
        let points: Vec<Point> = (0..64 * 2)
            .map(|tick| {
                let wave = swing * sin_turns((hertz * tick as f64) / 64.0);
                Point { x: CENTRE.x + wave * cos_turns(angle), y: CENTRE.y + wave * sin_turns(angle) }
            })
            .collect();
        let cues = shake_cues(&points, 48.0, UNGUARDED);
        assert!(!cues.is_empty());
        assert_eq!(shake_cues(&mirrored(&points, CENTRE.x), 48.0, UNGUARDED), cues);
    }
}

/// 🪣️ The heat matches a running leaky bucket over random histories.
fn heat_is_a_leaky_bucket(histories: usize) {
    let mut draw = Stream(7);
    for _ in 0..histories {
        let mut warmth = COLD;
        let (mut bucket, mut last, mut tick): (f64, Ticks, Ticks) = (0.0, 0, 0);
        for _ in 0..40 {
            tick += (draw.next() * 200.0).floor() as Ticks;
            let caress = if draw.next() < 0.3 { Caress::Hold } else { Caress::Click };
            let before = warmth;
            warmth = warmth_after(warmth, tick, caress);
            if tick < before.until {
                assert_eq!(warmth.tier, Tier::Enough);
                continue;
            }
            bucket = (bucket - ((tick - last) as f64 * 0.5) / 64.0).max(0.0) + if caress == Caress::Hold { HEAT_HOLD } else { 1.0 };
            last = tick;
            if bucket >= HEAT_ENOUGH {
                assert_eq!(warmth.tier, Tier::Enough);
                bucket = HEAT_FORGIVEN;
                last = tick + ENOUGH_TICKS;
            } else {
                assert_eq!(warmth.heat, bucket);
            }
        }
    }
}

/// 🚶️ Ordinary travel past forty pets sets nothing off, in the given variants.
fn travel_is_silent(sample: bool, variants: u32) {
    let (pointer_ticks, cues) = travelled(&fixture("gesture-recognition"), sample, variants);
    assert_eq!(cues, Vec::<Value>::new());
    assert!(pointer_ticks > 64 * 60 * 3);
}
//#endregion 🔖️Laws on drawn shapes

#[test]
fn a_press_alone_only_arms_however_long_the_stage_waits_for_less_than_a_hold() {
    let armed = press_step(IDLE, pressed_at(10.0, 20.0, MOUSE), 5, UNGUARDED);
    assert_eq!(armed, PressStep { state: Press { phase: PressPhase::Armed, x: 10.0, y: 20.0, since: 5, slop: SLOP_FINE }, signal: None });
    assert_eq!(signals_of(HOLD_TICKS, &[(1, pressed_at(0.0, 0.0, MOUSE))], &[]), Vec::new());
    assert_eq!(press_due(armed.state), Some(5 + HOLD_TICKS));
    assert_eq!(press_due(IDLE), None);
}

#[test]
fn a_press_is_a_click_when_released_before_the_hold_within_the_slop_and_a_hold_from_the_tick_of_the_hold_on() {
    let press = pressed_at(100.0, 100.0, MOUSE);
    assert_eq!(signals_of(40, &[(2, press), (1 + HOLD_TICKS, released_at(103.0, 104.0))], &[]), sighted(&[(1 + HOLD_TICKS, "click")]));
    assert_eq!(signals_of(40, &[(2, press), (2 + HOLD_TICKS, released_at(100.0, 100.0))], &[]), sighted(&[(2 + HOLD_TICKS, "hold"), (2 + HOLD_TICKS, "unhold")]));
    assert_eq!(signals_of(90, &[(2, press), (80, released_at(100.0, 100.0))], &[]), sighted(&[(2 + HOLD_TICKS, "hold"), (80, "unhold")]));
}

#[test]
fn a_press_is_a_pick_up_at_the_slop_and_beyond_for_a_mouse_and_a_pen_at_six_pixels_and_a_finger_at_ten() {
    assert_eq!(slop_of(Pointer::Mouse), SLOP_FINE);
    assert_eq!(slop_of(Pointer::Pen), SLOP_FINE);
    assert_eq!(slop_of(Pointer::Touch), SLOP_COARSE);
    for pointer in [Pointer::Mouse, Pointer::Pen, Pointer::Touch] {
        let slop = slop_of(pointer);
        let press = pressed_at(0.0, 0.0, pointer);
        assert_eq!(signals_of(20, &[(1, press), (3, dragged_to(slop - 0.25, 0.0)), (9, released_at(0.0, 0.0))], &[]), sighted(&[(9, "click")]));
        assert_eq!(signals_of(20, &[(1, press), (3, dragged_to(0.0, -slop)), (4, dragged_to(50.0, 50.0)), (9, released_at(60.0, 60.0))], &[]), sighted(&[(3, "lift"), (9, "drop")]));
    }
}

#[test]
fn the_grip_stays_where_it_was_pressed_while_the_pet_is_lifted_and_a_hold_that_leaves_the_slop_becomes_a_pick_up() {
    let lifted = press_step(press_step(IDLE, pressed_at(7.0, 9.0, Pointer::Pen), 3, UNGUARDED).state, dragged_to(40.0, 9.0), 4, UNGUARDED);
    assert_eq!(lifted, PressStep { state: Press { phase: PressPhase::Lifted, x: 7.0, y: 9.0, since: 3, slop: SLOP_FINE }, signal: Some(PressSignal::Lift) });
    assert_eq!(press_step(lifted.state, dragged_to(400.0, 900.0), 5, UNGUARDED), PressStep { state: lifted.state, signal: None });
    assert_eq!(signals_of(80, &[(1, pressed_at(0.0, 0.0, MOUSE)), (50, dragged_to(30.0, 0.0)), (70, released_at(30.0, 0.0))], &[]), sighted(&[(1 + HOLD_TICKS, "hold"), (50, "lift"), (70, "drop")]));
}

#[test]
fn a_press_is_aborted_by_a_cancellation_in_every_open_phase_and_a_cancellation_without_a_press_does_nothing() {
    let press = pressed_at(0.0, 0.0, MOUSE);
    assert_eq!(signals_of(20, &[(1, press), (5, PressInput::Cancelled), (8, released_at(0.0, 0.0))], &[]), sighted(&[(5, "abort")]));
    assert_eq!(signals_of(60, &[(1, press), (50, PressInput::Cancelled)], &[]), sighted(&[(1 + HOLD_TICKS, "hold"), (50, "abort")]));
    assert_eq!(signals_of(20, &[(1, press), (2, dragged_to(20.0, 0.0)), (9, PressInput::Cancelled)], &[]), sighted(&[(2, "lift"), (9, "abort")]));
    assert_eq!(signals_of(10, &[(3, PressInput::Cancelled), (4, released_at(0.0, 0.0)), (5, dragged_to(9.0, 9.0))], &[]), Vec::new());
}

#[test]
fn a_press_is_not_taken_over_a_control_is_aborted_by_a_still_stage_and_a_scroll_before_a_pick_up_and_works_in_a_quiet_stage() {
    let press = pressed_at(0.0, 0.0, MOUSE);
    let release = released_at(0.0, 0.0);
    let nudge = dragged_to(1.0, 0.0);
    assert_eq!(signals_of(20, &[(1, press), (5, release)], &[up("control")]), Vec::new());
    assert_eq!(signals_of(20, &[(1, press), (5, release)], &[up("still")]), Vec::new());
    assert_eq!(signals_of(20, &[(1, press), (3, nudge), (5, release)], &[UNGUARDED, UNGUARDED, up("still")]), sighted(&[(3, "abort")]));
    assert_eq!(signals_of(20, &[(1, press), (3, nudge), (5, release)], &[UNGUARDED, UNGUARDED, up("scrolled")]), sighted(&[(3, "abort")]));
    assert_eq!(signals_of(20, &[(1, press), (2, dragged_to(30.0, 0.0)), (4, dragged_to(30.0, 40.0)), (6, release)], &[UNGUARDED, UNGUARDED, UNGUARDED, up("scrolled"), UNGUARDED, up("scrolled")]), sighted(&[(2, "lift"), (6, "drop")]));
    assert_eq!(signals_of(20, &[(1, press), (5, release)], &[up("quiet"), UNGUARDED, UNGUARDED, UNGUARDED, up("quiet")]), sighted(&[(5, "click")]));
}

#[test]
fn an_open_press_is_aborted_when_the_next_one_comes_which_is_armed() {
    let inputs = [(1, pressed_at(0.0, 0.0, MOUSE)), (6, pressed_at(50.0, 0.0, Pointer::Touch)), (9, dragged_to(59.0, 0.0)), (10, released_at(59.0, 0.0))];
    assert_eq!(signals_of(20, &inputs, &[]), sighted(&[(6, "abort"), (10, "click")]));
}

#[test]
fn the_heat_leaks_half_a_unit_per_second_down_to_zero_and_never_grows_before_the_tick_it_was_measured_at() {
    assert_eq!(heat_at(3.0, 100, 100), 3.0);
    assert_eq!(heat_at(3.0, 100, 164), 2.5);
    assert_eq!(heat_at(3.0, 100, 100 + 64 * 6), 0.0);
    assert_eq!(heat_at(2.0, 600, 100), 2.0);
    assert_eq!(heat_after(0.0, 0, 50, Caress::Click), 1.0);
    assert_eq!(heat_after(1.0, 50, 50, Caress::Hold), 1.0 + HEAT_HOLD);
}

#[test]
fn the_heat_is_answered_by_tiers_whose_edges_belong_to_the_lower_tier_except_enough() {
    assert_eq!(TIERS, [Tier::Hello, Tier::Trick, Tier::Purr, Tier::Enough]);
    let tiers: Vec<Tier> = [0.0, 1.0, 1.0078125, 3.0, 3.0078125, 6.9921875, 7.0, 40.0].into_iter().map(tier_of).collect();
    assert_eq!(tiers, [Tier::Hello, Tier::Hello, Tier::Trick, Tier::Trick, Tier::Purr, Tier::Purr, Tier::Enough, Tier::Enough]);
}

#[test]
fn a_pet_clicked_twice_a_second_greets_plays_tricks_purrs_and_has_enough_then_ignores_clicks_and_forgives() {
    let mut warmth = COLD;
    let mut answers = Vec::new();
    for click in 0..12 {
        warmth = warmth_after(warmth, 32 * click, Caress::Click);
        answers.push(format!("{}{}", named(warmth.tier), warmth.run));
    }
    assert_eq!(answers, ["hello1", "trick1", "trick2", "purr1", "purr2", "purr3", "purr4", "purr5", "enough1", "enough2", "enough3", "enough4"]);
    assert_eq!(warmth.tricks, 2);
    assert_eq!(warmth.until, 32 * 8 + ENOUGH_TICKS);
    let forgiven = warmth_after(warmth, warmth.until, Caress::Click);
    assert_eq!(forgiven, Warmth { heat: HEAT_FORGIVEN + 1.0, since: warmth.until, until: warmth.until, tier: Tier::Trick, run: 1, tricks: 3 });
}

#[test]
fn a_held_pet_purrs_until_the_heat_has_had_enough_about_five_seconds_from_cold() {
    let mut warmth = COLD;
    let mut tick = 0;
    while warmth.tier != Tier::Enough {
        warmth = warmth_after(warmth, tick, Caress::Hold);
        if warmth.tier != Tier::Enough {
            assert_eq!(warmth.tier, Tier::Purr);
        }
        tick += 1;
    }
    assert!(tick as f64 / 64.0 > 4.5);
    assert!((tick as f64 / 64.0) < 5.0);
    assert_eq!((warmth.heat, warmth.run, warmth.tricks), (HEAT_FORGIVEN, 1, 0));
}

#[test]
fn every_committed_history_is_answered_as_numpys_closed_form_does() {
    for vector in entries(&fixture("gesture-recognition")["warmths"]) {
        assert_same(&vector["id"].to_string(), &json!(warmed(&vector["caresses"])), &vector["expected"]);
    }
}

#[test]
fn the_heat_matches_a_running_leaky_bucket_over_random_histories() {
    heat_is_a_leaky_bucket(12);
}

#[test]
fn a_circle_clockwise_on_screen_is_cued_as_circle_and_the_other_way_as_countercircle() {
    let clockwise = circling(CENTRE, |_| 80.0, 1.0, 1.0, 0.0, 160);
    assert!(clockwise[16].y > CENTRE.y + 70.0);
    assert_eq!(names_of(&hover_cues(&clockwise, BODY, |_| UNGUARDED)), ["circle"]);
    assert_eq!(names_of(&hover_cues(&circling(CENTRE, |_| 80.0, 1.0, -1.0, 0.0, 160), BODY, |_| UNGUARDED)), ["countercircle"]);
}

#[test]
fn a_circle_waits_until_the_pointer_has_crossed_again_the_axis_where_the_lap_began_and_then_rests() {
    let cues = hover_cues(&circling(CENTRE, |_| 80.0, 1.0, 1.0, 0.1, 64 * 6), BODY, |_| UNGUARDED);
    assert!(cues[0].0 as f64 > 64.0 * 1.15);
    assert!((cues[0].0 as f64) < 64.0 * 1.2);
    for pair in cues.windows(2) {
        assert!(pair[1].0 - pair[0].0 >= CIRCLE_REST);
    }
    assert!(cues.len() >= 2);
}

#[test]
fn any_circle_is_cued_at_the_same_tick_and_the_other_way_round_in_its_mirror() {
    mirrored_circles_agree(12);
}

#[test]
fn nothing_is_heard_inside_the_band_or_beyond_it_too_fast_too_slow_or_going_back_and_forth() {
    let reach = BODY.width.max(BODY.height);
    let silent = |points: Vec<Point>| assert_eq!(circle_cues(&points, BODY, |_| UNGUARDED), Vec::new());
    silent(circling(CENTRE, |_| reach / 2.0 + CIRCLE_MARGIN - 2.0, 1.0, 1.0, 0.0, 300));
    silent(circling(CENTRE, |_| reach * CIRCLE_REACH + 2.0, 1.0, 1.0, 0.0, 300));
    silent(circling(CENTRE, |_| 80.0, 64.0 / (4.0 * CIRCLE_FAST as f64) + 0.5, 1.0, 0.0, 300));
    silent(circling(CENTRE, |_| 80.0, 0.3, 1.0, 0.05, 64 * 8));
    silent((0..64 * 10).map(f64::from).map(|tick| 0.4 * sin_turns(tick / 70.0)).map(|angle| Point { x: CENTRE.x + 80.0 * cos_turns(angle), y: CENTRE.y + 80.0 * sin_turns(angle) }).collect());
}

#[test]
fn nothing_is_heard_of_a_lap_that_is_not_round_or_does_not_close() {
    assert_eq!(circle_cues(&circling(CENTRE, |tick| if tick % 32 < 16 { 36.0 } else { 130.0 }, 1.0, 1.0, 0.0, 64 * 4), BODY, |_| UNGUARDED), Vec::new());
    assert_eq!(circle_cues(&circling(CENTRE, spiral(1.006_355_503_360_101), 1.0, 1.0, 0.0, 160), BODY, |_| UNGUARDED), Vec::new());
    assert!(!circle_cues(&circling(CENTRE, spiral(1.004_107_855_837_071), 1.0, 1.0, 0.0, 160), BODY, |_| UNGUARDED).is_empty());
}

#[test]
fn nothing_is_heard_over_a_control_in_a_quiet_or_still_stage_and_for_a_while_after_a_scroll() {
    let points = circling(CENTRE, |_| 80.0, 1.0, 1.0, 0.0, 64 * 3);
    for guard in ["control", "quiet", "still"] {
        assert_eq!(circle_cues(&points, BODY, |_| up(guard)), Vec::new());
    }
    let plain = circle_cues(&points, BODY, |_| UNGUARDED)[0].0;
    let scrolled = circle_cues(&points, BODY, |tick| if tick == plain - 2 { up("scrolled") } else { UNGUARDED });
    assert!(scrolled[0].0 > plain - 2 + SCROLL_TICKS);
    assert_eq!(circle_step(no_circling(0), CENTRE, BODY, 10, up("scrolled")).state.rest, 10 + SCROLL_TICKS);
}

#[test]
fn a_circle_is_heard_on_a_device_that_reports_only_every_second_or_fourth_tick() {
    for every in [2, 4] {
        let all = circling(CENTRE, |_| 80.0, 1.0, 1.0, 0.0, 64 * 3);
        let points: Vec<Point> = (0..all.len()).map(|tick| all[tick - tick % every]).collect();
        assert_eq!(names_of(&circle_cues(&points, BODY, |_| UNGUARDED)), ["circle"]);
    }
}

#[test]
fn a_straight_pass_past_a_pet_is_never_heard_however_near_fast_or_slow() {
    straight_passes_are_silent(48);
}

#[test]
fn a_petting_is_cued_once_three_strokes_that_begin_at_a_reversal_are_done_and_again_with_every_three_more() {
    let cues = hover_cues(&petting(CENTRE, 14.0, 2.0, 64 * 4, 0.0, 0.0), BODY, |_| UNGUARDED);
    assert_eq!(cues.len(), 5);
    assert!(cues.iter().all(|(_, cue)| cue == "stroke"));
    assert!(cues[0].0 > 48);
    assert!(cues[0].0 < 64);
    for pair in cues.windows(2) {
        assert_eq!(pair[1].0 - pair[0].0, 48);
    }
}

#[test]
fn no_stroke_counts_that_is_too_short_wanders_lies_beside_the_body_or_is_too_slow_or_too_fast() {
    let silent = |points: Vec<Point>, body: Rect| assert_eq!(hover_cues(&points, body, |_| UNGUARDED), Vec::new());
    silent(petting(CENTRE, 8.0, 2.0, 64 * 4, 0.0, 0.0), BODY);
    silent(petting(CENTRE, 14.0, 2.0, 64 * 4, 16.0, 0.0), BODY);
    silent(petting(Point { x: CENTRE.x + 21.0, y: CENTRE.y }, 9.0, 2.0, 64 * 4, 0.0, 0.0), BODY);
    silent(petting(CENTRE, 14.0, 0.25, 64 * 8, 0.0, 0.0), BODY);
    let wide = Rect { x: 372.0, y: 272.0, width: 56.0, height: 56.0 };
    assert!(!hover_cues(&petting(CENTRE, 26.0, 2.0, 64 * 4, 0.0, 0.0), wide, |_| UNGUARDED).is_empty());
    silent(petting(CENTRE, 26.0, 12.0, 64 * 4, 0.0, 0.0), wide);
}

#[test]
fn stroking_forgets_everything_when_the_pointer_leaves_the_zone_or_rests_and_hears_nothing_under_a_guard() {
    let strokes = petting(CENTRE, 14.0, 2.0, 64 * 4, 0.0, 0.0);
    let left: Vec<Point> = strokes.iter().enumerate().map(|(tick, point)| if tick % 60 == 59 { Point { x: CENTRE.x, y: CENTRE.y + 60.0 } } else { *point }).collect();
    assert_eq!(hover_cues(&left, BODY, |_| UNGUARDED), Vec::new());
    let rests: Vec<Point> = strokes.iter().enumerate().flat_map(|(tick, point)| std::iter::repeat_n(*point, if tick % 24 == 0 { STROKE_PAUSE as usize + 2 } else { 1 })).collect();
    assert_eq!(hover_cues(&rests, BODY, |_| UNGUARDED), Vec::new());
    for guard in ["control", "quiet", "still"] {
        assert_eq!(hover_cues(&strokes, BODY, |_| up(guard)), Vec::new());
    }
    assert_eq!(stroke_step(no_stroking(0), CENTRE, BODY, 4, up("scrolled")).state.rest, 4 + SCROLL_TICKS);
}

#[test]
fn any_petting_is_cued_at_the_same_ticks_in_its_mirror() {
    mirrored_pettings_agree(12);
}

#[test]
fn a_shake_is_cued_after_four_counted_reversals_within_a_second_and_then_rests() {
    let cues = shake_cues(&petting(CENTRE, 30.0, 3.0, 64 * 4, 0.0, 0.0), 48.0, UNGUARDED);
    assert_eq!(names_of(&cues), ["shake", "shake"]);
    assert!(cues[0].0 < 64);
    assert!(cues[1].0 - cues[0].0 >= SHAKE_REST);
}

#[test]
fn no_swing_is_heard_that_is_too_short_or_too_slow_no_sway_no_carry_and_nothing_on_a_still_stage() {
    assert_eq!(shake_cues(&petting(CENTRE, 12.0, 4.0, 64 * 3, 0.0, 0.0), 48.0, UNGUARDED), Vec::new());
    assert_eq!(shake_cues(&petting(CENTRE, 30.0, 1.2, 64 * 4, 0.0, 0.0), 48.0, UNGUARDED), Vec::new());
    let carry: Vec<Point> = (0..64 * 3).map(f64::from).map(|tick| Point { x: CENTRE.x + tick * 4.0, y: CENTRE.y - tick }).collect();
    assert_eq!(shake_cues(&carry, 48.0, UNGUARDED), Vec::new());
    let shake = petting(CENTRE, 30.0, 3.0, 64 * 4, 0.0, 0.0);
    assert_eq!(shake_cues(&shake, 48.0, up("still")), Vec::new());
    for guard in ["control", "quiet", "scrolled"] {
        assert_eq!(shake_cues(&shake, 48.0, up(guard)), shake_cues(&shake, 48.0, UNGUARDED));
    }
}

#[test]
fn any_shake_is_cued_at_the_same_ticks_in_any_direction_and_in_its_mirror() {
    mirrored_shakes_agree(12);
}

#[test]
fn a_circle_silences_petting_for_its_rest_and_a_gesture_under_way_is_told() {
    let mut hover = no_hover(0);
    let mut busy = false;
    let mut circled = -1;
    for (tick, point) in circling(CENTRE, |_| 80.0, 1.0, 1.0, 0.0, 160).into_iter().enumerate() {
        let step = hover_step(hover, point, BODY, tick as Ticks, UNGUARDED);
        hover = step.state;
        busy |= hover_busy(&hover);
        if step.cue.is_some() {
            circled = tick as Ticks;
        }
    }
    assert!(busy);
    assert!(circled > 0);
    assert_eq!(hover.stroking.rest, circled + CIRCLE_REST);
    assert!(!hover_busy(&no_hover(0)));
}

#[test]
fn the_steps_take_their_states_by_value_and_leave_the_callers_as_they_were() {
    let (hover, shaking, press, warmth) = (no_hover(0), no_shaking(0), IDLE, COLD);
    for (tick, point) in circling(CENTRE, |_| 80.0, 1.0, 1.0, 0.0, 40).into_iter().enumerate() {
        let tick = tick as Ticks;
        let _ = (hover_step(hover, point, BODY, tick, UNGUARDED), shake_step(shaking, point, 48.0, tick, UNGUARDED), press_step(press, dragged_to(point.x, point.y), tick, UNGUARDED), warmth_after(warmth, tick, Caress::Hold));
    }
    assert_eq!((hover, shaking, press, warmth), (no_hover(0), no_shaking(0), IDLE, COLD));
}

#[test]
fn the_committed_constants_are_the_ones_of_the_twin() {
    let constants = &fixture("gesture-recognition")["constants"];
    let twin = json!({
        "slopFine": SLOP_FINE, "slopCoarse": SLOP_COARSE, "holdTicks": HOLD_TICKS, "heatClick": HEAT_CLICK, "heatHold": HEAT_HOLD, "heatLeak": HEAT_LEAK, "heatHello": HEAT_HELLO, "heatTrick": HEAT_TRICK, "heatEnough": HEAT_ENOUGH, "heatForgiven": HEAT_FORGIVEN, "enoughTicks": ENOUGH_TICKS, "scrollTicks": SCROLL_TICKS,
        "circleMargin": CIRCLE_MARGIN, "circleReach": CIRCLE_REACH, "circleHysteresis": CIRCLE_HYSTERESIS, "circleQuarters": CIRCLE_QUARTERS, "circleFast": CIRCLE_FAST, "circleSlow": CIRCLE_SLOW, "circleRound": CIRCLE_ROUND, "circleClose": CIRCLE_CLOSE, "circleAgainst": CIRCLE_AGAINST, "circleOutTicks": CIRCLE_OUT_TICKS, "circleRest": CIRCLE_REST,
        "strokeMargin": STROKE_MARGIN, "strokeHysteresis": STROKE_HYSTERESIS, "strokeLength": STROKE_LENGTH, "strokeSlow": STROKE_SLOW, "strokeFast": STROKE_FAST, "strokeSlant": STROKE_SLANT, "strokeSegments": STROKE_SEGMENTS, "strokeWindow": STROKE_WINDOW, "strokePause": STROKE_PAUSE,
        "shakeAmplitude": SHAKE_AMPLITUDE, "shakeHysteresis": SHAKE_HYSTERESIS, "shakeSpeed": SHAKE_SPEED, "shakeReversals": SHAKE_REVERSALS, "shakeWindow": SHAKE_WINDOW, "shakePause": SHAKE_PAUSE, "shakeRest": SHAKE_REST,
    });
    let committed = constants.as_object().expect("the committed constants");
    assert_eq!(committed.len(), twin.as_object().map_or(0, serde_json::Map::len));
    for (name, value) in committed {
        assert_eq!(twin[name].as_f64(), value.as_f64(), "{name}");
    }
}

#[test]
fn every_committed_press_circle_petting_and_held_path_is_answered_as_committed() {
    let document = fixture("gesture-recognition");
    let (quantum, hold, _) = frame(&document);
    for vector in entries(&document["presses"]) {
        assert_same(&vector["id"].to_string(), &json!(pressed(vector, quantum)), &vector["expected"]);
    }
    for vector in entries(&document["circles"]).iter().chain(entries(&document["strokes"])) {
        assert_same(&vector["id"].to_string(), &json!(hovered(&trail_of(&integers(&vector["path"]), hold), quantum, boxed(&vector["body"]), guard_of(vector))), &vector["expected"]);
    }
    for vector in entries(&document["shakes"]) {
        assert_same(&vector["id"].to_string(), &json!(held(&trail_of(&integers(&vector["path"]), hold), quantum, number(&vector["height"]), guard_of(vector))), &vector["expected"]);
    }
}

#[test]
fn every_clean_gesture_is_recognised_and_the_deliberate_ones_at_their_floors() {
    let document = fixture("gesture-recognition");
    for kind in ["circles", "strokes", "shakes"] {
        let deliberate: Vec<&Value> = entries(&document[kind]).iter().filter(|vector| vector.get("guard").is_none() && vector["label"] != "carry").collect();
        assert!(deliberate.iter().filter(|vector| vector["robust"] == true).all(|vector| !entries(&vector["expected"]).is_empty()), "{kind}");
        let recognised = deliberate.iter().filter(|vector| !entries(&vector["expected"]).is_empty()).count();
        assert!(recognised as f64 / deliberate.len() as f64 >= number(&document["floors"][kind]), "{kind}");
    }
    assert!(entries(&document["shakes"]).iter().filter(|vector| vector["label"] == "carry").all(|vector| entries(&vector["expected"]).is_empty()));
}

#[test]
fn ordinary_travel_past_forty_pets_sets_nothing_off() {
    travel_is_silent(true, 1);
}

#[test]
fn a_trail_is_mirrored_transposed_and_reversed_exactly_twice_the_same_variant_giving_it_back() {
    let document = fixture("gesture-recognition");
    let (_, hold, page) = frame(&document);
    let trail = trail_of(&integers(&document["travels"][0]["path"]), hold);
    let body = boxed(&document["bodies"][0]);
    for variant in [1, 2, 3, 8, 9] {
        assert_eq!(variant_of(&variant_of(&trail, page, variant), page, variant), trail);
        assert_eq!(box_variant(box_variant(body, page, variant), page, variant), body);
    }
}

#[test]
fn the_twin_uses_nothing_but_exact_operations() {
    for banned in [
        ".sin(",
        ".cos(",
        ".tan(",
        ".atan2(",
        ".exp(",
        ".ln(",
        ".log",
        ".powf(",
        ".powi(",
        ".mul_add(",
        ".hypot(",
        ".sqrt(",
        ".max(",
        ".min(",
        ".clamp(",
        "f64::max",
        "f64::min",
        "static ",
        "thread_local!",
        "HashMap",
        "BTreeMap",
        "std::time",
        "println!",
        "eprintln!",
        "dbg!",
    ] {
        assert!(!SOURCE.contains(banned), "{banned}");
    }
}

mod quick {
    use super::*;

    #[test]
    fn drawn_shapes_obey_their_laws() {
        heat_is_a_leaky_bucket(120);
        mirrored_circles_agree(120);
        straight_passes_are_silent(480);
        mirrored_pettings_agree(120);
        mirrored_shakes_agree(120);
    }

    #[test]
    fn all_ordinary_travel_in_four_variants_sets_nothing_off() {
        travel_is_silent(false, 4);
    }
}

mod exhaustive {
    use super::*;

    #[test]
    fn drawn_shapes_obey_their_laws() {
        heat_is_a_leaky_bucket(1200);
        mirrored_circles_agree(1200);
        straight_passes_are_silent(4800);
        mirrored_pettings_agree(1200);
        mirrored_shakes_agree(1200);
    }

    #[test]
    fn all_ordinary_travel_in_all_sixteen_variants_sets_nothing_off() {
        travel_is_silent(false, 16);
    }
}
