//! 📜️ The constants, the committed wall, ladder and grapple vectors and the exact operations of the design of the climbing unit laws. See [`super`].

use super::*;

#[test]
fn the_constants_are_the_starting_values_of_the_research_brief() {
    assert_eq!([CROSS_REACH, LUNGE_TICKS as f64], [1.5, 16.0]);
    assert_eq!([CLIMB_RISE, CLIMB_DESCENT, GRIP_BUDGET, SLIDE_START, SLIDE_GAIN, SLIDE_SPEED, SLIP_PUSH, SLIP_LIFT], [30.0, 40.0, 384.0, 30.0, 600.0, 160.0, 140.0, 160.0]);
    assert_eq!([LADDER_LEAN, LADDER_STEEP, LADDER_FLAT, LADDER_SHORT, LADDER_TALL, LADDER_RISE, LADDER_DESCENT], [0.25, 0.14, 0.4, 0.8, 8.0, 26.0, 34.0]);
    assert_eq!([HOOK_SPEED, HOOK_RETURN, ROPE_SHORT, ROPE_LONG, ROPE_ELEVATION, ZIP_SLANT, ZIP_SPEED], [640.0, 1280.0, 0.8, 8.0, 0.42, 0.35, 150.0]);
    assert_eq!([REEL_LEAST, LADDER_FOLLOW, LADDER_SHIFT, ROPE_DETOUR], [0.9, 8.0, 12.0, 0.3]);
    assert_eq!([MANTLE_TICKS, CLIMB_RAMP], [28, 6]);
}

