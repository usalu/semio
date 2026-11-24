//! 🎬️ PDF page view actions: the ten view tools, their en/de labels, and how an action becomes a [`ViewIntent`].
//! [Canvas camera](https://www.w3.org/TR/css-transforms-1/)

use super::*;
use semio_plugin::{ActionArgDef, ActionDefinition, ActionKind};
use semio_ui_locale::LocalizedLabel;

//#region 🎬️ViewActions
/// 🎬️ Every view action id, in the order the window lists them.
pub const VIEW_ACTION_IDS: [&str; 10] = ["zoom-in", "zoom-out", "fit-page", "fit-width", "actual-size", "go-to-page", "set-query", "toggle-match-case", "next-hit", "previous-hit"];

pub fn is_view_action(action: &str) -> bool {
    VIEW_ACTION_IDS.contains(&action)
}

/// 🎬️ The intent an action id stands for; `text` is the query and `page` the zero-based page.
pub fn view_intent(action: &str, text: &str, page: usize) -> Option<ViewIntent> {
    Some(match action {
        "zoom-in" => ViewIntent::ZoomIn,
        "zoom-out" => ViewIntent::ZoomOut,
        "fit-page" => ViewIntent::FitPage,
        "fit-width" => ViewIntent::FitWidth,
        "actual-size" => ViewIntent::ActualSize,
        "go-to-page" => ViewIntent::GoToPage(page),
        "set-query" => ViewIntent::SetQuery(text.to_string()),
        "toggle-match-case" => ViewIntent::ToggleMatchCase,
        "next-hit" => ViewIntent::NextHit,
        "previous-hit" => ViewIntent::PreviousHit,
        _ => return None,
    })
}

/// 🪟 The view action definitions for the page window, with en and de labels.
pub fn view_action_definitions() -> Vec<ActionDefinition> {
    let labels = [("Zoom In", "Vergrößern"), ("Zoom Out", "Verkleinern"), ("Fit Page", "Seite einpassen"), ("Fit Width", "Breite einpassen"), ("Actual Size", "Originalgröße"), ("Go To Page", "Gehe zu Seite"), ("Search Text", "Text suchen"), ("Match Case", "Groß-/Kleinschreibung"), ("Next Match", "Nächster Treffer"), ("Previous Match", "Vorheriger Treffer")];
    VIEW_ACTION_IDS.iter().zip(labels).map(|(id, (en, de))| {
        let args = match *id {
            "go-to-page" => vec![ActionArgDef::number("page", LocalizedLabel::native("Page number", "Seitenzahl")).required()],
            "set-query" => vec![ActionArgDef::text("text", LocalizedLabel::native("Query", "Suchbegriff")).min_length(0).required()],
            _ => Vec::new(),
        };
        let mut definition = ActionDefinition::bounded_catalog(*id, LocalizedLabel::native(en, de), ActionKind::View).with_args(args);
        definition.semantics.execution.interactive_job = semio_plugin::InteractiveJobClassification::Migrated;
        definition
    }).collect()
}
//#endregion 🎬️ViewActions
