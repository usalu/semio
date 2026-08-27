@capability-energy-model-1-mutate
@no-oracle-energy-model-mutation-semantics
@comparison-ordered-json-v1
@mutations-energy-model-1-any
Feature: Apply the one typed s.energy.model mutation to its committed specification vector

  `s.energy.model` is a semio-NATIVE artifact and no third party reads `.dsl.semio`, so no reference
  library is registered — recorded as the `energy-model-mutation-semantics` no-oracle decision in
  `🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json`, which also records why EnergyPlus and
  OpenStudio were surveyed and DECLINED, and why the `energyplus` weather reader already registered
  under `✏️s/🔌️plugins/🗄️stdio`'s `🌦️epw` subset is deliberately NOT reused here.

  ⚠️ THIS NO-ORACLE DECISION IS A DEBT, NOT A VERDICT, and is recorded as one. Declining a third-party
  LIBRARY is a different judgement from declining a SECOND IMPLEMENTATION, and only the first was ever
  made here. `mutate-cad-1` and `mutate-lowpoly-1` took Python second
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

  What distinguishes this subset is that it persists no content model of its own. Its entire
  substantive body is two composed CHILD HANDLES — `structure` pointing at `s.stdio.semio.value` and
  `zones` pointing at `s.stdio.semio.table` — which are always regenerated TOGETHER from one
  `crate::model::Model`, plus a `schema` string and an optional `referencedModel` link that no
  mutation may ever target. So the one declared kind, `replace-model`, is a paired-child overwrite
  keyed by a serialized model string, and its inverse re-serializes the model read back out of BASE:
  `replace` is its own inverse partner. Malformed or partial payload JSON is documented, honest
  degradation to `Model::default()`, never a fault.

  A gap, stated plainly rather than papered over. The ONLY committed specification vector for this
  vocabulary, `degrades-an-empty-model-payload-to-a-no-op`, carries `newModelJson` of `{}` — which
  degrades to the default model — over a before-snapshot that already holds the default model. It is
  therefore a NO-OP vector: before equals after byte for byte, the produced diff is empty, and the
  declared outcome is one `mutation.no-op` warning. That is a real and useful assertion about the
  documented degradation path, and this case asserts it exactly; what it is not is evidence that
  `replace-model` can replace a model. `replace-model` is consequently listed as unobservable in the
  adapter's own `mutation_is_observable` call, WITH this reason attached, and the forward semantics of
  this vocabulary remain uncovered until a second vector is committed from a real, non-default
  `Model`. No fixture was invented here to make the number look better.

  The identity round trip reads the artifact's own committed demo example, whose two child handles
  both carry the real content key `energy-scene-8c12e172a96d3f44` — the one committed document in
  this plugin where the handles are resolved rather than placeholders.

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
    When <id> is applied to that vector's before-snapshot through energy_model_mutation_report_json
    Then the applied snapshot, the produced diff and the outcome's diagnostics are exactly what the vector commits, and a kind the vector declares observable really moved the projection
    Examples:
      | id            |
      | replace-model |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot
    Given the committed <id> specification vector under 🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations
    When <id> is applied and then its own computed inverse is applied through energy_model_mutation_report_json
    Then the snapshot's projection is the before-snapshot's projection again, and any divergence is reported by JSON path
    Examples:
      | id            |
      | replace-model |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse the real committed energy model document, print it back and cross it against its binary encoding
    Given the real committed document asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    When the document is parsed, printed back to canonical DSL, parsed again, and separately encoded to a pack and decoded back
    Then every decoding agrees on one snapshot, and printing the canonical text a second time reproduces it byte for byte as ArtifactDsl's own fixpoint law requires
