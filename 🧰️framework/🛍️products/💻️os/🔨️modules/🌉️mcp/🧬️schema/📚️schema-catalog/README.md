# Design Notes

## Preserved Source Rationale

<a id="source-notes-5"></a>

### 🦀️.rs:5

#region 🔖️SchemaCatalog
📋️ `(ExportId, JSON Schema document)` for every schema the `os.mcp` scope owns — the ONE
registry every facet reads from and the normative source `🧬️schema/🔣️.json` (draft-07) and
`🧬️schema/🟦️.ts` are generated from (`repo run @semio-tech/framework-os-mcp-rs:schema-mirror`).
Three families, one catalogue:

1. **gateway wire types** — `schemars`-derived from the structs above (`RevisionStamp` …
   `GatewayError`);
2. **MCP protocol types** — `schemars`-derived from `🧭️protocol/🦀️.rs`. These MIRROR the external
   Model Context Protocol specification rather than defining it; they are catalogued so the
   surface this crate speaks is visible, never invisible;
3. **tool schemas** — the hand-authored `inputSchema`/`outputSchema` shapes every facet stamps a
   `$id` onto (`🔖️ToolSchemas` above), plus the [`hub_inference_approval_request_schema`]
   mirror of hub's own approval contract.

<a id="source-notes-101"></a>

### 🦀️.rs:101

🪞️ The whole `os.mcp` scope as ONE draft-07 document — the exact bytes
`🧬️schema/🔣️.json` carries, emitted by `semio-os-mcp schemas` and written by
`repo run @semio-tech/framework-os-mcp-rs:schema-mirror`.

Three real conversions turn [`schemas`]'s wire documents into `$defs` entries, none of them a
relabelling: the per-entry dialect/`$id` header is dropped (a `$defs` member may declare
neither), `schemars`' own `definitions` bucket is hoisted to the document's single `$defs` (every
hoisted name must already BE a registry export, or this panics rather than publishing a schema
nothing catalogues), and every `#/definitions/X` reference is repointed at `#/$defs/X`.

<a id="source-notes-163"></a>

### 🦀️.rs:163

🔢️ Replaces `schemars`' Rust-typed number `format` annotations with the draft-07 semantics they
stand for, because they are the ONE thing in these documents a strict validator cannot read:
`new Ajv({ strict: true })` throws `unknown format "double"` and refuses to compile the whole
mirror (observed live running `repo ./📜️script.ts schema-mirror`). An unsigned format becomes a
real `minimum: 0`; a signed/floating one carries no constraint JSON can express and is dropped.
Only the generated mirror is normalized — `schemas()` keeps the annotation for the MCP wire,
whose SDK validators accept it.
