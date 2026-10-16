//! 🏆️ The proctor end to end, the challenge sheets: every challenge deals its own sheet and earns its own points, expert and hard runs clock and score from the instants the device claims, an easy run questions far off cards until it closes and switching the challenge ends in runs exhausted.
//!
//! @see ../🦀️.rs — the end to end suite that mounts this module
//! @see ../../../🧫️fixtures/📚️catalog/🔣️.json — the catalog played

use super::*;

/// 🧩️ The presented task of `kind` in the sheet of a run view.
fn task_of(run: &Value, kind: &str) -> Value {
    run["sheet"]["tasks"].as_array().expect("tasks").iter().find(|task| task["kind"] == kind).expect("a task of that kind").clone()
}

#[test]
fn every_challenge_deals_its_own_sheet_and_earns_its_own_points() {
    let data = scratch("challenges");
    let running = boot(&data.0, development());
    let base = running.base.clone();
    let ada = id(0xada);
    accepted(&base, &json!({ "type": "identify-learner", "id": id(1), "learner": ada, "identity": { "kind": "pseudonym", "handle": "Ada" } }));
    let (power, homes) = (quiz_document("power"), quiz_document("homes"));
    let run_view = |run: &str| view(&base, &json!({ "type": "run", "run": run }));
    let mut keys: Vec<f64> = power["tasks"][0]["items"].as_array().expect("items").iter().map(|item| item["value"].as_f64().expect("a value")).collect();
    keys.sort_by(f64::total_cmp);

    let easy = id(0x100);
    accepted(&base, &start(&ada, &easy, "power", "easy", 10));
    let opened = run_view(&easy);
    let (sorting, matching) = (task_of(&opened, "sorting"), task_of(&opened, "matching"));
    assert_eq!((&sorting["keys"], opened["sheet"]["challenge"].as_str()), (&json!(keys), Some("easy")), "an easy sheet shows the ascending keys of a sorting");
    assert!(matching["dimensions"].as_array().expect("dimensions").iter().all(|dimension| dimension["cards"].is_array()), "and the cards of a matching");
    assert!(opened.get("opened").is_none() && opened.get("hints").is_none() && !opened["sheet"].to_string().contains("\"seconds\""), "an untimed run without answers has neither a clock nor hints: {opened}");
    let mut reversed = perfect(&power, &sorting);
    reversed["order"].as_array_mut().expect("order").reverse();
    accepted(&base, &record(&ada, &easy, &sorting, &reversed, 11));
    assert_eq!(
        run_view(&easy)["hints"]["appliances"],
        json!([
            { "kind": "compare", "item": "heat-pump", "other": "phone-charger", "factor": 5.0 / 3000.0, "verdict": "reversed" },
            { "kind": "compare", "item": "kettle", "other": "phone-charger", "factor": 50.0 / 3000.0, "verdict": "reversed" },
            { "kind": "compare", "item": "phone-charger", "other": "heat-pump", "factor": 3000.0 / 5.0, "verdict": "reversed" },
        ]),
        "with every key far off and the order reversed, each claim points the wrong way; of the four misses the three most wrong stand (the kettle's ties with the laptop's and comes first), in the learner's order"
    );
    accepted(&base, &record(&ada, &easy, &sorting, &perfect(&power, &sorting), 12));
    assert!(run_view(&easy).get("hints").is_none(), "a right answer earns no hint");
    accepted(&base, &record(&ada, &easy, &matching, &perfect(&power, &matching), 13));
    let submitted = accepted(&base, &submit(&ada, &easy, 14));
    let result = submitted[0]["result"].clone();
    assert_eq!((result["challenge"].as_str(), result["score"].as_f64(), result["points"].as_f64()), (Some("easy"), Some(1.0), Some(100.0)));
    assert_eq!(submitted[1..].iter().map(|event| event["badge"].as_str().expect("a badge")).collect::<Vec<_>>(), ["sorter"], "a perfect easy run earns no badge that asks for medium or harder");

    let medium = id(0x101);
    accepted(&base, &start(&ada, &medium, "power", "medium", 20));
    let opened = run_view(&medium);
    let (sorting, matching) = (task_of(&opened, "sorting"), task_of(&opened, "matching"));
    assert_eq!(sorting["keys"], json!(keys), "a medium sheet shows the keys as well");
    let mut guessed = perfect(&power, &sorting);
    guessed["guesses"] = json!({ "kettle": 2000 });
    assert_eq!(rejected(&base, &record(&ada, &medium, &sorting, &guessed, 21)), "answer-invalid", "where the keys show there is nothing to guess");
    accepted(&base, &record(&ada, &medium, &sorting, &reversed, 22));
    assert!(run_view(&medium).get("hints").is_none(), "a medium run gives no hints");
    accepted(&base, &record(&ada, &medium, &matching, &perfect(&power, &matching), 23));
    let result = accepted(&base, &submit(&ada, &medium, 24))[0]["result"].clone();
    assert_eq!((result["challenge"].as_str(), result["score"].as_f64(), result["points"].as_f64()), (Some("medium"), Some(0.5), Some(100.0)), "a reversed sorting and a perfect matching: {result}");

    let hard = id(0x102);
    accepted(&base, &start(&ada, &hard, "power", "hard", 30));
    let opened = run_view(&hard);
    let sheet = opened["sheet"].to_string();
    assert!(!sheet.contains("\"value\"") && !sheet.contains("\"keys\"") && !sheet.contains("\"cards\"") && !sheet.contains("\"seconds\""), "a hard sheet carries no number at all: {sheet}");
    let (sorting, matching) = (task_of(&opened, "sorting"), task_of(&opened, "matching"));
    assert_eq!(rejected(&base, &record(&ada, &hard, &matching, &json!({ "kind": "matching", "assignments": { "capacity": { "rooftop-pv": 0 } } }), 31)), "answer-invalid", "where the keys are hidden there is no card to assign");
    let mut guessed = perfect(&power, &sorting);
    guessed["guesses"]["heat-pump"] = json!(3.0e7);
    accepted(&base, &record(&ada, &hard, &sorting, &guessed, 32));
    accepted(&base, &record(&ada, &hard, &matching, &perfect(&power, &matching), 33));
    let hard_result = accepted(&base, &submit(&ada, &hard, 34))[0]["result"].clone();
    let scored = hard_result["tasks"].as_array().expect("tasks").iter().find(|task| task["kind"] == "sorting").expect("the sorting")["items"].clone();
    assert_eq!(scored.as_array().expect("items").iter().map(|item| (item["item"].as_str().expect("item"), item["guess"].as_f64(), item["miss"].as_bool())).collect::<Vec<_>>(), [("phone-charger", Some(5.0), Some(false)), ("laptop", Some(50.0), Some(false)), ("kettle", Some(2000.0), Some(false)), ("heat-pump", Some(3.0e7), Some(true))], "a guess four decades off misses");
    let score = hard_result["score"].as_f64().expect("a score");
    assert!(hard_result["challenge"] == "hard" && 0.5 < score && score < 1.0 && hard_result["points"].as_f64() == Some(score * 300.0), "the miss costs every pair it touches, the rest is scored: {hard_result}");

    let (paced, timed) = (id(0x103), id(0x104));
    accepted(&base, &start(&ada, &paced, "homes", "medium", 40));
    let systems = task_of(&run_view(&paced), "classification");
    assert_eq!(rejected(&base, &open_task(&ada, &paced, &systems, now(), 41)), "run-untimed");
    let switched = accepted(&base, &start(&ada, &timed, "homes", "expert", 42));
    assert_eq!(switched.iter().map(|event| (event["type"].as_str().expect("type"), event["run"].as_str().expect("run"))).collect::<Vec<_>>(), [("run-voided", paced.as_str()), ("run-started", timed.as_str())], "another challenge voids the open run of the quiz");
    assert_eq!(rejected(&base, &start(&ada, &id(0x105), "homes", "expert", 43)), "run-open", "the open run of the same challenge is resumed, not replaced");
    let opened = run_view(&timed);
    let systems = task_of(&opened, "classification");
    assert_eq!((systems["seconds"].as_u64(), &opened["opened"]), (Some(30 + 8 * 4), &json!({})), "an expert task allows thirty seconds and eight per item, and none is opened yet");
    assert!(systems["axes"].as_array().expect("axes").iter().all(|axis| axis.as_object().is_some_and(|axis| axis.keys().all(|key| ["id", "label", "short"].contains(&key.as_str())))) && systems["axes"][1]["short"] == json!({ "en": "cost", "de": "Kosten" }) && !opened["sheet"].to_string().contains("\"value\""), "an expert diagram shows shape only: {systems}");
    let shown = systems["items"].as_array().expect("items");
    let category = |item: &Value| homes["tasks"][0]["items"].as_array().expect("items").iter().find(|candidate| candidate["id"] == item["id"]).expect("item")["category"].clone();
    let partial = json!({ "kind": "classification", "assignments": shown[..2].iter().map(|item| (item["id"].as_str().expect("id").to_string(), category(item))).collect::<serde_json::Map<_, _>>() });
    assert_eq!(rejected(&base, &record(&ada, &timed, &systems, &partial, 44)), "task-unopened", "the clock starts before the first answer");
    let opened_at = now() + 120_000;
    let opening = accepted(&base, &open_task(&ada, &timed, &systems, opened_at, 45));
    assert_eq!((opening[0]["type"].as_str(), opening[0]["at"].as_u64()), (Some("task-opened"), Some(opened_at)), "the device keeps the time: an opening it dates two minutes ahead, within the lead, counts as it claims");
    assert_eq!(rejected(&base, &open_task(&ada, &timed, &systems, now(), 46)), "already-opened", "a task opens once; a repeat is refused and stores no receipt");
    let dated = |seed: u32, at: u64| {
        let mut sent = record(&ada, &timed, &systems, &partial, seed);
        sent["at"] = json!(at);
        sent
    };
    assert_eq!(accepted(&base, &dated(47, 0))[0]["at"].as_u64(), Some(opened_at), "an answer dated before its task opened counts from the opening");
    let limit = opened_at + systems["seconds"].as_u64().expect("seconds") * 1000;
    assert_eq!(accepted(&base, &dated(48, limit))[0]["at"].as_u64(), Some(limit), "an answer made at the last instant counts, whenever it arrives");
    assert_eq!(rejected(&base, &dated(49, limit + 1)), "time-up", "an answer made one millisecond later is refused, whenever it arrives");
    assert_eq!(run_view(&timed)["opened"], json!({ "systems": opened_at }));
    let expert_result = accepted(&base, &submit(&ada, &timed, 50))[0]["result"].clone();
    let unanswered = expert_result["tasks"][0]["items"].as_array().expect("items").iter().filter(|item| item.get("assigned").is_none()).count();
    let score = expert_result["score"].as_f64().expect("a score");
    assert!(expert_result["challenge"] == "expert" && unanswered == 2 && 0.0 < score && score < 1.0 && expert_result["points"].as_f64() == Some(score * 400.0), "a timed run is submitted with what was answered: {expert_result}");

    let learner = view(&base, &json!({ "type": "learner", "learner": ada }));
    let runs: Vec<(&str, &str, Option<f64>)> = learner["runs"].as_array().expect("runs").iter().map(|run| (run["challenge"].as_str().expect("challenge"), run["status"].as_str().expect("status"), run["points"].as_f64())).collect();
    assert_eq!(runs, [("expert", "submitted", expert_result["points"].as_f64()), ("medium", "voided", None), ("hard", "submitted", hard_result["points"].as_f64()), ("medium", "submitted", Some(100.0)), ("easy", "submitted", Some(100.0))]);
    let best = json!({ "power": { "challenge": "hard", "score": hard_result["score"], "points": hard_result["points"] }, "homes": { "challenge": "expert", "score": expert_result["score"], "points": expert_result["points"] } });
    let total = hard_result["points"].as_f64().expect("points") + expert_result["points"].as_f64().expect("points");
    assert_eq!((&learner["best"], learner["total"].as_f64()), (&best, Some(total)), "the best of a quiz is its run with the most points, a perfect easy run earns fewer than a good hard one");
    let board = view(&base, &json!({ "type": "leaderboard", "period": "all-time", "learner": ada }));
    assert_eq!((&board["own"]["best"], board["own"]["total"].as_f64()), (&best, Some(total)), "the board ranks by the same points: {board}");
    let (offline, device) = (id(0x106), now() - 600_000);
    let mut late = start(&ada, &offline, "power", "expert", 51);
    late["at"] = json!(device);
    assert_eq!(accepted(&base, &late)[0]["at"].as_u64(), Some(device), "a run started offline keeps the device's start, however late it arrives");
    let appliances = task_of(&run_view(&offline), "matching");
    assert_eq!(accepted(&base, &open_task(&ada, &offline, &appliances, 0, 52))[0]["at"].as_u64(), Some(device), "an opening is raised to the device's start, not to the delivery");
    let ranking = task_of(&run_view(&offline), "sorting");
    let sent = now();
    let lowered = accepted(&base, &open_task(&ada, &offline, &ranking, sent + 3_600_000, 53))[0]["at"].as_u64().expect("at");
    assert!(sent + quiz::CLOCK_LEAD <= lowered && lowered <= now() + quiz::CLOCK_LEAD, "an opening dated an hour ahead is lowered to five minutes past the proctor's clock, so it buys no time: {lowered}");
    let views =|base: &str| [json!({ "type": "learner", "learner": ada }), json!({ "type": "run", "run": easy }), json!({ "type": "run", "run": hard }), json!({ "type": "run", "run": timed }), json!({ "type": "leaderboard", "period": "all-time", "learner": ada })].map(|asked| view(base, &asked));
    let before = views(&base);
    running.shut_down();

    let running = boot(&data.0, development());
    assert_eq!(views(&running.base), before, "challenges, opened tasks, points and bests survive a restart");
    running.shut_down();
}

