//! 🧪️ Portable exclusive-phase vectors agree with the independent Tokio mutex.
use semio_framework_schema_registry::assembly;
use serde_json::Value;
use tokio::sync::Mutex;

#[test]
fn portable_assembly_exclusion_release_and_poison_refusal() {
    let corpus: Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧷️assembly/🔣️.json")).unwrap();
    let rows = corpus["scenarios"].as_array().unwrap();
    assert_eq!(rows.len(), 2);
    for row in rows {
        let reference = Mutex::new(());
        let mut owned = None;
        let mut independent = None;
        let mut actual = Vec::new();
        let mut oracle = Vec::new();
        for operation in row["operations"].as_array().unwrap() {
            match operation.as_str().unwrap() {
                "try" => {
                    let acquired = assembly::try_begin().unwrap();
                    actual.push(acquired.is_some());
                    if acquired.is_some() { owned = acquired; }
                    let acquired = reference.try_lock().ok();
                    oracle.push(acquired.is_some());
                    if acquired.is_some() { independent = acquired; }
                }
                "release" => {
                    actual.push(owned.take().is_some());
                    oracle.push(independent.take().is_some());
                }
                _ => unreachable!(),
            }
        }
        assert_eq!(serde_json::to_value(&actual).unwrap(), row["expected"]);
        assert_eq!(actual, oracle);
        drop(owned);
        drop(independent);
    }
    let transaction = assembly::begin().unwrap();
    assert!(assembly::try_begin().unwrap().is_none());
    drop(transaction);
    assert!(assembly::try_begin().unwrap().is_some());
    assert!(std::thread::spawn(|| {
        let _transaction = assembly::begin().unwrap();
        panic!("controlled writer failure");
    }).join().is_err());
    assert!(matches!(assembly::begin(), Err(assembly::Error::Unavailable)));
    assert!(matches!(assembly::try_begin(), Err(assembly::Error::Unavailable)));
    println!("[DEBUG] Registry assembly: two portable exclusion/release vectors match Tokio; one process-wide barrier refuses writer poison");
}
