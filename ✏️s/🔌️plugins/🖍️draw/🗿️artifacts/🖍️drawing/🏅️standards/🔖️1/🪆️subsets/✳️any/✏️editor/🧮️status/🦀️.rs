//! 🧮️ Projects window status from document counts and framework-owned selection.
use super::{drawing_manifest_action,DrawingPlayLabels,WindowEngagement,WindowEngagementInput,WindowEngagementStatus};
use std::collections::HashMap;

/// 🗣️ Formats counts with the addressed view's explicit locale and terminology.
pub(crate) fn text(layers:usize,selected:usize,labels:&DrawingPlayLabels)->String {
    let layer=if layers==1 {labels.layer_count_one} else {labels.layer_count_many};
    format!("{} · {}",layer.as_str().replace("{count}",&layers.to_string()),labels.selection_count.as_str().replace("{count}",&selected.to_string()))
}

/// 🪟️ Builds one addressed canvas engagement without introducing a second selection state.
pub(crate) fn engagements(layers:usize,selected:usize,input:String,view:&semio_framework_plugin::ViewModel)->HashMap<String,WindowEngagement> {
    let Some(window)=view.window_id.clone() else {return HashMap::new()};
    let labels=semio_framework_plugin::resolve_labels::<DrawingPlayLabels>(view);
    HashMap::from([(window,WindowEngagement {
        session_active:Some(false),options:None,
        input:Some(WindowEngagementInput {
            id:Some("drawing-canvas-engagement".into()),value:Some(input),placeholder:Some(labels.layer_name.as_str().into()),
            on_change:Some(drawing_manifest_action("engagementInput")),on_submit:Some(drawing_manifest_action("engagementSubmit")),
            disabled:None,on_repeat_last:None,on_abort:None,
        }),
        control:None,controls:None,
        status:Some(vec![WindowEngagementStatus{id:"drawing-layer-count".into(),text:text(layers,selected,labels)}]),
        possible_engagements:None,
    })])
}
