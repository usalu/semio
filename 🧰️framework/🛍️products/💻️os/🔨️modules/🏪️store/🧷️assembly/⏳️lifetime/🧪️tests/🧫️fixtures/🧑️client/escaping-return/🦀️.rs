fn escape_guard() -> impl Sized + 'static {
    let transaction = begin_artifact_assembly().unwrap();
    acquire_artifact_assembly_store_registry_guards(&transaction).unwrap()
}
fn main() {
    drop(escape_guard());
}