#[test]
fn committed_wall_climbing_holds_surveys_climbs_grips_slides_mantles_and_routes_are_reproduced() {
    let vectors = fixture("wall-climbing");
    let constants = json!({ "wallLip": WALL_LIP, "handHeight": HAND_HEIGHT, "climbRise": CLIMB_RISE, "climbDescent": CLIMB_DESCENT, "climbRamp": CLIMB_RAMP, "gripSpacing": GRIP_SPACING, "gripBudget": GRIP_BUDGET, "gripClimb": GRIP_CLIMB, "gripHang": GRIP_HANG, "gripRest": GRIP_REST, "gripBite": GRIP_BITE, "wallFollow": WALL_FOLLOW, "slideStart": SLIDE_START, "slideGain": SLIDE_GAIN, "slideSpeed": SLIDE_SPEED, "slipPush": SLIP_PUSH, "slipLift": SLIP_LIFT, "wallGrabTicks": WALL_GRAB_TICKS, "wallHangTicks": WALL_HANG_TICKS, "mantleTicks": MANTLE_TICKS, "mantleInset": MANTLE_INSET, "hoistHump": HOIST_HUMP, "hoistRise": HOIST_RISE, "crossReach": CROSS_REACH, "lungeTicks": LUNGE_TICKS });
    assert_close("constants", &constants, &vectors["constants"]);
    for vector in entries(&vectors["crossings"]) {
        let (size, pitches): (Size, Vec<Pitch>) = (typed(&vector["size"]), typed(&vector["pitches"]));
        let crossings: Vec<Vec<bool>> = pitches.iter().map(|from| pitches.iter().map(|to| crossable(from, to, size)).collect()).collect();
        let chains: Vec<Vec<Value>> = pitches.iter().map(|held| chain_of(held, &pitches, size).into_iter().map(|member| located(&pitches, Some(member))).collect()).collect();
        let mut ways = Vec::new();
        for wish in entries(&vector["ways"]) {
            let held = &pitches[typed::<usize>(&wish["pitch"])];
            let chain = chain_of(held, &pitches, size);
            let start = chain.iter().position(|member| std::ptr::eq(*member, held)).unwrap_or_default();
            let path = wall_path(&chain, start, typed(&wish["entry"]), number(&wish["x"]), number(&wish["y"]), typed(&wish["end"]), number(&wish["goal"]), wish["mantle"] == true, size);
            ways.push(json!({ "path": path.iter().map(|clamber| json!([clamber.x, clamber.y, clamber.hold, clamber.work])).collect::<Vec<_>>(), "cost": wall_cost(&path) }));
        }
        assert_close(&format!("crossing {}", vector["id"]), &json!({ "crossable": crossings, "chains": chains, "ways": ways }), &vector["expected"]);
    }
    for stage in group(&vectors, "holds") {
        let (size, perches, pitches): (Size, Vec<Perch>, Vec<Pitch>) = (typed(&stage["size"]), typed(&stage["perches"]), typed(&stage["pitches"]));
        let grips: Vec<Vec<Option<WallHold>>> = perches.iter().map(|perch| pitches.iter().map(|pitch| grip_for(perch, pitch, size)).collect()).collect();
        let rims: Vec<Value> = pitches.iter().map(|pitch| located(&perches, rim_for(pitch, &perches, size))).collect();
        let measures: Vec<Value> = pitches.iter().map(|pitch| json!({ "cling": cling_of(pitch, size), "ledge": ledge_of(pitch, size), "rim": rim_of(pitch, size), "foot": foot_of(pitch, size) })).collect();
        assert_close(&format!("hold {}", stage["id"]), &json!({ "grips": grips, "rims": rims, "measures": measures }), &stage["expected"]);
    }
    for vector in group(&vectors, "surveys") {
        let (size, held, pitches): (Size, Pitch, Vec<Pitch>) = (typed(&vector["size"]), typed(&vector["pitch"]), typed(&vector["pitches"]));
        let found = wall_holds(&held, &pitches, number(&vector["y"]), size);
        assert_close(&format!("survey {}", vector["id"]), &json!({ "pitch": located(&pitches, found), "slip": found.map_or_else(|| json(&slip_of(&held)), |_| Value::Null) }), &vector["expected"]);
    }
    for vector in group(&vectors, "climbs") {
        let (size, held, from, goal): (Size, Pitch, f64, f64) = (typed(&vector["size"]), typed(&vector["pitch"]), number(&vector["from"]), number(&vector["goal"]));
        let ticks = climb_ticks(from, goal);
        let mut heights = Vec::new();
        let mut height = from;
        for tick in 0..ticks {
            if (ticks as f64) > GRIP_BUDGET {
                break;
            }
            height = climb_step(height, goal, tick);
            heights.push(height);
        }
        let phases: Vec<f64> = heights.iter().map(|&reached| climb_phase(&held, reached, size)).collect();
        assert_close(&format!("climb {}", vector["id"]), &json!({ "ticks": ticks, "heights": heights, "phases": phases }), &vector["expected"]);
    }
    for vector in group(&vectors, "grips") {
        let mut left = number(&vector["grip"]);
        let mut grips = Vec::new();
        for run in entries(&vector["runs"]) {
            let (effort, ticks): (Effort, usize) = (typed(&run[0]), typed(&run[1]));
            for _ in 0..ticks {
                left = grip_step(left, effort);
            }
            grips.push(left);
        }
        assert_close(&format!("grip {}", vector["id"]), &json!(grips), &vector["expected"]);
    }
    for vector in group(&vectors, "slides") {
        let (mut slide, floor, ticks): (Fall, f64, usize) = (Fall { y: number(&vector["y"]), vy: number(&vector["vy"]) }, number(&vector["floor"]), typed(&vector["ticks"]));
        let (mut heights, mut speeds) = (Vec::new(), Vec::new());
        for _ in 0..ticks {
            slide = slide_step(slide.y, slide.vy, floor);
            heights.push(slide.y);
            speeds.push(slide.vy);
        }
        assert_close(&format!("slide {}", vector["id"]), &json!({ "heights": heights, "speeds": speeds }), &vector["expected"]);
    }
    for vector in group(&vectors, "mantles") {
        let (size, held): (Size, Pitch) = (typed(&vector["size"]), typed(&vector["pitch"]));
        let path: Vec<[f64; 2]> = (0..=MANTLE_TICKS).map(|tick| mantle_path(&held, size, tick as f64 / MANTLE_TICKS as f64)).map(|at| [at.x, at.y]).collect();
        assert_close(&format!("mantle {}", vector["id"]), &json!(path), &vector["expected"]);
    }
    for vector in group(&vectors, "hoists") {
        let (from, to, height, ticks): (Point, Point, f64, Ticks) = (typed(&vector["from"]), typed(&vector["to"]), number(&vector["height"]), typed(&vector["ticks"]));
        let path: Vec<[f64; 2]> = (0..=ticks).map(|tick| hoist_path(from, to, height, tick as f64 / ticks as f64)).map(|at| [at.x, at.y]).collect();
        assert_close(&format!("hoist {}", vector["id"]), &json!(path), &vector["expected"]);
    }
    for vector in group(&vectors, "routes") {
        let (size, perches, pitches): (Size, Vec<Perch>, Vec<Pitch>) = (typed(&vector["size"]), typed(&vector["perches"]), typed(&vector["pitches"]));
        let (from, to): (usize, usize) = (typed(&vector["from"]), typed(&vector["to"]));
        let legs = route_of(number(&vector["x"]), &perches[from], &perches[to], &[Gear::Climb], size, number(&vector["grip"]), &pitches, &[], &[]);
        let projected = legs.map_or(Value::Null, |legs| {
            Value::Array(
                legs.iter()
                    .map(|leg| match leg {
                        Leg::Wall { at, pitch, hold, exit, goal } => json!({ "at": at, "hold": hold, "goal": goal, "pitch": located(&pitches, Some(*pitch)), "exit": located(&pitches, Some(*exit)) }),
                        other => leg_value(other, &pitches, &[]),
                    })
                    .collect(),
            )
        });
        assert_close(&format!("route {}", vector["id"]), &projected, &vector["expected"]);
    }
}

