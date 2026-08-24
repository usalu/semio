@capability-semio-v1-presentation-mutate
@no-oracle-semio-presentation-mutation-semantics
@comparison-ordered-json-v1
@mutations-semio-v1-presentation
Feature: Apply every typed semio PRESENTATION mutation to the real committed deck artifact
  `s.stdio.semio.presentation` is a semio-NATIVE format: no third party reads or writes
  `.dsl.semio`/`.pack.semio`, so there is no reference implementation to register as an oracle.
  `python-pptx` was surveyed as the obvious candidate — this subset is deliberately modelled on pptx,
  the Python oracle host has landed, and a real `.pptx` fixture is committed in this repository — and
  rejected on three findings: reaching a `SemioPresentationSnapshot` from pptx bytes needs this
  repository's OWN pptx bridge, so the comparison would run our importer against our exporter;
  `python-pptx` cannot create slide masters or slide layouts at all, which removes a third of the
  vocabulary; and `set-snapshot`'s semantics are a whole-state structural comparison no presentation
  library models. That is recorded as the `semio-presentation-mutation-semantics` no-oracle decision
  in `../../🏅️standards/🔖️v1/🪆️subsets/✳️presentation/🧪️oracle/🔣️component.json`.

  What replaces the oracle is a REAL input rather than an invented one. The before-state of every
  scenario below is the committed example artifact
  `🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/📽️deck/🖼️assets/🗣️example.dsl.semio` — a real deck
  carrying one master with a title placeholder, one layout with a subtitle placeholder, and one slide
  whose shape tree exercises all four shape kinds (a text box, an embedded PNG picture, a one-cell
  table and an `other`-typed placeholder) plus a speaker-notes page. This subset's own
  `fixture_honesty_law` asserts it is byte-identical to `demo_semio_presentation_snapshot()`, so it
  can never silently drift back into a synthetic fixture. `identity-round-trip` reads that artifact
  and its `.pack.semio` sibling directly and pins that both decode to exactly the same committed
  snapshot; the fifteen mutation kinds are then applied to that snapshot as committed
  `(before, mutation, after)` specification vectors, transcribed from it once and read at run time by
  BOTH roles rather than transcribed into either role's source.

  Three kinds take a before-state that is the real artifact after one declared preparatory step, and
  say so: `remove-master` and `set-layout-master` start from the after-state of `insert-master`, and
  `remove-layout` from the after-state of `insert-layout`. That is deliberate and it is a finding
  about the vocabulary rather than fixture convenience. Masters and layouts are ID-keyed and
  `apply_named` appends, so undoing the removal of a non-terminal master restores it at the wrong
  position; and the deck's own single master and single layout are both referenced — by `layout1`
  and by `slide1` respectively — so removing them directly would leave dangling references. Slides
  and shapes need none of this: they are INDEX-addressed, `InsertSlide`/`InsertShape` carry the exact
  final index, and `remove-slide` therefore removes the deck's real only slide at index 0 and
  `remove-shape` its real embedded picture at index 1, both restored in place by their own inverse.

  The `oracle` role reads the committed after- (or before-) snapshot literally — no recomputation,
  no reimplementation of mutation semantics. The `subject` role decodes the committed before-snapshot
  and mutation payload and runs this repository's own `apply_semio_presentation_mutation`. The
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
    When <id> is applied through apply_semio_presentation_mutation
    Then the resulting snapshot matches the committed after-snapshot fixture for <id>
    Examples:
      | id                 |
      | no-mutation        |
      | set-snapshot       |
      | insert-slide       |
      | remove-slide       |
      | set-slide-layout   |
      | set-slide-notes    |
      | insert-shape       |
      | remove-shape       |
      | set-shape-frame    |
      | set-textbox-blocks |
      | insert-master      |
      | remove-master      |
      | insert-layout      |
      | remove-layout      |
      | set-layout-master  |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot fixture
    Given the committed specification fixtures for the <id> kind
      | role     | fixture                        |
      | before   | local://<id>/⬅️before.json      |
      | mutation | local://<id>/🦠️mutation.json    |
    When <id> is applied through apply_semio_presentation_mutation
    And the mutation's own computed inverse is applied through apply_semio_presentation_mutation
    Then the snapshot matches the committed before-snapshot fixture again, slide and shape order included
    Examples:
      | id                 |
      | no-mutation        |
      | set-snapshot       |
      | insert-slide       |
      | remove-slide       |
      | set-slide-layout   |
      | set-slide-notes    |
      | insert-shape       |
      | remove-shape       |
      | set-shape-frame    |
      | set-textbox-blocks |
      | insert-master      |
      | remove-master      |
      | insert-layout      |
      | remove-layout      |
      | set-layout-master  |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode the real committed deck artifact through both envelopes without transcribing it
    Given the real committed text artifact asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/📽️deck/🖼️assets/🗣️example.dsl.semio
    And the real committed binary artifact asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/📽️deck/🖼️assets/🎒️example.pack.semio
    When both envelopes are decoded and the deck is re-encoded through pack and dsl in turn
    Then every decode agrees and the result matches the committed snapshot local://no-mutation/⬅️before.json
