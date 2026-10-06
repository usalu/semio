//! 👆️ Subject adapter of the gesture-recognition case: the press machine, the heat and the three path gestures of the `pets` crate replay every committed trace tick by tick and hold themselves to the committed answers, projected exactly like the TypeScript adapter projects them.
//!
//! A path is a flat list of integers in quanta (a quarter of a pixel): the first point, then per tick either the step
//! `dx, dy` to the next point or one number `hold + n`, which repeats the latest point for `n` ticks. The sample at
//! index `k` is the pointer of tick `k`. A variant of a trace is the same trace mirrored, transposed or played
//! backwards — exact on integers, so every implementation sees the same numbers. Like the TypeScript adapter, this
//! one refuses any answer that differs from the committed one and any cue on ordinary travel.
//!
//! @see ./🥒️.feature
//! @see ./🟦️.ts — the TypeScript adapter, scenario for scenario
//! @see ../../🔨️modules/👆️gesture/🦀️.rs

use semio_repo_test_host::Adapter;

#[cfg(feature = "sut")]
mod subject {
    use pets::serde_json::{self, json, Map, Value};
    use pets::{
        hover_step, no_hover, no_shaking, press_step, shake_step, warmth_after, Caress, Cue, Dragged, Guards, Point, Pointer, PressInput, Pressed, Rect, Released, Ticks, CIRCLE_AGAINST, CIRCLE_CLOSE, CIRCLE_FAST, CIRCLE_HYSTERESIS, CIRCLE_MARGIN,
        CIRCLE_OUT_TICKS, CIRCLE_QUARTERS, CIRCLE_REACH, CIRCLE_REST, CIRCLE_ROUND, CIRCLE_SLOW, COLD, ENOUGH_TICKS, HEAT_CLICK, HEAT_ENOUGH, HEAT_FORGIVEN, HEAT_HELLO, HEAT_HOLD, HEAT_LEAK, HEAT_TRICK, HOLD_TICKS, IDLE, SCROLL_TICKS,
        SHAKE_AMPLITUDE, SHAKE_HYSTERESIS, SHAKE_PAUSE, SHAKE_REST, SHAKE_REVERSALS, SHAKE_SPEED, SHAKE_WINDOW, SLOP_COARSE, SLOP_FINE, STROKE_FAST, STROKE_HYSTERESIS, STROKE_LENGTH, STROKE_MARGIN, STROKE_PAUSE, STROKE_SEGMENTS, STROKE_SLANT,
        STROKE_SLOW, STROKE_WINDOW, UNGUARDED,
    };
    use semio_repo_test_host::{parse_json, Context, Outcome};

    const VECTORS: &str = "shared://👆️gesture-recognition/🔣️.json";

    /// 🧫️ The committed vectors.
    fn vectors(ctx: &Context<'_>) -> Result<Value, String> {
        serde_json::from_slice(&ctx.input_bytes(VECTORS)?).map_err(|error| format!("{VECTORS}: {error}"))
    }

