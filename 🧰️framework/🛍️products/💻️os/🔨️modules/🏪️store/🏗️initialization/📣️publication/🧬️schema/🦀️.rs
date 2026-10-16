//! 📣️ The selected actual native producer publishes its declared snapshot codec after original Store validation.

/// 🏭️ Publication preserves the original atomic registry and its typed native provenance.
pub trait ArtifactStorePublication<P> {
 fn publish_native_snapshot()->Result<(),semio_framework_os_io::ArtifactAssemblyRegistryError>;
}

/// 🧰️ Selects the authored native registration of a generic snapshot owner.
pub struct ArtifactSnapshotPublication;
impl<P:semio_framework_os_store_text_io::ArtifactStoreSnapshot> ArtifactStorePublication<P> for ArtifactSnapshotPublication {
 fn publish_native_snapshot()->Result<(),semio_framework_os_io::ArtifactAssemblyRegistryError>{semio_framework_os_io::publish_native_snapshot::<P>()}
}

impl<P:semio_framework_os_store_text_io::ArtifactStoreSnapshot> ArtifactStorePublication<P> for semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer {
 fn publish_native_snapshot()->Result<(),semio_framework_os_io::ArtifactAssemblyRegistryError>{semio_framework_os_io::publish_native_snapshot::<P>()}
}
