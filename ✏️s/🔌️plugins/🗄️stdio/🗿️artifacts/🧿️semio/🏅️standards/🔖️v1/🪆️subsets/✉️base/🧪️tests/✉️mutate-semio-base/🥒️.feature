@capability-semio-v1-base-mutate
@oracle-json-rust-semio-envelope-carrier-reader
@comparison-ordered-json-v1
@mutations-semio-v1-base
Feature: Route every typed semio ENVELOPE mutation over its JSON carrier, against a third-party reader
  `s.stdio.semio` is the ENVELOPE union over eighteen semio subsets. Its JSON carrier is published
  schema-first (`🧬️schema/📸️snapshot/🔣️.json`, `🧬️schema/🧬️mutations/🔣️.json`) and the subject
  reads and writes it only through the bridge derived from those schema types
  (`decode_semio_snapshot_json`/`encode_semio_snapshot_json`, `decode_semio_mutation_json`).

  The oracle is `json-rust-semio-envelope-carrier-reader`: json-rust 0.12, a JSON implementation
  no production crate links, reads every committed carrier and routes it by the envelope's published
  law. `setSnapshot` replaces the envelope with its payload; an `apply<Arm>` wrapper whose arm matches
  the envelope's `subset` reaches that arm and lands on the arm's own committed result, produced by
  that arm's independent implementation rather than by this repository's Rust; an `apply<Arm>`
  wrapper whose arm does not match is refused with `mutation.target-missing` and leaves the envelope
  as it stood; every inverse restores the envelope the mutation started from.

  Both roles report `{schema, subset, diagnostics, matchesReference, envelopeDigest}`, where
  `envelopeDigest` digests the whole resulting envelope with its keys ordered, so `ordered-json-v1`
  compares complete documents rather than a summary of them.

  The `identity-round-trip` scenario also carries the byte half of the identity law inside the
  subject: `.dsl.semio` and `.pack.semio` were produced by these very codecs, so re-printing and
  re-encoding must reproduce both committed example files byte for byte, and the binary twin must
  decode to the same envelope the text does.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: A wrapped <arm> mutation reaches the <arm> arm and lands on that arm's committed result
    Given the committed before-envelope shared://<vector>/⬅️before.json
    And the committed wrapped mutation shared://<vector>/🦠️mutation.json
    And the <arm> arm's own committed result shared://<vector>/➡️after.json
    When the mutation is applied through apply_semio_mutation
    Then the envelope equals the arm's committed result, still carries the <arm> subset and raises no diagnostic
    Examples:
      | id                 | arm          | vector                       |
      | apply-brep         | brep         | 🧊️apply-brep-applied         |
      | apply-mesh         | mesh         | 🔺️apply-mesh-applied         |
      | apply-model        | model        | 🏛️apply-model-applied        |
      | apply-value        | value        | 🔢️apply-value-applied        |
      | apply-document     | document     | 📑️apply-document-applied     |
      | apply-cad          | cad          | 📐️apply-cad-applied          |
      | apply-drawing      | drawing      | 🖊️apply-drawing-applied      |
      | apply-image        | image        | 🖼️apply-image-applied        |
      | apply-video        | video        | 🎬️apply-video-applied        |
      | apply-audio        | audio        | 🔊️apply-audio-applied        |
      | apply-animation    | animation    | 🎞️apply-animation-applied    |
      | apply-presentation | presentation | 📽️apply-presentation-applied |
      | apply-flow         | flow         | 🌊️apply-flow-applied         |
      | apply-text         | text         | 🔤️apply-text-applied         |
      | apply-table        | table        | 🗂️apply-table-applied        |
      | apply-graph        | graph        | 🕸️apply-graph-applied        |
      | apply-object       | object       | 📦️apply-object-applied       |
      | apply-kit          | kit          | 🧰️apply-kit-applied          |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing a wrapped <arm> mutation restores the committed before-envelope
    Given the committed before-envelope shared://<vector>/⬅️before.json
    And the committed wrapped mutation shared://<vector>/🦠️mutation.json
    When the mutation is applied through apply_semio_mutation
    And the mutation's own computed inverse is applied through apply_semio_mutation
    Then the envelope equals the committed before-envelope, still carries the <arm> subset and raises no diagnostic
    Examples:
      | id                 | arm          | vector                       |
      | apply-brep         | brep         | 🧊️apply-brep-applied         |
      | apply-mesh         | mesh         | 🔺️apply-mesh-applied         |
      | apply-model        | model        | 🏛️apply-model-applied        |
      | apply-value        | value        | 🔢️apply-value-applied        |
      | apply-document     | document     | 📑️apply-document-applied     |
      | apply-cad          | cad          | 📐️apply-cad-applied          |
      | apply-drawing      | drawing      | 🖊️apply-drawing-applied      |
      | apply-image        | image        | 🖼️apply-image-applied        |
      | apply-video        | video        | 🎬️apply-video-applied        |
      | apply-audio        | audio        | 🔊️apply-audio-applied        |
      | apply-animation    | animation    | 🎞️apply-animation-applied    |
      | apply-presentation | presentation | 📽️apply-presentation-applied |
      | apply-flow         | flow         | 🌊️apply-flow-applied         |
      | apply-text         | text         | 🔤️apply-text-applied         |
      | apply-table        | table        | 🗂️apply-table-applied        |
      | apply-graph        | graph        | 🕸️apply-graph-applied        |
      | apply-object       | object       | 📦️apply-object-applied       |
      | apply-kit          | kit          | 🧰️apply-kit-applied          |

  @id-mutate-set-snapshot
  @level-exhaustive
  @mode-differential
  Scenario: set-snapshot replaces the committed value-subset envelope wholesale
    Given the committed before-envelope shared://🧬️mutations/📸️set-snapshot/✉️replaces-the-envelope-wrapping-a-value-subset/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation shared://🧬️mutations/📸️set-snapshot/✉️replaces-the-envelope-wrapping-a-value-subset/🦠️mutation/🔣️.json
    And the committed after-envelope shared://🧬️mutations/📸️set-snapshot/✉️replaces-the-envelope-wrapping-a-value-subset/📸️snapshot/➡️after/🔣️.json
    When set-snapshot is applied through apply_semio_mutation
    Then the envelope equals the committed after-envelope, still carries the value subset and raises no diagnostic

  @id-inverse-set-snapshot
  @level-exhaustive
  @mode-property
  Scenario: Undoing set-snapshot restores the committed before-envelope
    Given the committed before-envelope shared://🧬️mutations/📸️set-snapshot/✉️replaces-the-envelope-wrapping-a-value-subset/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation shared://🧬️mutations/📸️set-snapshot/✉️replaces-the-envelope-wrapping-a-value-subset/🦠️mutation/🔣️.json
    When set-snapshot is applied through apply_semio_mutation
    And the mutation's own computed inverse is applied through apply_semio_mutation
    Then the envelope equals the committed before-envelope, still carries the value subset and raises no diagnostic

  @id-reasserts-the-envelope-unchanged
  @level-exhaustive
  @mode-differential
  Scenario: Reasserting the committed envelope through set-snapshot leaves it exactly as it stands
    Given the committed before-envelope shared://🧬️mutations/📸️set-snapshot/🪞️reasserts-the-value-envelope-unchanged/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation shared://🧬️mutations/📸️set-snapshot/🪞️reasserts-the-value-envelope-unchanged/🦠️mutation/🔣️.json
    And the committed after-envelope shared://🧬️mutations/📸️set-snapshot/🪞️reasserts-the-value-envelope-unchanged/📸️snapshot/➡️after/🔣️.json
    When set-snapshot is applied through apply_semio_mutation
    Then the envelope equals the committed before-envelope, still carries the value subset and raises no diagnostic

  @id-undoes-reasserting-the-envelope
  @level-exhaustive
  @mode-property
  Scenario: Undoing a reassertion is itself the identity
    Given the committed before-envelope shared://🧬️mutations/📸️set-snapshot/🪞️reasserts-the-value-envelope-unchanged/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation shared://🧬️mutations/📸️set-snapshot/🪞️reasserts-the-value-envelope-unchanged/🦠️mutation/🔣️.json
    When set-snapshot is applied through apply_semio_mutation
    And the mutation's own computed inverse is applied through apply_semio_mutation
    Then the envelope equals the committed before-envelope, still carries the value subset and raises no diagnostic

  @id-rejects-a-mismatched-arm
  @level-exhaustive
  @mode-error
  Scenario: A wrapped image mutation against a value envelope is refused, not applied
    Given the committed before-envelope shared://🧬️mutations/🖼️apply-image/🚫️refuses-a-value-envelope/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation shared://🧬️mutations/🖼️apply-image/🚫️refuses-a-value-envelope/🦠️mutation/🔣️.json
    And the committed after-envelope shared://🧬️mutations/🖼️apply-image/🚫️refuses-a-value-envelope/📸️snapshot/➡️after/🔣️.json
    When the wrapped image set-dimensions mutation is applied through apply_semio_mutation
    Then the outcome carries mutation.target-missing and the envelope equals the committed before-envelope

  @id-set-snapshot-changes-the-subset-kind
  @level-exhaustive
  @mode-differential
  Scenario: Only set-snapshot can retype the envelope from one subset to another
    Given the committed before-envelope shared://🧬️mutations/📸️set-snapshot/🔁️retypes-a-value-envelope-to-an-empty-image/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation shared://🧬️mutations/📸️set-snapshot/🔁️retypes-a-value-envelope-to-an-empty-image/🦠️mutation/🔣️.json
    And the committed after-envelope shared://🧬️mutations/📸️set-snapshot/🔁️retypes-a-value-envelope-to-an-empty-image/📸️snapshot/➡️after/🔣️.json
    When set-snapshot is applied through apply_semio_mutation
    Then the envelope equals the committed empty image envelope, carries the image subset and raises no diagnostic

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Rebuild the committed envelope from an empty one, and reproduce the real envelope artifact byte for byte
    Given the committed before-envelope shared://🧬️mutations/📸️set-snapshot/✉️replaces-the-envelope-wrapping-a-value-subset/📸️snapshot/⬅️before/🔣️.json
    And the real committed text artifact asset://🌐️envelope/🗣️.dsl.semio
    And its committed binary twin asset://🌐️envelope/🎒️.pack.semio
    When the empty envelope is replaced with the committed one through apply_semio_mutation
    And the text artifact is parsed and printed back to DSL, and the binary twin is decoded and re-encoded
    Then the envelope equals the committed before-envelope, carries the value subset and raises no diagnostic
    And both encodings decode to the same envelope and each re-encoding reproduces its committed file byte for byte
