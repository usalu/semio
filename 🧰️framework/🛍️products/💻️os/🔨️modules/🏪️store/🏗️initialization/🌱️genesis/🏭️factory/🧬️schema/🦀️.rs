//! 🌱️ Selected concrete factories consume each original baseline into its actual Store or reset envelope.

/// 🏭️ The child owns original genesis encoding, native publication and installed catalog admission.
pub struct OriginalStoreGenesisFactory<P,M>
where
 P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
 M: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + semio_framework_os_spr::Mutation<P>,
{
 pub construct:fn(P)->Result<semio_framework_os_store_engine::lifted_store::ArtifactStore<P,M>,semio_framework_os_vcs::VcsError>,
 pub envelope:fn(P)->Result<semio_framework_os_store_space_schema::ArtifactEnvelope<P,M>,semio_framework_os_vcs::VcsError>,
}
