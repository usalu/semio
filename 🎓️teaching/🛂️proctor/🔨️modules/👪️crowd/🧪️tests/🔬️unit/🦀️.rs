use super::*;
use crate::catalog::tests::fixture;
use quiz::{Challenge, ClassificationItemResult, DimensionResult, MatchingItemResult, SortingItemResult, CHALLENGES, CROWD_SCORE_BINS};

/// 🎲️ A small deterministic generator (SplitMix64), so every run folds the same results.
struct Dice(u64);

impl Dice {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut mixed = self.0;
        mixed = (mixed ^ (mixed >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        mixed = (mixed ^ (mixed >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        mixed ^ (mixed >> 31)
    }

    fn below(&mut self, bound: usize) -> usize {
        (self.next() % bound as u64) as usize
    }

    fn pick<'a>(&mut self, from: &'a [String]) -> &'a String {
        &from[self.below(from.len())]
    }
}

/// 🧪️ The ids a task's results may name: its own, plus a ghost nobody defined.
fn ids(own: impl Iterator<Item = String>) -> Vec<String> {
    own.chain(["ghost".to_string()]).collect()
}

/// 🎯️ A score on or beside a bin edge, or any thousandth.
fn score(dice: &mut Dice) -> f64 {
    let edges = [0.0, 0.0949, 0.095, 0.1, 0.8949, 0.895, 0.9, 0.995, 1.0];
    if dice.below(3) == 0 { edges[dice.below(edges.len())] } else { dice.below(1001) as f64 / 1000.0 }
}

/// ✂️ `quiz` with every task drawing `draw` items.
fn drawing(quiz: &Quiz, draw: Option<usize>) -> Quiz {
    let mut quiz = quiz.clone();
    for task in &mut quiz.tasks {
        match task {
            Task::Classification(task) => task.draw = draw,
            Task::Sorting(task) => task.draw = draw,
            Task::Matching(task) => task.draw = draw,
        }
    }
    quiz
}

/// 🎰️ One plausible, sometimes malformed result of `quiz` at any challenge: tasks in random order,
/// some left out, some repeated or of another kind, items repeated, orders of every length, unknown
/// ids sprinkled in, every score anywhere between 0 and 1; items assigned a key, guessed (an authored
/// value, one between two, one far off) or left unanswered, and sortings nobody guessed in.
fn result(quiz: &Quiz, dice: &mut Dice) -> RunResult {
    let mut tasks = Vec::new();
    for task in &quiz.tasks {
        for _ in 0..[0, 1, 1, 1, 2][dice.below(5)] {
            let own = match task.kind() {
                TaskKind::Classification => 0,
                TaskKind::Sorting => 1,
                TaskKind::Matching => 2,
            };
            let kind = if dice.below(10) == 0 { dice.below(3) } else { own };
            let items: Vec<String> = ids(match task {
                Task::Classification(task) => task.items.iter().map(|item| item.id.clone()).collect::<Vec<_>>().into_iter(),
                Task::Sorting(task) => task.items.iter().map(|item| item.id.clone()).collect::<Vec<_>>().into_iter(),
                Task::Matching(task) => task.items.iter().map(|item| item.id.clone()).collect::<Vec<_>>().into_iter(),
            });
            let chosen: Vec<String> = (0..1 + dice.below(items.len() + 1)).map(|_| dice.pick(&items).clone()).collect();
            let categories = ids(match task {
                Task::Classification(task) => task.categories.iter().map(|category| category.id.clone()).collect::<Vec<_>>().into_iter(),
                _ => vec!["heat-pump".to_string()].into_iter(),
            });
            let dimensions = ids(match task {
                Task::Matching(task) => task.dimensions.iter().map(|dimension| dimension.id.clone()).collect::<Vec<_>>().into_iter(),
                _ => vec!["capacity".to_string()].into_iter(),
            });
            let values = [0.0, 0.1, 0.2, 0.3, 5.0, 50.0, 120.0, 1475.0, 2000.0, 1e4, 173_205.0, 3e6, 1.4e9, 1e21, 1.5e-7];
            let guessing = dice.below(3);
            tasks.push(match kind {
                0 => TaskResult::Classification {
                    task: task.id().clone(),
                    score: score(dice),
                    items: chosen.iter().map(|item| ClassificationItemResult { item: item.clone(), assigned: (dice.below(5) > 0).then(|| dice.pick(&categories).clone()), correct: categories[0].clone(), credit: 1.0, explanation: None }).collect(),
                },
                1 => TaskResult::Sorting {
                    task: task.id().clone(),
                    score: score(dice),
                    items: chosen
                        .iter()
                        .enumerate()
                        .map(|(position, item)| {
                            let guess = (guessing == 1 && dice.below(4) > 0).then(|| values[dice.below(values.len())]);
                            let miss = match guessing {
                                0 => None,
                                1 => Some(guess.is_none() || dice.below(2) == 0),
                                _ => Some(true),
                            };
                            SortingItemResult { item: item.clone(), value: 1.0, position: (position + dice.below(2)) % chosen.len(), rank: position, guess, miss, explanation: None }
                        })
                        .collect(),
                },
                _ => TaskResult::Matching {
                    task: task.id().clone(),
                    score: score(dice),
                    dimensions: (0..1 + dice.below(dimensions.len() + 1))
                        .map(|_| DimensionResult {
                            dimension: dice.pick(&dimensions).clone(),
                            score: score(dice),
                            items: chosen.iter().map(|item| MatchingItemResult { item: item.clone(), assigned: (dice.below(5) > 0).then(|| values[dice.below(values.len())]), correct: 1.0, miss: (guessing > 0).then(|| dice.below(2) == 0), explanation: None }).collect(),
                        })
                        .collect(),
                },
            });
        }
    }
    let shuffled = (0..tasks.len()).rev().fold(tasks, |mut tasks, index| {
        let other = dice.below(index + 1);
        tasks.swap(index, other);
        tasks
    });
    let (challenge, score) = (CHALLENGES[dice.below(CHALLENGES.len())], score(dice));
    RunResult { quiz: quiz.id.clone(), challenge, score, points: quiz::points(score, challenge), tasks: shuffled }
}

#[test]
fn a_tally_folded_one_result_at_a_time_is_the_cores_crowd_view_bit_for_bit() {
    let catalog = fixture();
    let quizzes: Vec<&Quiz> = catalog.entries.iter().map(|entry| &entry.quiz).collect();
    let mut dice = Dice(0x5eed);
    let mut results: Vec<RunResult> = Vec::new();
    let mut tallies: BTreeMap<String, CrowdTally> = BTreeMap::new();
    for round in 0..300 {
        let quiz = quizzes[dice.below(quizzes.len())];
        let submitted = result(quiz, &mut dice);
        let stored = serde_json::to_vec(&tallies.get(&quiz.id).cloned().unwrap_or_default()).expect("a tally encodes");
        let mut tally: CrowdTally = serde_json::from_slice(&stored).expect("a stored tally decodes");
        tally.fold(quiz, &submitted);
        tallies.insert(quiz.id.clone(), tally);
        results.push(submitted);
        for quiz in &quizzes {
            let tally = tallies.get(&quiz.id).cloned().unwrap_or_default();
            for quiz in [(*quiz).clone(), drawing(quiz, Some(2)), drawing(quiz, Some(3))] {
                let oracle = quiz::crowd_view(&quiz, &results);
                assert_eq!(tally.view(&quiz), oracle, "round {round}, quiz {}", quiz.id);
                assert_eq!(serde_json::to_vec(&tally.view(&quiz)).unwrap(), serde_json::to_vec(&oracle).unwrap(), "round {round}");
            }
        }
    }
    let power = tallies.get("power").expect("power was played");
    let appliances = &power.sorting["appliances"];
    assert!(appliances.items.values().any(|positions| positions.answers > 1), "the sorting sums were exercised");
    assert!(appliances.items.values().any(|positions| positions.orders.len() > 3 && positions.orders.contains_key(&1)), "orders of many lengths were placed, one-item orders too");
    assert!(power.matching["sources"].values().flat_map(|tally| tally.items.values()).any(|counts| counts.contains_key("1e+21")), "values are keyed in JSON number syntax");
    let spread = |scores: &CrowdScores| scores.iter().filter(|count| **count > 0).count();
    assert!(spread(&power.scores) == CROWD_SCORE_BINS && spread(&appliances.scores) > 5 && power.matching["sources"].values().all(|tally| spread(&tally.scores) > 5), "the scores fell into bins all over the range");
    assert_eq!((power.scores.iter().sum::<usize>(), power.runs), (power.runs, results.iter().filter(|result| result.quiz == "power").count()));
}

#[test]
fn a_tally_keeps_the_orders_and_the_view_bins_them_against_the_places_the_quiz_presents_now() {
    let catalog = fixture();
    let power = &catalog.entry("power").expect("power").quiz;
    let Some(Task::Sorting(appliances)) = power.tasks.iter().find(|task| task.id() == "appliances") else { panic!("appliances is a sorting") };
    let ids: Vec<String> = appliances.items.iter().map(|item| item.id.clone()).collect();
    let ordered = |order: &[&String], score: f64| RunResult { quiz: "power".to_string(), challenge: Challenge::Medium, score, points: score * 200.0, tasks: vec![TaskResult::Sorting { task: "appliances".to_string(), score, items: order.iter().enumerate().map(|(position, item)| SortingItemResult { item: (*item).clone(), value: 1.0, position, rank: position, guess: None, miss: None, explanation: None }).collect() }] };
    let mut tally = CrowdTally::default();
    for (order, score) in [(vec![&ids[0]], 0.095), (vec![&ids[1], &ids[0]], 0.1), (vec![&ids[1], &ids[0], &ids[2]], 0.895), (ids.iter().rev().collect(), 1.0)] {
        tally.fold(power, &ordered(&order, score));
    }
    let first = &tally.sorting["appliances"].items[&ids[0]];
    assert_eq!((first.answers, &first.orders), (4, &BTreeMap::from([(1, BTreeMap::from([(0, 1)])), (2, BTreeMap::from([(1, 1)])), (3, BTreeMap::from([(1, 1)])), (ids.len(), BTreeMap::from([(ids.len() - 1, 1)]))])));
    assert_eq!((tally.scores, tally.sorting["appliances"].scores), ([0, 2, 0, 0, 0, 0, 0, 0, 0, 2], [0, 2, 0, 0, 0, 0, 0, 0, 0, 2]));
    let places = |draw: Option<usize>| tally.view(&drawing(power, draw)).tasks.iter().find(|task| task.task == "appliances").and_then(|task| task.items[0].places.clone());
    assert_eq!((places(Some(2)), places(Some(3))), (Some(vec![1, 3]), Some(vec![1, 1, 2])));
    assert_eq!(places(None).map(|places| (places.len(), places[0], places[ids.len() - 1], places.iter().sum::<usize>())), Some((ids.len(), 1, 2, 4)));
}

#[test]
fn a_guess_counts_under_the_nearest_authored_value_and_what_is_unanswered_counts_nowhere() {
    let catalog = fixture();
    let (power, homes) = (&catalog.entry("power").expect("power").quiz, &catalog.entry("homes").expect("homes").quiz);
    let matched = |item: &str, assigned: Option<f64>, miss: Option<bool>| MatchingItemResult { item: item.into(), assigned, correct: 1.0, miss, explanation: None };
    let sorted = |item: &str, position: usize| SortingItemResult { item: item.into(), value: 1.0, position, rank: position, guess: None, miss: Some(true), explanation: None };
    let guessed = RunResult {
        quiz: "power".into(),
        challenge: Challenge::Hard,
        score: 0.25,
        points: 75.0,
        tasks: vec![
            TaskResult::Sorting { task: "appliances".into(), score: 0.0, items: vec![sorted("kettle", 0), sorted("laptop", 1)] },
            TaskResult::Matching {
                task: "sources".into(),
                score: 0.5,
                dimensions: vec![
                    DimensionResult { dimension: "capacity".into(), score: 0.5, items: vec![matched("rooftop-pv", Some(2e4), Some(false)), matched("wind-turbine", Some(1e21), Some(true))] },
                    DimensionResult { dimension: "hours".into(), score: 0.5, items: vec![matched("rooftop-pv", Some(1475.0), Some(true)), matched("wind-turbine", Some(2100.0), Some(false)), matched("nuclear-plant", None, Some(true))] },
                ],
            },
        ],
    };
    let mut tally = CrowdTally::default();
    tally.fold(power, &guessed);
    let counts = |dimension: &str| tally.matching["sources"][dimension].items.iter().map(|(item, counts)| (item.as_str(), counts.keys().map(String::as_str).collect::<Vec<_>>())).collect::<Vec<_>>();
    assert_eq!(counts("capacity"), [("rooftop-pv", vec!["10000"]), ("wind-turbine", vec!["1400000000"])], "a guess counts under the authored value nearest on the logarithmic scale");
    assert_eq!(counts("hours"), [("rooftop-pv", vec!["950"]), ("wind-turbine", vec!["2000"])], "halfway between two values counts under the smaller; the unanswered plant counts nowhere");
    assert_eq!((tally.sorting["appliances"].items.len(), tally.sorting["appliances"].scores[0]), (0, 1), "a sorting nobody guessed in adds its score only");
    assert_eq!(tally.view(power), quiz::crowd_view(power, &[&guessed]));
    let unanswered = RunResult {
        quiz: "homes".into(),
        challenge: Challenge::Expert,
        score: 0.5,
        points: 200.0,
        tasks: vec![TaskResult::Classification { task: "systems".into(), score: 0.5, items: vec![ClassificationItemResult { item: "air-to-water".into(), assigned: None, correct: "heat-pump".into(), credit: 0.0, explanation: None }, ClassificationItemResult { item: "pellets".into(), assigned: Some("wood-stove".into()), correct: "wood-stove".into(), credit: 1.0, explanation: None }] }],
    };
    let mut timed = CrowdTally::default();
    timed.fold(homes, &unanswered);
    assert_eq!(timed.classification["systems"].items.keys().collect::<Vec<_>>(), ["pellets"]);
    assert_eq!(timed.view(homes), quiz::crowd_view(homes, &[&unanswered]));
}

#[test]
fn an_empty_tally_views_every_task_with_nobody_answering() {
    let catalog = fixture();
    let power = &catalog.entry("power").expect("power").quiz;
    let view = CrowdTally::default().view(power);
    assert_eq!(view, quiz::crowd_view::<RunResult>(power, &[]));
    assert_eq!((view.runs, view.scores), (0, [0; CROWD_SCORE_BINS]));
    assert_eq!(view.tasks.iter().map(|task| (task.task.as_str(), task.dimension.as_deref(), task.items.len(), task.scores)).collect::<Vec<_>>(), [("appliances", None, 0, [0; CROWD_SCORE_BINS]), ("sources", Some("capacity"), 0, [0; CROWD_SCORE_BINS]), ("sources", Some("hours"), 0, [0; CROWD_SCORE_BINS])]);
}
