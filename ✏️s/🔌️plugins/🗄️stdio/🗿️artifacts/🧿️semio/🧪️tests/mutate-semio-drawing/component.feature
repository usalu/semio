@capability-semio-v1-drawing-mutate
@no-oracle-semio-drawing-mutation-semantics
@comparison-ordered-json-v1
@mutations-semio-v1-drawing
Feature: Apply every typed semio DRAWING mutation to its committed specification fixtures
  `s.stdio.semio.drawing` is a semio-NATIVE format: no third party reads or writes `.dsl.semio`/
  `.pack.semio`, so there is no reference implementation to register as an oracle (recorded as the
  `semio-drawing-mutation-semantics` no-oracle decision in `../../🏅️standards/🔖️v1/🪆️subsets/
  ✳️drawing/🧪️oracle/🔣️component.json`, which also records why `usvg`/`resvg` and `lyon`/`kurbo`
  were surveyed and DELIBERATELY declined rather than merely absent). Every one of this subset's 17
  kinds carries an independently handcrafted `(before, mutation, after, diff)` specification fixture
  under its own leaf's `🧪️tests/` directory, and this feature re-exercises those SAME committed
  bytes end-to-end through `apply_semio_drawing_mutation` rather than calling `Mutation::diff`/
  `inverse` directly the way the in-crate fixture tests do.

  What distinguishes this subset is that its scene graph is ANONYMOUS. A `DrawNode` is a recursive
  group/path/text/image union with no id field at all, so every node-addressed verb is keyed by a
  structural path — layer index plus a child-index chain, `{"layer": 0, "path": [2]}` — and a verb
  that changed a node's position among its siblings silently re-targets every later verb. Only two
  things here are named: layers, which `delete-layer` addresses by `id`, and the STYLE table, which
  `replace-fill`, `change-stroke-color` and `change-stroke-width` address by `style_name` while the
  nodes reference it by name, so a style edit has to reach the shared record without touching the
  node tree at all. Four of the seventeen kinds are hierarchy rewrites — `group`/`ungroup` and
  `flatten`/`unflatten` — declared inverses of each other, and `unflatten` is the one payload that
  carries a whole captured subtree so a flattening can be undone exactly rather than approximately.

  The fixtures are chosen against exactly those hazards, not against the easy cases. `reorder-nodes`
  moves the LEADING child to the end, so an implementation that reordered by identity rather than by
  position has nothing to hold on to. `create-node` appends at index 3 of a root that has three
  children and `delete-node` removes the middle one, so an off-by-one lands in a different place in
  the tree. `drag-nodes` offsets two siblings at once by the same vector, so a per-node loop that
  invalidated its own paths as it went fails. `flatten` is applied to an IDENTITY-transformed nested
  group, which is the only case where flattening is information-preserving and therefore the only
  one where its inverse can be checked exactly. `delete-layer` removes the leading layer and keeps
  the overlay. And the three style verbs each change one field of `primary` and must leave the other
  three alone — `change-stroke-color` moves it to a translucent white, so an alpha channel dropped
  on the way through shows up.

  A word on what "real" can mean here, since it cannot mean what it means for PDF or DWG. There is
  no real-world corpus of `.dsl.semio` drawings outside this repository — the format exists nowhere
  else — so the identity round trip reads the artifact's own committed 394-byte sketch example,
  which is handcrafted rather than found, and is chosen because it carries every `DrawNode` variant
  and every `PathSegment` variant at once. Its image node is an `image/png` media type over three
  opaque payload bytes, not an encoded picture: this subset's codecs carry image payloads through
  without interpreting them, so what the round trip can honestly assert is that the media type and
  the bytes survive, and that is what it asserts.

  Because this case records a no-oracle decision, the runner executes NO oracle role — every
  assertion below therefore lives inside the subject handler, which compares the applied snapshot
  against the committed after-snapshot and the undone snapshot against the committed
  before-snapshot, and fails with both JSON documents printed. A handler that merely ran the
  mutation and returned would report a pass having checked nothing.

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
    Given the committed before-snapshot and mutation fixture for the <id> kind
    When <id> is applied through apply_semio_drawing_mutation
    Then the resulting snapshot matches the committed after-snapshot fixture for <id>
    Examples:
      | id                  |
      | create-layer        |
      | delete-layer        |
      | create-node         |
      | delete-node         |
      | move-node           |
      | drag-nodes          |
      | rotate              |
      | scale               |
      | reorder-nodes       |
      | group               |
      | ungroup             |
      | flatten             |
      | unflatten           |
      | replace-path        |
      | replace-fill        |
      | change-stroke-color |
      | change-stroke-width |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot fixture
    Given the committed before-snapshot and mutation fixture for the <id> kind
    When <id> is applied through apply_semio_drawing_mutation
    And the mutation's own computed inverse is applied through apply_semio_drawing_mutation
    Then the snapshot matches the committed before-snapshot fixture again
    Examples:
      | id                  |
      | create-layer        |
      | delete-layer        |
      | create-node         |
      | delete-node         |
      | move-node           |
      | drag-nodes          |
      | rotate              |
      | scale               |
      | reorder-nodes       |
      | group               |
      | ungroup             |
      | flatten             |
      | unflatten           |
      | replace-path        |
      | replace-fill        |
      | change-stroke-color |
      | change-stroke-width |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode the real committed sketch through both of its committed encodings
    Given the real committed text artifact asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🖍️sketch/🖼️assets/🗣️example.dsl.semio
    And its committed binary twin asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🖍️sketch/🖼️assets/🎒️example.pack.semio
    When the text artifact is parsed, printed back to DSL and parsed again, and the binary twin is decoded and re-encoded
    Then every decoding agrees on the same one-layer sketch, a path exercising move, line, cubic, quadratic, arc and close, a text node, an image node whose `image/png` media type and opaque payload bytes survive as-is, and an empty nested group
