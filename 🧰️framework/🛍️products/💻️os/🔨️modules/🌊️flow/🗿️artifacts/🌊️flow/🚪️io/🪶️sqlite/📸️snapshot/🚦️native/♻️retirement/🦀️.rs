//! ♻️ Flow native ownership guards using the actual typed cold retirement domain.
use super::*;
use semio_framework_dsl_record::__rt::DecodedFieldOwner;
/// 🛡️ Retains an actual owned field until complete parent publication.
pub(super) fn guarded<T>(value:T,retire:fn(T))->DecodedFieldOwner<T>{semio_framework_dsl_record::__rt::DecodedFieldOwner::new(value,retire)}
/// 🌊️ Releases the actual persisted snapshot through its existing cold retirement owner.
pub(super) fn snapshot(value:FlowHostSnapshot){value.retire_cold();}
/// 🎛️ Retires a completed widget, including neural dictionary, tree and GUI payloads.
pub(super) fn widget(value:Widget){value.retire_cold();}
/// 🌳️ Iterates the existing typed Flow tree retirement rather than dropping recursive payloads.
pub(super) fn tree(value:Tree){FlowRetirement::from_owner(FlowOwner::Tree(value)).retire_cold();}
/// 🖼️ Retires GUI maps and nested previews through their actual typed owner.
pub(super) fn gui(value:FlowGui){FlowRetirement::from_owner(FlowOwner::Gui(value)).retire_cold();}
/// 🗂️ Releases shared layout nodes through existing ordered retirement.
pub(super) fn layouts(value:crate::OrderedMap<WidgetLayout>){FlowRetirement::from_owner(FlowOwner::Layouts(value)).retire_cold();}
/// 📋️ Retires partial actual widget vectors with their admitted backing retained.
pub(super) fn widgets(value:Vec<Widget>){FlowRetirement::from_owner(FlowOwner::Widgets(value)).retire_cold();}
/// 🧠️ Retires partial actual neurons including their optional boxed trees.
pub(super) fn neurons(value:Vec<Neuron>){FlowRetirement::from_owner(FlowOwner::Neurons(value)).retire_cold();}
