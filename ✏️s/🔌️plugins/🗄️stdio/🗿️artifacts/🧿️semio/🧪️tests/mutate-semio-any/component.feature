@capability-semio-v1-any-mutate
@no-oracle-semio-envelope-routing
@comparison-ordered-json-v1
@mutations-semio-v1-any
Feature: Route every typed semio ENVELOPE mutation into the subset arm it names
  `s.stdio.semio` is the ENVELOPE union over all eighteen semio subsets, and a semio-NATIVE format:
  no third party reads or writes `.dsl.semio`/`.pack.semio`, so no oracle is registered and the
  `semio-envelope-routing` no-oracle decision is recorded instead (see the subset's own
  `🧪️oracle/🔣️.json`).

  What this subset OWNS is not the payload semantics of its eighteen wrapper variants — those belong
  to the arms, are handcrafted in their own `🧬️mutations/<kind>/🧪️tests/` leaves and are measured by
  their own cases — but the envelope-level ROUTING. Three laws, and nothing else:

  1. a wrapped mutation whose arm MATCHES the base snapshot's arm threads through, changing the
     document without changing its subset kind and without raising a diagnostic;
  2. a wrapped mutation whose arm does NOT match is refused with `mutation.target-missing`, leaving
     the document exactly as it stood — never silently corrupting it;
  3. `set-snapshot` is the only verb that can change the subset kind at all.

  Every scenario therefore compares the one projection the envelope can answer without naming any
  arm's snapshot type: `{schema, subset, diagnostics, matchesReference}`, where `subset` is the
  envelope's own runtime discriminator, `diagnostics` are the fault codes the outcome carried, and
  `matchesReference` says whether the resulting envelope equals the document the scenario started
  from. `mutate-<arm>` expects `matchesReference` false — the delegated verb genuinely reached the
  arm and changed it; `inverse-<arm>` expects it true — that is the metamorphic inverse law.

  The two envelope-owned verbs, `no-mutation` and `set-snapshot`, are exercised against the committed
  `(before, mutation, after, diff)` specification vector under this subset's own
  `🧬️mutations/📄set-snapshot/🧪️tests/` leaf, already unit-tested inside the production crate and read
  here as `asset://` references — never copied.

  Honest limit, recorded rather than papered over: the eighteen delegating arms run against each
  arm's OWN empty snapshot, not a real-world document. No real-world `.semio` artifact exists outside
  this repository, and the committed example assets under `📚️examples/` can only be decoded through
  `store::ArtifactDsl`/`store::ArtifactPack` — traits reached solely through a private
  `extern crate … as store;` alias that nothing re-exports, so an adapter compiled as an external
  crate cannot name them (the same structural gap wave 7 recorded for `kit`, `object`, `text` and
  `table`). Re-exporting those two traits is the single change that would most strengthen this case.

  ⚖️ Because this case records a no-oracle decision, the runner executes NO oracle role: it resolves
  an oracle implementation from an `@oracle-` tag this feature deliberately does not carry, so the
  comparison profile never receives two sides to compare and the `oracle` handlers below are the
  written statement of the reference answer rather than a second running party. Every law this
  feature claims is therefore asserted INSIDE the subject handler, which fails with both documents
  printed. A handler that merely ran the mutation and returned would report a pass having checked
  nothing. Here that means the four envelope-level facts each scenario measures — the
  subset tag after routing, the fault codes raised, and whether the routed document still equals the
  one it started from — are checked against the routing law stated once in the adapter and read by
  both roles, so a delegated verb that quietly reached no arm at all cannot pass.

  The `identity-round-trip` scenario carries the BYTE half of the identity law as well as the
  semantic half. `.dsl.semio` is a fixed-layout record grammar and `.pack.semio` is its binary twin,
  and both committed example files were produced by these very codecs — so re-printing the parsed
  snapshot and re-encoding it must reproduce those files BYTE FOR BYTE, and the scenario asserts
  exactly that through the shared `law::carrier_is_exact`. The must-differ tripwire the wave applies
  to third-party carriers would be backwards here: a re-emission that DIFFERED would be the defect,
  not the evidence. The two encodings also cross-check each other — the binary twin has to decode to
  the same document the text does, which no single codec can arrange on its own.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: A wrapped <id> mutation routes into the <id> arm and changes it
    Given an envelope wrapping an empty <id> snapshot
    When the wrapped <delegated verb> mutation is applied through apply_semio_mutation
    Then the envelope still carries the <id> subset, raises no diagnostic, and no longer matches the document it started from
    Examples:
      | id           | delegated verb                                |
      | brep         | create-vertex on an empty solid model         |
      | mesh         | create-mesh on an empty mesh document         |
      | model        | insert-spatial-node on an empty spatial model |
      | value        | set-node on an empty value graph              |
      | document     | insert-style on an empty document             |
      | cad          | add-layer on an empty drawing database        |
      | drawing      | create-layer on an empty vector drawing       |
      | image        | set-dimensions on an empty raster             |
      | video        | insert-stream on an empty container           |
      | audio        | set-sample-rate on an empty waveform          |
      | animation    | insert-timeline on an empty animation         |
      | presentation | insert-slide on an empty deck                 |
      | flow         | insert-node on an empty flow graph            |
      | text         | insert-run on an empty rich-text document     |
      | table        | create-column on an empty table               |
      | graph        | create-node on an empty graph                 |
      | object       | move-object on an untranslated object         |
      | kit          | add-type on an empty kit                      |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing a wrapped <id> mutation restores the envelope it started from
    Given an envelope wrapping an empty <id> snapshot
    When the wrapped <delegated verb> mutation is applied through apply_semio_mutation
    And the mutation's own computed inverse is applied through apply_semio_mutation
    Then the envelope still carries the <id> subset, raises no diagnostic, and matches the document it started from
    Examples:
      | id           | delegated verb                                |
      | brep         | create-vertex on an empty solid model         |
      | mesh         | create-mesh on an empty mesh document         |
      | model        | insert-spatial-node on an empty spatial model |
      | value        | set-node on an empty value graph              |
      | document     | insert-style on an empty document             |
      | cad          | add-layer on an empty drawing database        |
      | drawing      | create-layer on an empty vector drawing       |
      | image        | set-dimensions on an empty raster             |
      | video        | insert-stream on an empty container           |
      | audio        | set-sample-rate on an empty waveform          |
      | animation    | insert-timeline on an empty animation         |
      | presentation | insert-slide on an empty deck                 |
      | flow         | insert-node on an empty flow graph            |
      | text         | insert-run on an empty rich-text document     |
      | table        | create-column on an empty table               |
      | graph        | create-node on an empty graph                 |
      | object       | move-object on an untranslated object         |
      | kit          | add-type on an empty kit                      |

  @id-mutate-no-mutation
  @level-exhaustive
  @mode-conformance
  Scenario: no-mutation leaves the committed value-subset envelope exactly as it stands
    Given the committed before-envelope asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🧪️tests/replaces-the-envelope-wrapping-a-value-subset/📸️snapshot/⬅️before/🔣️component.json
    When the nullary mutation is applied through apply_semio_mutation
      """
      {"mutation": "noMutation"}
      """
    Then the envelope still carries the value subset, raises no diagnostic, and matches the document it started from

  @id-inverse-no-mutation
  @level-exhaustive
  @mode-property
  Scenario: Undoing no-mutation is itself no-mutation
    Given the committed before-envelope asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🧪️tests/replaces-the-envelope-wrapping-a-value-subset/📸️snapshot/⬅️before/🔣️component.json
    When the nullary mutation is applied through apply_semio_mutation
      """
      {"mutation": "noMutation"}
      """
    And the mutation's own computed inverse is applied through apply_semio_mutation
    Then the envelope still carries the value subset, raises no diagnostic, and matches the document it started from

  @id-mutate-set-snapshot
  @level-exhaustive
  @mode-conformance
  Scenario: set-snapshot replaces the committed value-subset envelope wholesale
    Given the committed before-envelope asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🧪️tests/replaces-the-envelope-wrapping-a-value-subset/📸️snapshot/⬅️before/🔣️component.json
    And the committed mutation fixture asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🧪️tests/replaces-the-envelope-wrapping-a-value-subset/🦠️mutation/🔣️component.json
    And the committed after-envelope asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🧪️tests/replaces-the-envelope-wrapping-a-value-subset/📸️snapshot/➡️after/🔣️component.json
    When set-snapshot is applied through apply_semio_mutation
    Then the envelope still carries the value subset, raises no diagnostic, and no longer matches the document it started from

  @id-inverse-set-snapshot
  @level-exhaustive
  @mode-property
  Scenario: Undoing set-snapshot restores the committed before-envelope
    Given the committed before-envelope asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🧪️tests/replaces-the-envelope-wrapping-a-value-subset/📸️snapshot/⬅️before/🔣️component.json
    And the committed mutation fixture asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🧪️tests/replaces-the-envelope-wrapping-a-value-subset/🦠️mutation/🔣️component.json
    When set-snapshot is applied through apply_semio_mutation
    And the mutation's own computed inverse is applied through apply_semio_mutation
    Then the envelope still carries the value subset, raises no diagnostic, and matches the document it started from

  @id-rejects-a-mismatched-arm
  @level-exhaustive
  @mode-error
  Scenario: A wrapped image mutation against a value envelope is refused, not applied
    Given the committed before-envelope asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🧪️tests/replaces-the-envelope-wrapping-a-value-subset/📸️snapshot/⬅️before/🔣️component.json
    When a wrapped image set-dimensions mutation is applied through apply_semio_mutation
    Then the outcome carries mutation.target-missing and the envelope still matches the document it started from

  @id-set-snapshot-changes-the-subset-kind
  @level-exhaustive
  @mode-conformance
  Scenario: Only set-snapshot can retype the envelope from one subset to another
    Given the committed before-envelope asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🧪️tests/replaces-the-envelope-wrapping-a-value-subset/📸️snapshot/⬅️before/🔣️component.json
    When set-snapshot is applied with an envelope wrapping an empty image snapshot
    Then the envelope carries the image subset, raises no diagnostic, and no longer matches the document it started from

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Rebuild the committed envelope from an empty one, and reproduce the real envelope artifact byte for byte
    Given the committed before-envelope asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🧪️tests/replaces-the-envelope-wrapping-a-value-subset/📸️snapshot/⬅️before/🔣️component.json
    And the real committed text artifact asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🌐️envelope/🖼️assets/🗣️example.dsl.semio
    And its committed binary twin asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🌐️envelope/🖼️assets/🎒️example.pack.semio
    When the empty envelope is replaced with the committed one through apply_semio_mutation
    And the text artifact is parsed and printed back to DSL, and the binary twin is decoded and re-encoded
    Then the envelope carries the value subset, raises no diagnostic, and matches the committed before-envelope
    And both encodings decode to the same envelope and each re-encoding reproduces its committed file byte for byte
