//! 🧬️ Flow fields delegate their original heap ownership to typed full-grant retirement.

use crate::{CameraJson,FlowArtifact,FlowChannelRef,FlowGui,FlowHostSnapshot,FlowLayoutEntry,FlowNodeGui,FlowPreviewGui,NodeChrome,SynapseSpec,Widget,WidgetLayout};
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

metadata!(CameraJson,WidgetLayout);
semio_framework_value::artifact_retire_struct!(FlowHostSnapshot {schema,camera,widgets,synapses,layout});
semio_framework_value::artifact_retire_struct!(FlowArtifact {schema,tree,ui});
semio_framework_value::artifact_retire_struct!(FlowGui {camera,nodes,previews});
semio_framework_value::artifact_retire_struct!(FlowNodeGui {layout,chrome});
semio_framework_value::artifact_retire_struct!(FlowPreviewGui {id,source,mode,preview,expanded,layout});
semio_framework_value::artifact_retire_struct!(FlowChannelRef {neuron,channel});
semio_framework_value::artifact_retire_struct!(SynapseSpec {id,from,to,from_port,to_port});
semio_framework_value::artifact_retire_struct!(FlowLayoutEntry {id,layout});
owned_enum!(NodeChrome {Plain {},Slider {label},Note {text},Image {src},Variable {name,schema}});
owned_enum!(Widget {Neuron {id,neuron_kind,params,input_ports,output_ports},InputSlider {id,label},InputNote {id,text},InputImage {id,src},Variable {id,name,schema},OutputPreview {id,preview,expanded},OutputAction {id,action},OutputExport {id,format},Cluster {id,name,tree,flow}});

macro_rules! owned_record {
    ($type:ty {$($field:ident),+})=>{impl RetireOwned for $type {
        fn retirement(self)->Box<dyn RetirementCursor> {let Self {$($field,)*..}=self;sequence(vec![$(deferred($field)),+])}
        fn retirement_birth_bytes(&self)->Option<usize> {sequence_birth_bytes(&[$(deferred_birth_bytes_for(&self.$field)),+])}
        fn controlled_retirement_supported()->bool {true}
    }};
}
owned_record!(crate::AddWidget {widget});
owned_record!(crate::RemoveWidget {id});
owned_record!(crate::MoveWidget {id});
owned_record!(crate::ChangeWidget {id,widget});
owned_record!(crate::AddSynapse {synapse});
owned_record!(crate::RemoveSynapse {id});
owned_record!(crate::MoveSynapse {id});
owned_record!(crate::ChangeSynapse {id,synapse});
owned_record!(crate::ChangeLayout {entries});
impl RetireOwned for crate::FlowMutation {
    fn retirement(self)->Box<dyn RetirementCursor> {match self {Self::AddWidget(value)=>value.retirement(),Self::RemoveWidget(value)=>value.retirement(),Self::MoveWidget(value)=>value.retirement(),Self::ChangeWidget(value)=>value.retirement(),Self::AddSynapse(value)=>value.retirement(),Self::RemoveSynapse(value)=>value.retirement(),Self::MoveSynapse(value)=>value.retirement(),Self::ChangeSynapse(value)=>value.retirement(),Self::ChangeLayout(value)=>value.retirement()}}
    fn retirement_birth_bytes(&self)->Option<usize> {match self {Self::AddWidget(value)=>value.retirement_birth_bytes(),Self::RemoveWidget(value)=>value.retirement_birth_bytes(),Self::MoveWidget(value)=>value.retirement_birth_bytes(),Self::ChangeWidget(value)=>value.retirement_birth_bytes(),Self::AddSynapse(value)=>value.retirement_birth_bytes(),Self::RemoveSynapse(value)=>value.retirement_birth_bytes(),Self::MoveSynapse(value)=>value.retirement_birth_bytes(),Self::ChangeSynapse(value)=>value.retirement_birth_bytes(),Self::ChangeLayout(value)=>value.retirement_birth_bytes()}}
    fn controlled_retirement_supported()->bool {true}
}
