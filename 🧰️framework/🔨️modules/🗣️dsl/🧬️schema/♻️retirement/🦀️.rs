//! ♻️ Finite Record ownership uses the neutral typed constructor and release authorities.
use super::{RecordSpecProducer,WireEdgeLabel,WireNode,WireValue};
use semio_framework_value::retirement::RetirementCursor;

semio_framework_value::artifact_retire_struct!(WireNode{id,kind,port});
semio_framework_value::artifact_retire_struct!(WireEdgeLabel{id,kind});
semio_framework_value::artifact_retire_struct!(WireValue{from,edge,edge_label,properties});
semio_framework_value::artifact_retire_leaf!(RecordSpecProducer);
semio_framework_value::artifact_retire_leaf!(super::RecordLayout,super::ExprOp);

