use super::*;

fn contract()->serde_json::Value{serde_json::from_str(include_str!("../🔣️.json")).expect("the quiet slice contract fixture")}

/// ⚖️ LAW: the shared slice constant is the fixture's slice, and the predicate is open exactly on the fixture rows.
#[test]
fn the_quiet_slice_closes_on_a_host_bound_output_a_yield_a_lost_clock_or_the_slice(){
 let contract=contract();
 assert_eq!(contract["sliceUs"].as_u64().unwrap(),QUIET_TURN_SLICE_US);
 for row in contract["open"].as_array().unwrap(){
  let executing_us=row["executingUs"].as_u64();
  assert_eq!(quiet_slice_open(row["hostBoundGrew"].as_bool().unwrap(),row["yielding"].as_bool().unwrap(),executing_us),row["open"].as_bool().unwrap(),"{}",row["name"]);
 }
}

/// 🔁️ Turns a model turn loop spends on `units` quiet units of `unit_us` each: it steps while the slice is open and the old one-step pacing is the slice of zero.
fn turns_for(units:u64,unit_us:u64,slice_open:impl Fn(Option<u64>)->bool)->u64{
 let(mut remaining,mut turns)=(units,0);
 while remaining>0{
  turns+=1;
  let mut executing_us=0;
  loop{
   remaining-=1;
   executing_us+=unit_us;
   if remaining==0||!slice_open(Some(executing_us)){break}
  }
 }
 turns
}

/// ⚖️ LAW: quiet units share turns by the slice, a unit longer than the slice still advances one per turn, and the old pacing spent one turn per unit.
///
/// 🐛️ Before the slice every unit that carried a receipt ended its turn: 10 048 units were 10 048 turns, the measured FEM 2D selection.
#[test]
fn quiet_units_share_turns_by_the_slice_where_the_one_step_pacing_spent_a_turn_each(){
 for row in contract()["drain"].as_array().unwrap(){
  let(units,unit_us,turns)=(row["units"].as_u64().unwrap(),row["unitUs"].as_u64().unwrap(),row["turns"].as_u64().unwrap());
  assert_eq!(turns_for(units,unit_us,|executing_us|quiet_slice_open(false,false,executing_us)),turns,"{}",row["name"]);
  assert_eq!(turns_for(units,unit_us,|_|false),units,"{}: the one-step pacing",row["name"]);
 }
}

#[test]
fn opening_reuses_the_slice_only_while_the_same_pending_owner_makes_paid_progress() {
    for case in contract()["opening"].as_array().unwrap() {
        let pacing = if case["metered"].as_bool() == Some(true) { OpeningPacing::Metered } else { OpeningPacing::Interactive };
        let mut paid = 0;
        let mut attempts = 0;
        let mut captures = 0;
        for step in case["steps"].as_array().unwrap() {
            attempts += 1;
            let previous = paid;
            paid += step["paid"].as_u64().unwrap();
            let ready = step["ready"].as_bool().unwrap();
            captures += u64::from(ready);
            if !opening_slice_open(pacing, ready, paid != previous, step["cancelled"].as_bool().unwrap(), step["yielding"].as_bool().unwrap(), step["elapsedUs"].as_u64()) {
                break;
            }
        }
        assert_eq!(serde_json::json!([attempts, paid, captures]), case["outcome"], "{}", case["name"]);
    }
}

#[test]
fn live_archive_burst_requires_a_funded_owner_and_a_quiet_execution_slice() {
    for row in contract()["archiveMaintenance"].as_array().unwrap() {
        let mut queries = 0;
        let result = archive_maintenance_slice_open(
            row["idleIngress"].as_bool().unwrap(), row["cancelled"].as_bool().unwrap(),
            row["yielding"].as_bool().unwrap(), row["startedUs"].as_u64(), row["currentUs"].as_u64(),
            || { queries += 1; Ok::<_, ()>(row["funded"].as_bool().unwrap()) },
        ).unwrap();
        assert_eq!(queries, row["queries"].as_u64().unwrap(), "{}", row["name"]);
        assert_eq!(result, row["open"].as_bool().unwrap(), "{}", row["name"]);
    }
    assert_eq!(archive_maintenance_slice_open(true, false, false, Some(0), Some(1), || Err("quote rejected")), Err("quote rejected"));
}
