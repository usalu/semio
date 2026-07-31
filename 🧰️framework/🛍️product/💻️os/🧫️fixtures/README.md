# 🎬️ Media-flow planner fixtures

Scripted `OsWorkflow` + dirty-instance-set vectors that keep `plan_workflow` (Rust,
`framework/product/os/core/rs/lib.rs`) and `planWorkflow` (TS twin, `framework/product/os/core/js/index.ts`)
in lockstep. Both sides replay every `*.dsl` file here and assert an **exact-order** match against
`expected-deliveries` — both implementations are deterministic (topological DFS driven purely by
`graph.nodes`/`graph.edges` insertion order, no hashmap-iteration-order dependence), so exact order is a
meaningful assertion, not a flaky one.

- **Rust** — `framework/product/os/core/rs/lib.rs` `workflow::tests::workflow_fixtures_match_expected_deliveries`.
- **TypeScript** — `framework/product/os/core/js/index.ts`'s `import.meta.vitest` block, `"planWorkflow matches shared fixtures"` (decodes the `.spk` sibling via a wasm export — no JSON on either side).

## Format (`WorkflowFixture`)

Each fixture is a `dsl::DslDocument` (`WorkflowFixture` in `lib.rs`, `#[dsl(extension = "workflow-fixture")]`)
shipped as a `.dsl`/`.spk` pair — the repo's constitutional text/binary document representation, not JSON.
`workflow_fixture_dsl_and_spk_pairs_are_canonical_and_equivalent` asserts both files decode to the identical
document, the `.dsl` text is already its own canonical `print_dsl` fixpoint, and the `.spk` bytes match a
fresh canonical `encode_pack()` byte-for-byte.

```
name=single-edge
graph {
  schema=s.workflow
  nodes [id:TEXT instance-id:TEXT x:NUM y:NUM width:NUM height:NUM inputs:LIST outputs:LIST] {
    node-1 app-1 0 0 160 72 [ id="app-1:in" artifact-kind="2d.drawing" direction=in ] [ id="app-1:out" artifact-kind="2d.drawing" direction=out ]
    node-2 app-2 200 0 160 72 [ id="app-2:in" artifact-kind="2d.drawing" direction=in ] [ id="app-2:out" artifact-kind="2d.drawing" direction=out ]
  }
  edges [id:TEXT source-node-id:TEXT source-port-id:TEXT target-node-id:TEXT target-port-id:TEXT contract:BLOCK] {
    edge-1 node-1 "app-1:out" node-2 "app-2:in" {
      kind_id="2d.drawing" class=data form=value wire_kind=document wire_schema="2d.drawing"
    }
  }
}
dirty-instance-ids=[ app-1 ]
expected-deliveries [edge-id:TEXT producer-instance-id:TEXT producer-port-id:TEXT consumer-instance-id:TEXT consumer-port-id:TEXT] {
  edge-1 app-1 "app-1:out" app-2 "app-2:in"
}
```

Field grammar mirrors `OsWorkflow`/`OsWorkflowNode`/`OsWorkflowEdge`/`OsMediaPort`/`WorkflowDelivery`'s
`dsl::DslRecord` derives; `contract` uses `MediaContract`'s hand-written `dsl::DslField` (see
`🔖️MediaContractDsl` in `lib.rs`) — the same grammar as `s/plugin/space/example/✏️demo.s`'s `workflow` block.

Regenerating a fixture (e.g. after a grammar change) means constructing the `WorkflowFixture` value in Rust
and writing `store::DocumentDsl::print_dsl`/`store::DocumentPack::encode_pack` back to disk — never hand-edit
the `.spk` bytes, and never hand-format the `.dsl` text without re-running it through the printer (the
canonical-fixpoint test will catch drift either way).