/// 🧾️ The receipts the proctor serving `data` keeps for `learner`: one per accepted command, none per refusal.
fn receipts(data: &Path, learner: &str) -> u64 {
    let database = proctor::storage::Database::open_read_only(data).expect("a read-only handle beside the serving proctor");
    database.erasure(&[proctor::actors::learner_key(TENANT, learner)]).expect("the rows of the learner").actors[0].receipts
}

/// 📐️ The concordance score of authored `values` in ascending order when only the pairs of items `known` keep their
/// weight: every pair touching another item is discordant, its weight the distance of the two values in decades.
fn concordance(values: &[f64], known: impl Fn(usize) -> bool) -> f64 {
    let pairs = (0..values.len()).flat_map(|left| (left + 1..values.len()).map(move |right| (left, right)));
    let (total, kept) = pairs.fold((0.0, 0.0), |(total, kept), (left, right)| {
        let weight = (values[right].log10() - values[left].log10()).abs();
        (total + weight, if known(left) && known(right) { kept + weight } else { kept })
    });
    kept / total
}

/// 🗳️ Scored items by id: the guess or assigned value and the miss mark.
type Scored = std::collections::BTreeMap<String, (Option<f64>, Option<bool>)>;

/// 🔎️ The [`Scored`] items of `task` in a run result, of the matching `dimension` when one is named.
fn scored(result: &Value, task: &str, dimension: Option<&str>) -> Scored {
    let task = result["tasks"].as_array().expect("tasks").iter().find(|scored| scored["task"] == task).expect("the scored task");
    let items = match dimension {
        Some(dimension) => &task["dimensions"].as_array().expect("dimensions").iter().find(|scored| scored["dimension"] == dimension).expect("the scored dimension")["items"],
        None => &task["items"],
    };
    items.as_array().expect("items").iter().map(|item| (item["item"].as_str().expect("item").to_string(), (item.get("guess").or_else(|| item.get("assigned")).and_then(Value::as_f64), item["miss"].as_bool()))).collect()
}

