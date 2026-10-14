
fn main() {
    let transaction = begin_artifact_assembly().unwrap();
    let guards = acquire_artifact_assembly_store_registry_guards(&transaction).unwrap();
    drop(transaction);
    let successor = begin_artifact_assembly().unwrap();
    drop(successor);
    drop(guards);
    println!("Original Store API released the barrier while registry guards survived.");
}
