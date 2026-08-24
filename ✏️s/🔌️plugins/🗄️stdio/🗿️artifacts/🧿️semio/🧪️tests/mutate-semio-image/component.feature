@capability-semio-v1-image-mutate
@no-oracle-semio-image-mutation-semantics
@comparison-ordered-json-v1
@mutations-semio-v1-image
Feature: Apply every typed semio IMAGE mutation to its committed specification fixtures
  `s.stdio.semio.image` is a semio-NATIVE format: no third party reads or writes `.dsl.semio`/
  `.pack.semio`, and `SemioImageSnapshot` is a NEUTRAL raster model rather than a wire format any
  decoder emits — handing `png`/`gif`/`image` a file our own encoder produced would compare this
  implementation with itself, so no oracle is registered and the `semio-image-mutation-semantics`
  no-oracle decision is recorded instead (see the subset's own `🧪️oracle/🔣️component.json`).

  Twelve of this subset's thirteen kinds already carry an independently handcrafted
  `(before, mutation, after, diff, outcome)` specification vector under their own leaf's `🧪️tests/`
  directory, authored by hand and already unit-tested inside the production crate itself; this
  feature re-exercises those SAME committed files end-to-end through `apply_semio_image_mutation`
  instead of calling `Mutation::diff`/`inverse` directly the way the in-crate tests do. Every
  fixture is declared here as an `asset://` reference into its own committed leaf directory (never
  copied, never duplicated) and read at run time through the host's `Context::fixture_json`, so the
  `oracle` role (which reads the committed answer literally, with no recomputation) and the
  `subject` role (which decodes the same bytes into real `SemioImageSnapshot`/`SemioImageMutation`
  values and runs the real entry point) both read the exact same committed bytes rather than a
  hand-transcribed copy that could drift from them. The thirteenth kind, `no-mutation`, is nullary
  and owns no leaf of its own; it is exercised as the identity law against a committed
  before-snapshot.

  A bytes-level decode/re-encode round trip is NOT expressible from an owner-root test case for this
  subset. `.dsl.semio`/`.pack.semio` are produced by `store::ArtifactDsl`/`store::ArtifactPack`,
  traits reached only through a private `extern crate … as store;` alias that nothing re-exports, so
  an adapter compiled as an external crate cannot name them — the same structural gap wave 7 recorded
  for `kit`, `object`, `text` and `table`. What IS reachable, and what `identity-round-trip` asserts,
  is the equivalent completeness law one level up: starting from `SemioImageSnapshot::default()`, the
  subset's own full-replace `set-snapshot` diff must reconstruct the committed three-frame document
  field for field, so no slot of the typed model is silently dropped on the way through.

  ⚖️ Because this case records a no-oracle decision, the runner executes NO oracle role: it resolves
  an oracle implementation from an `@oracle-` tag this feature deliberately does not carry, so the
  comparison profile never receives two sides to compare and the `oracle` handlers below are the
  written statement of the reference answer rather than a second running party. Every law this
  feature claims is therefore asserted INSIDE the subject handler, which fails with both documents
  printed. A handler that merely ran the mutation and returned would report a pass having checked
  nothing. Here that means the applied snapshot is checked against the committed
  after-snapshot with the colorspace tag, the plane geometry and the frame payload together, so an
  edit that reached the right frame at the wrong bit depth fails; `no-mutation` is the identity, so
  its expected answer is the before-snapshot itself rather than the leaf fixture it borrows its
  document from.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to its committed before-snapshot fixture
    Given the committed before-snapshot fixture asset://🏅️standards/🔖️v1/🪆️subsets/✳️image/🧬️schema/🧬️mutations/<dir>/🧪️tests/<slug>/📸️snapshot/⬅️before/🔣️component.json for the <id> kind
    And the committed mutation fixture asset://🏅️standards/🔖️v1/🪆️subsets/✳️image/🧬️schema/🧬️mutations/<dir>/🧪️tests/<slug>/🦠️mutation/🔣️component.json for the <id> kind
    And the committed after-snapshot fixture asset://🏅️standards/🔖️v1/🪆️subsets/✳️image/🧬️schema/🧬️mutations/<dir>/🧪️tests/<slug>/📸️snapshot/➡️after/🔣️component.json for the <id> kind
    When <id> is applied through apply_semio_image_mutation
    Then the resulting snapshot matches the committed after-snapshot fixture for <id>
    Examples:
      | id                    | dir                     | slug                                                        |
      | set-snapshot          | 📸️set-snapshot          | retargets-the-document-onto-a-grayscale-sixteen-bit-variant |
      | set-dimensions        | 📐️set-dimensions        | widens-the-frameless-canvas-to-four-by-two                  |
      | set-colorspace        | 🌈️set-colorspace        | records-the-source-colorspace-as-rgba                       |
      | set-bit-depth         | 🔢️set-bit-depth         | raises-the-source-bit-depth-to-sixteen                      |
      | set-icc               | 🎨️set-icc               | attaches-an-icc-profile-where-there-was-none                |
      | insert-frame          | ➕️insert-frame          | appends-a-second-frame-at-the-end                           |
      | remove-frame          | 📄remove-frame           | removes-the-leading-frame                                   |
      | move-frame            | 🔀️move-frame            | moves-the-last-frame-to-the-front                           |
      | set-frame-delay       | ⏱️set-frame-delay       | slows-the-second-frame-down                                 |
      | set-frame-pixels      | 🟪️set-frame-pixels      | repaints-the-only-frame-black                               |
      | set-metadata-entry    | 🏷️set-metadata-entry    | rewrites-the-existing-author-entry                          |
      | remove-metadata-entry | 🗑️remove-metadata-entry | removes-the-comment-entry-and-keeps-the-author-entry        |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot fixture
    Given the committed before-snapshot fixture asset://🏅️standards/🔖️v1/🪆️subsets/✳️image/🧬️schema/🧬️mutations/<dir>/🧪️tests/<slug>/📸️snapshot/⬅️before/🔣️component.json for the <id> kind
    And the committed mutation fixture asset://🏅️standards/🔖️v1/🪆️subsets/✳️image/🧬️schema/🧬️mutations/<dir>/🧪️tests/<slug>/🦠️mutation/🔣️component.json for the <id> kind
    When <id> is applied through apply_semio_image_mutation
    And the mutation's own computed inverse is applied through apply_semio_image_mutation
    Then the snapshot matches the committed before-snapshot fixture again
    Examples:
      | id                    | dir                     | slug                                                        |
      | set-snapshot          | 📸️set-snapshot          | retargets-the-document-onto-a-grayscale-sixteen-bit-variant |
      | set-dimensions        | 📐️set-dimensions        | widens-the-frameless-canvas-to-four-by-two                  |
      | set-colorspace        | 🌈️set-colorspace        | records-the-source-colorspace-as-rgba                       |
      | set-bit-depth         | 🔢️set-bit-depth         | raises-the-source-bit-depth-to-sixteen                      |
      | set-icc               | 🎨️set-icc               | attaches-an-icc-profile-where-there-was-none                |
      | insert-frame          | ➕️insert-frame          | appends-a-second-frame-at-the-end                           |
      | remove-frame          | 📄remove-frame           | removes-the-leading-frame                                   |
      | move-frame            | 🔀️move-frame            | moves-the-last-frame-to-the-front                           |
      | set-frame-delay       | ⏱️set-frame-delay       | slows-the-second-frame-down                                 |
      | set-frame-pixels      | 🟪️set-frame-pixels      | repaints-the-only-frame-black                               |
      | set-metadata-entry    | 🏷️set-metadata-entry    | rewrites-the-existing-author-entry                          |
      | remove-metadata-entry | 🗑️remove-metadata-entry | removes-the-comment-entry-and-keeps-the-author-entry        |

  @id-mutate-no-mutation
  @level-exhaustive
  @mode-conformance
  Scenario: no-mutation leaves the committed three-frame document exactly as it stands
    Given the committed before-snapshot fixture asset://🏅️standards/🔖️v1/🪆️subsets/✳️image/🧬️schema/🧬️mutations/🔀️move-frame/🧪️tests/moves-the-last-frame-to-the-front/📸️snapshot/⬅️before/🔣️component.json for the no-mutation kind
    When the nullary mutation is applied through apply_semio_image_mutation
      """
      {"mutation": "noMutation"}
      """
    Then the resulting snapshot is the committed before-snapshot fixture, field for field

  @id-inverse-no-mutation
  @level-exhaustive
  @mode-property
  Scenario: Undoing no-mutation is itself no-mutation
    Given the committed before-snapshot fixture asset://🏅️standards/🔖️v1/🪆️subsets/✳️image/🧬️schema/🧬️mutations/🔀️move-frame/🧪️tests/moves-the-last-frame-to-the-front/📸️snapshot/⬅️before/🔣️component.json for the no-mutation kind
    When the nullary mutation is applied through apply_semio_image_mutation
      """
      {"mutation": "noMutation"}
      """
    And the mutation's own computed inverse is applied through apply_semio_image_mutation
    Then the snapshot matches the committed before-snapshot fixture again

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Rebuilding the committed document from an empty snapshot carries every field
    Given the committed before-snapshot fixture asset://🏅️standards/🔖️v1/🪆️subsets/✳️image/🧬️schema/🧬️mutations/🔀️move-frame/🧪️tests/moves-the-last-frame-to-the-front/📸️snapshot/⬅️before/🔣️component.json for the no-mutation kind
    When the empty snapshot is replaced with it through apply_semio_image_mutation
    Then the rebuilt snapshot equals the committed fixture, field for field
