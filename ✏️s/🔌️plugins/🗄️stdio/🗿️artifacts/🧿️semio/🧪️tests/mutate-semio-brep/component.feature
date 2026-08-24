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
  production entry point, and the `ordered-json-v1` profile compares the two structurally.

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
