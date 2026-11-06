# Design Notes

## Preserved Source Rationale

<a id="source-notes-5"></a>

### 🦀️.rs:5

#region 🔖️Audit
🖱️ Verb-id word runs that NAME a raw live-surface gesture route. A route whose id reads like one
of these is dispatched by a cursor, a key or an engagement draft — never by an agent choosing a
tool — but `manifest::derive_audience` cannot see that: a pointer handler that commits a real
operation is `ActionKind::Mutation`, structurally identical to a panel-dispatched `patchLayer`.
So the lexicon is not a classifier — [`audit_source`] uses it only to ask the question the
derivation cannot: "this id names an event; did anyone actually declare its audience?".
