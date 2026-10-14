fn main() {
    let transaction = semio_framework_schema_registry::assembly::begin().unwrap();
    let guards = acquire_artifact_assembly_store_registry_guards(&transaction).unwrap();
    drop(guards);
    drop(transaction);
    println!("Registry API released registry guards before the barrier.");
}
