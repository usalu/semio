//! 🧵️ Authoring wire and funded document catalog every registered fixture installs beside its bounded retirement owners.

use protocol::{OpBinary, OpText, SemanticMutation};
use semio_framework_value::{retained_clone::RetainedCloneBirthDemand, ValueError, ValueRefusalKind};
use std::sync::Arc;

/// 🫳️ Names a fixture mutation's own `OpBinary` image as its authoring wire, borrowed from the original operation.
macro_rules! fixture_operation_text {
    ($($mutation:ty),+ $(,)?) => {$(
        impl store::ArtifactOperationText for $mutation {
            fn operation_text_node(&self, path: &[usize]) -> Result<store::ArtifactOperationTextNode<'_>, ::semio_framework_value::ValueError> {
                if path.is_empty() { Ok(store::ArtifactOperationTextNode::Scalar) } else { Err(::semio_framework_value::ValueError::literal(::semio_framework_value::ValueRefusalKind::InvalidValue, "fixture operation image has no children")) }
            }

            fn operation_text_scalar(&self, path: &[usize], offset: usize, output: &mut [u8]) -> Result<(usize, bool), store::ArtifactPreparedOperationError> {
                if !path.is_empty() { return Err(::semio_framework_value::ValueError::literal(::semio_framework_value::ValueRefusalKind::InvalidValue, "fixture operation image has no children").into()); }
                let image = ::protocol::OpBinary::encode_op(self).map_err(|error| ::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvariantViolated, error.to_string()))?;
                let count = output.len().min(image.len().saturating_sub(offset));
                output[..count].copy_from_slice(&image[offset..offset + count]);
                Ok((count, offset + count >= image.len()))
            }
        }
    )+};
}
pub(crate) use fixture_operation_text;

fn wire_source<M: OpBinary + store::ArtifactOperationText>(mutation: &M) -> Option<store::ArtifactPreparedOperationSource<'_>> {
    Some(store::ArtifactPreparedOperationSource::Text { header: b"fixture", body: mutation })
}

fn wire_schema<P, M: SemanticMutation<P>>(mutation: &M) -> Option<(&str, &str)> {
    let semantics = mutation.semantics();
    Some((semantics.entity, semantics.kind))
}

/// 📬️ The fixture's document preparation authority: bounded retained preparation wrapped in the mutation's own wire.
pub(crate) fn preparation_factory<P, M>(prefix: &'static str) -> Arc<dyn store::ArtifactStoreOneItemPreparationFactory<P, M>>
where
    P: Clone + Send + Sync + semio_framework_value::retirement::RetireOwned + semio_framework_value::retained_clone::RetainedClone + 'static,
    M: Clone + OpBinary + SemanticMutation<P> + store::ArtifactOperationText + store::ArtifactCanonicalJsonTree + Send + Sync + semio_framework_value::retirement::RetireOwned + 'static,
{
    let _ = prefix;
    store::operation_wire_preparation_factory(store::mutation_apply_preparation_factory::<P, M>(), wire_source::<M>, wire_schema::<P, M>)
}

/// 📐️ The wire-carrying catalog's source birth: the bounded catalog plus the wire-wrapped paged preparation factory.
fn wired_birth_demand<P, M>() -> Result<RetainedCloneBirthDemand, ValueError>
where
    P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::ArtifactPack + semio_framework_value::retirement::RetireOwned + semio_framework_value::retained_clone::RetainedClone + Send + Sync + 'static,
    M: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + OpBinary + OpText + SemanticMutation<P> + store::ArtifactOperationText + store::ArtifactCanonicalJsonTree + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
{
    let base = store::bounded_artifact_store_owners_birth_demand::<P, M>()?;
    let preparation = store::operation_wire_preparation_factory_source_birth_demand::<P, M>(RetainedCloneBirthDemand { capacity_bytes: store::paged_one_item_factory_birth_bytes::<P, M, store::MutationApplyEdit<P, M>>(), depth: 1 })?;
    let capacity_bytes = base.capacity_bytes.checked_add(preparation.capacity_bytes).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "fixture catalog original source tree capacity overflow"))?;
    Ok(RetainedCloneBirthDemand { capacity_bytes, depth: base.depth.max(preparation.depth) })
}

/// 📏️ Quotes the funded wire catalog source for the owner hooks.
pub(crate) fn document_store_owners_source_demands<P, M>() -> Result<semio_framework_value::RetirementDemand, ValueError>
where
    P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::ArtifactPack + semio_framework_value::retirement::RetireOwned + semio_framework_value::retained_clone::RetainedClone + Send + Sync + 'static,
    M: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + OpBinary + OpText + SemanticMutation<P> + store::ArtifactOperationText + store::ArtifactCanonicalJsonTree + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
{
    let birth = wired_birth_demand::<P, M>()?;
    Ok(semio_framework_value::RetirementDemand { capacity_bytes: birth.capacity_bytes, depth: birth.depth, ..Default::default() })
}

/// 🗃️ Admits the bounded document catalog together with the fixture's wire-carrying preparation authority under the caller's grant.
pub(crate) fn admit_document_store_owners<P, M>(prefix: &'static str, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<(store::DocumentStoreOwners<P, M>, semio_framework_value::retained_clone::RetainedCloneProgress), store::DocumentStoreOwnersAdmissionError<P, M>>
where
    P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::ArtifactPack + semio_framework_value::retirement::RetireOwned + semio_framework_value::retained_clone::RetainedClone + Send + Sync + 'static,
    M: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + OpBinary + OpText + SemanticMutation<P> + store::ArtifactOperationText + store::ArtifactCanonicalJsonTree + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
{
    let preparation = store::operation_wire_preparation_factory_source_birth_demand::<P, M>(RetainedCloneBirthDemand { capacity_bytes: store::paged_one_item_factory_birth_bytes::<P, M, store::MutationApplyEdit<P, M>>(), depth: 1 }).map_err(|error| store::DocumentStoreOwnersAdmissionError { error, owners: None, progress: Default::default() })?;
    store::bounded_artifact_store_owners_with_one_item_preparation(grant, preparation, || preparation_factory::<P, M>(prefix))
}

/// 🗃️ Funds the bounded document catalog together with the fixture's wire-carrying preparation authority.
pub(crate) fn funded_document_store_owners<P, M>(prefix: &'static str) -> Result<store::DocumentStoreOwners<P, M>, ValueError>
where
    P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::ArtifactPack + semio_framework_value::retirement::RetireOwned + semio_framework_value::retained_clone::RetainedClone + Send + Sync + 'static,
    M: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + OpBinary + OpText + SemanticMutation<P> + store::ArtifactOperationText + store::ArtifactCanonicalJsonTree + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
{
    store::fund_document_store_owners(wired_birth_demand::<P, M>()?, |grant| admit_document_store_owners::<P, M>(prefix, grant))
}
