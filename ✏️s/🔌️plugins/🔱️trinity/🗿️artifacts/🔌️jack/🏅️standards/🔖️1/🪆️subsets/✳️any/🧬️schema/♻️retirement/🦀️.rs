//! ♻️ Jack document metadata and its sole composed child retire through real owners.
use semio_framework_value::retirement::{deferred, deferred_birth_bytes, deferred_birth_bytes_for, sequence, sequence_birth_bytes, RetireOwned, RetirementCursor};

semio_framework_value::artifact_retire_struct!(crate::Manifest {node_kinds,edge_kinds,port_kinds});
semio_framework_value::artifact_retire_struct!(crate::NodeKindDef {name,properties,port_kinds});
semio_framework_value::artifact_retire_struct!(crate::EdgeKindDef {name,properties});
semio_framework_value::artifact_retire_struct!(crate::PortKindDef {name,direction,properties});
semio_framework_value::artifact_retire_struct!(crate::Camera {x,y,zoom});

/// 🔗️ Retires the metadata fields, the child handle and the shared local content owner as separate deferred fields.
impl RetireOwned for crate::JackSnapshot {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        let Self { schema, name, manifest_id, manifest, camera, mut content, root_node_id, query } = self;
        let owner = content.take_local_owner::<crate::JackContentOwner>().ok().flatten();
        sequence(vec![deferred(schema), deferred(name), deferred(manifest_id), deferred(manifest), deferred(camera), deferred(content), deferred(owner), deferred(root_node_id), deferred(query)])
    }
    fn retirement_birth_bytes(&self) -> Option<usize> {
        sequence_birth_bytes(&[
            deferred_birth_bytes_for(&self.schema),
            deferred_birth_bytes_for(&self.name),
            deferred_birth_bytes_for(&self.manifest_id),
            deferred_birth_bytes_for(&self.manifest),
            deferred_birth_bytes_for(&self.camera),
            deferred_birth_bytes_for(&self.content),
            deferred_birth_bytes::<Option<std::sync::Arc<crate::JackContentOwner>>>(),
            deferred_birth_bytes_for(&self.root_node_id),
            deferred_birth_bytes_for(&self.query),
        ])
    }
    fn controlled_retirement_supported() -> bool {
        true
    }
}
