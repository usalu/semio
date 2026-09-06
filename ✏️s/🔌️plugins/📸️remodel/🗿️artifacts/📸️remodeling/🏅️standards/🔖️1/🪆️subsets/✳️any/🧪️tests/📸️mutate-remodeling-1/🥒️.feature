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
  verb table. It imports nothing from the Rust it judges and transliterates none of it.

  📍️ WHERE A VECTOR LIVES IS THE FEATURE'S OWN ANSWER, not either implementation's. Each row of the
  two differential outlines carries a `<vector>` — the `<kind directory>/🧪️tests/<case directory>`
  pair as it stands ON DISK — and the scenario's doc string turns it into the three `asset://` URIs
  that address the committed leaf. Both implementations resolve those URIs through the test context
  at RUN time (`ctx.fixture_json` in Rust, `ctx.fixture_bytes` in Python), so both read the same
  committed bytes, the plan pins their digests, and neither carries a transcribed copy of a fixture
  path that can drift away from the directory it names. That drift is not hypothetical: the
  2026-09-05 repo-wide path-shortening pass renamed every case directory here and left 99
  compile-time `include_str!` literals and this feature's own Examples columns addressing names that
  no longer existed. A path that appears exactly once, in the row that owns it, cannot repeat that.

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
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️replace-job/🧪️tests/🎨️advances-the-job-c1e878/📸️snapshot/⬅️before/🔣️.json`,
  local://🦠️commit-reconstruction-mutation.json pairs that leaf's committed `job` payload with
  `⭐replace-sparse`'s committed `sparse` payload (a plain point buffer, deliberately NOT a replayable
  staging handle), and local://➡️commit-reconstruction-after.json is the before-document unchanged,
  because the documented answer is `mutation.invalid-reconstruction-sparse` and a refused commit must
  leave the scene untouched. Note also that `commit-reconstruction`'s own inverse restores only `job`
  and the six result slots — never `assets` or `durable_artifacts` — so the inverse law holds for this
  refusal vector and would NOT hold for a commit that published new assets; that is a real weakness of
  the kind, recorded here rather than hidden by the vector that dodges it. Because this kind's two
  scenarios carry no `<vector>` row, the runner executes NO oracle role for them, every assertion
  lives in the subject handler, and the three `local://` fixtures above are the only committed bytes
  either half reads by a name this feature does not carry.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Applying <id> reaches its committed after-document
    Given the committed specification vector for the <id> kind
      """
      {
        "kind": "<id>",
        "before": "asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json",
        "mutation": "asset://🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️.json",
        "after": "asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️.json"
      }
      """
    When <id> is applied through apply_remodeling_mutation_json
    Then the resulting document is the committed after-document, the mutation moved it, and the two implementations agree
    Examples:
      | id                        | vector                                                       |
      | create-stream             | 🌱create-stream/🧪️tests/🎥️adds-stream-c-458900                |
      | delete-stream             | 🪓delete-stream/🧪️tests/🚫️removes-stream-b-0f62a7             |
      | change-stream-sync        | ⏱️change-stream-sync/🧪️tests/t038                            |
      | add-stream-frame          | ➕add-stream-frame/🧪️tests/🎞️appends-a-third-8ac259           |
      | remove-stream-frame       | ➖remove-stream-frame/🧪️tests/🚫️removes-the-last-304bdf       |
      | replace-stream-source     | 🔁replace-stream-source/🧪️tests/🧹️clears-the-video-143f2b     |
      | create-asset              | 🧷create-asset/🧪️tests/🖼️stores-a-new-d56283                  |
      | delete-asset              | 🗞️delete-asset/🧪️tests/🗑️removes-asset-a-170889              |
      | create-camera-calibration | 🔭create-camera-calibration/🧪️tests/📷️adds-the-cam-c-82c8fb   |
      | update-camera-calibration | 🛠️update-camera-calibration/🧪️tests/🔍️refines-the-cam-0eaef0 |
      | delete-camera-calibration | 🚫delete-camera-calibration/🧪️tests/🚫️removes-the-cam-f90b89  |
      | create-rig-extrinsic      | ⛓️create-rig-extrinsic/🧪️tests/🔗️adds-a-rig-2df5df           |
      | delete-rig-extrinsic      | ✂️delete-rig-extrinsic/🧪️tests/✂️drops-the-cam-a-a1f8a2      |
      | update-rig-extrinsic      | 🔩update-rig-extrinsic/🧪️tests/📍️retunes-the-cam-4ca5a2       |
      | create-gcp                | 🧿create-gcp/🧪️tests/📍️adds-gcp-tower-d71a54                  |
      | delete-gcp                | 🚮delete-gcp/🧪️tests/🚫️removes-gcp-209b7d                     |
      | add-gcp-observation       | 🔎add-gcp-observation/🧪️tests/🔎️adds-the-first-05b1b5         |
      | remove-gcp-observation    | 🚷remove-gcp-observation/🧪️tests/🚫️removes-the-only-f82e64    |
      | update-ingest-params      | 🥣update-ingest-params/🧪️tests/🔍️tightens-the-499c47          |
      | update-feature-params     | 🌠update-feature-params/🧪️tests/🔎️switches-the-423de9         |
      | update-match-params       | 🪢update-match-params/🧪️tests/🌳️switches-the-652d03           |
      | update-sfm-params         | 🧮update-sfm-params/🧪️tests/🎯️switches-the-7f0371             |
      | update-dense-params       | 🌁update-dense-params/🧪️tests/🔬️raises-the-dense-ddb263       |
      | update-mesh-params        | 🕸️update-mesh-params/🧪️tests/t039                            |
      | update-motion-params      | 🏎️update-motion-params/🧪️tests/🏃️enables-motion-2444a3       |
      | update-geo-params         | 🌐update-geo-params/🧪️tests/🌐️enables-georefere-18a68a        |
      | replace-job               | 🏗️replace-job/🧪️tests/🎨️advances-the-job-c1e878              |
      | replace-sparse            | ⭐replace-sparse/🧪️tests/✨️swaps-in-an-6d9ae4                 |
      | replace-dense             | ☁️replace-dense/🧪️tests/☁️swaps-in-a-two-c688db              |
      | replace-mesh-result       | 🧱replace-mesh-result/🧪️tests/🕸️swaps-in-an-f23e71            |
      | replace-trajectory        | 🛣️replace-trajectory/🧪️tests/🧹️clears-the-d2f81a             |
      | replace-tracks            | 🚂replace-tracks/🧪️tests/⏸️replaces-the-d40c68                |
      | replace-geo-products      | 🗾replace-geo-products/🧪️tests/🗺️adds-the-dtm-and-64d5bb      |
      | replace-qc                | 🧾replace-qc/🧪️tests/📋️records-a-qc-f5caf4                    |

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
    Given the committed specification vector for the <id> kind
      """
      {
        "kind": "<id>",
        "before": "asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json",
        "mutation": "asset://🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️.json",
        "after": "asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️.json"
      }
      """
    When <id> and then every step of its own computed inverse are applied through undo_remodeling_mutation_json
    Then the document is the committed before-document again, member positions included, and the two implementations agree
    Examples:
      | id                        | vector                                                       |
      | create-stream             | 🌱create-stream/🧪️tests/🎥️adds-stream-c-458900                |
      | delete-stream             | 🪓delete-stream/🧪️tests/🚫️removes-stream-b-0f62a7             |
      | change-stream-sync        | ⏱️change-stream-sync/🧪️tests/t038                            |
      | add-stream-frame          | ➕add-stream-frame/🧪️tests/🎞️appends-a-third-8ac259           |
      | remove-stream-frame       | ➖remove-stream-frame/🧪️tests/🚫️removes-the-last-304bdf       |
      | replace-stream-source     | 🔁replace-stream-source/🧪️tests/🧹️clears-the-video-143f2b     |
      | create-asset              | 🧷create-asset/🧪️tests/🖼️stores-a-new-d56283                  |
      | delete-asset              | 🗞️delete-asset/🧪️tests/🗑️removes-asset-a-170889              |
      | create-camera-calibration | 🔭create-camera-calibration/🧪️tests/📷️adds-the-cam-c-82c8fb   |
      | update-camera-calibration | 🛠️update-camera-calibration/🧪️tests/🔍️refines-the-cam-0eaef0 |
      | delete-camera-calibration | 🚫delete-camera-calibration/🧪️tests/🚫️removes-the-cam-f90b89  |
      | create-rig-extrinsic      | ⛓️create-rig-extrinsic/🧪️tests/🔗️adds-a-rig-2df5df           |
      | delete-rig-extrinsic      | ✂️delete-rig-extrinsic/🧪️tests/✂️drops-the-cam-a-a1f8a2      |
      | update-rig-extrinsic      | 🔩update-rig-extrinsic/🧪️tests/📍️retunes-the-cam-4ca5a2       |
      | create-gcp                | 🧿create-gcp/🧪️tests/📍️adds-gcp-tower-d71a54                  |
      | delete-gcp                | 🚮delete-gcp/🧪️tests/🚫️removes-gcp-209b7d                     |
      | add-gcp-observation       | 🔎add-gcp-observation/🧪️tests/🔎️adds-the-first-05b1b5         |
      | remove-gcp-observation    | 🚷remove-gcp-observation/🧪️tests/🚫️removes-the-only-f82e64    |
      | update-ingest-params      | 🥣update-ingest-params/🧪️tests/🔍️tightens-the-499c47          |
      | update-feature-params     | 🌠update-feature-params/🧪️tests/🔎️switches-the-423de9         |
      | update-match-params       | 🪢update-match-params/🧪️tests/🌳️switches-the-652d03           |
      | update-sfm-params         | 🧮update-sfm-params/🧪️tests/🎯️switches-the-7f0371             |
      | update-dense-params       | 🌁update-dense-params/🧪️tests/🔬️raises-the-dense-ddb263       |
      | update-mesh-params        | 🕸️update-mesh-params/🧪️tests/t039                            |
      | update-motion-params      | 🏎️update-motion-params/🧪️tests/🏃️enables-motion-2444a3       |
      | update-geo-params         | 🌐update-geo-params/🧪️tests/🌐️enables-georefere-18a68a        |
      | replace-job               | 🏗️replace-job/🧪️tests/🎨️advances-the-job-c1e878              |
      | replace-sparse            | ⭐replace-sparse/🧪️tests/✨️swaps-in-an-6d9ae4                 |
      | replace-dense             | ☁️replace-dense/🧪️tests/☁️swaps-in-a-two-c688db              |
      | replace-mesh-result       | 🧱replace-mesh-result/🧪️tests/🕸️swaps-in-an-f23e71            |
      | replace-trajectory        | 🛣️replace-trajectory/🧪️tests/🧹️clears-the-d2f81a             |
      | replace-tracks            | 🚂replace-tracks/🧪️tests/⏸️replaces-the-d40c68                |
      | replace-geo-products      | 🗾replace-geo-products/🧪️tests/🗺️adds-the-dtm-and-64d5bb      |
      | replace-qc                | 🧾replace-qc/🧪️tests/📋️records-a-qc-f5caf4                    |

  @id-inverse
  @level-exhaustive
  @mode-differential
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