#[test]
fn committed_ladder_geometry_vectors_are_reproduced() {
    let vectors = fixture("ladder-geometry");
    let constants = json!({ "ladderLean": LADDER_LEAN, "ladderSteep": LADDER_STEEP, "ladderFlat": LADDER_FLAT, "ladderShort": LADDER_SHORT, "ladderTall": LADDER_TALL, "ladderTuck": LADDER_TUCK, "ladderHorns": LADDER_HORNS, "rungSpacing": RUNG_SPACING, "ladderFooting": LADDER_FOOTING, "ladderGirth": LADDER_GIRTH, "ladderRise": LADDER_RISE, "ladderDescent": LADDER_DESCENT, "ladderRamp": LADDER_RAMP, "ladderExit": LADDER_EXIT, "ladderFollow": LADDER_FOLLOW, "ladderShift": LADDER_SHIFT, "ladderMountTicks": LADDER_MOUNT_TICKS, "ladderDismountTicks": LADDER_DISMOUNT_TICKS, "ladderRaiseTicks": LADDER_RAISE_TICKS, "ladderIdle": LADDER_IDLE, "ladderLife": LADDER_LIFE, "toppleStiffness": TOPPLE_STIFFNESS, "toppleDamping": TOPPLE_DAMPING, "topplePush": TOPPLE_PUSH, "toppleStep": TOPPLE_STEP });
    assert_close("constants", &constants, &vectors["constants"]);
    for vector in group(&vectors, "placements") {
        let (size, low, high, held, keepouts): (Size, Perch, Perch, Pitch, Vec<Rect>) = (typed(&vector["size"]), typed(&vector["low"]), typed(&vector["high"]), typed(&vector["pitch"]), typed(&vector["keepouts"]));
        let placed = ladder_for(&low, &high, &held, &keepouts, size)
            .map_or(Value::Null, |ladder| json!({ "length": ladder_length(&ladder), "rungs": ladder_rungs(&ladder), "lean": ladder_lean(&ladder), "exit": ladder_exit(&ladder, size), "landing": ladder_landing(&ladder, size), "ladder": ladder }));
        assert_close(&format!("placement {}", vector["id"]), &placed, &vector["expected"]);
    }
    for vector in group(&vectors, "climbs") {
        let (size, ladder): (Size, LadderStand) = (typed(&vector["size"]), typed(&vector["ladder"]));
        let exit = ladder_exit(&ladder, size);
        let goal = if vector["down"] == true { 0.0 } else { exit };
        let mut travel = if vector["down"] == true { exit } else { 0.0 };
        let mut travels = Vec::new();
        while travel != goal && travels.len() < 1024 {
            travel = ladder_step(travel, goal, travels.len() as Ticks);
            travels.push(travel);
        }
        let feet: Vec<[f64; 2]> = travels.iter().map(|&reached| ladder_at(&ladder, reached)).map(|at| [at.x, at.y]).collect();
        let phases: Vec<f64> = travels.iter().map(|&reached| ladder_phase(reached)).collect();
        assert_close(&format!("climb {}", vector["id"]), &json!({ "ticks": travels.len(), "travels": travels, "feet": feet, "phases": phases }), &vector["expected"]);
    }
    for vector in group(&vectors, "surveys") {
        let (size, ladder, perches, pitches, keepouts): (Size, LadderStand, Vec<Perch>, Vec<Pitch>, Vec<Rect>) = (typed(&vector["size"]), typed(&vector["ladder"]), typed(&vector["perches"]), typed(&vector["pitches"]), typed(&vector["keepouts"]));
        assert_close(&format!("survey {}", vector["id"]), &json(&ladder_holds(&ladder, &perches, &pitches, &keepouts, size)), &vector["expected"]);
    }
    for vector in group(&vectors, "spills") {
        let (size, ladder, heights): (Size, LadderStand, Vec<f64>) = (typed(&vector["size"]), typed(&vector["ladder"]), typed(&vector["heights"]));
        let spills: Vec<Option<Toss>> = heights.iter().map(|&height| spill_of(&ladder, height, size)).collect();
        assert_close(&format!("spill {}", vector["id"]), &json(&spills), &vector["expected"]);
    }
}

