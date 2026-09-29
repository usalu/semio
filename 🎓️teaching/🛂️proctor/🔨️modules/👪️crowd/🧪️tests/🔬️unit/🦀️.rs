use super::*;
use crate::catalog::tests::fixture;
use quiz::{ClassificationItemResult, DimensionResult, MatchingItemResult, SortingItemResult};

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

/// 🎰️ One plausible, sometimes malformed result of `quiz`: tasks in random order, some left out,
/// some repeated or of another kind, items repeated, unknown ids sprinkled in.
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
            let values = [0.0, 0.1, 0.2, 0.3, 5.0, 50.0, 120.0, 2000.0, 1e21, 1.5e-7];
            tasks.push(match kind {
                0 => TaskResult::Classification {
                    task: task.id().clone(),
                    score: 0.5,
                    items: chosen.iter().map(|item| ClassificationItemResult { item: item.clone(), assigned: dice.pick(&categories).clone(), correct: categories[0].clone(), credit: 1.0, explanation: None }).collect(),
                },
                1 => TaskResult::Sorting {
                    task: task.id().clone(),
                    score: 0.5,
                    items: chosen.iter().enumerate().map(|(position, item)| SortingItemResult { item: item.clone(), value: 1.0, position: (position + dice.below(2)) % chosen.len(), rank: position, explanation: None }).collect(),
                },
                _ => TaskResult::Matching {
                    task: task.id().clone(),
                    score: 0.5,
                    dimensions: (0..1 + dice.below(dimensions.len() + 1))
                        .map(|_| DimensionResult { dimension: dice.pick(&dimensions).clone(), score: 0.5, items: chosen.iter().map(|item| MatchingItemResult { item: item.clone(), assigned: values[dice.below(values.len())], correct: 1.0, explanation: None }).collect() })
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
    RunResult { quiz: quiz.id.clone(), score: 0.5, tasks: shuffled }
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
        tally.fold(&submitted);
        tallies.insert(quiz.id.clone(), tally);
        results.push(submitted);
        for quiz in &quizzes {
            let tally = tallies.get(&quiz.id).cloned().unwrap_or_default();
            let oracle = quiz::crowd_view(quiz, &results);
            assert_eq!(tally.view(quiz), oracle, "round {round}, quiz {}", quiz.id);
            assert_eq!(serde_json::to_vec(&tally.view(quiz)).unwrap(), serde_json::to_vec(&oracle).unwrap(), "round {round}");
        }
    }
    let power = tallies.get("power").expect("power was played");
    assert!(power.sorting["appliances"].values().any(|positions| positions.answers > 1), "the sorting sums were exercised");
    assert!(power.matching["sources"].values().flat_map(|items| items.values()).any(|counts| counts.contains_key("1e+21")), "values are keyed in JSON number syntax");
}

#[test]
fn an_empty_tally_views_every_task_with_nobody_answering() {
    let catalog = fixture();
    let power = &catalog.entry("power").expect("power").quiz;
    let view = CrowdTally::default().view(power);
    assert_eq!(view, quiz::crowd_view::<RunResult>(power, &[]));
    assert_eq!(view.runs, 0);
    assert_eq!(view.tasks.iter().map(|task| (task.task.as_str(), task.dimension.as_deref(), task.items.len())).collect::<Vec<_>>(), [("appliances", None, 0), ("sources", Some("capacity"), 0), ("sources", Some("hours"), 0)]);
}
