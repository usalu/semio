@capability-s-home-1-mutate
@no-oracle-s-home-mutation-semantics
@comparison-ordered-json-v1
@mutations-s-home-1-any
Feature: Apply the one typed s.space.home mutation to its committed specification vector

  `s.space.home` is a semio-NATIVE artifact: nothing outside this repository reads `.shome.dsl.semio`
  or its `.pack.semio` twin, so no reference library is registered — recorded as the
  `s-home-mutation-semantics` no-oracle decision in
  `🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`, which also records why the candidate
  category is empty rather than merely unexplored.

  ⚠️ THIS NO-ORACLE DECISION IS A DEBT, NOT A VERDICT, and is recorded as one. Declining a third-party
  LIBRARY is a different judgement from declining a SECOND IMPLEMENTATION, and only the first was ever
  made here. `mutate-writer-1` and `mutate-playbook-1` took Python second
  implementations over this same `.dsl.semio` carrier in this wave, so the same is writable for this
  subset from `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️component.json`, the rules of
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

  What distinguishes this subset is how little of it is mutable. The launcher document persists two
  fields, `schema` and `catalog_generation`, of which only the second may ever be written by a
  mutation — so the whole vocabulary is one root-scalar setter with no id, no path and no collection
  to address. The counter is not authored content: it is the re-materialize trigger the studio list
  watches, bumped after a create, import or delete on the catalog port. Because the verb is a SETTER,
  its inverse cannot be derived structurally from the diff; it has to read the previous value out of
  BASE and re-pin it, and that is the single most likely thing for an implementation to get wrong.

  The committed specification vector is chosen against exactly that hazard. It pins the counter from
  3 to 7, not from 0 to 1, so an implementation that INCREMENTED rather than SET lands on 4 and the
  forward scenario fails; and the leaf's only guard is an equal-counter `mutation.no-op` warning, so
  the vector's declared message-free `applied` outcome is itself an assertion that 3 and 7 were
  compared rather than assumed. The committed diff pins that exactly one of `SHomeDiff`'s four
  optional fields is written: an artifact-lane counter pin that reached into `activePanelTab` or
  `locale` would be a config-lane leak and shows up as a diff divergence.

  The identity round trip reads the artifact's own committed demo example, a 40-byte document that is
  the smallest complete `.dsl.semio` envelope in this plugin — a preamble line plus one body line
  carrying `schema` and `gen`.

  Where the assertions live. This case records a no-oracle decision, so the runner dispatches NO
  oracle role at all: `oracleDecision` resolves an oracle implementation from an `@oracle-` tag, this
  feature has none, and the comparison profile therefore never receives two sides to compare. Every
  law below is asserted INSIDE the adapter's handler, through the shared law module
  `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law/🦀️component.rs` that the stdio subsets use — `divergence` for
  a path-named first difference, `mutation_is_observable` for the forward law, `inverse_restores` for
  the inverse law, `round_trip_preserves` and `carrier_is_exact` for the identity law. A handler that
  applied the mutation and returned would report a pass having checked nothing, which is exactly the
  failure this platform exists to prevent.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> and land on the committed after-snapshot, diff and outcome
    Given the committed <id> specification vector under 🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations
    When <id> is applied to that vector's before-snapshot through s_home_mutation_report_json
    Then the applied snapshot, the produced diff and the outcome's diagnostics are exactly what the vector commits, and a kind the vector declares observable really moved the projection
    Examples:
      | id                        |
      | change-catalog-generation |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot
    Given the committed <id> specification vector under 🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations
    When <id> is applied and then its own computed inverse is applied through s_home_mutation_report_json
    Then the snapshot's projection is the before-snapshot's projection again, and any divergence is reported by JSON path
    Examples:
      | id                        |
      | change-catalog-generation |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse the real committed launcher document, print it back and cross it against its binary encoding
    Given the real committed document asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    When the document is parsed, printed back to canonical DSL, parsed again, and separately encoded to a pack and decoded back
    Then every decoding agrees on one snapshot, and printing the canonical text a second time reproduces it byte for byte as ArtifactDsl's own fixpoint law requires
