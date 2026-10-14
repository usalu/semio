fn main() {
    let guards;
    {
        let transaction = semio_framework_schema_registry::assembly::begin().unwrap();
        guards = acquire_artifact_assembly_store_registry_guards(&transaction).unwrap();
    }
    drop(guards);
}