#[test]
fn expert_and_hard_runs_clock_guess_and_score_sortings_and_matchings_from_the_instants_the_device_claims() {
    let data = scratch("clock");
    let running = boot(&data.0, development());
    let base = running.base.clone();
    let ada = id(0xada);
    accepted(&base, &sign_up(1, &ada));
    let power = quiz_document("power");
    let run_view = |run: &str| view(&base, &json!({ "type": "run", "run": run }));

    let timed = id(0x100);
    let begun = start(&ada, &timed, "power", "expert", 10);
    let started_at = begun["at"].as_u64().expect("at");
    accepted(&base, &begun);
    let opened = run_view(&timed);
    let (sorting, matching) = (task_of(&opened, "sorting"), task_of(&opened, "matching"));
    assert_eq!((sorting["seconds"].as_u64(), matching["seconds"].as_u64()), (Some(30 + 12 * 4), Some(30 + 12 * 3 * 2)), "a sorting allows twelve seconds per item, a matching twelve per item and dimension");
    assert!(sorting.get("keys").is_none() && matching["dimensions"].as_array().expect("dimensions").iter().all(|dimension| dimension.get("cards").is_none()), "an expert sheet hides the keys: {opened}");

    let kept = receipts(&data.0, &ada);
    let mut malformed = open_task(&ada, &timed, &sorting, started_at, 11);
    for at in [json!(-1), json!(2.5), json!("soon"), Value::Null] {
        malformed["at"] = at.clone();
        assert!(rejected(&base, &malformed).starts_with("command-malformed"), "an opening dated {at} is no command");
    }
    malformed.as_object_mut().expect("a command").remove("at");
    assert!(rejected(&base, &malformed).starts_with("command-malformed"), "an opening is dated");
    for at in [json!(quiz::MAX_TIMESTAMP + 1), json!(u64::MAX)] {
        malformed["at"] = at.clone();
        assert!(rejected(&base, &malformed).starts_with("id-invalid"), "an opening dated {at}, beyond a safe integer, is refused");
    }
    malformed["at"] = json!(started_at);
    malformed["task"] = json!("Appliances");
    assert!(rejected(&base, &malformed).starts_with("id-invalid"), "a task is a slug");
    malformed.as_object_mut().expect("a command").remove("task");
    assert!(rejected(&base, &malformed).starts_with("command-malformed"), "an opening names its task");
    let mut undated = record(&ada, &timed, &sorting, &perfect(&power, &sorting), 12);
    for (at, refusal) in [(json!(-1), "command-malformed"), (json!(1.5), "command-malformed"), (json!(quiz::MAX_TIMESTAMP + 1), "id-invalid")] {
        undated["at"] = at.clone();
        assert!(rejected(&base, &undated).starts_with(refusal), "an answer dated {at} is refused: {refusal}");
    }
    assert_eq!(receipts(&data.0, &ada), kept, "a command refused at admission stores no receipt");

    let sorting_opened = started_at + 1_000;
    assert_eq!(accepted(&base, &open_task(&ada, &timed, &sorting, sorting_opened, 13))[0]["at"].as_u64(), Some(sorting_opened));
    assert_eq!(receipts(&data.0, &ada), kept + 1, "an opening stores its receipt");
    for repeat in 0..5u32 {
        assert_eq!(rejected(&base, &open_task(&ada, &timed, &sorting, sorting_opened + u64::from(repeat) * 7_000, 14 + repeat)), "already-opened");
    }
    assert_eq!(receipts(&data.0, &ada), kept + 1, "a repeated opening is refused, so the receipts do not grow");
    let mut partial = perfect(&power, &sorting);
    partial["guesses"] = json!({ "phone-charger": 5, "laptop": 50 });
    let sorting_limit = sorting_opened + 78_000;
    let mut sorted = record(&ada, &timed, &sorting, &partial, 20);
    sorted["at"] = json!(sorting_limit);
    assert_eq!(accepted(&base, &sorted)[0]["at"].as_u64(), Some(sorting_limit), "a partial sorting made at the last instant counts");
    let mut late = record(&ada, &timed, &sorting, &perfect(&power, &sorting), 21);
    late["at"] = json!(sorting_limit + 1);
    assert_eq!(rejected(&base, &late), "time-up", "the whole sorting made a millisecond later is refused");

    let matching_opened = started_at + 5_000;
    accepted(&base, &open_task(&ada, &timed, &matching, matching_opened, 22));
    let capacities = json!({ "kind": "matching", "guesses": { "capacity": perfect(&power, &matching)["guesses"]["capacity"].clone() } });
    let mut guessed = record(&ada, &timed, &matching, &capacities, 23);
    guessed["at"] = json!(matching_opened + 10_000);
    assert_eq!(accepted(&base, &guessed)[0]["at"].as_u64(), Some(matching_opened + 10_000), "the capacities guessed in time count");
    let mut late = record(&ada, &timed, &matching, &perfect(&power, &matching), 24);
    late["at"] = json!(matching_opened + 102_001);
    assert_eq!(rejected(&base, &late), "time-up", "the hours guessed after the matching's 102 seconds are refused");
    assert_eq!(run_view(&timed)["opened"], json!({ "appliances": sorting_opened, "sources": matching_opened }));

    let result = accepted(&base, &submit(&ada, &timed, 25))[0]["result"].clone();
    let missed = |guess: Option<f64>| (guess, Some(guess.is_none()));
    assert_eq!(scored(&result, "appliances", None), Scored::from([("phone-charger", Some(5.0)), ("laptop", Some(50.0)), ("kettle", None), ("heat-pump", None)].map(|(item, guess)| (item.to_string(), missed(guess)))), "an unguessed item misses: {result}");
    assert_eq!(scored(&result, "sources", Some("capacity")), Scored::from([("rooftop-pv", 1e4), ("wind-turbine", 3e6), ("nuclear-plant", 1.4e9)].map(|(item, guess)| (item.to_string(), (Some(guess), Some(false))))));
    assert_eq!(scored(&result, "sources", Some("hours")), Scored::from(["rooftop-pv", "wind-turbine", "nuclear-plant"].map(|item| (item.to_string(), (None, Some(true))))), "a dimension left unguessed misses everywhere");
    let sorting_score = concordance(&[5.0, 50.0, 2000.0, 3000.0], |index| index < 2);
    let score = result["score"].as_f64().expect("a score");
    assert!(result["challenge"] == "expert" && (score - (sorting_score + 0.5) / 2.0).abs() < 1e-12 && result["points"].as_f64() == Some(score * 400.0), "only the pair of the two guessed items keeps its weight, the matching scores a half: {result}");
    assert_eq!(rejected(&base, &open_task(&ada, &timed, &matching, matching_opened, 26)), "run-closed", "a submitted run opens nothing");

    let hard = id(0x101);
    accepted(&base, &start(&ada, &hard, "power", "hard", 30));
    let opened = run_view(&hard);
    let (sorting, matching) = (task_of(&opened, "sorting"), task_of(&opened, "matching"));
    assert_eq!(rejected(&base, &open_task(&ada, &hard, &sorting, now(), 31)), "run-untimed", "a hard run has no clock");
    let mut off = perfect(&power, &matching);
    off["guesses"]["capacity"]["rooftop-pv"] = json!(1e7);
    accepted(&base, &record(&ada, &hard, &sorting, &perfect(&power, &sorting), 32));
    accepted(&base, &record(&ada, &hard, &matching, &off, 33));
    let result = accepted(&base, &submit(&ada, &hard, 34))[0]["result"].clone();
    assert_eq!(scored(&result, "sources", Some("capacity")), Scored::from([("rooftop-pv", 1e7, true), ("wind-turbine", 3e6, false), ("nuclear-plant", 1.4e9, false)].map(|(item, guess, miss)| (item.to_string(), (Some(guess), Some(miss))))), "a capacity guessed a thousandfold misses: the reach is the root of the capacities' spread: {result}");
    assert!(scored(&result, "sources", Some("hours")).values().all(|(_, miss)| *miss == Some(false)) && scored(&result, "appliances", None).values().all(|(_, miss)| *miss == Some(false)));
    let capacity = concordance(&[1e4, 3e6, 1.4e9], |index| index > 0);
    let score = result["score"].as_f64().expect("a score");
    assert!(result["challenge"] == "hard" && (score - (1.0 + (capacity + 1.0) / 2.0) / 2.0).abs() < 1e-12 && result["points"].as_f64() == Some(score * 300.0), "the miss costs every pair it touches: {result}");
    running.shut_down();
}

