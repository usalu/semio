fn main() {
    let guards;
    {
        let transaction = begin_artifact_assembly().unwrap();
        guards = acquire_artifact_assembly_store_registry_guards(&transaction).unwrap();
    }
    drop(guards);
}
