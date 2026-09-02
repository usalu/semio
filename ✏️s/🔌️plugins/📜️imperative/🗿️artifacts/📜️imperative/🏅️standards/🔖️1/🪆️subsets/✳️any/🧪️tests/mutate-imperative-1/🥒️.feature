@capability-imperative-1-mutate
@no-oracle-imperative-1-nested-step-list-mutation-semantics
@comparison-ordered-json-v1
@mutations-imperative-1-any
Feature: Apply every typed imperative-program mutation to its committed vector and for real
  `imperative.document` is a semio-NATIVE program document. Nothing third-party reads
  `.imperative.dsl.semio`, so no reference library is registered — recorded as the
  `imperative-1-nested-step-list-mutation-semantics` no-oracle decision in
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json`.

  ⚠️ THIS NO-ORACLE DECISION IS A DEBT, NOT A VERDICT, and is recorded as one. Declining a third-party
  LIBRARY is a different judgement from declining a SECOND IMPLEMENTATION, and only the first was ever
  made here. `mutate-writer-1` and `mutate-playbook-1` took Python second
  implementations over this same `.dsl.semio` carrier in this wave, so the same is writable for this
  subset from `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`, the rules of
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` and the
  committed vectors this feature already replays. What blocks it TODAY is stated in the decision and
  is one edit: this case's vectors are not declared as `asset://` fixtures — the `Examples` table
  carries the payloads inline and the adapter reads the committed files through `include_str!` — so
  the plan pins none of their digests and a Python reference cannot read them at all. Separately, `identity-round-trip` would still be refused: this subset's committed
  snapshot text grammar is the repository-wide placeholder `payload = OCTET+`, whose header production
  declares `"schema" SP "stdio.json"` against an artifact whose own first line says otherwise.
  Until that is done, every assertion below still lives in the SUBJECT role, and the ceiling is the
  one this decision has always had: no second producer runs beside it, so a mistake shared by the
  handcrafted vector and the production code passes unseen.

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

  Because this case records a no-oracle decision the runner executes NO oracle role, so every
  assertion lives inside the subject handler. A handler that merely ran the mutation and returned
  would report a pass having checked nothing.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to its committed vector and then for real
    Given the committed specification vector for the <id> kind and its cached program
    When <id> is replayed against its vector and then applied for real
      """
      {"kind": "<id>", "code": "<code>", "level": "<level>", "program": <program>, "params": <params>}
      """
    Then the vector reports exactly <code> at <level> with the flow handle untouched, and the real application moves the handle
    Examples:
      | id               | code                    | level   | program | params |
      | create-step      | mutation.duplicate-id   | Fatal   | {"steps": [{"id": "step-1", "kind": "log.print", "params": {}, "bodies": {}}, {"id": "step-2", "kind": "log.print", "params": {}, "bodies": {}}]} | {"mutation": "createStep", "pathRef": {}, "step": {"id": "step-9", "kind": "log.print", "params": {}, "bodies": {}}} |
      | delete-step      | mutation.target-missing | Error   | {"steps": [{"id": "step-1", "kind": "log.print", "params": {}, "bodies": {}}, {"id": "step-3", "kind": "control.if", "params": {}, "bodies": {"then": {"steps": [{"id": "step-3a", "kind": "log.print", "params": {}, "bodies": {}}]}}}]} | {"mutation": "deleteStep", "pathRef": {"owner": "step-3", "slot": "then"}, "id": "step-3a"} |
      | reorder-steps    | mutation.no-op          | Warning | {"steps": [{"id": "step-1", "kind": "log.print", "params": {}, "bodies": {}}, {"id": "step-2", "kind": "log.print", "params": {}, "bodies": {}}, {"id": "step-3", "kind": "log.print", "params": {}, "bodies": {}}]} | {"mutation": "reorderSteps", "pathRef": {}, "id": "step-3", "toIndex": 0} |
      | edit-step-params | mutation.no-op          | Warning | {"steps": [{"id": "step-1", "kind": "log.print", "params": {"message": "Guten Tag"}, "bodies": {}}, {"id": "step-2", "kind": "log.print", "params": {}, "bodies": {}}]} | {"mutation": "editStepParams", "pathRef": {}, "id": "step-1", "newParams": {"message": "Gruezi"}} |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the seeded program
    Given the committed before-snapshot for the <id> kind and its cached program
    When the real <id> payload is applied to it and then its own computed inverse steps are applied
      """
      {"kind": "<id>", "code": "<code>", "level": "<level>", "program": <program>, "params": <params>}
      """
    Then the document equals the before-snapshot again, flow handle included — which for a content-addressed child means the whole program came back
    Examples:
      | id               | code                    | level   | program | params |
      | create-step      | mutation.duplicate-id   | Fatal   | {"steps": [{"id": "step-1", "kind": "log.print", "params": {}, "bodies": {}}, {"id": "step-2", "kind": "log.print", "params": {}, "bodies": {}}]} | {"mutation": "createStep", "pathRef": {}, "step": {"id": "step-9", "kind": "log.print", "params": {}, "bodies": {}}} |
      | delete-step      | mutation.target-missing | Error   | {"steps": [{"id": "step-1", "kind": "log.print", "params": {}, "bodies": {}}, {"id": "step-3", "kind": "control.if", "params": {}, "bodies": {"then": {"steps": [{"id": "step-3a", "kind": "log.print", "params": {}, "bodies": {}}]}}}]} | {"mutation": "deleteStep", "pathRef": {"owner": "step-3", "slot": "then"}, "id": "step-3a"} |
      | reorder-steps    | mutation.no-op          | Warning | {"steps": [{"id": "step-1", "kind": "log.print", "params": {}, "bodies": {}}, {"id": "step-2", "kind": "log.print", "params": {}, "bodies": {}}, {"id": "step-3", "kind": "log.print", "params": {}, "bodies": {}}]} | {"mutation": "reorderSteps", "pathRef": {}, "id": "step-3", "toIndex": 0} |
      | edit-step-params | mutation.no-op          | Warning | {"steps": [{"id": "step-1", "kind": "log.print", "params": {"message": "Guten Tag"}, "bodies": {}}, {"id": "step-2", "kind": "log.print", "params": {}, "bodies": {}}]} | {"mutation": "editStepParams", "pathRef": {}, "id": "step-1", "newParams": {"message": "Gruezi"}} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Read the real committed program document through its own DSL carrier and print it back
    Given the real committed text artifact asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    When the artifact is parsed, printed back to `.imperative.dsl.semio` and parsed again
    Then every decoding agrees on the same two composed children — an `s.stdio.semio@v1/flow` program and an `s.stdio.semio@v1/text` narrative — and the printed text reproduces the committed file byte for byte
