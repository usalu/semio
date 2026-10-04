//! ♻️ Jack document metadata and its sole composed child retire through real owners.
semio_framework_value::artifact_retire_struct!(crate::JackSnapshot {schema,name,manifest_id,manifest,camera,content,root_node_id,query});
semio_framework_value::artifact_retire_struct!(crate::Manifest {node_kinds,edge_kinds,port_kinds});
semio_framework_value::artifact_retire_struct!(crate::NodeKindDef {name,properties,port_kinds});
semio_framework_value::artifact_retire_struct!(crate::EdgeKindDef {name,properties});
semio_framework_value::artifact_retire_struct!(crate::PortKindDef {name,direction,properties});
semio_framework_value::artifact_retire_struct!(crate::Camera {x,y,zoom});
