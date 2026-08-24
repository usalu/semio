@capability-raster-1-mutate
@no-oracle-raster-mutation-semantics
@comparison-ordered-json-v1
@mutations-raster-1-any
Feature: Apply every typed raster-document mutation to its committed specification vector
  `s.raster.raster` is a semio-NATIVE artifact and — this is the part worth stating
  plainly — it is a LAYERED DOCUMENT, not an image file. Its pixels live behind ids in a root `assets`
  pool and its vocabulary edits the layer tree around them, so the raster crates this repository
  already links (`png`, `image`, `tiff`) are readers of a different artifact entirely and registering
  one here would be a category error rather than an oracle. That is recorded as the
  `raster-mutation-semantics` no-oracle decision in
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`, whose substitutes are the
  committed per-kind specification vectors plus the inverse law. This case re-exercises those SAME
  committed bytes through `apply_raster_mutation_json`/`undo_raster_mutation_json`.

  What distinguishes this subset is that its layer nodes are three genuinely different kinds over one
  shared base: a pixel layer carries an `assetIds` list, an adjustment layer carries an
  `adjustmentKind` plus a parameter map, and a group layer carries `children`. So
  `change-layer-adjustment-kind` applies to exactly one of the three, `add-layer-asset` and
  `remove-layer-asset` are the only kinds that join the tree to the root `assets` pool, and
  `reorder-layers` can lift a node OUT of a group rather than merely permute siblings — which is what
  its committed vector does, so an inverse that re-appended at the root instead of back inside the
  frame group fails.

  Two of the twelve kinds carry only a NEGATIVE committed vector today, and this case asserts what
  each one actually declares rather than pretending an application happened: `add-layer-asset`
  re-attaches an asset the document already holds and the leaf declares `status: "applied"` with a
  `mutation.no-op` warning, so its before- and after-documents are identical BY DESIGN and the
  observability law is deliberately not claimed for it; `remove-layer-asset` names an asset the
  document never attached and the leaf declares `status: "rejected"`, `code:
  "mutation.target-missing"`. Accepting vectors for those two kinds do not exist yet and are a real
  gap.

  Because this case records a no-oracle decision the runner executes NO oracle role, so every
  assertion below lives in the subject handler, which compares against the committed after-document
  through the shared `⚖️law` module and fails with the first divergence named by JSON path.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Applying <id> reaches its committed after-document
    Given the committed before-document and mutation payload of the <id> specification vector
    When <id> is applied through apply_raster_mutation_json
    Then the resulting document is the committed after-document, and the mutation moved it
    Examples:
      | id                           |
      | create-layer                 |
      | delete-layer                 |
      | reorder-layers               |
      | rename-layer                 |
      | change-layer-visible         |
      | change-layer-opacity         |
      | change-layer-blend-mode      |
      | move-layer                   |
      | resize-layer                 |
      | change-layer-adjustment-kind |

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Applying <id> is the declared no-op its vector records
    Given the committed before-document and mutation payload of the <id> specification vector
    When <id> is applied through apply_raster_mutation_json
    Then the document is unchanged and the declared <code> diagnostic was raised
    Examples:
      | id                           | code            |
      | add-layer-asset              | mutation.no-op  |

  @id-mutate
  @level-exhaustive
  @mode-error
  Scenario Outline: Applying <id> is refused exactly as its vector declares
    Given the committed before-document and mutation payload of the <id> specification vector
    When <id> is applied through apply_raster_mutation_json
    Then the document is left untouched and the declared <code> refusal was raised
    Examples:
      | id                           | code                    |
      | remove-layer-asset           | mutation.target-missing |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores its committed before-document
    Given the committed before-document and mutation payload of the <id> specification vector
    When <id> and then every step of its own computed inverse are applied through undo_raster_mutation_json
    Then the document is the committed before-document again, member positions included
    Examples:
      | id                           |
      | create-layer                 |
      | delete-layer                 |
      | reorder-layers               |
      | rename-layer                 |
      | change-layer-visible         |
      | change-layer-opacity         |
      | change-layer-blend-mode      |
      | move-layer                   |
      | resize-layer                 |
      | change-layer-adjustment-kind |
      | add-layer-asset              |
      | remove-layer-asset           |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse and reprint the real committed example without passing bytes through
    Given the real committed example asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    When it is parsed, printed back to DSL and parsed again through round_trip_raster_dsl
    Then both parses agree on one document, and the reprinted text reproduces the committed example byte for byte
