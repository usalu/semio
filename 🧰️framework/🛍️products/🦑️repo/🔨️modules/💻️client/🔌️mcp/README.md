# Summary

The unified native `repo` executable serves MCP through `repo mcp`. `SEMIO_REPO_MCP_CLIENT` selects the client profile. Ordinary repository and build commands use the same binary; protocol traffic is confined to the explicit MCP route.

# Build surface

`repo mcp` exposes the build and dev surface as tools, so agents never shell out to build tools. Every exposed row of the tool-neutral operations catalogue (`📚️library/🏗️build/⚙️operations/🔣️catalog.json`) is one tool (`run`, `test`, `run_many`, `affected`, `exec`, `cargo`, `show_projects`, `show_project`, `show_targets`, `graph`, `inputs`, `leases_status`, `capacity_status`, `cache_status`, `incremental_*`, `concurrency_*`, ...). Name, input schema, argument vector, English and German description, annotations and task support are derived from the catalogue and one authored exposure policy (`🔌️mcp/🖼️assets/🏗️build-surface.json`, schema `🔌️mcp/🧬️schema/🏗️build-surface.json`); a catalogue row that is neither exposed nor excluded with a reason in both languages fails the start of the tools and the tests. The resource `repo://build-surface` lists what is exposed and what is left out.

Each call runs the `repo` executable itself without a shell, with no standard input and a cleaned environment, and answers with one structured result (`$defs.Result`): exit code, exact command, parsed JSON or the bounded output tail, agent, progress count and hints. It reports `notifications/progress` from the child's typed `[script]` lines, stops the child when the call is cancelled (interrupt, then kill after the grace period) and also runs as an MCP task (`tools/call` with `task`, `tasks/get|result|list|cancel`). Graph queries read the sealed graph by default (`graph_policy` current) and never wait for a rebuild. The argument `agent` (default: the agent of the connection) names the agent session of the call: the server asks `repo session open --json` once per agent (the client's own session variable, the connection, or the explicit name decide the identity) and passes only the session address as `SEMIO_AGENT_SESSION` to the child, so leases, capacity grants, cache lanes, Bazel slots, test sandboxes and compiler sessions attribute to that agent. The tools `session_open`, `session_status`, `session_list` and `session_close` expose the session itself. `tool`, `workers` and `language` select the build tool, the workers and the text language through `SEMIO_BUILD_TOOL`, `SEMIO_BUILD_WORKERS` and `SEMIO_LANGUAGE`.

The server keeps a ring event log (a long session never loses its connection to a full log), isolates a panicking tool, interrupts its children when the connection ends and prints one readiness line on stderr.

# Build-Oberfläche

`repo mcp` stellt die Build- und Dev-Oberfläche als Werkzeuge bereit, damit Agenten nie Build-Werkzeuge direkt aufrufen. Jede freigegebene Zeile des werkzeugneutralen Operationskatalogs ist ein Werkzeug; Name, Eingabeschema, Argumentvektor, englische und deutsche Beschreibung, Hinweise und Task-Unterstützung werden aus dem Katalog und einer geschriebenen Freigabe-Policy abgeleitet. Eine Katalogzeile, die weder freigegeben noch mit Begründung in beiden Sprachen ausgeschlossen ist, lässt Start und Tests der Werkzeuge scheitern. Jeder Aufruf liefert ein strukturiertes Ergebnis, meldet Fortschritt, endet bei Abbruch und ordnet Leases, Kapazitätszusagen und Cache-Bahnen dem aufrufenden Agenten zu.

# Docs

## 🧬️schema

Scope `repo.client.mcp` (`https://json.schemas.assets.semio-tech.com/repo/client/mcp/schema.json`).

- `🧬️schema/🔗️.graphql` — the normative SDL: the query, mutation, input, enum and object types the CLI
  executor answers.
- `🧬️schema/🔣️.json` — draft-07 facet of the same 62 exports, so a resolved selection or a mutation
  input can be validated outside a GraphQL runtime. Field arguments live in the SDL only; a JSON
  instance is one already-resolved selection, so every object field is optional and nullability is
  carried by the field's own shape.

Export names, field names, field order, field nullability and the required set of every `input` are kept
identical in both files by `🧪️tests/🔬️schema/🟦️.ts`, which also compiles the JSON facet under ajv.

## 🧪️tests

- `🤝️protocol-contract/🐹️.go` — MCP wire protocol contract.
- `🧰️protocol-harness/🐹️.go` — shared harness of the protocol contract cases.
- `🚦️protocol-lifecycle/🐹️.go` — lifecycle and pagination.
- `🛑️protocol-cancellation/🐹️.go` — cancellation, pipe saturation and reconnect.
- `📏️protocol-bounds/🐹️.go` — payload, nesting, identifier and cursor bounds.
- `📈️protocol-progress-replay/🐹️.go` — progress, dropped peers, error envelopes and deterministic replay.
- `🔬️schema/🟦️.ts` — SDL ↔ JSON Schema parity and the draft-07 oracle.

# 💯️Requirements

The GraphQL surface is a contract of this module, not of the CLI binary: `🔗️graphql/📦️packages/🐹️go`
implements the executor and the CLI imports it, the canonical GraphQL operation documents validate against it, and neither may add a field that this
module does not declare.
