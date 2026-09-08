# Summary

MCP server exposing the repo CLI's GraphQL surface to agent clients.

# Docs

## 🧬️schema

Scope `repo.client.mcp` (`https://semio.tech/schema/repo/client/mcp/schema.json`).

- `🧬️schema/🔗️.graphql` — the normative SDL: the query, mutation, input, enum and object types the CLI
  executor answers and the VS Code extension's typed documents are written against.
- `🧬️schema/🔣️.json` — draft-07 facet of the same 62 exports, so a resolved selection or a mutation
  input can be validated outside a GraphQL runtime. Field arguments live in the SDL only; a JSON
  instance is one already-resolved selection, so every object field is optional and nullability is
  carried by the field's own shape.

Export names, field names, field order, field nullability and the required set of every `input` are kept
identical in both files by `🧪️tests/🔬️schema/🟦️.ts`, which also compiles the JSON facet under ajv.

## 🧪️tests

- `🤝️protocol-contract/🐹️.go` — MCP wire protocol contract.
- `🔬️schema/🟦️.ts` — SDL ↔ JSON Schema parity and the draft-07 oracle.

# 💯️Requirements

The GraphQL surface is a contract of this module, not of the CLI binary: `⌨️cli/internal/graphql`
implements the executor, `🧩️vscode` writes documents against it, and neither may add a field that this
module does not declare.
