fn escape_guard() -> impl Sized + 'static {
    let transaction = semio_framework_schema_registry::assembly::begin().unwrap();
    acquire_artifact_assembly_store_registry_guards(&transaction).unwrap()
}
fn main() {
    drop(escape_guard());
}
