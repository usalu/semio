# Design Notes

## Preserved Source Rationale

<a id="source-notes-1"></a>

### 🦀️.rs:1

🧬️ The `os.mcp` schema registry — the ONE place this crate declares a schema.

It owns three families, all published through [`schemas`]: the gateway wire types
(`📋️master.md` §"MCP tool names"/§"Observe"/§"Verify"/§"Idempotency", `schemars`-derived from the
structs below and validated at kernel/backend boundaries the same way
`🧰️framework/🔨️modules/🧬️schema` validates artifact snapshots); the MCP protocol types, mirrored
from `🧭️protocol/🦀️.rs`; and every tool `inputSchema`/`outputSchema` shape the facets stamp a
`semio://capability/{id}/{input|output}` `$id` onto ([`🔖️ToolSchemas`]). No facet under `🌉️mcp/`
writes a schema literal of its own.

[`schema_mirror_document`] projects the whole registry into the committed draft-07 document
`🧬️schema/🔣️.json`, from which `🧬️schema/🟦️.ts` is generated
(`repo run @semio-tech/framework-os-mcp-rs:schema-mirror`, whose emitter is `semio-os-mcp
schemas`). `GatewayError` is re-exported from `crate::errors` rather than redefined here — one
type, one owning facet.
