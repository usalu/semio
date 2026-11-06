# Design Notes

## Preserved Source Rationale

<a id="source-notes-94"></a>

### 🦀️.rs:94

⏭️ Admits backing pages until one slot is free — the page-exact reservation every retained
builder performs by hand (`UiPatchOps::try_push`), so a cold push never owns more than the next
page. A full-capacity reservation here priced one `ActionBinding` at 32 slots × 2 KiB × 3 census
copies (~240 KiB) and one `RowAction` at ~300 KiB, which capped a surface at ~16 actionable rows
of its 8 MiB reconcile budget (process3d workshop, 12 machines + 4 catalogs: `ui.surface-render
… bytes: 8390387`, ticket 26/09/15/DEV-PROCESS-REACT-E2E).