    /// 📚️ A list the document or one of its vectors carries.
    fn list<'a>(holder: &'a Value, name: &str) -> Result<&'a [Value], String> {
        holder.get(name).and_then(Value::as_array).map(Vec::as_slice).ok_or_else(|| format!("{VECTORS} carries no list {name}"))
    }

    /// 🔢️ A number member of a vector.
    fn number(vector: &Value, name: &str) -> Result<f64, String> {
        vector.get(name).and_then(Value::as_f64).ok_or_else(|| format!("vector {vector} carries no number {name}"))
    }

    /// 🔡️ A text member of a vector.
    fn text<'a>(vector: &'a Value, name: &str) -> Result<&'a str, String> {
        vector.get(name).and_then(Value::as_str).ok_or_else(|| format!("vector {vector} carries no text {name}"))
    }

    /// 🧮️ The whole numbers of a list.
    fn integers(value: &Value) -> Vec<i64> {
        value.as_array().into_iter().flatten().filter_map(Value::as_f64).map(|number| number as i64).collect()
    }

    /// 🧵️ A pointer path decoded into one point per tick, in quanta.
    struct Trail {
        xs: Vec<i64>,
        ys: Vec<i64>,
    }

    /// 🧶️ The points of a path, one per tick, in quanta.
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
                y += path.get(index + 1).copied().unwrap_or(0);
                trail.xs.push(x);
                trail.ys.push(y);
                index += 2;
            }
        }
        trail
    }

    /// 🪞️ A trail in one of its sixteen variants on a page of `page` quanta: bit 0 mirrors left and right, bit 1 top and bottom, bit 2 swaps the axes, bit 3 plays it backwards.
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
    fn box_variant(body: &[i64], page: [i64; 2], variant: u32) -> Result<[i64; 4], String> {
        let &[left, top, width, height] = body else {
            return Err(format!("{body:?} is no body of four numbers"));
        };
        let x = if variant & 1 == 0 { left } else { page[0] - left - width };
        let y = if variant & 2 == 0 { top } else { page[1] - top - height };
        Ok(if variant & 4 == 0 { [x, y, width, height] } else { [y, x, height, width] })
    }

    /// 🚧️ The guards with the named one up, or none.
    fn guarded(guard: Option<&str>) -> Guards {
        match guard {
            Some("control") => Guards { control: true, ..UNGUARDED },
            Some("scrolled") => Guards { scrolled: true, ..UNGUARDED },
            Some("quiet") => Guards { quiet: true, ..UNGUARDED },
            Some("still") => Guards { still: true, ..UNGUARDED },
            _ => UNGUARDED,
        }
    }

    /// 🪶️ Every cue of the hover gestures while the pointer follows a trail past one body.
    fn hovered(trail: &Trail, quantum: f64, body: [i64; 4], guards: Guards) -> Vec<(usize, Cue)> {
        let rect = Rect { x: body[0] as f64 / quantum, y: body[1] as f64 / quantum, width: body[2] as f64 / quantum, height: body[3] as f64 / quantum };
        let mut cues = Vec::new();
        let mut hover = no_hover(0);
        for (tick, (x, y)) in trail.xs.iter().zip(&trail.ys).enumerate() {
            let step = hover_step(hover, Point { x: *x as f64 / quantum, y: *y as f64 / quantum }, rect, tick as Ticks, guards);
            hover = step.state;
            if let Some(cue) = step.cue {
                cues.push((tick, cue));
            }
        }
        cues
    }

    /// 🥤️ Every cue of the shake while the grip follows a trail.
    fn held(trail: &Trail, quantum: f64, height: f64, guards: Guards) -> Vec<(usize, Cue)> {
        let mut cues = Vec::new();
        let mut shaking = no_shaking(0);
        for (tick, (x, y)) in trail.xs.iter().zip(&trail.ys).enumerate() {
            let step = shake_step(shaking, Point { x: *x as f64 / quantum, y: *y as f64 / quantum }, height / quantum, tick as Ticks, guards);
            shaking = step.state;
            if let Some(cue) = step.cue {
                cues.push((tick, cue));
            }
        }
        cues
    }

    /// 🔔️ Cues as `[tick, cue]` rows.
    fn rows(cues: &[(usize, Cue)]) -> Value {
        json!(cues.iter().map(|(tick, cue)| json!([tick, cue])).collect::<Vec<_>>())
    }

    /// 🕹️ Every signal of a press: per tick first the passing of the tick, then the events that arrive at it, in order.
    fn pressed(vector: &Value, quantum: f64) -> Result<Value, String> {
        let mut signals = Vec::new();
        let mut press = IDLE;
        for tick in 1..=number(vector, "ticks")? as Ticks {
            let passed = press_step(press, PressInput::Ticked, tick, UNGUARDED);
            press = passed.state;
            if let Some(signal) = passed.signal {
                signals.push(json!([tick, signal]));
            }
            for event in list(vector, "events")? {
                if number(event, "at")? as Ticks != tick {
                    continue;
                }
                let x = event.get("x").and_then(Value::as_f64).unwrap_or(0.0) / quantum;
                let y = event.get("y").and_then(Value::as_f64).unwrap_or(0.0) / quantum;
                let input = match text(event, "kind")? {
                    "pressed" => PressInput::Pressed(Pressed { x, y, pointer: event.get("pointer").map_or(Ok(Pointer::Mouse), |pointer| serde_json::from_value(pointer.clone()).map_err(|error| error.to_string()))? }),
                    "cancelled" => PressInput::Cancelled,
                    "released" => PressInput::Released(Released { x, y }),
                    _ => PressInput::Dragged(Dragged { x, y }),
                };
                let names: Vec<&str> = event.get("guards").and_then(Value::as_array).into_iter().flatten().filter_map(Value::as_str).collect();
                let guards = Guards { control: names.contains(&"control"), scrolled: names.contains(&"scrolled"), quiet: names.contains(&"quiet"), still: names.contains(&"still") };
                let step = press_step(press, input, tick, guards);
                press = step.state;
                if let Some(signal) = step.signal {
                    signals.push(json!([tick, signal]));
                }
            }
        }
        Ok(Value::Array(signals))
    }

    /// 💓️ The answer to every caress of a history: the heat right after it, the tier, the run, the tricks so far and the tick until which the pet has had enough.
    fn warmed(caresses: &[Value]) -> Result<Value, String> {
        let mut answers = Vec::new();
        let mut warmth = COLD;
        for caress in caresses {
            let tick = caress[0].as_f64().ok_or_else(|| format!("caress {caress} has no tick"))? as Ticks;
            let kind: Caress = serde_json::from_value(caress[1].clone()).map_err(|error| error.to_string())?;
            warmth = warmth_after(warmth, tick, kind);
            answers.push(json!([warmth.heat, warmth.tier, warmth.run, warmth.tricks, warmth.until]));
        }
        Ok(Value::Array(answers))
    }

    /// 🟰️ Whether two answers are the same: numbers as the same double, everything else alike.
    fn same(left: &Value, right: &Value) -> bool {
        match (left, right) {
            (Value::Number(one), Value::Number(other)) => one.as_f64() == other.as_f64(),
            (Value::Array(one), Value::Array(other)) => one.len() == other.len() && one.iter().zip(other).all(|(one, other)| same(one, other)),
            _ => left == right,
        }
    }

    /// 🧾️ Refuses an answer that is not the committed one.
    fn agreed(scenario: &str, vector: &Value, produced: Value) -> Result<Value, String> {
        if same(&produced, &vector["expected"]) {
            Ok(produced)
        } else {
            Err(format!("{scenario}/{}: the subject answers {produced}, the committed vector says {}", vector["id"], vector["expected"]))
        }
    }

    /// 📐️ The quantum, the hold token and the page of the committed document.
    fn frame(document: &Value) -> Result<(f64, i64, [i64; 2]), String> {
        let page = integers(&document["page"]);
        let &[width, height] = page.as_slice() else {
            return Err("the page is no pair of numbers".to_string());
        };
        Ok((number(document, "quantum")?, number(document, "hold")? as i64, [width, height]))
    }

    /// 🎐️ The cues of one hover vector in one of its mirrored variants.
    fn hover_cues(document: &Value, vector: &Value, variant: u32) -> Result<Vec<(usize, Cue)>, String> {
        let (quantum, hold, page) = frame(document)?;
        Ok(hovered(&variant_of(&trail_of(&integers(&vector["path"]), hold), page, variant), quantum, box_variant(&integers(&vector["body"]), page, variant)?, guarded(vector.get("guard").and_then(Value::as_str))))
    }

    /// 🎢️ The cues of one held vector in one of its mirrored variants.
    fn held_cues(document: &Value, vector: &Value, variant: u32) -> Result<Vec<(usize, Cue)>, String> {
        let (quantum, hold, page) = frame(document)?;
        Ok(held(&variant_of(&trail_of(&integers(&vector["path"]), hold), page, variant), quantum, number(vector, "height")?, guarded(vector.get("guard").and_then(Value::as_str))))
    }

    /// 📊️ How many traces of one kind there are in the four mirrored variants, on how many the gesture of their label was cued, and on how many something else was; a mirror turns a circle the other way round. Refused when anything else was cued, or when a mirror changes a tick.
    fn tallied(traces: &[Value], cues_of: impl Fn(&Value, u32) -> Result<Vec<(usize, Cue)>, String>) -> Result<Value, String> {
        let (mut count, mut detected, mut wrong) = (0, 0, 0);
        for vector in traces {
            let label = text(vector, "label")?;
            if vector.get("guard").is_some() || label == "carry" {
                continue;
            }
            let upright: Vec<usize> = cues_of(vector, 0)?.iter().map(|(tick, _)| *tick).collect();
            for variant in 0..4 {
                let turned = (label == "circle" || label == "countercircle") && (variant == 1 || variant == 2);
                let wanted = if turned {
                    if label == "circle" {
                        "countercircle"
                    } else {
                        "circle"
                    }
                } else {
                    label
                };
                let cues = cues_of(vector, variant)?;
                if cues.iter().map(|(tick, _)| *tick).collect::<Vec<_>>() != upright {
                    return Err(format!("detection: the mirror {variant} of {} is cued at other ticks than the trace itself", vector["id"]));
                }
                count += 1;
                detected += usize::from(cues.iter().any(|(_, cue)| json!(cue) == wanted));
                wrong += usize::from(cues.iter().any(|(_, cue)| json!(cue) != wanted));
            }
        }
        if wrong != 0 {
            return Err(format!("detection: {wrong} deliberate traces were answered with another gesture than theirs"));
        }
        Ok(json!({ "traces": count, "detected": detected, "wrong": wrong }))
    }

    /// 🔭️ Every cue ordinary travel sets off: each chosen stretch, in each of the committed variants (the sample in the first only), past every body; refused when anything is cued.
    fn travelled(document: &Value, sample: bool) -> Result<Value, String> {
        let (quantum, hold, page) = frame(document)?;
        let variants = if sample { 1 } else { number(document, "variants")? as u32 };
        let bodies: Vec<Vec<i64>> = list(document, "bodies")?.iter().map(integers).collect();
        let mut cues = Vec::new();
        let mut pointer_ticks = 0;
        for travel in list(document, "travels")? {
            if sample && travel["sample"] != true {
                continue;
            }
            let trail = trail_of(&integers(&travel["path"]), hold);
            for variant in 0..variants {
                let turned = variant_of(&trail, page, variant);
                pointer_ticks += turned.xs.len();
                for (index, body) in bodies.iter().enumerate() {
                    for (tick, cue) in hovered(&turned, quantum, box_variant(body, page, variant)?, UNGUARDED) {
                        cues.push(json!([travel["id"], variant, index, tick, cue]));
                    }
                }
            }
        }
        if !cues.is_empty() {
            return Err(format!("travel: ordinary travel set off {} cues, the first {}", cues.len(), json!(cues.iter().take(5).collect::<Vec<_>>())));
        }
        Ok(json!({ "pointerTicks": pointer_ticks, "petTicks": pointer_ticks * bodies.len(), "cues": cues }))
    }

    /// 🗝️ A group of vectors answered one by one, keyed by vector id.
    fn keyed(vectors: &[Value], answer: impl Fn(&Value) -> Result<Value, String>) -> Result<Value, String> {
        let mut projection = Map::new();
        for vector in vectors {
            projection.insert(text(vector, "id")?.to_string(), answer(vector)?);
        }
        Ok(Value::Object(projection))
    }

    /// 📤️ A projection as the host carries it.
    fn projected(projection: &Value) -> Result<Outcome, String> {
        Ok(Outcome::projection(parse_json(&projection.to_string())?))
    }

    /// 🎚️ The thresholds of the twin as the vectors name them.
    pub fn constants(_: &Context<'_>) -> Result<Outcome, String> {
        projected(&json!({
            "slopFine": SLOP_FINE, "slopCoarse": SLOP_COARSE, "holdTicks": HOLD_TICKS, "heatClick": HEAT_CLICK, "heatHold": HEAT_HOLD, "heatLeak": HEAT_LEAK, "heatHello": HEAT_HELLO, "heatTrick": HEAT_TRICK, "heatEnough": HEAT_ENOUGH, "heatForgiven": HEAT_FORGIVEN, "enoughTicks": ENOUGH_TICKS, "scrollTicks": SCROLL_TICKS,
            "circleMargin": CIRCLE_MARGIN, "circleReach": CIRCLE_REACH, "circleHysteresis": CIRCLE_HYSTERESIS, "circleQuarters": CIRCLE_QUARTERS, "circleFast": CIRCLE_FAST, "circleSlow": CIRCLE_SLOW, "circleRound": CIRCLE_ROUND, "circleClose": CIRCLE_CLOSE, "circleAgainst": CIRCLE_AGAINST, "circleOutTicks": CIRCLE_OUT_TICKS, "circleRest": CIRCLE_REST,
            "strokeMargin": STROKE_MARGIN, "strokeHysteresis": STROKE_HYSTERESIS, "strokeLength": STROKE_LENGTH, "strokeSlow": STROKE_SLOW, "strokeFast": STROKE_FAST, "strokeSlant": STROKE_SLANT, "strokeSegments": STROKE_SEGMENTS, "strokeWindow": STROKE_WINDOW, "strokePause": STROKE_PAUSE,
            "shakeAmplitude": SHAKE_AMPLITUDE, "shakeHysteresis": SHAKE_HYSTERESIS, "shakeSpeed": SHAKE_SPEED, "shakeReversals": SHAKE_REVERSALS, "shakeWindow": SHAKE_WINDOW, "shakePause": SHAKE_PAUSE, "shakeRest": SHAKE_REST,
        }))
    }

    /// 🖲️ The signals of every committed press, as committed.
    pub fn presses(ctx: &Context<'_>) -> Result<Outcome, String> {
        let document = vectors(ctx)?;
        let (quantum, _, _) = frame(&document)?;
        projected(&keyed(list(&document, "presses")?, |vector| agreed("presses", vector, pressed(vector, quantum)?))?)
    }

    /// 🌡️ The answers to every committed history of attention, as committed.
    pub fn warmth(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "warmths")?, |vector| agreed("warmth", vector, warmed(list(vector, "caresses")?)?))?)
    }

    /// 🌀️ The cues of every committed circle, as committed.
    pub fn circles(ctx: &Context<'_>) -> Result<Outcome, String> {
        let document = vectors(ctx)?;
        projected(&keyed(list(&document, "circles")?, |vector| agreed("circles", vector, rows(&hover_cues(&document, vector, 0)?)))?)
    }

    /// 🐈️ The cues of every committed petting, as committed.
    pub fn strokes(ctx: &Context<'_>) -> Result<Outcome, String> {
        let document = vectors(ctx)?;
        projected(&keyed(list(&document, "strokes")?, |vector| agreed("strokes", vector, rows(&hover_cues(&document, vector, 0)?)))?)
    }

    /// 🫨️ The cues of every committed held path, as committed.
    pub fn shakes(ctx: &Context<'_>) -> Result<Outcome, String> {
        let document = vectors(ctx)?;
        projected(&keyed(list(&document, "shakes")?, |vector| agreed("shakes", vector, rows(&held_cues(&document, vector, 0)?)))?)
    }

    /// 🎯️ The rates of the deliberate gestures in their four mirrored variants.
    pub fn detection(ctx: &Context<'_>) -> Result<Outcome, String> {
        let document = vectors(ctx)?;
        projected(&json!({
            "circles": tallied(list(&document, "circles")?, |vector, variant| hover_cues(&document, vector, variant))?,
            "strokes": tallied(list(&document, "strokes")?, |vector, variant| hover_cues(&document, vector, variant))?,
            "shakes": tallied(list(&document, "shakes")?, |vector, variant| held_cues(&document, vector, variant))?,
        }))
    }

    /// 🧳️ The sample stretches of ordinary travel past every body set nothing off.
    pub fn travel_sample(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&travelled(&vectors(ctx)?, true)?)
    }

    /// ⏳️ Every stretch of ordinary travel in each of its variants past every body sets nothing off.
    pub fn travel_hours(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&travelled(&vectors(ctx)?, false)?)
    }
}

/// 🧭️ Subject role only — the oracle is numpy and scipy in `🐍️.py`, and the oracle-only build links nothing of the crate.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built
        .subject("constants", subject::constants)
        .subject("presses", subject::presses)
        .subject("warmth", subject::warmth)
        .subject("circles", subject::circles)
        .subject("strokes", subject::strokes)
        .subject("shakes", subject::shakes)
        .subject("detection", subject::detection)
        .subject("travel-sample", subject::travel_sample)
        .subject("travel-hours", subject::travel_hours);
    built
}
