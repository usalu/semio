fn main() {
    let transaction = semio_framework_schema_registry::assembly::begin().unwrap();
    let guards = acquire_artifact_assembly_store_registry_guards(&transaction).unwrap();
    drop(transaction);
    let successor = semio_framework_schema_registry::assembly::begin().unwrap();
    drop(successor);
    drop(guards);
    println!("Registry API released the barrier while registry guards survived.");
}
