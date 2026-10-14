
fn main() {
    let transaction = begin_artifact_assembly().unwrap();
    let guards = acquire_artifact_assembly_store_registry_guards(&transaction).unwrap();
    drop(guards);
    drop(transaction);
    println!("Original Store API released registry guards before the barrier.");
}
