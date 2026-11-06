# Design Notes

## Preserved Source Rationale

<a id="source-notes-157"></a>

### 🦀️.rs:157

🏷️ EN/DE display label for a `menu.group.<category>` row, which the guest emits with `label: None`
by contract (the host owns the chrome vocabulary). Taxonomy categories resolve through
[`ribbon_parent_label`]; [`CONTEXT_MENU_OVERFLOW_CATEGORY`] — the one id outside the closed 20-id
taxonomy, and therefore the one that used to render with an EMPTY label — resolves here. `None`
for a row that is not a group row, or a group row whose category is unknown. Twin of ui-react's
`contextMenuGroupLabel` (`🛠️ShellHelpers/🟦️.tsx`), which reads `ui.contextMenu.more` for the same id.

<a id="source-notes-277"></a>

### 🦀️.rs:277

💸️ >9-interactive-row emission (D2 rule): the first 5 plain leaves outside any header section stay
primaries; every header's trailing run of leaves folds into a group keyed by that header's own
(slugified) label; every other plain leaf buckets into `menu.group.<category_of(action) ?? "actions">`;
pre-existing group rows pass through unchanged; groups then sort in taxonomy order and, if the
primaries+groups row count is still over budget, the excess trailing groups fold into one
`menu.group.more`. Destructive leaves are carried separately and appended last, after a separator.

<a id="source-notes-351"></a>

### 🦀️.rs:351

🗃️ Pure organizer enforced at every context-menu funnel (SDK `VcsArtifactApp::context_menu`, shell
builders) — recurses into `children`, normalizes separators (labeled = kept header, bare
leading/trailing/doubled = dropped), merges duplicate `menu.group.<category>` rows (deduping their
children by id), then applies the ≤9-row / >9-row emission policy from D2 of the grouped-context-menu
mechanism design (`context_menu_emit_within_budget`/`context_menu_emit_over_budget`).
`category_of` resolves a leaf's dispatched action id to a `RIBBON_PARENT_CATEGORIES` id (`None`
buckets into `"actions"`) — pass `AppActionRegistry::category_of` at the SDK funnel, or
`ActionDefinition.category` lookups in shell builders.

<a id="source-notes-391"></a>

### 🦀️.rs:391

🔨️ Shell fallback context menu (window-background right-click, "no plugin resolved a menu") builder
— filters to `in_palette` actions, emits one leaf per action (an `arg_carrying` action routes
through the reserved `"shell.openActionPane"` action id with `{"actionId": id}` args so the host can
prompt for arguments before dispatch), appends a fixed `"shell.openPalette"` leaf when
`include_palette`, then runs the whole thing through `organize_context_menu` (D5) exactly like every
plugin-emitted menu.
