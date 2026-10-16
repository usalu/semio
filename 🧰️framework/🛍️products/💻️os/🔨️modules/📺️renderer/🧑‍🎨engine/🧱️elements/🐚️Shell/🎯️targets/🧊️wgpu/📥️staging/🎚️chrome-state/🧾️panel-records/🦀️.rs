//! 🎚️ `shell-chrome-state` staging of `panel-records`: RENDERER-INTEGRATION-2, the items the direction graph homes in this crate (they were in a higher crate or in the old chunk file).

use super::*;

//#region ShellLifecycle
//#region 🧾️PanelRecordsAssembler
/// 🎯️ The ids "use current selection" stages for a reference argument: the live selection of its `domain` (or, when
/// it names none, of the one domain that selects anything), kept only at its `granularity` when it declares one.
pub fn reference_selection_ids(selection: &HashMap<String, semio_framework::DomainSelection>, domain: Option<&str>, granularity: Option<&str>) -> Vec<String> {
    let picked = match domain {
        Some(domain) => selection.get(domain),
        None => {
            let mut live = selection.values().filter(|selected| !selected.ids.is_empty());
            let first = live.next();
            if live.next().is_some() {
                None
            } else {
                first
            }
        }
    };
    picked.filter(|selected| granularity.is_none_or(|granularity| selected.granularity == granularity)).map(|selected| selected.ids.clone()).unwrap_or_default()
}
//#endregion 🧾️PanelRecordsAssembler
//#endregion ShellLifecycle