#[test]
fn an_easy_run_questions_far_off_cards_and_misplaced_items_until_it_closes() {
    let data = scratch("hints");
    let running = boot(&data.0, development());
    let base = running.base.clone();
    let ada = id(0xada);
    accepted(&base, &sign_up(1, &ada));
    let (power, homes) = (quiz_document("power"), quiz_document("homes"));
    let run_view = |run: &str| view(&base, &json!({ "type": "run", "run": run }));

    let easy = id(0x100);
    accepted(&base, &start(&ada, &easy, "power", "easy", 10));
    let opened = run_view(&easy);
    let (sorting, matching) = (task_of(&opened, "sorting"), task_of(&opened, "matching"));
    let item = |task: &Value, id: &str| task["items"].as_array().expect("items").iter().find(|item| item["id"] == id).expect("a presented item").clone();
    assert_eq!((item(&matching, "nuclear-plant")["short"].clone(), matching["dimensions"][1]["quantity"]["short"].clone()), (json!({ "en": "Nuclear", "de": "AKW" }), json!({ "en": "hours", "de": "Stunden" })), "short labels reach the sheet for the hints to name");
    assert!(item(&sorting, "kettle").get("familiar").is_none() && !opened.to_string().contains("familiar"), "whether an item is familiar stays with the proctor: {sorting}");
    let mut misordered = perfect(&power, &sorting);
    let order = misordered["order"].as_array_mut().expect("order");
    order.swap(1, 3);
    assert_eq!(order, &[json!("phone-charger"), json!("heat-pump"), json!("kettle"), json!("laptop")]);
    accepted(&base, &record(&ada, &easy, &sorting, &misordered, 11));
    assert_eq!(
        run_view(&easy)["hints"]["appliances"],
        json!([
            { "kind": "compare", "item": "heat-pump", "other": "kettle", "factor": 50.0 / 2000.0, "verdict": "reversed" },
            { "kind": "compare", "item": "laptop", "other": "phone-charger", "factor": 3000.0 / 5.0, "verdict": "over" },
        ]),
        "the charger and the kettle on their own keys claim ×60 equally wrong for the heat pump and the laptop; the familiar kettle wins over the charger's smaller claim for the heat pump, and the laptop takes the charger the heat pump does not name"
    );
    accepted(&base, &record(&ada, &easy, &sorting, &perfect(&power, &sorting), 12));
    let mut swapped = perfect(&power, &matching);
    let capacity = swapped["assignments"]["capacity"].as_object_mut().expect("capacity cards");
    let (wind, nuclear) = (capacity["wind-turbine"].clone(), capacity["nuclear-plant"].clone());
    capacity.insert("wind-turbine".into(), nuclear);
    capacity.insert("nuclear-plant".into(), wind);
    let hours = swapped["assignments"]["hours"].as_object_mut().expect("hours cards");
    let (pv, nuclear) = (hours["rooftop-pv"].clone(), hours["nuclear-plant"].clone());
    hours.insert("rooftop-pv".into(), nuclear);
    hours.insert("nuclear-plant".into(), pv);
    accepted(&base, &record(&ada, &easy, &matching, &swapped, 13));
    let capacity = [
        json!({ "kind": "compare", "item": "nuclear-plant", "other": "rooftop-pv", "dimension": "capacity", "factor": 300.0, "verdict": "under" }),
        json!({ "kind": "compare", "item": "wind-turbine", "other": "rooftop-pv", "dimension": "capacity", "factor": 140_000.0, "verdict": "over" }),
    ];
    let place = |hint: &Value| matching["items"].as_array().expect("items").iter().position(|item| item["id"] == hint["item"]);
    let mut hours = [
        json!({ "kind": "compare", "item": "nuclear-plant", "other": "wind-turbine", "dimension": "hours", "difference": -1050.0, "verdict": "reversed" }),
        json!({ "kind": "compare", "item": "rooftop-pv", "other": "wind-turbine", "dimension": "hours", "difference": 5800.0, "verdict": "reversed" }),
    ];
    hours.sort_by_key(place);
    let first = capacity.iter().min_by_key(|hint| place(hint)).expect("a capacity hint").clone();
    assert_eq!(
        run_view(&easy)["hints"]["sources"],
        json!([first, hours[0], hours[1]]),
        "each far-off card is questioned against the item whose card is right: a nuclear plant only 300 times a rooftop understates, a wind turbine 140 000 times a rooftop overstates, full-load hours swapped point the other way; of the four the three most wrong stand — both of the hours, 6850 h off, and of the capacities, equally ×467 off, the first in sheet order — per dimension in sheet order"
    );
    assert_eq!(run_view(&easy)["hints"].as_object().map(|hinted| hinted.keys().cloned().collect::<Vec<_>>()), Some(vec!["sources".to_string()]), "a right sorting earns no hint beside the matching's");
    let result = accepted(&base, &submit(&ada, &easy, 14))[0]["result"].clone();
    let closed = run_view(&easy);
    assert!(closed.get("hints").is_none() && closed["status"] == "submitted", "a submitted run hints at nothing: {closed}");
    assert_eq!(scored(&result, "sources", Some("capacity")).values().filter(|(_, miss)| miss.is_some()).count(), 0, "where the keys show an assignment carries no miss mark: {result}");

    let classified = id(0x101);
    accepted(&base, &start(&ada, &classified, "homes", "easy", 20));
    let systems = task_of(&run_view(&classified), "classification");
    let place = |hint: &Value| systems["items"].as_array().expect("items").iter().position(|item| item["id"] == hint["item"]);
    let in_sheet_order = |mut hints: Vec<Value>| {
        hints.sort_by_key(place);
        json!({ "systems": hints })
    };
    let misplaced = |moves: &[(&str, &str)]| {
        let mut answer = perfect(&homes, &systems);
        for (item, category) in moves {
            answer["assignments"][*item] = json!(category);
        }
        answer
    };
    accepted(&base, &record(&ada, &classified, &systems, &misplaced(&[("air-to-water", "gas-boiler"), ("pellets", "heat-pump")]), 21));
    assert_eq!(
        run_view(&classified)["hints"],
        in_sheet_order(vec![json!({ "kind": "profile", "item": "air-to-water", "category": "gas-boiler", "axis": "efficiency" }), json!({ "kind": "profile", "item": "pellets", "category": "heat-pump", "axis": "efficiency", "other": "condensing", "above": true })]),
        "a misplaced item is asked about the profile on the axis farthest off relative to its reach (a tie takes the first axis): the pellets as a heat pump would lie above the condensing boiler, rightly placed, though they lie below it; the air-to-water unit as a gas boiler has nobody in between"
    );
    let systems_axes = systems["axes"].as_array().expect("axes");
    assert_eq!((systems_axes[1]["short"].clone(), systems["items"].as_array().expect("items").iter().find(|item| item["id"] == "air-to-water").expect("the unit")["short"].clone()), (json!({ "en": "cost", "de": "Kosten" }), json!({ "en": "Air unit", "de": "Luftgerät" })));
    accepted(&base, &record(&ada, &classified, &systems, &misplaced(&[("air-to-water", "wood-stove")]), 26));
    assert_eq!(run_view(&classified)["hints"], json!({ "systems": [{ "kind": "profile", "item": "air-to-water", "category": "wood-stove", "axis": "efficiency", "other": "condensing", "above": false }] }), "as a wood stove the heat pump would lie below the condensing boiler");
    accepted(&base, &record(&ada, &classified, &systems, &misplaced(&[("air-to-water", "gas-boiler"), ("pellets", "district-heating")]), 22));
    assert_eq!(run_view(&classified)["hints"], in_sheet_order(vec![json!({ "kind": "profile", "item": "air-to-water", "category": "gas-boiler", "axis": "efficiency" }), json!({ "kind": "category", "item": "pellets", "category": "district-heating" })]), "district heating has no profile, and nobody else stands there or belongs with the pellets: the category itself is questioned");
    accepted(&base, &record(&ada, &classified, &systems, &misplaced(&[("condensing", "district-heating"), ("pellets", "district-heating")]), 23));
    assert_eq!(run_view(&classified)["hints"], in_sheet_order(vec![json!({ "kind": "group", "item": "condensing", "other": "pellets", "together": true }), json!({ "kind": "group", "item": "pellets", "other": "condensing", "together": true })]), "two of different systems put together");
    accepted(&base, &record(&ada, &classified, &systems, &misplaced(&[("ground-source", "district-heating")]), 24));
    assert_eq!(run_view(&classified)["hints"], json!({ "systems": [{ "kind": "group", "item": "ground-source", "other": "air-to-water", "together": false }] }), "two heat pumps put apart");
    let switched = accepted(&base, &start(&ada, &id(0x102), "homes", "medium", 25));
    assert_eq!(switched[0]["type"], "run-voided");
    let voided = run_view(&classified);
    assert!(voided.get("hints").is_none() && voided["status"] == "voided", "a voided run hints at nothing: {voided}");
    running.shut_down();
}

