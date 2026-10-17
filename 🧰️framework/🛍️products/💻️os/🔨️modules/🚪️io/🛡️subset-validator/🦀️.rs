//! 🛡️ Subset validator: the subset validator registry and the subset validation run.
//!
//! Holds the `🔖️SubsetValidator` region of the module; the root re-exports its items.

use super::*;

//#region 🔖️SubsetValidator

/// 🪶️ A subset's conformance policy over one owned typed snapshot and its exact relational
/// projection. The owning artifact's `ArtifactSqliteSnapshotHost::validate_sqlite_snapshot_subset`
/// resolves it by dialect through `validate_registered_snapshot_subset`, so a base model never
/// names the subsets that extend it: the dependency points from each subset to its base only.
pub trait SnapshotSubsetValidator: SubsetValidator {
    type Snapshot: 'static;
    fn validate_snapshot(snapshot: &Self::Snapshot, database: &semio_framework_io_sqlite_snapshot::SqliteDatabase, control: &mut semio_framework_io_sqlite_snapshot::SqliteSnapshotControl<'_>) -> semio_framework_io_schema::IoResult<()>;
}



// 🚫️async: E4 fn-pointer slot — `SubsetValidatorEntry.validate` is a bare `fn` pointer;
// `SubsetValidator::validate` stays `async fn` (a real trait method) and this thunk drives it
// to completion synchronously via `resolve_ready`, same pattern as the `IoEntry` constructors.
fn validate_payload_thunk<V: SubsetValidator>(payload: &IoPayload) -> Vec<Diagnostic> {
    ::semio_framework_async::poll::resolve_ready(V::validate(payload))
}

/// 🎹️ Erases a typed `SubsetValidator` impl into a `SubsetValidatorEntry` row -- the
/// `ComposerEntry::of::<C>()`-style helper for this trait.
pub fn subset_validator_entry_of<V: SubsetValidator>() -> SubsetValidatorEntry {
    SubsetValidatorEntry { dialect: V::DIALECT, validate: validate_payload_thunk::<V>, validate_snapshot: None }
}

/// 🎹️ Erases a `SnapshotSubsetValidator` into a row that serves both payload and owned-snapshot validation.
pub fn snapshot_subset_validator_entry_of<V: SnapshotSubsetValidator>() -> SubsetValidatorEntry {
    fn validate_snapshot<V: SnapshotSubsetValidator>(snapshot: &dyn std::any::Any, database: &semio_framework_io_sqlite_snapshot::SqliteDatabase, control: &mut semio_framework_io_sqlite_snapshot::SqliteSnapshotControl<'_>) -> semio_framework_io_schema::IoResult<()> {
        match snapshot.downcast_ref::<V::Snapshot>() {
            Some(snapshot) => V::validate_snapshot(snapshot, database, control),
            None => Err(semio_framework_io_schema::IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("snapshot type does not own subset {}", ArtifactDialect::from(V::DIALECT).to_coordinate())))),
        }
    }
    SubsetValidatorEntry { dialect: V::DIALECT, validate: validate_payload_thunk::<V>, validate_snapshot: Some(validate_snapshot::<V>) }
}

/// 🛡️ Runs the owned-snapshot validator registered for exactly `dialect`. The base `*` subset is
/// unconstrained and always clean; any other dialect without a registered snapshot validator is
/// refused, because a real subset must never pass unchecked.
pub fn validate_registered_snapshot_subset<S: 'static>(dialect: &ArtifactDialect, snapshot: &S, database: &semio_framework_io_sqlite_snapshot::SqliteDatabase, control: &mut semio_framework_io_sqlite_snapshot::SqliteSnapshotControl<'_>) -> semio_framework_io_schema::IoResult<()> {
    if dialect.subset == SubsetId::ANY.0 {
        return Ok(semio_framework_io_schema::IoOutcome::clean(()));
    }
    let validate = subset_validator_registry().read().map_err(|_| semio_framework_io_schema::IoError::from_value_error(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "subset validator registry unavailable")))?.get(dialect).and_then(|entry| entry.validate_snapshot);
    match validate {
        Some(validate) => validate(snapshot, database, control),
        None => Err(semio_framework_io_schema::IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner, format!("owned snapshot subset {} has no semantic validator", dialect.to_coordinate())))),
    }
}






/// 🚫️ A subset validation cannot execute because its registry is unavailable or incomplete.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SubsetValidationError {
    Missing { dialect: ArtifactDialect },
    Unavailable(IoRegistryUnavailable),
}


/// 📌️ Registers one subset validator without replacing an established dialect owner.
pub fn register_subset_validator(entry: &'static SubsetValidatorEntry) -> Result<(), SubsetValidatorRegistryError> {
    register_subset_validators(&[entry])
}


