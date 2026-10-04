//! 🚧️ Subject adapter of the clearance-proof case: the constants and every function of the clearance twin of the `pets` crate answer every committed vector, projected exactly like the TypeScript adapter projects them.
//!
//! Boxes are committed as rows `[x0, y0, x1, y1]` and claims as what they are made of (`owner`, `from`, `span`, the path
//! as one box per tick, `rest`); the adapter builds the claims with `claim_of` and answers boxes as rows again. The
//! TypeScript twin floors a fractional span (`Math.max(Math.floor(span), 1)`); the Rust twin takes whole ticks, so the
//! adapter floors the committed span before it asks. A side is projected as the TypeScript twin's number (1 right,
//! −1 left). The crate links its JSON codec with `float_roundtrip`, so every committed decimal is the double the
//! TypeScript adapter reads.
//!
//! @see ./🥒️.feature
//! @see ./🟦️.ts — the TypeScript adapter, scenario for scenario
//! @see ../../🔨️modules/🚧️clearance/🦀️.rs

use semio_repo_test_host::Adapter;

#[cfg(feature = "sut")]
mod subject {
    use pets::serde_json::{self, json, Map, Value};
    use pets::{
        body_of, canopied, claim_clear, claim_of, column_over, evicted, free_at, guarded_stride, head_under, leaning, lift_onto, must_poof, near_misses, obstacles_of, order_kept, order_of, overlaps, pruned, pushed_out, released, rests_on,
        scoot_fraction, scooted, seat_of, slice_at, slide_of, slide_side, slide_stride, slot_in, spot_on, vaults, Body, Claim, Extent, Facing, Pair, Perch, Point, Posture, Size, Slug, Ticks, DELAY, FOREVER, LANE_LIFT, LEAN, MARGIN, PATIENCE, PUSHES,
        SCOOT_HASTE, SEAM, SLIDE_OFF_GAIN, SLIDE_OFF_LIMIT, SLIDE_OFF_SPEED, STEERINGS, UPRIGHT,
    };
    use semio_repo_test_host::{parse_json, Context, Outcome};

    const VECTORS: &str = "shared://🚧️clearance-proof/🔣️.json";

