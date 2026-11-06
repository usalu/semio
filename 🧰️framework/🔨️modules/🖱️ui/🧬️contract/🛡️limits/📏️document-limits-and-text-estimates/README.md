# Design Notes

## Preserved Source Rationale

<a id="source-notes-5"></a>

### 🦀️.rs:5

#region 🔖️Limits
🛡️ Quotas a [`crate::UiSnapshot`]/[`crate::UiPatch`] must stay within. `max_nodes`/`max_depth` bound
the document shape and are enforced by [`validate_snapshot`] (surfaced as
[`UiContractViolation::NodeQuota`]/[`UiContractViolation::DepthQuota`]); `max_children`/
`max_text_bytes`/`max_patch_ops`/`max_patch_bytes` bound one incoming [`crate::UiPatch`] and are
enforced directly by [`apply_patch`] (surfaced as [`PatchRejection::QuotaExceeded`]) — rejecting a
patch before it is even applied to the shadow draft is cheaper than discovering the violation after.

<a id="source-notes-49"></a>

### 🦀️.rs:49

📏️ Intake steps ONE retained node costs at its first publication, measured with ~1.8× headroom.

Every renderer drives the retained intake one PHASE per `advance` — a LEB128 byte, a text body, an
attach — so a fixed step count caps a surface patch at a few KiB no matter how large the grant is.
Both the React and the wgpu targets therefore price an intake through
[`UiDocumentLimits::retained_ui_intake_step_ceiling`] instead. The numbers are declared once,
language-agnostically, in `🧵️retained/🧪️tests/🧫️fixtures/📥️intake/🔣️.json`'s `budget` and mirrored by the
TypeScript twin `RETAINED_UI_INTAKE_STEPS_PER_NODE`
(`📃️UiDocumentStore/📥️intake/🟦️.ts`).
