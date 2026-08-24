@capability-semio-v1-model-mutate
@no-oracle-semio-model-mutation-semantics
@comparison-ordered-json-v1
@mutations-semio-v1-model
Feature: Apply every typed semio MODEL mutation to the real committed building artifact
  `stdio.semio.model` is a semio-NATIVE format: no third party reads or writes `.dsl.semio`/
  `.pack.semio`, so there is no reference implementation to register as an oracle. The two
  candidates that exist for the IFC spatial-structure shape this subset models — IfcOpenShell on the
  landed Python host, and the already-registered `ruststep` reader — were surveyed and rejected on
  evidence, because both can only reach a `SemioModelSnapshot` through this repository's OWN IFC
  import/export bridge, which would compare our importer against our exporter with a third party
  merely re-reading the result. That is recorded as the `semio-model-mutation-semantics` no-oracle
  decision in `../../🏅️standards/🔖️v1/🪆️subsets/✳️model/🧪️oracle/🔣️component.json`.

  What replaces the oracle is a REAL input rather than an invented one. The before-state of every
  scenario below is the committed example artifact
  `🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🏢️building/🖼️assets/🗣️example.dsl.semio` — a real
  two-level IFC-shaped model (site → ground storey, one external wall carrying a `Pset_WallCommon`
  fire-rating property set, one `containedIn` relation) which this subset's own `fixture_honesty_law`
  asserts is byte-identical to `demo_semio_model_snapshot()`, so it can never silently drift back
  into a synthetic fixture. `identity-round-trip` reads that artifact and its `.pack.semio` sibling
  directly and pins that both decode to exactly the same committed snapshot; the eleven mutation
  kinds are then applied to that snapshot as committed `(before, mutation, after)` specification
  vectors, transcribed from it once and read at run time by BOTH roles rather than transcribed into
  either role's source.

  Four of the eleven kinds take a before-state that is the real artifact after one declared
  preparatory step, and say so: `remove-spatial-node` and `remove-element` start from the after-state
  of `insert-spatial-node` and `insert-element` respectively. That is deliberate. The vocabulary's
  `InsertSpatialNode`/`InsertElement` carry no index, and `apply_named` appends, so undoing the
  removal of a NON-terminal member restores the member but not its position — removing an appended
  member is the only shape under which the inverse law is actually true for this vocabulary, and
  removing the real committed members would additionally leave dangling `spatialId`/relation
  endpoints that `SemioModelValidator` rejects at compose time. Both facts are findings about the
  vocabulary, recorded here rather than hidden by a fixture that happens to avoid them.

  The `oracle` role reads the committed after- (or before-) snapshot literally — no recomputation,
  no reimplementation of mutation semantics. The `subject` role decodes the committed before-snapshot
  and mutation payload and runs this repository's own `apply_semio_model_mutation`. The
  `ordered-json-v1` profile compares the two structurally.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to its committed before-snapshot fixture
    Given the committed specification fixtures for the <id> kind
      | role     | fixture                        |
      | before   | local://<id>/⬅️before.json      |
      | mutation | local://<id>/🦠️mutation.json    |
      | after    | local://<id>/➡️after.json       |
    When <id> is applied through apply_semio_model_mutation
    Then the resulting snapshot matches the committed after-snapshot fixture for <id>
    Examples:
      | id                  |
      | no-mutation         |
      | set-snapshot        |
      | insert-spatial-node |
      | remove-spatial-node |
      | set-spatial-node    |
      | insert-element      |
      | remove-element      |
      | set-element         |
      | insert-relation     |
      | remove-relation     |
      | set-relation        |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot fixture
    Given the committed specification fixtures for the <id> kind
      | role     | fixture                        |
      | before   | local://<id>/⬅️before.json      |
      | mutation | local://<id>/🦠️mutation.json    |
    When <id> is applied through apply_semio_model_mutation
    And the mutation's own computed inverse is applied through apply_semio_model_mutation
    Then the snapshot matches the committed before-snapshot fixture again, member order included
    Examples:
      | id                  |
      | no-mutation         |
      | set-snapshot        |
      | insert-spatial-node |
      | remove-spatial-node |
      | set-spatial-node    |
      | insert-element      |
      | remove-element      |
      | set-element         |
      | insert-relation     |
      | remove-relation     |
      | set-relation        |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode the real committed building artifact through both envelopes without transcribing it
    Given the real committed text artifact asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🏢️building/🖼️assets/🗣️example.dsl.semio
    And the real committed binary artifact asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🏢️building/🖼️assets/🎒️example.pack.semio
    When both envelopes are decoded and the model is re-encoded through pack and dsl in turn
    Then every decode agrees and the result matches the committed snapshot local://no-mutation/⬅️before.json
