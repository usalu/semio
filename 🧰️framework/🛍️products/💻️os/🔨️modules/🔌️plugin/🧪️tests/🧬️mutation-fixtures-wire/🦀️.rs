//! 🧵️ Authoring wire and funded document catalog every registered fixture installs beside its bounded retirement owners.

use crate::app::{bounded_config_store_one_item_preparation_factory, bounded_config_store_one_item_preparation_factory_source_birth_demand};
use protocol::{OpBinary, OpText, SemanticMutation};
use semio_framework_value::{retained_clone::RetainedCloneBirthDemand, ValueError, ValueRefusalKind};
use std::sync::Arc;

/// 🫳️ Names a fixture mutation's own `OpBinary` image as its authoring wire, borrowed from the original operation.
macro_rules! fixture_operation_text {
    ($($mutation:ty),+ $(,)?) => {$(
        impl store::ArtifactOperationText for $mutation {
            fn operation_text_node(&self, path: &[usize]) -> Result<store::ArtifactOperationTextNode<'_>, String> {
                if path.is_empty() { Ok(store::ArtifactOperationTextNode::Scalar) } else { Err("fixture operation image has no children".into()) }
            }

            fn operation_text_scalar(&self, path: &[usize], offset: usize, output: &mut [u8]) -> Result<(usize, bool), String> {
                if !path.is_empty() { return Err("fixture operation image has no children".into()); }
                let image = ::protocol::OpBinary::encode_op(self).map_err(|error| error.to_string())?;
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
    P: Clone + Send + Sync + semio_framework_value::retirement::RetireOwned + 'static,
    M: Clone + OpBinary + SemanticMutation<P> + store::ArtifactOperationText + Send + Sync + semio_framework_value::retirement::RetireOwned + 'static,
{
    store::operation_wire_preparation_factory(bounded_config_store_one_item_preparation_factory::<P, M>(prefix, 4_096), wire_source::<M>, wire_schema::<P, M>)
}

/// 🗃️ Funds the bounded document catalog together with the fixture's wire-carrying preparation authority.
pub(crate) fn funded_document_store_owners<P, M>(prefix: &'static str) -> Result<store::DocumentStoreOwners<P, M>, ValueError>
where
    P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::ArtifactPack + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    M: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + OpBinary + OpText + SemanticMutation<P> + store::ArtifactOperationText + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
{
    let base = store::bounded_artifact_store_owners_birth_demand::<P, M>()?;
    let preparation = store::operation_wire_preparation_factory_source_birth_demand::<P, M>(bounded_config_store_one_item_preparation_factory_source_birth_demand::<P, M>())?;
    let capacity_bytes = base.capacity_bytes.checked_add(preparation.capacity_bytes).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "fixture catalog original source tree capacity overflow"))?;
    let birth = RetainedCloneBirthDemand { capacity_bytes, depth: base.depth.max(preparation.depth) };
    store::fund_document_store_owners(birth, |grant| store::bounded_artifact_store_owners_with_one_item_preparation(grant, preparation, || preparation_factory::<P, M>(prefix)))
}
