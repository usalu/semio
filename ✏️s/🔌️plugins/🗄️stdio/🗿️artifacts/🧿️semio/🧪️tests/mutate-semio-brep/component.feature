@capability-semio-v1-brep-mutate
@no-oracle-semio-brep-mutation-semantics
@comparison-ordered-json-v1
@mutations-semio-v1-brep
Feature: Apply every typed semio BREP mutation to its committed specification fixtures
  `s.stdio.semio.brep` is a semio-NATIVE format: no third party reads or writes `.dsl.semio`/
  `.pack.semio`, so there is no reference implementation to register as an oracle (recorded as the
  `semio-brep-mutation-semantics` no-oracle decision in `../../🏅️standards/🔖️v1/🪆️subsets/✳️brep/
  🧪️oracle/🔣️component.json`). This subset's 13 kinds cover the entity-lifecycle pairs
  (create/delete-vertex, create/delete-edge, create/delete-face, create/delete-shell,
  create/delete-solid) plus two structured-payload replacements (replace-curve, replace-surface) and
  one scalar reposition (move-vertex); `delete-vertex` additionally cascades into its dependent
  edges, since an edge cannot exist with a dangling endpoint. Every one of these 13 kinds already
  carries an independently handcrafted `(before, mutation, after, diff)` specification fixture under
  its own leaf's `🧪️tests/` directory, authored by hand and already unit-tested inside the
  production crate itself — this feature re-exercises those SAME committed fixtures end-to-end
  through `apply_semio_brep_mutation`, the entry point this ticket added, instead of calling
  `Mutation::diff`/`inverse` directly the way the in-crate tests do. Each scenario names its three
  fixture files as `asset://` URIs resolved against this artifact's own root, so BOTH roles read the
  committed JSON directly — nothing about a fixture is transcribed into either role's source, and
  the planner digest-pins every file it hands over. The `oracle` role reads the committed after- (or
  before-) snapshot literally, with no recomputation and no reimplementation of mutation semantics;
  the `subject` role decodes the committed before-snapshot and mutation payload, runs the real
  production entry point,.

  ⚖️ Because this case records a no-oracle decision, the runner executes NO oracle role: it resolves
  an oracle implementation from an `@oracle-` tag this feature deliberately does not carry, so the
  comparison profile never receives two sides to compare and the `oracle` handlers below are the
  written statement of the reference answer rather than a second running party. Every law this
  feature claims is therefore asserted INSIDE the subject handler, which fails with both documents
  printed. A handler that merely ran the mutation and returned would report a pass having checked
  nothing. Here that means the applied topology is checked against the committed
  after-snapshot in full, so a `delete-face` that removed the face but left its loop pointing at a
  severed edge fails, and the undone topology against the committed before-snapshot, so a
  `delete-solid` whose inverse re-added the solid without its shells fails too.

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
  Scenario Outline: Apply <id> to its committed before-snapshot fixture
    Given the committed specification fixtures for the <id> kind
      | role     | fixture                                                                                        |
      | before   | asset://🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/<dir>/📸️snapshot/⬅️before/🔣️component.json |
      | mutation | asset://🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/<dir>/🦠️mutation/🔣️component.json         |
      | after    | asset://🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/<dir>/📸️snapshot/➡️after/🔣️component.json  |
    When <id> is applied through apply_semio_brep_mutation
    Then the resulting snapshot matches the committed after-snapshot fixture for <id>
    Examples:
      | id              | dir                                                                                   |
      | create-vertex   | 🏗️create-vertex/🧪️tests/adds-an-apex-vertex-above-the-square                            |
      | delete-vertex   | 🗑️delete-vertex/🧪️tests/removes-a-corner-vertex-and-cascades-into-its-two-incident-edges |
      | create-edge     | 🔗create-edge/🧪️tests/adds-a-diagonal-edge-across-the-square                             |
      | delete-edge     | ✂️delete-edge/🧪️tests/removes-the-closing-edge-and-keeps-its-two-vertices                |
      | create-face     | 🔷create-face/🧪️tests/adds-an-opposing-face-over-the-same-loop                           |
      | delete-face     | 🚮delete-face/🧪️tests/removes-the-only-face-and-leaves-its-loop-behind                   |
      | create-shell    | 🐚create-shell/🧪️tests/adds-a-second-shell-that-reuses-the-face-with-flipped-sense       |
      | delete-shell    | 💥delete-shell/🧪️tests/removes-the-only-shell-and-leaves-its-faces-behind                |
      | create-solid    | 🧊create-solid/🧪️tests/adds-a-second-solid-that-treats-the-shell-as-a-void               |
      | delete-solid    | 🕳️delete-solid/🧪️tests/removes-the-only-solid-and-leaves-its-shell-behind                |
      | replace-curve   | ➰replace-curve/🧪️tests/swaps-the-first-edges-line-for-a-circular-arc                    |
      | replace-surface | 🗺️replace-surface/🧪️tests/swaps-the-faces-plane-for-a-cylinder                           |
      | move-vertex     | 📍move-vertex/🧪️tests/lifts-the-third-corner-off-the-base-plane                          |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot fixture
    Given the committed specification fixtures for the <id> kind
      | role     | fixture                                                                                        |
      | before   | asset://🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/<dir>/📸️snapshot/⬅️before/🔣️component.json |
      | mutation | asset://🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/<dir>/🦠️mutation/🔣️component.json         |
      | after    | asset://🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/<dir>/📸️snapshot/➡️after/🔣️component.json  |
    When <id> is applied through apply_semio_brep_mutation
    And the mutation's own computed inverse is applied through apply_semio_brep_mutation
    Then the snapshot matches the committed before-snapshot fixture again
    Examples:
      | id              | dir                                                                                   |
      | create-vertex   | 🏗️create-vertex/🧪️tests/adds-an-apex-vertex-above-the-square                            |
      | delete-vertex   | 🗑️delete-vertex/🧪️tests/removes-a-corner-vertex-and-cascades-into-its-two-incident-edges |
      | create-edge     | 🔗create-edge/🧪️tests/adds-a-diagonal-edge-across-the-square                             |
      | delete-edge     | ✂️delete-edge/🧪️tests/removes-the-closing-edge-and-keeps-its-two-vertices                |
      | create-face     | 🔷create-face/🧪️tests/adds-an-opposing-face-over-the-same-loop                           |
      | delete-face     | 🚮delete-face/🧪️tests/removes-the-only-face-and-leaves-its-loop-behind                   |
      | create-shell    | 🐚create-shell/🧪️tests/adds-a-second-shell-that-reuses-the-face-with-flipped-sense       |
      | delete-shell    | 💥delete-shell/🧪️tests/removes-the-only-shell-and-leaves-its-faces-behind                |
      | create-solid    | 🧊create-solid/🧪️tests/adds-a-second-solid-that-treats-the-shell-as-a-void               |
      | delete-solid    | 🕳️delete-solid/🧪️tests/removes-the-only-solid-and-leaves-its-shell-behind                |
      | replace-curve   | ➰replace-curve/🧪️tests/swaps-the-first-edges-line-for-a-circular-arc                    |
      | replace-surface | 🗺️replace-surface/🧪️tests/swaps-the-faces-plane-for-a-cylinder                           |
      | move-vertex     | 📍move-vertex/🧪️tests/lifts-the-third-corner-off-the-base-plane                          |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode the real solid artifact through both of its committed encodings and reproduce each byte for byte
    Given the real committed text artifact asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🧊️solid/🖼️assets/🗣️example.dsl.semio
    And its committed binary twin asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🧊️solid/🖼️assets/🎒️example.pack.semio
    When the text artifact is parsed and printed back to DSL, and the binary twin is decoded and re-encoded
    Then both encodings decode to the same solid and each re-encoding reproduces its committed file byte for byte