#[test]
fn committed_grapple_reach_vectors_are_reproduced() {
    let vectors = fixture("grapple-reach");
    let constants = json!({ "hookSpeed": HOOK_SPEED, "hookReturn": HOOK_RETURN, "ropeShort": ROPE_SHORT, "ropeLong": ROPE_LONG, "ropeElevation": ROPE_ELEVATION, "ropeMargin": ROPE_MARGIN, "ropeDetour": ROPE_DETOUR, "ropeFollow": ROPE_FOLLOW, "ropeRise": ROPE_RISE, "muzzleForward": MUZZLE_FORWARD, "muzzleHeight": MUZZLE_HEIGHT, "hookInset": HOOK_INSET, "hookLift": HOOK_LIFT, "zipSlant": ZIP_SLANT, "zipSpeed": ZIP_SPEED, "zipRamp": ZIP_RAMP, "ropeAimTicks": ROPE_AIM_TICKS, "ropeRecoilTicks": ROPE_RECOIL_TICKS, "ropeTugTicks": ROPE_TUG_TICKS, "ropeHoistTicks": ROPE_HOIST_TICKS, "ropeShrugTicks": ROPE_SHRUG_TICKS, "ropeMissChance": ROPE_MISS_CHANCE, "ropeMissOvershoot": ROPE_MISS_OVERSHOOT, "ropeRest": ROPE_REST, "ropeSulk": ROPE_SULK });
    assert_close("constants", &constants, &vectors["constants"]);
    for vector in group(&vectors, "shots") {
        let (size, feet, perches, keepouts): (Size, Point, Vec<Perch>, Vec<Rect>) = (typed(&vector["size"]), typed(&vector["feet"]), typed(&vector["perches"]), typed(&vector["keepouts"]));
        assert_close(&format!("shot {}", vector["id"]), &json(&shot_for(feet, &perches, &keepouts, size)), &vector["expected"]);
    }
    for vector in group(&vectors, "flights") {
        let (from, to, speed): (Point, Point, f64) = (typed(&vector["from"]), typed(&vector["to"]), number(&vector["speed"]));
        let ticks = hook_ticks(from, to, speed);
        let path: Vec<[f64; 2]> = (0..ticks + 2).map(|tick| hook_step(from, to, speed, tick)).map(|at| [at.x, at.y]).collect();
        assert_close(&format!("flight {}", vector["id"]), &json!({ "ticks": ticks, "path": path }), &vector["expected"]);
    }
    for vector in group(&vectors, "zips") {
        let (size, shot): (Size, Shot) = (typed(&vector["size"]), typed(&vector["shot"]));
        assert_close(&format!("zip {}", vector["id"]), &hauled(&shot, size, zip_step), &vector["expected"]);
    }
    for vector in group(&vectors, "swings") {
        let (size, shot): (Size, Shot) = (typed(&vector["size"]), typed(&vector["shot"]));
        assert_close(&format!("swing {}", vector["id"]), &hauled(&shot, size, sway_step), &vector["expected"]);
    }
    for vector in group(&vectors, "surveys") {
        let (shot, perches, keepouts): (Shot, Vec<Perch>, Vec<Rect>) = (typed(&vector["shot"]), typed(&vector["perches"]), typed(&vector["keepouts"]));
        assert_close(&format!("survey {}", vector["id"]), &json(&shot_holds(&shot, &perches, &keepouts)), &vector["expected"]);
    }
    for vector in group(&vectors, "landings") {
        let (size, shot, edge): (Size, Shot, Perch) = (typed(&vector["size"]), typed(&vector["shot"]), typed(&vector["perch"]));
        assert_close(&format!("landing {}", vector["id"]), &json!({ "landing": landing_for(&shot, &edge, size), "miss": miss_of(&shot, &edge) }), &vector["expected"]);
    }
    for vector in group(&vectors, "routes") {
        let (size, gear, perches): (Size, Vec<Gear>, Vec<Perch>) = (typed(&vector["size"]), typed(&vector["gear"]), typed(&vector["perches"]));
        let (pitches, ladders, keepouts): (Vec<Pitch>, Vec<LadderStand>, Vec<Rect>) = (typed(&vector["pitches"]), typed(&vector["ladders"]), typed(&vector["keepouts"]));
        let (from, to): (usize, usize) = (typed(&vector["from"]), typed(&vector["to"]));
        let legs = route_of(number(&vector["x"]), &perches[from], &perches[to], &gear, size, number(&vector["grip"]), &pitches, &ladders, &keepouts);
        let projected = legs.map_or(Value::Null, |legs| Value::Array(legs.iter().map(|leg| leg_value(leg, &pitches, &ladders)).collect()));
        assert_close(&format!("route {}", vector["id"]), &projected, &vector["expected"]);
    }
}

#[test]
fn source_uses_nothing_but_the_exact_operations_of_the_design() {
    let code: Vec<&str> = SOURCE.lines().filter(|line| !line.trim_start().starts_with("//")).collect();
    let transcendental = [".sin(", ".cos(", ".tan(", ".atan2(", ".exp(", ".ln(", ".log", ".powf(", ".powi(", ".mul_add(", ".hypot(", ".cbrt("];
    let native = [".max(", ".min(", ".clamp(", "f64::max", "f64::min", "std::time", "println!", "eprintln!", "dbg!", "static ", "thread_local!"];
    for banned in transcendental.into_iter().chain(native) {
        assert!(code.iter().all(|line| !line.contains(banned)), "{banned}");
    }
    for exact in [".sqrt()", ".abs()", ".floor()", "larger(", "smaller("] {
        assert!(code.iter().any(|line| line.contains(exact)), "{exact}");
    }
}