    /// 🧫️ The committed vectors.
    fn vectors(ctx: &Context<'_>) -> Result<Value, String> {
        serde_json::from_slice(&ctx.fixture_bytes(VECTORS)?).map_err(|error| format!("{VECTORS}: {error}"))
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

    /// ⏱️ A whole number of ticks a vector carries.
    fn ticks(vector: &Value, name: &str) -> Result<Ticks, String> {
        Ok(number(vector, name)? as Ticks)
    }

    /// 📦️ A committed row as a box.
    fn extent(row: &Value) -> Result<Extent, String> {
        let edges: Vec<f64> = row.as_array().map(|edges| edges.iter().filter_map(Value::as_f64).collect()).unwrap_or_default();
        match edges.as_slice() {
            [x0, y0, x1, y1] => Ok(Extent { x0: *x0, y0: *y0, x1: *x1, y1: *y1 }),
            _ => Err(format!("{row} is no row of four numbers")),
        }
    }

    /// 🔲️ Committed rows as boxes.
    fn extents(rows: &[Value]) -> Result<Vec<Extent>, String> {
        rows.iter().map(extent).collect()
    }

    /// 🧾️ A box as the row it is committed as.
    fn row(extent: Extent) -> Value {
        json!([extent.x0, extent.y0, extent.x1, extent.y1])
    }

    /// 🐾️ Committed bodies as bodies.
    fn embodied(bodies: &[Value]) -> Result<Vec<Body>, String> {
        bodies.iter().map(|entry| Ok(Body { owner: text(entry, "owner")?.to_string(), extent: extent(&entry["box"])? })).collect()
    }

    /// 🎫️ What a claim is made of as the claim `claim_of` makes of it; the span floored as the TypeScript twin floors it.
    fn claimed(plan: &Value) -> Result<Claim, String> {
        let rest = if plan["rest"].is_null() { None } else { Some(extent(&plan["rest"])?) };
        Ok(claim_of(text(plan, "owner")?, ticks(plan, "from")?, &extents(list(plan, "path")?)?, number(plan, "span")?.floor() as Ticks, rest))
    }

    /// 🗂️ Committed claims as claims.
    fn claims_of(plans: &[Value]) -> Result<Vec<Claim>, String> {
        plans.iter().map(claimed).collect()
    }

    /// 🧍️ The posture a vector names.
    fn postured(stance: &Value, size: Size) -> Result<Posture, String> {
        Ok(match text(stance, "kind")? {
            "leaning" => leaning(number(stance, "height")?, number(stance, "sine")?),
            "canopied" => canopied(size, serde_json::from_value(stance["canopy"].clone()).map_err(|error| error.to_string())?),
            "plain" => Posture { left: number(stance, "left")?, right: number(stance, "right")?, above: number(stance, "above")? },
            _ => UPRIGHT,
        })
    }

    /// 🪵️ The perch a vector carries.
    fn perch(vector: &Value) -> Result<Perch, String> {
        serde_json::from_value(vector["perch"].clone()).map_err(|error| format!("perch: {error}"))
    }

    /// 👥️ Pairs as `[first, second]` rows.
    fn paired(pairs: &[Pair]) -> Value {
        json!(pairs.iter().map(|pair| [pair.first.as_str(), pair.second.as_str()]).collect::<Vec<_>>())
    }

    /// 🔤️ The slugs of a committed list.
    fn slugs(values: &[Value]) -> Vec<Slug> {
        values.iter().filter_map(Value::as_str).map(str::to_string).collect()
    }

    /// 🧲️ A committed side as a facing.
    fn facing(side: f64) -> Facing {
        if side > 0.0 {
            Facing::Right
        } else {
            Facing::Left
        }
    }

    /// 📍️ The free place a vector asks for.
    fn placed(place: &Value) -> Result<Value, String> {
        let obstacles = extents(list(place, "obstacles")?)?;
        let (x, own) = (number(place, "x")?, extent(&place["box"])?);
        Ok(json!(match text(place, "kind")? {
            "slot" => slot_in(x, own, number(place, "low")?, number(place, "high")?, &obstacles),
            "spot" => spot_on(&perch(place)?, x, own, number(place, "foot")?, &obstacles),
            _ => column_over(&perch(place)?, x, own, number(place, "drop")?, number(place, "foot")?, &obstacles),
        }))
    }

    /// 🪜️ The order, the kept order or the vault a vector asks for.
    fn ordered(order: &Value) -> Result<Value, String> {
        Ok(match text(order, "kind")? {
            "rank" => json!(order_of(&embodied(list(order, "bodies")?)?)),
            "kept" => json!(order_kept(&slugs(list(order, "before")?), &slugs(list(order, "after")?))),
            _ => json!(vaults(extent(&order["hopper"])?, number(order, "rise")?, extent(&order["hurdle"])?)),
        })
    }

    /// 🚦️ Everything asked about one plan.
    fn planned(vector: &Value) -> Result<Value, String> {
        let plan = claimed(&vector["plan"])?;
        let bodies = embodied(list(vector, "bodies")?)?;
        let claims = claims_of(list(vector, "claims")?)?;
        let probe = extent(&vector["probe"])?;
        let prober = text(vector, "prober")?;
        let ticks: Vec<Ticks> = list(vector, "ticks")?.iter().filter_map(Value::as_f64).map(|tick| tick as Ticks).collect();
        Ok(json!({
            "slices": plan.slices.iter().map(|slice| json!([slice.from, slice.until, slice.extent.x0, slice.extent.y0, slice.extent.x1, slice.extent.y1])).collect::<Vec<_>>(),
            "at": ticks.iter().map(|tick| slice_at(&plan, *tick).map_or(Value::Null, row)).collect::<Vec<_>>(),
            "clear": claim_clear(&plan, &bodies, &claims),
            "free": ticks.iter().map(|tick| free_at(probe, prober, &bodies, &claims, *tick)).collect::<Vec<_>>(),
            "obstacles": ticks.iter().map(|tick| obstacles_of(prober, &bodies, &claims, *tick).len()).collect::<Vec<_>>(),
            "pruned": ticks.iter().map(|tick| pruned(&claims, *tick).iter().map(|claim| json!([claim.owner, claim.slices.len()])).collect::<Vec<_>>()).collect::<Vec<_>>(),
            "released": released(&claims, prober).iter().map(|claim| claim.owner.clone()).collect::<Vec<_>>(),
        }))
    }

    /// 💺️ The seating of a perch and the first tick of the scoot to it.
    fn seated(vector: &Value) -> Result<Value, String> {
        let bodies = embodied(list(vector, "bodies")?)?;
        let seating = seat_of(&bodies, &perch(vector)?, number(vector, "margin")?);
        let shifts: Vec<f64> = seating.seats.iter().map(|seat| seat.shift).collect();
        let fraction = scoot_fraction(&shifts, number(vector, "stride")?);
        let mut places = Vec::new();
        for seat in &seating.seats {
            let start = bodies.iter().find(|entry| entry.owner == seat.owner).ok_or_else(|| format!("no body of {}", seat.owner))?.extent.x0;
            places.push(scooted(start, start + seat.shift, fraction));
        }
        Ok(json!({ "seats": seating.seats.iter().map(|seat| json!([seat.owner, seat.shift])).collect::<Vec<_>>(), "leavers": seating.leavers, "fraction": fraction, "places": places }))
    }

    /// 🎩️ What a vector asks about heads.
    fn headed(head: &Value) -> Result<Value, String> {
        Ok(match text(head, "kind")? {
            "landing" => {
                let after = extent(&head["after"])?;
                let bodies = embodied(list(head, "bodies")?)?;
                head_under(extent(&head["before"])?, after, text(head, "owner")?, &bodies).map_or(json!({ "host": null, "lift": null }), |host| json!({ "host": host.owner, "lift": lift_onto(after, host.extent) }))
            }
            "strides" => json!(list(head, "ticks")?.iter().filter_map(Value::as_f64).map(|tick| slide_stride(tick as Ticks)).collect::<Vec<_>>()),
            "slide" => {
                let slide = slide_of(extent(&head["rider"])?, facing(number(head, "side")?), ticks(head, "ticks")?, number(head, "low")?, number(head, "high")?, &extents(list(head, "obstacles")?)?);
                json!([slide.stride, slide.side.sign()])
            }
            "resting" => json!(rests_on(extent(&head["rider"])?, extent(&head["host"])?)),
            _ => json!(slide_side(extent(&head["rider"])?, extent(&head["host"])?).sign()),
        })
    }

    /// 🫸️ The push a vector asks for: `[x, y]`, or `null` when the rounds are over before the body is free.
    fn pushed(vector: &Value) -> Result<Value, String> {
        let push = pushed_out(extent(&vector["box"])?, &extents(list(vector, "obstacles")?)?, number(vector, "iterations")? as usize);
        Ok(push.map_or(Value::Null, |Point { x, y }| json!([x, y])))
    }

    /// 💨️ The last resort a vector asks about.
    fn resorted(resort: &Value) -> Result<Value, String> {
        let bodies = embodied(list(resort, "bodies")?)?;
        let claims = claims_of(list(resort, "claims")?)?;
        let tick = ticks(resort, "tick")?;
        if text(resort, "kind")? == "evict" {
            return Ok(json!(evicted(&bodies, &claims, tick)));
        }
        let stay = if resort["stay"].is_null() { None } else { Some(extent(&resort["stay"])?) };
        Ok(json!(must_poof(text(resort, "owner")?, stay, &claims_of(list(resort, "plans")?)?, &bodies, &claims, tick)))
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

    /// 🎛️ The tuning constants.
    pub fn constants(_: &Context<'_>) -> Result<Outcome, String> {
        projected(
            &json!({ "margin": MARGIN, "seam": SEAM, "forever": FOREVER, "steerings": STEERINGS, "pushes": PUSHES, "patience": PATIENCE, "delay": DELAY, "slideOffSpeed": SLIDE_OFF_SPEED, "slideOffGain": SLIDE_OFF_GAIN, "slideOffLimit": SLIDE_OFF_LIMIT, "lean": LEAN, "laneLift": LANE_LIFT, "scootHaste": SCOOT_HASTE }),
        )
    }

    /// 📐️ The four edges of every committed body.
    pub fn bodies(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "bodies")?, |vector| {
            let size: Size = serde_json::from_value(vector["size"].clone()).map_err(|error| format!("size: {error}"))?;
            let feet: Point = serde_json::from_value(vector["feet"].clone()).map_err(|error| format!("feet: {error}"))?;
            Ok(row(body_of(text(vector, "id")?, feet, size, number(vector, "hover")?, postured(&vector["posture"], size)?, number(vector, "margin")?).extent))
        })?)
    }

    /// 🚨️ The overlapping and the near pairs of every committed crowd.
    pub fn crowds(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "crowds")?, |vector| {
            let bodies = embodied(list(vector, "bodies")?)?;
            Ok(json!({ "overlaps": paired(&overlaps(&bodies)), "nears": paired(&near_misses(&bodies, number(vector, "margin")?)) }))
        })?)
    }

    /// 🎯️ Every committed free place.
    pub fn spots(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "spots")?, placed)?)
    }

    /// 👣️ Every committed guarded stride.
    pub fn strides(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "strides")?, |vector| Ok(json!(guarded_stride(extent(&vector["box"])?, number(vector, "stride")?, &extents(list(vector, "obstacles")?)?))))?)
    }

    /// 📶️ Every committed order, kept order and vault.
    pub fn orders(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "orders")?, ordered)?)
    }

    /// 🛤️ Every committed plan with everything asked about it.
    pub fn claims(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "claims")?, planned)?)
    }

    /// 🛋️ Every committed seating and the first tick of its scoot.
    pub fn seatings(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "seatings")?, seated)?)
    }

    /// 🪨️ Every committed landing, rest, side and slide.
    pub fn heads(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "heads")?, headed)?)
    }

    /// 🤲️ Every committed push.
    pub fn pushes(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "pushes")?, pushed)?)
    }

    /// 🚪️ Every committed last resort.
    pub fn resorts(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "resorts")?, resorted)?)
    }
}

/// 🧭️ Subject role only — the oracle is scipy and numpy in `🐍️.py`, and the oracle-only build links nothing of the crate.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built
        .subject("constants", subject::constants)
        .subject("bodies", subject::bodies)
        .subject("crowds", subject::crowds)
        .subject("spots", subject::spots)
        .subject("strides", subject::strides)
        .subject("orders", subject::orders)
        .subject("claims", subject::claims)
        .subject("seatings", subject::seatings)
        .subject("heads", subject::heads)
        .subject("pushes", subject::pushes)
        .subject("resorts", subject::resorts);
    built
}
