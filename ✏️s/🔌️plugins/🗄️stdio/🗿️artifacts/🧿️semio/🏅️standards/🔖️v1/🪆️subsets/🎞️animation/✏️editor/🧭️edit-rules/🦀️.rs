//! 🧭️ The animation editor's edit rules: which snapshot pointer raises which ONE concrete animation mutation kind. An edit no kind expresses is refused, never approximated.

use crate::editor::semio_base::edit_plumbing::{ent, ins, rem, INDEX};
use semio_s_artifact_stdio_contract::editing::{EditRules, RowKey, Selector};

const TIMELINE: Selector = Selector::Index("timelineIndex");
const CHANNEL: Selector = Selector::Index("channelIndex");
const AT: RowKey = RowKey::Index("index");

/// 📚 Every pointer an animation editor edit resolves, one rule per kind that sets, inserts or removes through it.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[
        ent("/timelines/*/name", "set-timeline-name", &[INDEX], "name"),
        ent("/timelines/*/channels/*/target", "set-channel-target", &[TIMELINE, INDEX], "target"),
        ent("/timelines/*/channels/*/interpolation", "set-channel-interpolation", &[TIMELINE, INDEX], "interpolation"),
        ent("/timelines/*/channels/*/keyframes/*/t", "set-keyframe-time", &[TIMELINE, CHANNEL, INDEX], "t"),
        ent("/timelines/*/channels/*/keyframes/*/value", "set-keyframe-value", &[TIMELINE, CHANNEL, INDEX], "value"),
    ],
    inserts: &[
        ins("/timelines", "insert-timeline", &[], Some("index"), "timeline"),
        ins("/timelines/*/channels", "insert-channel", &[TIMELINE], Some("index"), "channel"),
        ins("/timelines/*/channels/*/keyframes", "insert-keyframe", &[TIMELINE, CHANNEL], Some("index"), "keyframe"),
    ],
    removes: &[
        rem("/timelines", "remove-timeline", &[], AT),
        rem("/timelines/*/channels", "remove-channel", &[TIMELINE], AT),
        rem("/timelines/*/channels/*/keyframes", "remove-keyframe", &[TIMELINE, CHANNEL], AT),
    ],
};
