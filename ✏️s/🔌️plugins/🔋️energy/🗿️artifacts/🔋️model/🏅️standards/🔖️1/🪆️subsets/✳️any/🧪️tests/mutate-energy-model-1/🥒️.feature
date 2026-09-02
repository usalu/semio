@capability-energy-model-1-mutate
@oracle-energy-model-1-python-independent
@comparison-ordered-json-v1
@mutations-energy-model-1-any
Feature: Apply the one typed s.energy.model mutation against an independent Python implementation

  `s.energy.model` is a semio-NATIVE artifact and no third party reads or writes `.dsl.semio` — the
  recorded survey (kept verbatim in `🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json`'s history)
  named and DECLINED EnergyPlus and OpenStudio, and the `energyplus` weather reader already
  registered under `✏️s/🔌️plugins/🗄️stdio`'s `🌦️epw` subset is deliberately NOT reused here — it
  reads a different format for a different purpose. The second producer a differential comparison
  needs is therefore a second IMPLEMENTATION, and `🐍️component.py` beside this file is it: written
  in Python from this subset's own committed `🧬️schema/📸️snapshot/🔣️.json` and mutation payload
  schema, and from `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/`'s
  `📓️taxonomy.md` (the `replace` verb: "whole-value swap of a large structured sub-payload …
  inverse partner: `replace` (old payload)") and `📓️derivation-rules.md`. It imports nothing from
  the Rust it judges and transliterates none of it. The no-oracle decision this replaces
  (`energy-model-mutation-semantics`) is narrowed to an empty `capabilities` list rather than deleted,
  because its own investigation (the EnergyPlus/OpenStudio survey) remains the honest record of what
  was checked.

  Both implementations read the SAME committed bytes: `(before, mutation, after, diff, outcome)`
  under `🧬️schema/🧬️mutations/♻️replace-model/🧪️tests/degrades-an-empty-model-payload-to-a-no-op/`
  is now a declared `asset://` fixture rather than an `include_str!`-only literal, so the plan pins
  its digest and a Python reference can resolve it at all.

  What distinguishes this subset is that it persists no content model of its own. Its entire
  substantive body is two composed CHILD HANDLES — `structure` pointing at `s.stdio.semio.value` and
  `zones` pointing at `s.stdio.semio.table` — which are always regenerated TOGETHER from one model,
  plus a `schema` string and an optional `referencedModel` link that no mutation may ever target. So
  the one declared kind, `replace-model`, is a paired-child overwrite keyed by a serialized model
  string (`newModelJson`), and its inverse re-serializes the model read back out of BASE: `replace`
  is its own inverse partner.

  ⚠️ Honest boundary, stated plainly rather than papered over. The ONLY committed specification
  vector for this vocabulary, `degrades-an-empty-model-payload-to-a-no-op`, carries `newModelJson` of
  `{}` over a before-snapshot that already holds the default model — a documented NO-OP: before
  equals after byte for byte, the diff is empty, and the outcome is one `mutation.no-op` warning.
  Both implementations below assert exactly that degradation path, which is real and useful evidence
  about the documented degrade-on-empty-or-malformed-payload rule; what neither implementation claims
  is that `replace-model` can replace a model with a real, non-default one — no committed vector
  exercises that path, and `🐍️component.py` raises rather than guessing at a shape (the `model`
  object's own field layout) that no schema in this repository states. `replace-model` is
  consequently still listed as unobservable in the Rust adapter's own `mutation_is_observable` call,
  and no fixture was invented here to make either side's coverage look wider than it is.

  `identity-round-trip` stays Rust-subject-only, unaffected by this change: this subset's committed
  snapshot text grammar is the repository-wide placeholder `payload = OCTET+`, whose header
  production declares `"schema" SP "stdio.json"` against an artifact whose own first line says
  otherwise, so a second reader has nothing to parse from — the same finding `mutate-iso16757-1`,
  `mutate-vdi3805-1`, `mutate-din16798-1` and `mutate-program-1` record against their own subsets.

  Where the assertions live. `mutate-replace-model` and `inverse-replace-model` now dispatch BOTH an
  oracle role (the Python implementation, reached through this plugin's `oracleHostPackages` entry)
  and a subject role (this repository's own `energy_model_mutation_report_json`), each independently
  asserting the forward/inverse laws in role before the two are compared byte for byte.
  `identity-round-trip` keeps asserting through the shared law module
  `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law/🦀️component.rs` that the stdio subsets use — `divergence` for
  a path-named first difference, `round_trip_preserves` and `carrier_is_exact` for the identity law.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to its committed specification vector
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🎯️outcome/🔣️.json
    When both implementations apply the committed mutation to the committed before-snapshot
    Then each reaches the committed after-snapshot under the committed outcome status and the two agree
    Examples:
      | id            | dir             | fixture                                       |
      | replace-model | ♻️replace-model | degrades-an-empty-model-payload-to-a-no-op    |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores its committed before-snapshot
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🎯️outcome/🔣️.json
    When each implementation applies the committed mutation and then its OWN computed inverse
    Then both restore the before-snapshot and agree on the mutated and the restored document
    Examples:
      | id            | dir             | fixture                                       |
      | replace-model | ♻️replace-model | degrades-an-empty-model-payload-to-a-no-op    |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse the real committed energy model document, print it back and cross it against its binary encoding
    Given the real committed document asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    When the document is parsed, printed back to canonical DSL, parsed again, and separately encoded to a pack and decoded back
    Then every decoding agrees on one snapshot, and printing the canonical text a second time reproduces it byte for byte as ArtifactDsl's own fixpoint law requires