#[test]
fn switching_the_challenge_back_and_forth_ends_in_runs_exhausted() {
    let data = scratch("switching");
    let running = boot(&data.0, development());
    let base = running.base.clone();
    let grace = id(0x9ace);
    accepted(&base, &sign_up(1, &grace));
    let per_quiz = 200u32;
    for started in 0..per_quiz {
        let challenge = if started.is_multiple_of(2) { "easy" } else { "medium" };
        let events = accepted(&base, &start(&grace, &id(0x1_0000 + started), "power", challenge, 0x2_0000 + started));
        assert_eq!(events.iter().map(|event| event["type"].as_str().expect("type")).collect::<Vec<_>>(), if started == 0 { vec!["run-started"] } else { vec!["run-voided", "run-started"] }, "start {started}");
    }
    assert_eq!(rejected(&base, &start(&grace, &id(0x3_0000), "power", "hard", 0x3_0001)), "runs-exhausted", "every started run counts against the cap of the quiz, the voided ones too");
    assert_eq!(rejected(&base, &start(&grace, &id(0x3_0002), "power", "medium", 0x3_0003)), "run-open", "the open run is still resumed");
    let learner = view(&base, &json!({ "type": "learner", "learner": grace }));
    let statuses: Vec<&str> = learner["runs"].as_array().expect("runs").iter().map(|run| run["status"].as_str().expect("status")).collect();
    assert_eq!((statuses.len(), statuses.iter().filter(|status| **status == "voided").count(), statuses[0]), (per_quiz as usize, per_quiz as usize - 1, "open"), "the learner's runs stop growing at the cap");
    assert_eq!(accepted(&base, &start(&grace, &id(0x3_0004), "homes", "hard", 0x3_0005))[0]["type"], "run-started", "the cap is per quiz");
    running.shut_down();
}
