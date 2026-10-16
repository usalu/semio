//! 🧬️ Flow fields delegate their original heap ownership to typed full-grant retirement.

use crate::{FlowArtifact,FlowChannelRef,FlowGui,FlowHostComponents,FlowLayoutEntry,FlowNodeGui,FlowPreviewGui,NodeChrome,SynapseSpec,Widget,WidgetLayout};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor,deferred,deferred_birth_bytes_for,sequence,sequence_birth_bytes};

macro_rules! metadata {
    ($($type:ty),+)=>{$(impl RetireOwned for $type {
        fn retirement(self)->Box<dyn RetirementCursor> {().retirement()}
        fn retirement_birth_bytes(&self)->Option<usize> {().retirement_birth_bytes()}
        fn controlled_retirement_supported()->bool {true}
    })+};
}
macro_rules! owned_enum {
    ($type:ident {$($variant:ident {$($field:ident),*}),+})=>{impl RetireOwned for $type {
        fn retirement(self)->Box<dyn RetirementCursor> {match self {$(Self::$variant {$($field,)*..}=>sequence(vec![$(deferred($field)),*])),+}}
        fn retirement_birth_bytes(&self)->Option<usize> {match self {$(Self::$variant {$($field,)*..}=>sequence_birth_bytes(&[$(deferred_birth_bytes_for($field)),*])),+}}
        fn controlled_retirement_supported()->bool {true}
    }};
}

metadata!(WidgetLayout);
semio_framework_value::artifact_retire_struct!(FlowHostComponents {schema,camera,widgets,synapses,layout});
semio_framework_value::artifact_retire_struct!(FlowArtifact {schema,tree,ui});
semio_framework_value::artifact_retire_struct!(FlowGui {camera,nodes,previews});
semio_framework_value::artifact_retire_struct!(FlowNodeGui {layout,chrome});
semio_framework_value::artifact_retire_struct!(FlowPreviewGui {id,source,mode,preview,expanded,layout});
semio_framework_value::artifact_retire_struct!(FlowChannelRef {neuron,channel});
semio_framework_value::artifact_retire_struct!(SynapseSpec {id,from,to,from_port,to_port});
semio_framework_value::artifact_retire_struct!(FlowLayoutEntry {id,layout});
owned_enum!(NodeChrome {Plain {},Slider {label},Note {text},Image {src},Variable {name,schema}});
owned_enum!(Widget {Neuron {id,neuron_kind,params,input_ports,output_ports},InputSlider {id,label},InputNote {id,text},InputImage {id,src},Variable {id,name,schema},OutputPreview {id,preview,expanded},OutputAction {id,action},OutputExport {id,format},Cluster {id,name,tree,flow}});
