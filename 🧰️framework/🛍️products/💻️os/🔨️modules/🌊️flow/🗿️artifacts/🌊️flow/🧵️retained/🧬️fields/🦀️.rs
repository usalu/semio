//! 🧬️ Flow record fields delegate their original heap ownership to typed reply-shaped retirement.

use crate::{FlowArtifact,FlowChannelRef,FlowGui,FlowHostComponents,FlowLayoutEntry,FlowNodeGui,FlowPreviewGui,SynapseSpec};

semio_framework_value::artifact_retire_struct!(FlowHostComponents {schema,camera,widgets,synapses,layout});
semio_framework_value::artifact_retire_struct!(FlowArtifact {schema,tree,ui});
semio_framework_value::artifact_retire_struct!(FlowGui {camera,nodes,previews});
semio_framework_value::artifact_retire_struct!(FlowNodeGui {layout,chrome});
semio_framework_value::artifact_retire_struct!(FlowPreviewGui {id,source,mode,preview,expanded,layout});
semio_framework_value::artifact_retire_struct!(FlowChannelRef {neuron,channel});
semio_framework_value::artifact_retire_struct!(SynapseSpec {id,from,to,from_port,to_port});
semio_framework_value::artifact_retire_struct!(FlowLayoutEntry {id,layout});
