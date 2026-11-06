# Design Notes

## Preserved Source Rationale

<a id="source-notes-154"></a>

### 🦀️.rs:154

✂️ Splits a window's top-level measures into `(general, utility_options)`.

A top-level [`WindowMeasure::Group`] tagged with `active_utility_id: Some(id)` is *utility-scoped chrome*:
its **children** land in `utility_options` **only** when `id == active_utility_id`, and the tagged wrapper
is dropped from both buckets otherwise (it is irrelevant to whichever utility — or no utility — is
currently active). The wrapper itself is a routing envelope only — never rendered — so activating a
utility shows its option tree directly (no duplicate utility-name group header). Every untagged group
and every non-group top-level measure stays in `general`, unchanged. Tagging is a top-level concept only.
