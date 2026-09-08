
use super::*;

#[test]
fn summary_tables_accumulate() {
    let mut s = SummaryTables::default();
    s.add_annual("Electricity", 1000.0, "kWh");
    assert_eq!(s.annual_energy.len(), 1);
}
