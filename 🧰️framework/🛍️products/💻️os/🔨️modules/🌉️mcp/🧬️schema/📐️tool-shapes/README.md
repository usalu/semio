# Design Notes

## Preserved Source Rationale

<a id="source-notes-8"></a>

### 🦀️.rs:8

🧷️ Rewrites every boolean sub-schema into its object equivalent, in place and recursively.

JSON Schema 2020-12 allows `true`/`false` as complete schemas, and `schemars` emits `true` for a
free-form `serde_json::Value` field. The MCP SDKs, however, validate tool `inputSchema`/
`outputSchema` with a Zod model that requires every sub-schema to be an **object** — a bare
`true` makes the official client reject the whole `tools/list` response with a `$ZodError`
(observed against `@modelcontextprotocol/sdk` 1.30.0 on `action_prepare.outputSchema.properties
.preview`). `{}` and `{"not": {}}` are the semantically identical object forms, so this narrows
the encoding without changing what any schema accepts.

<a id="source-notes-42"></a>

### 🦀️.rs:42

🔁️ Converts a `schemars` 0.8 draft-07 document into JSON Schema 2020-12 in place.

MCP requires tool `inputSchema`/`outputSchema` to be 2020-12, but `schemars` 0.8 (the version
this workspace pins) emits draft-07. The two differ, for the struct shapes this crate mirrors,
only in where subschema definitions live and how they are referenced — so this is a real
conversion of those two things, not a relabelling of the `$schema` URI:
`definitions` → `$defs`, and every `#/definitions/X` reference → `#/$defs/X`.
Anything already declaring 2020-12 is left untouched.

<a id="source-notes-120"></a>

### 🦀️.rs:120

🧯️ Widens one tool's `outputSchema` to `anyOf [its success shape, the typed tool error, the result wrapper]`. The
official MCP SDK validates `structuredContent` against `outputSchema` on EVERY result, `isError`
ones included (`@modelcontextprotocol/sdk` 1.30.0 `Client.callTool`, measured 2026-09-26: an
`APPROVAL_REQUIRED` from `action_invoke` became a thrown `-32602 … must have required property
'affectedResources'`), so a schema that names only the success shape turns every typed error into
a client-side exception the agent never sees. The dialect, `$id`, `title` and the `$defs` every
`#/$defs/…` reference resolves against stay at the root. Idempotent.
