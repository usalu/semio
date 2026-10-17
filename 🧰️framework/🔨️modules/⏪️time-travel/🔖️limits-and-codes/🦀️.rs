//! 🔖️ Limits and stable refusal codes of the time-travel session: the text ceiling, the fault and refusal code constants and their two classifiers. See [`super`].


//#region 🔖️Limits
/// 📏️ UTF-8 byte ceiling of a fault code and of an alternative name.
pub const TIME_TRAVEL_TEXT_MAX_BYTES: usize = 256;

/// 🔒️ Refusal code of an artifact-lane app command while a session is not `Inactive` (issued by the host).
pub const TIME_TRAVEL_FROZEN_CODE: &str = "timeTravel.frozen";

/// 🧯️ Fault a cancelled replay leaves behind, so hosts can label it and offer [`TimeTravelEvent::Rerun`].
pub const TIME_TRAVEL_CANCELLED_CODE: &str = "timeTravel.cancelled";

/// 🛑️ Host refusal: a mutating tool run, an agent transaction or a session on another store holds the instance.
pub const TIME_TRAVEL_BUSY_CODE: &str = "timeTravel.busy";

/// 👻️ Host refusal: the named mutation is not (or no longer) applied in the edited store.
pub const TIME_TRAVEL_UNKNOWN_MUTATION_CODE: &str = "timeTravel.unknown-mutation";

/// 🔐️ Host refusal: the mutation declares no input schema or emits foreign steps.
pub const TIME_TRAVEL_NOT_EDITABLE_CODE: &str = "timeTravel.not-editable";

/// 🔍️ Host refusal: the input pointer addresses no input of the edited mutation.
pub const TIME_TRAVEL_UNKNOWN_INPUT_CODE: &str = "timeTravel.unknown-input";

/// ❎️ Host refusal: the value does not take the input's shape or fails its payload schema; the draft is kept.
pub const TIME_TRAVEL_INVALID_INPUT_CODE: &str = "timeTravel.invalid-input";

/// 🫥️ Host refusal: the input's selection domain holds nothing at its granularity.
pub const TIME_TRAVEL_NO_SELECTION_CODE: &str = "timeTravel.no-selection";

/// 🖊️ Host refusal: a new alternative needs a name and no locale supplied the default.
pub const TIME_TRAVEL_NAME_REQUIRED_CODE: &str = "timeTravel.name-required";

/// 🔤️ Host refusal: the alternative name is blank or longer than [`TIME_TRAVEL_TEXT_MAX_BYTES`].
pub const TIME_TRAVEL_NAME_INVALID_CODE: &str = "timeTravel.name-invalid";

/// 🧬️ Host refusal: the payload schema compiles no validator or describes no inputs, so no draft is admitted.
pub const TIME_TRAVEL_SCHEMA_UNAVAILABLE_CODE: &str = "timeTravel.schema-unavailable";

/// 💥️ Driver fault: the store refused or broke the Report replay.
pub const TIME_TRAVEL_REPLAY_FAULTED_CODE: &str = "timeTravel.replay-faulted";

/// 🧨️ Driver fault: the finalize commit failed for a reason other than a stale base or a blocking report.
pub const TIME_TRAVEL_COMMIT_FAILED_CODE: &str = "timeTravel.commit-failed";

/// 🧩️ Driver fault: the composed member store the session edits was closed mid-session.
pub const TIME_TRAVEL_MEMBER_GONE_CODE: &str = "timeTravel.member-gone";

/// 🪨️ Host refusal: the store's supersede law does not admit withdrawing the mutation.
pub const TIME_TRAVEL_NOT_WITHDRAWABLE_CODE: &str = "timeTravel.not-withdrawable";

/// 🥅️ Host refusal: a draft verb arrived while no draft editor is open on the session's mutation.
pub const TIME_TRAVEL_EDITOR_CLOSED_CODE: &str = "timeTravel.editor-closed";

/// 🔣️ A fault code is non-empty, whitespace-free and at most [`TIME_TRAVEL_TEXT_MAX_BYTES`].
pub fn is_time_travel_fault_code(code: &str) -> bool {
    !code.is_empty() && code.len() <= TIME_TRAVEL_TEXT_MAX_BYTES && !code.chars().any(char::is_whitespace)
}

/// 🪪️ An alternative name is not blank (Unicode `White_Space`) and at most [`TIME_TRAVEL_TEXT_MAX_BYTES`].
pub fn is_time_travel_alternative_name(name: &str) -> bool {
    name.len() <= TIME_TRAVEL_TEXT_MAX_BYTES && name.chars().any(|character| !character.is_whitespace())
}
//#endregion 🔖️Limits
