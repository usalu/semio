use super::super::{RemodelingEntity, RemodelingRows};
use super::*;
use semio_framework_value::retirement::controlled::ControlledRetirement;

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
struct Record {
    id: String,
    value: u32,
}

impl protocol::RetainedKeyedRow for Record {
    fn retained_key(&self) -> &String {
        &self.id
    }
}
impl RemodelingEntity<RemodelingNoPatch> for Record {
    fn key(&self) -> String {
        self.id.clone()
    }
    fn write_patch(&mut self, _: &RemodelingNoPatch) -> Result<(), protocol::MutationApplyError> {
        Ok(())
    }
    fn restore_patch(&self, _: &RemodelingNoPatch) -> RemodelingNoPatch {
        RemodelingNoPatch {}
    }
    fn merge_patch(_: &mut RemodelingNoPatch, _: RemodelingNoPatch) {}
}
fn grant(d: RetirementDemand) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: d.copy_bytes, maximum_capacity_bytes: d.capacity_bytes, maximum_release_bytes: d.release_bytes, maximum_depth: d.depth }
}
fn observe<T>(action: impl FnOnce() -> T) -> (T, usize, usize) {
    semio_framework_trace_memory_testing::observe_backing(action)
}
fn record(value: &serde_json::Value) -> Record {
    Record { id: value["id"].as_str().unwrap().to_owned(), value: u32::try_from(value["value"].as_u64().unwrap()).unwrap() }
}
fn row(value: &serde_json::Value) -> RemodelingRow<Record, RemodelingNoPatch> {
    match value["row"].as_str().unwrap() {
        "insert" => RemodelingRow::Insert { entity: record(&value["entity"]) },
        "replace" => RemodelingRow::Replace { entity: record(&value["entity"]) },
        "remove" => RemodelingRow::Remove { key: value["key"].as_str().unwrap().to_owned() },
        "patch" => RemodelingRow::Patch { key: value["key"].as_str().unwrap().to_owned(), patch: RemodelingNoPatch {} },
        _ => panic!("unrecognized neutral row"),
    }
}
fn retire<T: RetireOwned>(value: T) {
    let mut owner = ControlledRetirement::new(value).unwrap();
    for _ in 0..4096 {
        if owner.terminal_is_empty() {
            return;
        }
        let selected = grant(owner.next_demand(4096).unwrap());
        let (step, born, released) = observe(|| owner.step(selected).unwrap());
        assert!(step.progress().fits(selected));
        assert_eq!((born, released), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
    }
    panic!("returned row value did not retire");
}
fn close(edit: &mut OwnedRowEdit<Record>) {
    edit.begin_close();
    for _ in 0..4096 {
        if edit.terminal_is_empty() {
            return;
        }
        let selected = grant(edit.close_demand(4096).unwrap());
        let work = edit.close_work_demand(4096).unwrap();
        let (denied, born, released) = observe(|| edit.close_step(RetainedCloneGrant::default()));
        assert_eq!((born, released), (0, 0));
        assert_eq!(denied.work, 0);
        assert_eq!(denied.result.unwrap().progress(), Default::default());
        let (turn, born, released) = observe(|| edit.close_step(selected));
        assert!(turn.work <= work);
        let progress = turn.result.unwrap().progress();
        assert!(progress.fits(selected));
        assert_eq!((born, released), (progress.retained_capacity_bytes, progress.released_bytes));
    }
    panic!("owned row edit did not close");
}

fn run(case: &serde_json::Value, cut: Option<(usize, usize)>) -> Vec<usize> {
    let rows = case["rows"].as_array().unwrap().iter().map(row).collect::<Vec<_>>();
    let base = case["base"].as_array().unwrap().iter().map(record).collect::<Vec<_>>();
    let reference = RemodelingRows { rows: rows.clone() }.apply_rows(&base);
    let mut input = Vec::with_capacity(base.len() + rows.len());
    input.extend(base);
    let mut input = Some(input);
    let mut counts = Vec::new();
    let mut refusal = None;
    for (index, row) in rows.into_iter().enumerate() {
        let mut edit = OwnedRowEdit::new(input.take().unwrap(), row);
        let mut turns = 0;
        while edit.status() == RowEditStatus::Pending && cut.is_none_or(|(row, turn)| row != index || turns < turn) {
            let demand = edit.demand(64).unwrap();
            let work = edit.work_demand(64).unwrap();
            let selected = grant(demand);
            for denied in [
                Some(RetainedCloneGrant::default()),
                Some(RetainedCloneGrant { maximum_items: 0, ..selected }),
                (demand.copy_bytes > 0).then_some(RetainedCloneGrant { maximum_copy_bytes: demand.copy_bytes - 1, ..selected }),
                (demand.capacity_bytes > 0).then_some(RetainedCloneGrant { maximum_capacity_bytes: demand.capacity_bytes - 1, ..selected }),
                (demand.release_bytes > 0).then_some(RetainedCloneGrant { maximum_release_bytes: demand.release_bytes - 1, ..selected }),
                Some(RetainedCloneGrant { maximum_depth: 0, ..selected }),
            ]
            .into_iter()
            .flatten()
            {
                let (turn, born, released) = observe(|| edit.advance(64, denied));
                assert_eq!((born, released), (0, 0));
                assert_eq!(turn.work, 0);
                assert_eq!(turn.result.unwrap().progress(), Default::default());
            }
            let (turn, born, released) = observe(|| edit.advance(0, selected));
            assert_eq!((born, released), (0, 0));
            assert_eq!(turn.work, 0);
            assert_eq!(turn.result.unwrap().progress(), Default::default());
            let (turn, born, released) = observe(|| edit.advance(64, selected));
            assert!(turn.work <= work);
            let progress = turn.result.unwrap().progress();
            assert!(progress.fits(selected));
            assert_eq!((born, released), (progress.retained_capacity_bytes, progress.released_bytes));
            turns += 1;
            assert!(turns < 4096);
        }
        counts.push(turns);
        let cancelled = cut.is_some_and(|(row, _)| row == index);
        if !cancelled {
            match edit.status() {
                RowEditStatus::Ready => {
                    let selected = grant(edit.take_demand().unwrap());
                    let (turn, born, released) = observe(|| edit.take(selected));
                    assert_eq!((born, released), (0, 0));
                    let (value, progress) = turn.result.unwrap().unwrap();
                    assert!(progress.fits(selected));
                    input = Some(value);
                }
                RowEditStatus::Refused(kind) => {
                    refusal = Some(kind);
                }
                RowEditStatus::Pending => panic!("uncancelled row did not settle"),
            }
        }
        close(&mut edit);
        if cancelled || refusal.is_some() {
            break;
        }
    }
    if cut.is_none() {
        if let Some(refusal) = refusal {
            assert!(reference.is_err());
            assert_eq!(refusal.code(), case["refused"].as_str().unwrap());
        } else {
            let actual = input.as_ref().unwrap();
            let expected = case["expected"].as_array().unwrap().iter().map(record).collect::<Vec<_>>();
            assert_eq!(actual, &expected);
            assert_eq!(actual, &reference.unwrap());
        }
    }
    if let Some(value) = input {
        retire(value);
    }
    counts
}

/// 🧱️ Actual owned row edits match portable sequential first-match semantics and return every cancelled frontier.
#[test]
fn owned_row_edits_match_neutral_sequences_and_paid_cancellation() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️tests/🧫️owned-rows-edit/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let counts = run(case, None);
        for (row, count) in counts.into_iter().enumerate() {
            for cut in 0..=count {
                run(case, Some((row, cut)));
            }
        }
    }
}
