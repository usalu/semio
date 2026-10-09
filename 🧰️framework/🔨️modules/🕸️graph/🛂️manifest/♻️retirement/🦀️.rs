//! ♻️ Complete graph fields use measured deferred ownership retirement.

semio_framework_value::artifact_retire_struct!(super::PropertyDef {name,kind,value_type,expr});
semio_framework_value::artifact_retire_leaf!(super::PropertyKind);

semio_framework_value::artifact_retire_leaf!(super::PortDirection);
