//! 📄️ Accepts one authenticated, receipt-sealed directory page as one bounded item of the Home transient lane.
//!
//! The page lands ONLY through the retained route of either Home surface ([`HomeDirectoryPageWork`]): the work reads the
//! projection its job captured at dispatch, answers the typed receipt the host acknowledges the page by, and hands the
//! transient lane at most one [`ApplyDirectoryPage`] item — the page itself, never the projection, never an edit.

use crate::standards::v1::subsets::any::schema::mutations::text::SHomeMutation;
use crate::SHomeSnapshot;
use crate::editor::home::config::{HomeConfig, HomeConfigMutation, HOME_DIRECTORY_PAGE_BYTES};
use crate::editor::home::transient::{ApplyDirectoryPage, DirectoryPageAdmission, DirectoryProjectionReceiptV1, HomeDirectoryProjection, HomeTransient, HomeTransientMutation};
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};
use semio_framework_plugin::{AppEvent, ArtifactApp, ArtifactView, ConfigView, Emit, EphemeralEmit, Fault, FaultOrigin};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "apply-directory-event-page")]
pub struct ApplyDirectoryEventPage {
    /// 📄️ Canonical `DirectoryEventPageV1` JSON returned by the authenticated hub.
    pub page_json: String,
}
//#endregion 🔖️Payload

//#region 🔖️Handle
/// 🚫️ The direct lane never lands a page: without the retained route there is no transient item and no receipt, and the
/// host's feed would re-offer the page for ever.
pub fn handle(_payload: &ApplyDirectoryEventPage, _doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    Err(Fault::new(FaultOrigin::App, "s.home.directory-event-page.requires-retained-job", "a directory page lands only as the retained job"))
}

/// 📬️ The whole answer to one sealed page against the projection a job captured: the typed terminal receipt, and the one
/// transient item that folds the page — none when the projection already holds exactly this frontier.
pub struct DirectoryPageAnswer {
    pub receipt: DirectoryProjectionReceiptV1,
    pub item: Option<HomeTransientMutation>,
}

/// 📬️ Judges one canonical page against `directory` without folding it; the fold happens once, as the transient item.
pub fn directory_page_answer(page_json: &str, directory: &HomeDirectoryProjection) -> Result<DirectoryPageAnswer, Fault> {
    let page = store::os_directory::DirectoryEventPageV1::parse_canonical_json(page_json).map_err(|_| Fault::new(FaultOrigin::App, "s.home.directory-event-page-invalid", "the page is not a canonical sealed directory page"))?;
    let item = match directory.admit_page(&page)? {
        DirectoryPageAdmission::Held => None,
        DirectoryPageAdmission::Fold => Some(ApplyDirectoryPage { page_json: page_json.to_owned() }.into()),
    };
    Ok(DirectoryPageAnswer { receipt: DirectoryProjectionReceiptV1::of_page(&page), item })
}
//#endregion 🔖️Handle

//#region 🔖️Work
/// 📬️ The retained page route of BOTH Home surfaces (the editor's `applyDirectoryEventPage` and the read-only viewer's):
/// one step, which requires the signed-in human, reads the projection from the job's captured transient snapshot and
/// completes with the receipt event plus at most one transient item. The viewer lists exactly the rows the editor lists
/// because both fold the same pages through this one route (ticket 26/09/23 S16).
pub struct HomeDirectoryPageWork<A: ArtifactApp> {
    tool_id: &'static str,
    page_json: fn(&A::Command) -> Option<&str>,
    consumed: bool,
}

impl<A: ArtifactApp> HomeDirectoryPageWork<A> {
    /// 📬️ The route for `tool_id`, reading the page out of the surface's own command with `page_json`.
    pub fn new(tool_id: &'static str, page_json: fn(&A::Command) -> Option<&str>) -> Self {
        Self { tool_id, page_json, consumed: false }
    }
}

impl<A> ArtifactCommandWork<A> for HomeDirectoryPageWork<A>
where
    A: ArtifactApp<Transient = HomeTransient, TransientMutation = HomeTransientMutation>,
{
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, command: &A::Command, _snapshot: &A::Snapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<A>>) -> Option<usize> {
        (self.page_json)(command).filter(|page_json| page_json.len() <= HOME_DIRECTORY_PAGE_BYTES).map(|_| 1)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, A>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<A>, Fault> {
        if self.consumed {
            return Err(Fault::new(FaultOrigin::App, "s.home.directory-event-page.work-repeated", "a directory page work steps once"));
        }
        self.consumed = true;
        let context = input.context.ok_or_else(|| Fault::new(FaultOrigin::App, "s.home.directory-event-page.context-missing", "a directory page needs the job's captured transient projection"))?;
        context.view_state.as_ref().and_then(crate::home_session_identity).ok_or_else(|| Fault::new(FaultOrigin::App, "s.home.session-identity-required", "a directory page needs a signed-in session"))?;
        let page_json = (self.page_json)(input.command).ok_or_else(|| Fault::new(FaultOrigin::App, "s.home.directory-event-page-input-missing", "the command carries no page"))?;
        let answer = directory_page_answer(page_json, context.transient.directory())?;
        let event = AppEvent { kind: DirectoryProjectionReceiptV1::SCHEMA.into(), payload: semio_framework_value::ToValue::to_value(&answer.receipt) };
        let emit = Emit { events: vec![event], ..Default::default() };
        let ephemeral = EphemeralEmit { presence: Vec::new(), transient: answer.item.into_iter().collect(), window_transient: Vec::new() };
        Ok(ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral })
    }
}
//#endregion 🔖️Work

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
