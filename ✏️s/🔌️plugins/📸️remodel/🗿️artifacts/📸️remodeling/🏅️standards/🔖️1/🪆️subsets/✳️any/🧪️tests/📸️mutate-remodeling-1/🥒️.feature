@capability-remodeling-1-mutate
@oracle-remodeling-1-python-independent
@comparison-ordered-json-v1
@mutations-remodeling-1-any
Feature: Apply every typed remodeling-scene mutation to its committed specification vector and against an independent Python implementation
  `s.remodeling.remodeling` is a semio-NATIVE artifact, and the document is a reconstruction
  JOB — streams, calibrations, ground control points, the eight parameter blocks a pipeline runs
  under, and the engine-owned results — not a point cloud or a mesh file. A reader of COLMAP, LAS or
  PLY output would therefore be judging a different artifact, and nothing reads `.dsl.semio`. That is
  recorded as the `remodeling-mutation-semantics` no-oracle decision in
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json`. That decision is narrowed to an empty
  `capabilities` list rather than deleted (it already was, by a prior shard of this same ticket),
  because its own investigation remains the honest record of what was checked; a dated note is
  appended recording that the `asset://` blocker it named is now resolved.

  🐍️ `🐍️component.py` beside this file is the second IMPLEMENTATION that decision named as the
  remaining debt: 34 of this vocabulary's 35 kinds, written in Python from this subset's own
  committed `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json` document shape and each kind's own
  committed `(before, mutation, after)` leaf fixture, and from
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/📓️taxonomy.md`'s
  verb table. It imports nothing from the Rust it judges and transliterates none of it. Both
  implementations now read the SAME committed bytes for those 34 kinds: every `(before, mutation,
  after)` path is a declared `asset://` fixture rather than an `include_str!`-only literal, so the
  plan pins its digest and a Python reference can resolve it. `commit-reconstruction` — the 35th kind
  — and `identity-round-trip` both stay exactly as they were, asserted by the Rust subject alone, for
  the reasons given below and in the module docstring.

  🔑 One real content-address hazard survives, narrowly: `create-asset` mints a NEW
  `assets.<key>.childId` via `std::collections::hash_map::DefaultHasher` — an algorithm the Rust
  standard library explicitly documents as unspecified, not merely un-surveyed. The Python reference
  compares every other field exactly and only checks that digest's SHAPE, adopting the committed
  value for equality rather than fabricating an independent match — stated in the reference's own
  docstring, not concealed. `delete-asset`'s inverse sidesteps the hazard entirely, because the
  committed BEFORE-document already carries the target's fully-formed handle verbatim.

  ⛓️ `delete-stream` cascades into any GCP observation naming that stream (the committed vector
  severs `gcp-corner`'s one observation alongside the stream, exactly as `delete-gcp` cascades into
  its OWN observations when a whole GCP goes). The Python reference's inverse restores BOTH the
  stream and each severed observation, in original order, per `taxonomy.md` rule 5 ("re-`connect`ed
  after `create`, in reverse dependency order") — independently derived from the specification, not
  read off production's own single-step `↩️inverse/🦀️.rs` for this kind.

  📄️ `commit-reconstruction` is the one kind with NO committed leaf vector, and the reason is
  structural rather than an oversight: its diff reads process-global staging state
  (`commit_staged_remodeling_reconstruction`, `durable_staged_remodeling_asset`) that a
  `(before, mutation, after)` triple cannot carry. This case exercises it through its own documented
  refusal path instead, using a vector assembled ONCE from committed sibling content and kept in this
  case's own fixtures — local://⬅️commit-reconstruction-before.json is a byte copy of
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️replace-job/🧪️tests/🎨️advances-the-job-to-texturing/📸️snapshot/⬅️before/🔣️.json`,
  local://🦠️commit-reconstruction-mutation.json pairs that leaf's committed `job` payload with
  `⭐replace-sparse`'s committed `sparse` payload (a plain point buffer, deliberately NOT a replayable
  staging handle), and local://➡️commit-reconstruction-after.json is the before-document unchanged,
  because the documented answer is `mutation.invalid-reconstruction-sparse` and a refused commit must
  leave the scene untouched. Note also that `commit-reconstruction`'s own inverse restores only `job`
  and the six result slots — never `assets` or `durable_artifacts` — so the inverse law holds for this
  refusal vector and would NOT hold for a commit that published new assets; that is a real weakness of
  the kind, recorded here rather than hidden by the vector that dodges it. Because this scenario
  outline is not converted, the runner executes NO oracle role for it, and every assertion lives in
  the subject handler, exactly as before.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Applying <id> reaches its committed after-document
    Given the committed before-document asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-document asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    When <id> is applied through apply_remodeling_mutation_json
    Then the resulting document is the committed after-document, the mutation moved it, and the two implementations agree
    Examples:
      | id                        | dir                         | fixture                                                     |
      | create-stream             | 🌱create-stream              | 🎥️adds-stream-c-bound-to-cam-b                                |
      | delete-stream             | 🪓delete-stream              | 🚫️removes-stream-b-and-cascades-its-gcp-observation           |
      | change-stream-sync        | ⏱️change-stream-sync        | ⏱️shifts-stream-a-sync-offset-to-minus-seven-and-a-half       |
      | add-stream-frame          | ➕add-stream-frame           | 🎞️appends-a-third-frame-to-stream-a                           |
      | remove-stream-frame       | ➖remove-stream-frame        | 🚫️removes-the-last-frame-of-stream-a                          |
      | replace-stream-source     | 🔁replace-stream-source      | 🧹️clears-the-video-source-of-stream-a                         |
      | create-asset              | 🧷create-asset               | 🖼️stores-a-new-jpeg-frame-asset                               |
      | delete-asset              | 🗞️delete-asset              | 🗑️removes-asset-a-and-reports-its-stale-references            |
      | create-camera-calibration | 🔭create-camera-calibration  | 📷️adds-the-cam-c-fisheye-calibration                          |
      | update-camera-calibration | 🛠️update-camera-calibration | 🔍️refines-the-cam-a-focal-length-and-rms                      |
      | delete-camera-calibration | 🚫delete-camera-calibration  | 🚫️removes-the-cam-b-calibration                               |
      | create-rig-extrinsic      | ⛓️create-rig-extrinsic      | 🔗️adds-a-rig-extrinsic-for-cam-b                              |
      | delete-rig-extrinsic      | ✂️delete-rig-extrinsic      | ✂️drops-the-cam-a-rig-extrinsic                               |
      | update-rig-extrinsic      | 🔩update-rig-extrinsic       | 📍️retunes-the-cam-a-rig-translation                           |
      | create-gcp                | 🧿create-gcp                 | 📍️adds-gcp-tower-with-one-observation                         |
      | delete-gcp                | 🚮delete-gcp                 | 🚫️removes-gcp-corner-and-cascades-its-observation             |
      | add-gcp-observation       | 🔎add-gcp-observation        | 🔎️adds-the-first-observation-to-gcp-ridge                     |
      | remove-gcp-observation    | 🚷remove-gcp-observation     | 🚫️removes-the-only-observation-of-gcp-corner                  |
      | update-ingest-params      | 🥣update-ingest-params       | 🔍️tightens-the-ingest-sharpness-gate                          |
      | update-feature-params     | 🌠update-feature-params      | 🔎️switches-the-detector-to-akaze                              |
      | update-match-params       | 🪢update-match-params        | 🌳️switches-the-matcher-to-a-kd-tree                           |
      | update-sfm-params         | 🧮update-sfm-params          | 🎯️switches-the-robust-loss-to-cauchy                          |
      | update-dense-params       | 🌁update-dense-params        | 🔬️raises-the-dense-resolution-and-confidence-gate             |
      | update-mesh-params        | 🕸️update-mesh-params        | 🔳️doubles-the-texture-size-and-drops-the-watertight-guarantee |
      | update-motion-params      | 🏎️update-motion-params      | 🏃️enables-motion-tracking                                     |
      | update-geo-params         | 🌐update-geo-params          | 🌐️enables-georeferencing-with-an-origin                       |
      | replace-job               | 🏗️replace-job               | 🎨️advances-the-job-to-texturing                               |
      | replace-sparse            | ⭐replace-sparse             | ✨️swaps-in-an-uncolored-four-point-sparse-cloud               |
      | replace-dense             | ☁️replace-dense             | ☁️swaps-in-a-two-point-classified-dense-cloud                 |
      | replace-mesh-result       | 🧱replace-mesh-result        | 🕸️swaps-in-an-imported-untextured-mesh                        |
      | replace-trajectory        | 🛣️replace-trajectory        | 🧹️clears-the-camera-trajectory                                |
      | replace-tracks            | 🚂replace-tracks             | ⏸️replaces-the-moving-track-with-two-static-tracks            |
      | replace-geo-products      | 🗾replace-geo-products       | 🗺️adds-the-dtm-and-ortho-rasters                              |
      | replace-qc                | 🧾replace-qc                 | 📋️records-a-qc-report-carrying-a-watertight-summary           |

  @id-mutate
  @level-exhaustive
  @mode-error
  Scenario Outline: Applying <id> is refused exactly as its vector declares
    Given the committed before-document and mutation payload of the <id> specification vector
    When <id> is applied through apply_remodeling_mutation_json
    Then the document is left untouched and the declared <code> refusal was raised
    Examples:
      | id                        | code                                   |
      | commit-reconstruction     | mutation.invalid-reconstruction-sparse |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores its committed before-document
    Given the committed before-document asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    When <id> and then every step of its own computed inverse are applied through undo_remodeling_mutation_json
    Then the document is the committed before-document again, member positions included, and the two implementations agree
    Examples:
      | id                        | dir                         | fixture                                                     |
      | create-stream             | 🌱create-stream              | 🎥️adds-stream-c-bound-to-cam-b                                |
      | delete-stream             | 🪓delete-stream              | 🚫️removes-stream-b-and-cascades-its-gcp-observation           |
      | change-stream-sync        | ⏱️change-stream-sync        | ⏱️shifts-stream-a-sync-offset-to-minus-seven-and-a-half       |
      | add-stream-frame          | ➕add-stream-frame           | 🎞️appends-a-third-frame-to-stream-a                           |
      | remove-stream-frame       | ➖remove-stream-frame        | 🚫️removes-the-last-frame-of-stream-a                          |
      | replace-stream-source     | 🔁replace-stream-source      | 🧹️clears-the-video-source-of-stream-a                         |
      | create-asset              | 🧷create-asset               | 🖼️stores-a-new-jpeg-frame-asset                               |
      | delete-asset              | 🗞️delete-asset              | 🗑️removes-asset-a-and-reports-its-stale-references            |
      | create-camera-calibration | 🔭create-camera-calibration  | 📷️adds-the-cam-c-fisheye-calibration                          |
      | update-camera-calibration | 🛠️update-camera-calibration | 🔍️refines-the-cam-a-focal-length-and-rms                      |
      | delete-camera-calibration | 🚫delete-camera-calibration  | 🚫️removes-the-cam-b-calibration                               |
      | create-rig-extrinsic      | ⛓️create-rig-extrinsic      | 🔗️adds-a-rig-extrinsic-for-cam-b                              |
      | delete-rig-extrinsic      | ✂️delete-rig-extrinsic      | ✂️drops-the-cam-a-rig-extrinsic                               |
      | update-rig-extrinsic      | 🔩update-rig-extrinsic       | 📍️retunes-the-cam-a-rig-translation                           |
      | create-gcp                | 🧿create-gcp                 | 📍️adds-gcp-tower-with-one-observation                         |
      | delete-gcp                | 🚮delete-gcp                 | 🚫️removes-gcp-corner-and-cascades-its-observation             |
      | add-gcp-observation       | 🔎add-gcp-observation        | 🔎️adds-the-first-observation-to-gcp-ridge                     |
      | remove-gcp-observation    | 🚷remove-gcp-observation     | 🚫️removes-the-only-observation-of-gcp-corner                  |
      | update-ingest-params      | 🥣update-ingest-params       | 🔍️tightens-the-ingest-sharpness-gate                          |
      | update-feature-params     | 🌠update-feature-params      | 🔎️switches-the-detector-to-akaze                              |
      | update-match-params       | 🪢update-match-params        | 🌳️switches-the-matcher-to-a-kd-tree                           |
      | update-sfm-params         | 🧮update-sfm-params          | 🎯️switches-the-robust-loss-to-cauchy                          |
      | update-dense-params       | 🌁update-dense-params        | 🔬️raises-the-dense-resolution-and-confidence-gate             |
      | update-mesh-params        | 🕸️update-mesh-params        | 🔳️doubles-the-texture-size-and-drops-the-watertight-guarantee |
      | update-motion-params      | 🏎️update-motion-params      | 🏃️enables-motion-tracking                                     |
      | update-geo-params         | 🌐update-geo-params          | 🌐️enables-georeferencing-with-an-origin                       |
      | replace-job               | 🏗️replace-job               | 🎨️advances-the-job-to-texturing                               |
      | replace-sparse            | ⭐replace-sparse             | ✨️swaps-in-an-uncolored-four-point-sparse-cloud               |
      | replace-dense             | ☁️replace-dense             | ☁️swaps-in-a-two-point-classified-dense-cloud                 |
      | replace-mesh-result       | 🧱replace-mesh-result        | 🕸️swaps-in-an-imported-untextured-mesh                        |
      | replace-trajectory        | 🛣️replace-trajectory        | 🧹️clears-the-camera-trajectory                                |
      | replace-tracks            | 🚂replace-tracks             | ⏸️replaces-the-moving-track-with-two-static-tracks            |
      | replace-geo-products      | 🗾replace-geo-products       | 🗺️adds-the-dtm-and-ortho-rasters                              |
      | replace-qc                | 🧾replace-qc                 | 📋️records-a-qc-report-carrying-a-watertight-summary           |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores its committed before-document (subject-only)
    Given the committed before-document and mutation payload of the <id> specification vector
    When <id> and then every step of its own computed inverse are applied through undo_remodeling_mutation_json
    Then the document is the committed before-document again, member positions included
    Examples:
      | id                        |
      | commit-reconstruction     |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse and reprint the real committed example without passing bytes through
    Given the real committed example asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    When it is parsed, printed back to DSL and parsed again through round_trip_remodeling_dsl
    Then both parses agree on one document, and the reprinted text reproduces the committed example byte for byte
