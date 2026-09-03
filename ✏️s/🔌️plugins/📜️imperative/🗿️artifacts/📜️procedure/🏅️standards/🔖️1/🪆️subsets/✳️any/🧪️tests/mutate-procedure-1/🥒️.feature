@capability-procedure-1-mutate
@oracle-procedure-1-python-independent
@comparison-ordered-json-v1
@mutations-procedure-1-any
Feature: Apply every typed imperative-program mutation to its committed vector, for real, and against an independent Python implementation
  `procedure.document` is a semio-NATIVE program document. Nothing third-party reads
  `.imperative.dsl.semio`, so no reference LIBRARY exists. The second producer a differential
  comparison needs is therefore a second IMPLEMENTATION, and `🐍️component.py` beside this file is it:
  all four kinds of this vocabulary, written in Python from this subset's own committed
  `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json` and each mutation's own payload
  schema, and from
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/📓️taxonomy.md`'s
  `create`/`delete`/`reorder`/`edit` verb entries and `📓️derivation-rules.md`'s collection recipes. It
  imports nothing from the Rust it judges and transliterates none of it. The no-oracle decision this
  replaces (`procedure-1-nested-step-list-mutation-semantics`) is narrowed to an empty
  `capabilities` list rather than deleted, because its own investigation remains the honest record of
  what was checked.

  All four committed vectors are DEGENERATE at the document projection — two refusals, two
  `Warning`-level no-ops — so the Python side computes and cross-checks the OUTCOME each vector
  commits to (`duplicate-id`, `target-missing`, both no-op guards) against the four addressed program
  trees, which this feature's own `Examples` tables below already state; the document snapshot itself
  never carries the step tree, only a content-addressed handle to it, so the Python side transcribes
  those same committed `program` values rather than reading them off a `📸️snapshot` fixture. Every
  `(before, mutation, after, outcome)` path each `<id>` names is now also declared as an `asset://`
  fixture under the matching leaf's own `🧪️tests/<fixture>/`, so the plan pins its digest and both
  implementations read the SAME committed bytes for the document-level projection.

  Two facts shape everything below. The document PERSISTS NO STEPS: it carries a schema string and
  two content-addressed child handles, one `s.stdio.semio.flow` and one `s.stdio.semio.text`, and
  the program lives in a working scene keyed by the flow handle. So the persisted projection moves
  if and only if the program moved — an exact observability surface — and a decoded before-snapshot
  stands for no program at all until one is cached against its handle. And the addressing is
  NESTED: every kind takes a `PathRef`, `{}` for the root program and `{"owner": "step-3", "slot":
  "then"}` for a branch body inside it, so the same step id in two scopes is two different targets.

  The `program` column is what the committed vector's flow handle stands for. Each triad leaf's own
  Rust test caches exactly this program before replaying its vector (see each
  `🧪️tests/<fixture>/🦀️component.rs::cached_program`); this feature restates it once, in the one
  place both legs of the scenario read it from, and the restatement is self-checking — a program
  that drifted from the leaf's would stop producing the committed diagnostic and the scenario would
  fail on the `code` column rather than pass quietly.

  All four committed vectors leave the document byte-identical, for two different reasons apiece: a
  duplicate id at the addressed path and a root id addressed inside a branch body are refusals, at
  Fatal and Error respectively, while a reorder index clamped past the end and a param edit that
  rewrites the value already present are `Warning`-level no-ops. The `code` and `level` columns
  carry that per kind and the handler requires the exact pair, because the document alone cannot
  tell the four apart. Each scenario then applies the real-effect payload in `params` to the same
  seeded program and requires the flow handle to MOVE — and `delete-step`'s payload deliberately
  deletes `step-3a` INSIDE the branch body its own vector was refused from, so the two legs test the
  two sides of the same scoping rule.

  `mutate-<id>`/`inverse-<id>` now dispatch BOTH an oracle role (the Python implementation, reached
  through this plugin's `oracleHostPackages` entry, comparing the OUTCOME each committed vector
  commits to) and a subject role (this repository's own real dispatch, unaffected by this change —
  the same `Given`/`When`/`Then` and the same docstring payload as before). A handler that merely ran
  the mutation and returned would report a pass having checked nothing.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to its committed vector and then for real
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🎯️outcome/🔣️.json
    And the committed specification vector for the <id> kind and its cached program
    When <id> is replayed against its vector and then applied for real
      """
      {"kind": "<id>", "code": "<code>", "level": "<level>", "program": <program>, "params": <params>}
      """
    Then the vector reports exactly <code> at <level> with the flow handle untouched, and the real application moves the handle, and the two implementations agree on the committed outcome
    Examples:
      | id               | dir                 | fixture                                                       | code                    | level   | program | params |
      | create-step      | 🌱create-step        | rejects-a-duplicate-step-id-at-the-root-path                  | mutation.duplicate-id   | Fatal   | {"steps": [{"id": "step-1", "kind": "log.print", "params": {}, "bodies": {}}, {"id": "step-2", "kind": "log.print", "params": {}, "bodies": {}}]} | {"mutation": "createStep", "pathRef": {}, "step": {"id": "step-9", "kind": "log.print", "params": {}, "bodies": {}}} |
      | delete-step      | 🗑️delete-step        | rejects-a-root-step-id-addressed-inside-a-branch-body         | mutation.target-missing | Error   | {"steps": [{"id": "step-1", "kind": "log.print", "params": {}, "bodies": {}}, {"id": "step-3", "kind": "control.if", "params": {}, "bodies": {"then": {"steps": [{"id": "step-3a", "kind": "log.print", "params": {}, "bodies": {}}]}}}]} | {"mutation": "deleteStep", "pathRef": {"owner": "step-3", "slot": "then"}, "id": "step-3a"} |
      | reorder-steps    | 🔀reorder-steps      | warns-that-an-over-clamped-index-leaves-the-tail-step-in-place | mutation.no-op          | Warning | {"steps": [{"id": "step-1", "kind": "log.print", "params": {}, "bodies": {}}, {"id": "step-2", "kind": "log.print", "params": {}, "bodies": {}}, {"id": "step-3", "kind": "log.print", "params": {}, "bodies": {}}]} | {"mutation": "reorderSteps", "pathRef": {}, "id": "step-3", "toIndex": 0} |
      | edit-step-params | 🔧edit-step-params   | warns-that-step-1-already-carries-the-requested-params        | mutation.no-op          | Warning | {"steps": [{"id": "step-1", "kind": "log.print", "params": {"message": "Guten Tag"}, "bodies": {}}, {"id": "step-2", "kind": "log.print", "params": {}, "bodies": {}}]} | {"mutation": "editStepParams", "pathRef": {}, "id": "step-1", "newParams": {"message": "Gruezi"}} |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the seeded program
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🎯️outcome/🔣️.json
    And the committed before-snapshot for the <id> kind and its cached program
    When the real <id> payload is applied to it and then its own computed inverse steps are applied
      """
      {"kind": "<id>", "code": "<code>", "level": "<level>", "program": <program>, "params": <params>}
      """
    Then the document equals the before-snapshot again, flow handle included — which for a content-addressed child means the whole program came back, and both implementations agree
    Examples:
      | id               | dir                 | fixture                                                       | code                    | level   | program | params |
      | create-step      | 🌱create-step        | rejects-a-duplicate-step-id-at-the-root-path                  | mutation.duplicate-id   | Fatal   | {"steps": [{"id": "step-1", "kind": "log.print", "params": {}, "bodies": {}}, {"id": "step-2", "kind": "log.print", "params": {}, "bodies": {}}]} | {"mutation": "createStep", "pathRef": {}, "step": {"id": "step-9", "kind": "log.print", "params": {}, "bodies": {}}} |
      | delete-step      | 🗑️delete-step        | rejects-a-root-step-id-addressed-inside-a-branch-body         | mutation.target-missing | Error   | {"steps": [{"id": "step-1", "kind": "log.print", "params": {}, "bodies": {}}, {"id": "step-3", "kind": "control.if", "params": {}, "bodies": {"then": {"steps": [{"id": "step-3a", "kind": "log.print", "params": {}, "bodies": {}}]}}}]} | {"mutation": "deleteStep", "pathRef": {"owner": "step-3", "slot": "then"}, "id": "step-3a"} |
      | reorder-steps    | 🔀reorder-steps      | warns-that-an-over-clamped-index-leaves-the-tail-step-in-place | mutation.no-op          | Warning | {"steps": [{"id": "step-1", "kind": "log.print", "params": {}, "bodies": {}}, {"id": "step-2", "kind": "log.print", "params": {}, "bodies": {}}, {"id": "step-3", "kind": "log.print", "params": {}, "bodies": {}}]} | {"mutation": "reorderSteps", "pathRef": {}, "id": "step-3", "toIndex": 0} |
      | edit-step-params | 🔧edit-step-params   | warns-that-step-1-already-carries-the-requested-params        | mutation.no-op          | Warning | {"steps": [{"id": "step-1", "kind": "log.print", "params": {"message": "Guten Tag"}, "bodies": {}}, {"id": "step-2", "kind": "log.print", "params": {}, "bodies": {}}]} | {"mutation": "editStepParams", "pathRef": {}, "id": "step-1", "newParams": {"message": "Gruezi"}} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Read the real committed program document through its own DSL carrier and print it back
    Given the real committed text artifact asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    When the artifact is parsed, printed back to `.imperative.dsl.semio` and parsed again
    Then every decoding agrees on the same two composed children — an `s.stdio.semio@v1/flow` program and an `s.stdio.semio@v1/text` narrative — and the printed text reproduces the committed file byte for byte
