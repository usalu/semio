# 🎬️ Workflow Planner Testing Examples

Each `🔣️.json` file is a plain testing example containing an actual workflow graph, dirty node IDs and expected deliveries. Fixtures have no artifact identity, independent schema or production codec operations.

Rust and TypeScript replay all five examples and compare exact delivery order. TypeScript independently compares delivered edge sets with graphology reachability. Rust checks JSON with serde_json and exercises the canonical `WorkflowSnapshot` DSL/pack round trip using each witness graph.

The runtime workflow document and its schema live under `🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow`; test metadata stays here.