/// 🔬️ Verifies subset-validator entries without changing their established owners.
pub async fn preflight_subset_validators(entries: &[&'static SubsetValidatorEntry]) -> Result<(), SubsetValidatorRegistryError> {
    let assembly = semio_framework_schema_registry::assembly::begin().map_err(|_| SubsetValidatorRegistryError::Unavailable(IoRegistryUnavailable { registry: "artifact-assembly" }))?;
    preflight_subset_validators_in_assembly(&assembly, entries).await
}

/// 🔬️ Verifies subset validators while one artifact assembly owns the shared publication barrier.
pub async fn preflight_subset_validators_in_assembly(_assembly: &semio_framework_schema_registry::assembly::Transaction, entries: &[&'static SubsetValidatorEntry]) -> Result<(), SubsetValidatorRegistryError> {
    let registry = subset_validator_registry().read().map_err(|_| SubsetValidatorRegistryError::Unavailable(IoRegistryUnavailable { registry: "subset-validator" }))?;
    validate_subset_validators(&registry, entries)
}

/// 📌️ Registers subset-validator entries only when the entire candidate set is conflict-free.
pub fn register_subset_validators(entries: &[&'static SubsetValidatorEntry]) -> Result<(), SubsetValidatorRegistryError> {
    let assembly = semio_framework_schema_registry::assembly::begin().map_err(|_| SubsetValidatorRegistryError::Unavailable(IoRegistryUnavailable { registry: "artifact-assembly" }))?;
    register_subset_validators_in_assembly(&assembly, entries)
}

/// 📌️ Publishes preflighted subset validators while one artifact assembly owns the shared barrier.
pub fn register_subset_validators_in_assembly(_assembly: &semio_framework_schema_registry::assembly::Transaction, entries: &[&'static SubsetValidatorEntry]) -> Result<(), SubsetValidatorRegistryError> {
    let mut reg = subset_validator_registry().write().map_err(|_| SubsetValidatorRegistryError::Unavailable(IoRegistryUnavailable { registry: "subset-validator" }))?;
    validate_subset_validators(&reg, entries)?;
    for entry in entries {
        let dialect = ArtifactDialect::from(entry.dialect);
        reg.entry(dialect).or_insert(entry);
    }
    Ok(())
}

/// 📚️ Every dialect key currently registered in `SUBSET_VALIDATOR_REGISTRY`.
pub async fn list_registered_subset_validator_dialects() -> Result<Vec<Dialect>, SubsetValidatorRegistryError> {
    let registry = subset_validator_registry().read().map_err(|_| SubsetValidatorRegistryError::Unavailable(IoRegistryUnavailable { registry: "subset-validator" }))?;
    Ok(registry.values().map(|entry| entry.dialect).collect())
}

/// 🛡️ The generic validate-on-build hook (D5): if `dialect.subset` is anything other than
/// `SubsetId::ANY` and a validator is registered for that EXACT dialect, run it and fold its
/// `Diagnostic`s onto `diagnostics`. Advisory only -- a validator that itself returns diagnostics
/// never fails composition; diagnostics are soft signals here exactly like `Composition<T>`/
/// `Analysis<T>` already carry elsewhere in this file (a subset composer wanting a HARD gate
/// enforces that itself, inside its own `compose`, before ever returning `Ok` -- see the PDF/A
/// pilot). Called from every generic compose-dispatch path in this module (`io_dispatch`,
/// `wire_artifact_compose`) so every future subset gets this for free the moment it registers a
/// validator -- no dispatch call site needs to change again. A poisoned registry lock is surfaced
/// as a typed dispatch failure.
///
/// `ANY` always short-circuits (nothing to validate against the unconstrained base subset). A
/// real (non-`ANY`) dialect with NO registered validator is, since ticket
/// 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES, treated as a defect rather than silence
/// -- every real subset is expected to register one (`policyStandardSubsetVocabularyBreaches`
/// checks this statically) -- and emits one `io.subset.validator-missing` Warning naming the
/// coordinate. Still never hard-fails here: the receiving side of a cross-plugin wire compose may
/// legitimately not host the owning plugin's validator locally, and a missing validator is exactly
/// the kind of thing a diagnostic (not a dispatch error) exists to surface.
pub(super) async fn run_subset_validation(dialect: Dialect, payload: &IoPayload, diagnostics: &mut Vec<Diagnostic>) -> Result<(), SubsetValidationError> {
    if dialect.subset == SubsetId::ANY {
        return Ok(());
    }
    let reg = subset_validator_registry().read().map_err(|_| SubsetValidationError::Unavailable(IoRegistryUnavailable { registry: "subset-validator" }))?;
    match reg.get(&ArtifactDialect::from(dialect)) {
        Some(entry) => diagnostics.extend((entry.validate)(payload)),
        None => return Err(SubsetValidationError::Missing { dialect: ArtifactDialect::from(dialect) }),
    }
    Ok(())
}
//#endregion 🔖️SubsetValidator
