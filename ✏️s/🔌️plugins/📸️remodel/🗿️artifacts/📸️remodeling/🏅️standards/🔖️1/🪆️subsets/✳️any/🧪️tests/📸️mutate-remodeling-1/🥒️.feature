@capability-remodeling-1-mutate
@oracle-remodeling-1-python-independent
@comparison-ordered-json-v1
@mutations-remodeling-1-any
Feature: Apply every typed remodeling-scene mutation to its committed specification vector and against an independent Python implementation
  `s.remodel.remodeling` is a semio-NATIVE artifact, and the document is a reconstruction
  JOB — streams, calibrations, ground control points, the eight parameter blocks a pipeline runs
  under, and the engine-owned results — not a point cloud or a mesh file. A reader of COLMAP, LAS or
  PLY output would therefore be judging a different artifact, and nothing reads `.dsl.semio`. That is
  recorded as the `remodeling-mutation-semantics` no-oracle decision in
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json`. That decision is narrowed to an empty
  `capabilities` list rather than deleted (it already was, by a prior shard of this same ticket),
  because its own investigation remains the honest record of what was checked; a dated note is
  appended recording that the `asset://` blocker it named is now resolved.

  🐍️ `🐍️component.py` beside this file is the second IMPLEMENTATION that decision named as the
  remaining debt: all 35 of this vocabulary's kinds — 34 as applied mutations and
  `commit-reconstruction` as the refusal its own vector declares — written in Python from this subset's own
  committed `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json` document shape and each kind's own
  committed `(before, mutation, after)` leaf fixture, and from
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/📓️taxonomy.md`'s
  verb table. It imports nothing from the Rust it judges and transliterates none of it.

  📍️ WHERE A VECTOR LIVES IS THE FEATURE'S OWN ANSWER, not either implementation's. Each row of the
  two differential outlines carries a `<vector>` — the `<kind directory>/<case directory>`
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

  ⛓️ OWNERSHIP AND ORDER are the two laws every vector here is measured against. A `delete-*` takes
  what its record OWNS with it (a stream carries its frames, a GCP its observations) and REFUSES with
  `mutation.referenced` when something that merely names it would be left dangling — a GCP observation
  addressing the stream, a stream binding or rig entry naming the camera, a frame, texture or geo
  product naming the asset. And every keyed collection is held in ascending key order, so a
  `create-*`/`add-*` puts a member back exactly where a `delete-*`/`remove-*` took it from. Together
  they make every one of the 35 kinds' inverses a single step of this same vocabulary that restores
  the committed BEFORE-document exactly, member positions included (`taxonomy.md` rule 5): the
  `inverse-` scenario below is planned for EVERY committed vector, with no exceptions.

  📊️ Every kind carries four ROLES as far as its own semantics reach: the `toy` vector on the
  two-stream unit scene these fixtures were first authored against, a `realworld` vector on the
  ten-frame two-camera orbit survey (a durable-artifact store, a four-point GCP network observed
  across both streams, a 104-point sparse cloud, a dense cloud with confidence and classification
  lanes, a ten-pose trajectory, motion tracks, geo products and a QC report), a `refusal` vector
  per distinct guard the kind's own `🔺️diff/🦀️.rs` raises, and an `edge` vector for first/last/
  empty position — or, for a kind whose ONLY non-applying path is an identical resubmission, for
  the `mutation.no-op` Warning it raises instead. A refusal ships no `🔺️diff/🔣️.json`: the
  `🚫️.absent` marker beside it is this repository's own way of committing that there is no delta.

  📄️ `commit-reconstruction` additionally keeps a case-local vector, and the reason is
  structural rather than an oversight: its diff reads process-global staging state
  (`commit_staged_remodeling_reconstruction`, `durable_staged_remodeling_asset`) that a
  `(before, mutation, after)` triple cannot carry. This case exercises it through its own documented
  refusal path instead, using a vector assembled ONCE from committed sibling content and kept in this
  owner's shared fixtures — shared://🏁️commit-reconstruction/⬅️before.json is a byte copy of
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️replace-job/🎨️advances-the-job-c1e878/📸️snapshot/⬅️before/🔣️.json`,
  shared://🏁️commit-reconstruction/🦠️mutation.json pairs that leaf's committed `job` payload with
  `⭐replace-sparse`'s committed `sparse` payload (a plain point buffer, deliberately NOT a replayable
  staging handle), and shared://🏁️commit-reconstruction/➡️after.json is the before-document unchanged,
  because the documented answer is `mutation.invalid-reconstruction-sparse` and a refused commit must
  leave the scene untouched. Note also that `commit-reconstruction`'s own inverse restores only `job`
  and the six result slots — never `assets` or `durable_artifacts` — so the inverse law holds for this
  refusal vector and would NOT hold for a commit that published new assets; that is a real weakness of
  the kind, recorded here rather than hidden by the vector that dodges it. Its OTHER three guards — invalid-reconstruction-asset, invalid-reconstruction-mesh and the sparse
  guard again on the real-world survey — need no staging state at all and ship as ordinary leaf
  vectors under `🧬️schema/🧬️mutations/🏁commit-reconstruction/`. This vector's own two scenarios
  address that vector by the same doc-string mechanism as every other row — three `shared://` URIs in
  place of the `asset://` triple a `<vector>` builds — so both halves resolve it at run time and both
  halves answer for it, the reference deriving the refusal from the payload's own shape rather than
  adopting production's verdict.

  🔁 `identity-round-trip` compares the two halves on the PRINTED CARRIER rather than on a parsed
  document, because the reference cannot parse `.dsl.semio`: this subset's committed text grammar is
  the repository-wide placeholder whose whole body is `payload = OCTET+`, which the oracle registry's
  own `remodeling-mutation-semantics` entry already reports. What the reference CAN state from the
  committed bytes alone is what the carrier must be after a faithful parse-and-reprint — those same
  bytes. The subject asserts the other half, that reparsing its own printout yields the document it
  first parsed, inside its handler, where a byte comparison could not reach it.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Applying <id> reaches its committed after-document
    Given the committed specification vector for the <id> kind
      """
      {
        "kind": "<kind>",
        "before": "shared://🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json",
        "mutation": "shared://🧬️mutations/<vector>/🦠️mutation/🔣️.json",
        "after": "shared://🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️.json"
      }
      """
    When <kind> is applied through apply_remodeling_mutation_json
    Then the resulting document is the committed after-document, the mutation moved it, and the two implementations agree
    Examples:
      | id                                  | kind                      | vector                                                       |
      | add-gcp-observation                 | add-gcp-observation       | 🔎add-gcp-observation/🔎️adds-the-first-05b1b5         |
      | add-gcp-observation-realworld       | add-gcp-observation       | 🔎add-gcp-observation/🔎️picks-the-south-eb0c4d        |
      | add-stream-frame                    | add-stream-frame          | ➕add-stream-frame/🎞️appends-a-third-8ac259           |
      | add-stream-frame-realworld          | add-stream-frame          | ➕add-stream-frame/🎞️appends-an-0c2164                |
      | change-stream-sync                  | change-stream-sync        | ⏱️change-stream-sync/⏱️shifts-stream-a-5b442c        |
      | change-stream-sync-realworld        | change-stream-sync        | ⏱️change-stream-sync/⏱️retimes-the-50dd75            |
      | create-asset                        | create-asset              | 🧷create-asset/🖼️stores-a-new-d56283                  |
      | create-asset-realworld              | create-asset              | 🧷create-asset/🖼️stores-an-9f39e1                     |
      | create-asset-upsert                 | create-asset              | 🧷create-asset/♻️overwrites-an-a34b9d                 |
      | create-camera-calibration           | create-camera-calibration | 🔭create-camera-calibration/📷️adds-the-cam-c-82c8fb   |
      | create-camera-calibration-realworld | create-camera-calibration | 🔭create-camera-calibration/📷️adds-a-fourth-97e912    |
      | create-gcp                          | create-gcp                | 🧿create-gcp/📍️adds-gcp-tower-d71a54                  |
      | create-gcp-realworld                | create-gcp                | 🧿create-gcp/📍️adds-a-quay-7569de                     |
      | create-gcp-unobserved               | create-gcp                | 🧿create-gcp/🕳️adds-a-control-298de4                  |
      | create-rig-extrinsic                | create-rig-extrinsic      | ⛓️create-rig-extrinsic/🔗️adds-a-rig-2df5df           |
      | create-rig-extrinsic-realworld      | create-rig-extrinsic      | ⛓️create-rig-extrinsic/🔗️places-the-0d0b8d           |
      | create-stream                       | create-stream             | 🌱create-stream/🎥️adds-stream-c-458900                |
      | create-stream-realworld             | create-stream             | 🌱create-stream/🛰️adds-a-third-61fb5d                 |
      | create-stream-unbound               | create-stream             | 🌱create-stream/🎞️adds-an-unbound-2b2373              |
      | delete-asset                        | delete-asset              | 🗞️delete-asset/🧹️drops-the-spare-c6ffb6              |
      | delete-asset-realworld              | delete-asset              | 🗞️delete-asset/🗑️sweeps-the-503b27                   |
      | delete-camera-calibration           | delete-camera-calibration | 🚫delete-camera-calibration/🚫️removes-the-cam-f90b89  |
      | delete-camera-calibration-realworld | delete-camera-calibration | 🚫delete-camera-calibration/🚫️removes-the-40cba4      |
      | delete-gcp                          | delete-gcp                | 🚮delete-gcp/🚫️removes-gcp-209b7d                     |
      | delete-gcp-realworld                | delete-gcp                | 🚮delete-gcp/🚮removes-the-south-42cd9e                |
      | delete-gcp-unobserved               | delete-gcp                | 🚮delete-gcp/🕳️removes-an-8f3868                      |
      | delete-rig-extrinsic                | delete-rig-extrinsic      | ✂️delete-rig-extrinsic/✂️drops-the-cam-a-a1f8a2      |
      | delete-rig-extrinsic-first          | delete-rig-extrinsic      | ✂️delete-rig-extrinsic/⏮️unplaces-the-f5b35e         |
      | delete-rig-extrinsic-realworld      | delete-rig-extrinsic      | ✂️delete-rig-extrinsic/✂️unplaces-the-a39356         |
      | delete-stream                       | delete-stream             | 🪓delete-stream/⏮️removes-the-first-c0fc2a            |
      | delete-stream-realworld             | delete-stream             | 🪓delete-stream/🪓removes-the-spare-556d1d             |
      | remove-gcp-observation              | remove-gcp-observation    | 🚷remove-gcp-observation/🚫️removes-the-only-f82e64    |
      | remove-gcp-observation-first        | remove-gcp-observation    | 🚷remove-gcp-observation/⏮️drops-the-first-9ebf0b     |
      | remove-gcp-observation-realworld    | remove-gcp-observation    | 🚷remove-gcp-observation/🚷drops-the-middle-282fb7     |
      | remove-stream-frame                 | remove-stream-frame       | ➖remove-stream-frame/🚫️removes-the-last-304bdf       |
      | remove-stream-frame-first           | remove-stream-frame       | ➖remove-stream-frame/⏮️drops-the-first-d98a0f        |
      | remove-stream-frame-realworld       | remove-stream-frame       | ➖remove-stream-frame/✂️drops-the-middle-2d6d53       |
      | replace-dense                       | replace-dense             | ☁️replace-dense/☁️swaps-in-a-two-c688db              |
      | replace-dense-realworld             | replace-dense             | ☁️replace-dense/☁️swaps-in-a-denser-4174e1           |
      | replace-geo-products                | replace-geo-products      | 🗾replace-geo-products/🗺️adds-the-dtm-and-64d5bb      |
      | replace-geo-products-clears         | replace-geo-products      | 🗾replace-geo-products/🧹️clears-the-geo-f4886e        |
      | replace-geo-products-realworld      | replace-geo-products      | 🗾replace-geo-products/🗺️records-a-dtm-6e132a         |
      | replace-job                         | replace-job               | 🏗️replace-job/🎨️advances-the-job-c1e878              |
      | replace-job-realworld               | replace-job               | 🏗️replace-job/🎨️advances-the-555298                  |
      | replace-mesh-result                 | replace-mesh-result       | 🧱replace-mesh-result/🕸️swaps-in-an-f23e71            |
      | replace-mesh-result-realworld       | replace-mesh-result       | 🧱replace-mesh-result/🕸️swaps-the-c43d9c              |
      | replace-qc                          | replace-qc                | 🧾replace-qc/📋️records-a-qc-f5caf4                    |
      | replace-qc-clears                   | replace-qc                | 🧾replace-qc/🧹️clears-the-qc-1d2249                   |
      | replace-qc-realworld                | replace-qc                | 🧾replace-qc/✅️files-a-qc-report-64d222               |
      | replace-sparse                      | replace-sparse            | ⭐replace-sparse/✨️swaps-in-an-6d9ae4                 |
      | replace-sparse-realworld            | replace-sparse            | ⭐replace-sparse/✨️swaps-in-a-re-3cfa6d               |
      | replace-stream-source               | replace-stream-source     | 🔁replace-stream-source/🧹️clears-the-video-143f2b     |
      | replace-stream-source-attaches      | replace-stream-source     | 🔁replace-stream-source/📼️attaches-a-607df8           |
      | replace-stream-source-realworld     | replace-stream-source     | 🔁replace-stream-source/🎥️reingests-the-311c32        |
      | replace-tracks                      | replace-tracks            | 🚂replace-tracks/⏸️replaces-the-d40c68                |
      | replace-tracks-empty                | replace-tracks            | 🚂replace-tracks/🕳️clears-every-760061                |
      | replace-tracks-realworld            | replace-tracks            | 🚂replace-tracks/🏃️swaps-in-two-166265                |
      | replace-trajectory                  | replace-trajectory        | 🛣️replace-trajectory/🧹️clears-the-d2f81a             |
      | replace-trajectory-clears           | replace-trajectory        | 🛣️replace-trajectory/🕳️drops-the-6436a8              |
      | replace-trajectory-realworld        | replace-trajectory        | 🛣️replace-trajectory/🛣️swaps-in-a-three-49b17f       |
      | update-camera-calibration           | update-camera-calibration | 🛠️update-camera-calibration/🔍️refines-the-cam-0eaef0 |
      | update-camera-calibration-realworld | update-camera-calibration | 🛠️update-camera-calibration/🔍️refines-the-9fd25a     |
      | update-dense-params                 | update-dense-params       | 🌁update-dense-params/🔬️raises-the-dense-ddb263       |
      | update-dense-params-realworld       | update-dense-params       | 🌁update-dense-params/🧊️sharpens-the-25044c           |
      | update-feature-params               | update-feature-params     | 🌠update-feature-params/🔎️switches-the-423de9         |
      | update-feature-params-realworld     | update-feature-params     | 🌠update-feature-params/🌟️moves-the-3621f6            |
      | update-geo-params                   | update-geo-params         | 🌐update-geo-params/🌐️enables-georefere-18a68a        |
      | update-geo-params-realworld         | update-geo-params         | 🌐update-geo-params/🌐️halves-the-002a17               |
      | update-ingest-params                | update-ingest-params      | 🥣update-ingest-params/🔍️tightens-the-499c47          |
      | update-ingest-params-realworld      | update-ingest-params      | 🥣update-ingest-params/📥️widens-the-73f33e            |
      | update-match-params                 | update-match-params       | 🪢update-match-params/🌳️switches-the-652d03           |
      | update-match-params-realworld       | update-match-params       | 🪢update-match-params/🌳️switches-to-a-kd-d6fa4b       |
      | update-mesh-params                  | update-mesh-params        | 🕸️update-mesh-params/🔳️doubles-the-c245d5            |
      | update-mesh-params-realworld        | update-mesh-params        | 🕸️update-mesh-params/🔳️halves-the-voxel-21b53d       |
      | update-motion-params                | update-motion-params      | 🏎️update-motion-params/🏃️enables-motion-2444a3       |
      | update-motion-params-realworld      | update-motion-params      | 🏎️update-motion-params/🏃️triples-the-4bb69f          |
      | update-rig-extrinsic                | update-rig-extrinsic      | 🔩update-rig-extrinsic/📍️retunes-the-cam-4ca5a2       |
      | update-rig-extrinsic-realworld      | update-rig-extrinsic      | 🔩update-rig-extrinsic/📍️retunes-the-675f52           |
      | update-sfm-params                   | update-sfm-params         | 🧮update-sfm-params/🎯️switches-the-7f0371             |
      | update-sfm-params-realworld         | update-sfm-params         | 🧮update-sfm-params/🎯️tightens-the-850036             |

  @id-mutate
  @level-exhaustive
  @mode-error
  Scenario Outline: Applying <id> is refused exactly as its vector declares
    Given the committed refusal vector for the <id> kind
      """
      {
        "kind": "<kind>",
        "before": "shared://🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json",
        "mutation": "shared://🧬️mutations/<vector>/🦠️mutation/🔣️.json",
        "after": "shared://🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️.json",
        "code": "<code>"
      }
      """
    When <kind> is applied through apply_remodeling_mutation_json
    Then the document is left untouched and the declared <code> refusal was raised
    Examples:
      | id                                   | kind                      | vector                                                        | code                                   |
      | add-gcp-observation-missing          | add-gcp-observation       | 🔎add-gcp-observation/🚫️refuses-to-pick-3c0570         | mutation.target-missing                |
      | add-stream-frame-kind                | add-stream-frame          | ➕add-stream-frame/🎬️refuses-a-frame-81beea            | mutation.invariant                     |
      | add-stream-frame-missing             | add-stream-frame          | ➕add-stream-frame/🚫️refuses-to-c93e98                 | mutation.target-missing                |
      | change-stream-sync-missing           | change-stream-sync        | ⏱️change-stream-sync/🚫️refuses-to-8095d3              | mutation.target-missing                |
      | commit-reconstruction-asset          | commit-reconstruction     | 🏁commit-reconstruction/🖼️rejects-an-e9fa51            | mutation.invalid-reconstruction-asset  |
      | commit-reconstruction-mesh           | commit-reconstruction     | 🏁commit-reconstruction/🕸️rejects-an-5d3a60            | mutation.invalid-reconstruction-mesh   |
      | commit-reconstruction-sparse         | commit-reconstruction     | 🏁commit-reconstruction/⭐️rejects-an-2e5568            | mutation.invalid-reconstruction-sparse |
      | create-asset-staging-handle          | create-asset              | 🧷create-asset/🚫️refuses-an-asset-cb0d4b               | mutation.invalid-asset-payload         |
      | create-camera-calibration-duplicate  | create-camera-calibration | 🔭create-camera-calibration/🚫️refuses-a-camera-e92a02  | mutation.duplicate-id                  |
      | create-gcp-duplicate                 | create-gcp                | 🧿create-gcp/🚫️refuses-a-19c1ab                        | mutation.duplicate-id                  |
      | create-rig-extrinsic-duplicate       | create-rig-extrinsic      | ⛓️create-rig-extrinsic/🚫️refuses-a-second-95e04d      | mutation.duplicate-id                  |
      | create-rig-extrinsic-unknown-camera  | create-rig-extrinsic      | ⛓️create-rig-extrinsic/🚫️refuses-a-rig-cb71ba         | mutation.invariant                     |
      | create-stream-duplicate-id           | create-stream             | 🌱create-stream/🔂️rejects-a-6b58da                     | mutation.duplicate-id                  |
      | create-stream-unknown-camera         | create-stream             | 🌱create-stream/👻️rejects-a-stream-aac5c2              | mutation.invariant                     |
      | delete-asset-geo-product             | delete-asset              | 🗞️delete-asset/🗺️refuses-to-5c6f74                    | mutation.referenced                    |
      | delete-asset-missing                 | delete-asset              | 🗞️delete-asset/🚫️refuses-to-c4563a                    | mutation.target-missing                |
      | delete-asset-referenced-frames       | delete-asset              | 🗞️delete-asset/🖼️refuses-to-f9541f                    | mutation.referenced                    |
      | delete-camera-calibration-missing    | delete-camera-calibration | 🚫delete-camera-calibration/🚫️refuses-to-73655a        | mutation.target-missing                |
      | delete-camera-calibration-referenced | delete-camera-calibration | 🚫delete-camera-calibration/⛓️refuses-to-remove-3c8f32 | mutation.referenced                    |
      | delete-gcp-missing                   | delete-gcp                | 🚮delete-gcp/🚫️refuses-to-12366b                       | mutation.target-missing                |
      | delete-rig-extrinsic-missing         | delete-rig-extrinsic      | ✂️delete-rig-extrinsic/🚫️refuses-to-1805df            | mutation.target-missing                |
      | delete-stream-missing                | delete-stream             | 🪓delete-stream/🚫️refuses-to-3c20ff                    | mutation.target-missing                |
      | delete-stream-referenced             | delete-stream             | 🪓delete-stream/⛓️refuses-to-remove-422a37             | mutation.referenced                    |
      | remove-gcp-observation-out-of-range  | remove-gcp-observation    | 🚷remove-gcp-observation/🚫️refuses-an-109cf1           | mutation.target-missing                |
      | remove-stream-frame-out-of-range     | remove-stream-frame       | ➖remove-stream-frame/🚫️refuses-a-frame-e7c374         | mutation.target-missing                |
      | replace-geo-products-absent          | replace-geo-products      | 🗾replace-geo-products/🚫️refuses-to-clear-b8c54a       | mutation.target-missing                |
      | replace-mesh-result-staged           | replace-mesh-result       | 🧱replace-mesh-result/🚫️refuses-a-48f3a6               | mutation.incomplete-mesh               |
      | replace-qc-absent                    | replace-qc                | 🧾replace-qc/🚫️refuses-to-clear-30cbb5                 | mutation.target-missing                |
      | replace-stream-source-missing        | replace-stream-source     | 🔁replace-stream-source/🚫️refuses-to-f7f40d            | mutation.target-missing                |
      | replace-trajectory-absent            | replace-trajectory        | 🛣️replace-trajectory/🚫️refuses-to-clear-524569        | mutation.target-missing                |
      | update-camera-calibration-missing    | update-camera-calibration | 🛠️update-camera-calibration/🚫️refuses-to-b60a39       | mutation.target-missing                |
      | update-feature-params-invariant      | update-feature-params     | 🌠update-feature-params/🚫️refuses-a-d82e38             | mutation.invariant                     |
      | update-geo-params-invariant          | update-geo-params         | 🌐update-geo-params/🚫️refuses-a-zero-fa917f            | mutation.invariant                     |
      | update-ingest-params-invariant       | update-ingest-params      | 🥣update-ingest-params/🚫️refuses-an-59752a             | mutation.invariant                     |
      | update-match-params-invariant        | update-match-params       | 🪢update-match-params/🚫️refuses-a-ratio-65dcb9         | mutation.invariant                     |
      | update-rig-extrinsic-missing         | update-rig-extrinsic      | 🔩update-rig-extrinsic/🚫️refuses-to-2cfb53             | mutation.target-missing                |

  @id-mutate
  @level-exhaustive
  @mode-error
  Scenario Outline: Applying <id> is warned as a no-op and moves nothing
    Given the committed no-op vector for the <id> kind
      """
      {
        "kind": "<kind>",
        "before": "shared://🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json",
        "mutation": "shared://🧬️mutations/<vector>/🦠️mutation/🔣️.json",
        "after": "shared://🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️.json",
        "code": "<code>"
      }
      """
    When <kind> is applied through apply_remodeling_mutation_json
    Then the document is left untouched and the declared <code> refusal was raised
    Examples:
      | id                             | kind                      | vector                                                      | code           |
      | add-gcp-observation-noop       | add-gcp-observation       | 🔎add-gcp-observation/🔁️warns-that-this-dca661       | mutation.no-op |
      | add-stream-frame-noop          | add-stream-frame          | ➕add-stream-frame/🔁️warns-that-the-1e8abe           | mutation.no-op |
      | change-stream-sync-noop        | change-stream-sync        | ⏱️change-stream-sync/🔁️warns-that-the-a98c13        | mutation.no-op |
      | replace-dense-noop             | replace-dense             | ☁️replace-dense/🔁️warns-that-the-675b6e             | mutation.no-op |
      | replace-job-noop               | replace-job               | 🏗️replace-job/🔁️warns-that-the-bdf2e9               | mutation.no-op |
      | replace-mesh-result-noop       | replace-mesh-result       | 🧱replace-mesh-result/🔁️warns-that-the-b39bab        | mutation.no-op |
      | replace-sparse-noop            | replace-sparse            | ⭐replace-sparse/🔁️warns-that-the-56a3a9             | mutation.no-op |
      | replace-tracks-noop            | replace-tracks            | 🚂replace-tracks/🔁️warns-that-the-8dbf82             | mutation.no-op |
      | update-camera-calibration-noop | update-camera-calibration | 🛠️update-camera-calibration/🔁️warns-that-the-697b4f | mutation.no-op |
      | update-dense-params-noop       | update-dense-params       | 🌁update-dense-params/🔁️warns-that-the-4e65c8        | mutation.no-op |
      | update-feature-params-noop     | update-feature-params     | 🌠update-feature-params/🔁️warns-that-the-b6b7dc      | mutation.no-op |
      | update-geo-params-noop         | update-geo-params         | 🌐update-geo-params/🔁️warns-that-the-efc6e8          | mutation.no-op |
      | update-ingest-params-noop      | update-ingest-params      | 🥣update-ingest-params/🔁️warns-that-the-8eaad8       | mutation.no-op |
      | update-match-params-noop       | update-match-params       | 🪢update-match-params/🔁️warns-that-the-414aae        | mutation.no-op |
      | update-mesh-params-noop        | update-mesh-params        | 🕸️update-mesh-params/🔁️warns-that-the-887e9f        | mutation.no-op |
      | update-motion-params-noop      | update-motion-params      | 🏎️update-motion-params/🔁️warns-that-the-83ff67      | mutation.no-op |
      | update-rig-extrinsic-noop      | update-rig-extrinsic      | 🔩update-rig-extrinsic/🔁️warns-that-the-89422a       | mutation.no-op |
      | update-sfm-params-noop         | update-sfm-params         | 🧮update-sfm-params/🔁️warns-that-the-79a92a          | mutation.no-op |

  @id-mutate
  @level-exhaustive
  @mode-error
  Scenario Outline: Applying <id> is refused by the case-local staging vector it declares
    Given the case-local refusal vector for the <id> kind
      """
      {
        "kind": "<kind>",
        "before": "shared://🏁️commit-reconstruction/⬅️before.json",
        "mutation": "shared://🏁️commit-reconstruction/🦠️mutation.json",
        "after": "shared://🏁️commit-reconstruction/➡️after.json",
        "code": "<code>"
      }
      """
    When <kind> is applied through apply_remodeling_mutation_json
    Then the document is left untouched and the declared <code> refusal was raised
    Examples:
      | id                    | kind                  | code                                   |
      | commit-reconstruction | commit-reconstruction | mutation.invalid-reconstruction-sparse |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores its committed before-document
    Given the committed specification vector for the <id> kind
      """
      {
        "kind": "<kind>",
        "before": "shared://🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json",
        "mutation": "shared://🧬️mutations/<vector>/🦠️mutation/🔣️.json",
        "after": "shared://🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️.json"
      }
      """
    When <kind> and then every step of its own computed inverse are applied through undo_remodeling_mutation_json
    Then the document is the committed before-document again, member positions included, and the two implementations agree
    Examples:
      | id                                  | kind                      | vector                                                       |
      | add-gcp-observation                 | add-gcp-observation       | 🔎add-gcp-observation/🔎️adds-the-first-05b1b5         |
      | add-gcp-observation-realworld       | add-gcp-observation       | 🔎add-gcp-observation/🔎️picks-the-south-eb0c4d        |
      | add-stream-frame                    | add-stream-frame          | ➕add-stream-frame/🎞️appends-a-third-8ac259           |
      | add-stream-frame-realworld          | add-stream-frame          | ➕add-stream-frame/🎞️appends-an-0c2164                |
      | change-stream-sync                  | change-stream-sync        | ⏱️change-stream-sync/⏱️shifts-stream-a-5b442c        |
      | change-stream-sync-realworld        | change-stream-sync        | ⏱️change-stream-sync/⏱️retimes-the-50dd75            |
      | create-asset                        | create-asset              | 🧷create-asset/🖼️stores-a-new-d56283                  |
      | create-asset-realworld              | create-asset              | 🧷create-asset/🖼️stores-an-9f39e1                     |
      | create-asset-upsert                 | create-asset              | 🧷create-asset/♻️overwrites-an-a34b9d                 |
      | create-camera-calibration           | create-camera-calibration | 🔭create-camera-calibration/📷️adds-the-cam-c-82c8fb   |
      | create-camera-calibration-realworld | create-camera-calibration | 🔭create-camera-calibration/📷️adds-a-fourth-97e912    |
      | create-gcp                          | create-gcp                | 🧿create-gcp/📍️adds-gcp-tower-d71a54                  |
      | create-gcp-realworld                | create-gcp                | 🧿create-gcp/📍️adds-a-quay-7569de                     |
      | create-gcp-unobserved               | create-gcp                | 🧿create-gcp/🕳️adds-a-control-298de4                  |
      | create-rig-extrinsic                | create-rig-extrinsic      | ⛓️create-rig-extrinsic/🔗️adds-a-rig-2df5df           |
      | create-rig-extrinsic-realworld      | create-rig-extrinsic      | ⛓️create-rig-extrinsic/🔗️places-the-0d0b8d           |
      | create-stream                       | create-stream             | 🌱create-stream/🎥️adds-stream-c-458900                |
      | create-stream-realworld             | create-stream             | 🌱create-stream/🛰️adds-a-third-61fb5d                 |
      | create-stream-unbound               | create-stream             | 🌱create-stream/🎞️adds-an-unbound-2b2373              |
      | delete-asset                        | delete-asset              | 🗞️delete-asset/🧹️drops-the-spare-c6ffb6              |
      | delete-asset-realworld              | delete-asset              | 🗞️delete-asset/🗑️sweeps-the-503b27                   |
      | delete-camera-calibration           | delete-camera-calibration | 🚫delete-camera-calibration/🚫️removes-the-cam-f90b89  |
      | delete-camera-calibration-realworld | delete-camera-calibration | 🚫delete-camera-calibration/🚫️removes-the-40cba4      |
      | delete-gcp                          | delete-gcp                | 🚮delete-gcp/🚫️removes-gcp-209b7d                     |
      | delete-gcp-realworld                | delete-gcp                | 🚮delete-gcp/🚮removes-the-south-42cd9e                |
      | delete-gcp-unobserved               | delete-gcp                | 🚮delete-gcp/🕳️removes-an-8f3868                      |
      | delete-rig-extrinsic                | delete-rig-extrinsic      | ✂️delete-rig-extrinsic/✂️drops-the-cam-a-a1f8a2      |
      | delete-rig-extrinsic-first          | delete-rig-extrinsic      | ✂️delete-rig-extrinsic/⏮️unplaces-the-f5b35e         |
      | delete-rig-extrinsic-realworld      | delete-rig-extrinsic      | ✂️delete-rig-extrinsic/✂️unplaces-the-a39356         |
      | delete-stream                       | delete-stream             | 🪓delete-stream/⏮️removes-the-first-c0fc2a            |
      | delete-stream-realworld             | delete-stream             | 🪓delete-stream/🪓removes-the-spare-556d1d             |
      | remove-gcp-observation              | remove-gcp-observation    | 🚷remove-gcp-observation/🚫️removes-the-only-f82e64    |
      | remove-gcp-observation-first        | remove-gcp-observation    | 🚷remove-gcp-observation/⏮️drops-the-first-9ebf0b     |
      | remove-gcp-observation-realworld    | remove-gcp-observation    | 🚷remove-gcp-observation/🚷drops-the-middle-282fb7     |
      | remove-stream-frame                 | remove-stream-frame       | ➖remove-stream-frame/🚫️removes-the-last-304bdf       |
      | remove-stream-frame-first           | remove-stream-frame       | ➖remove-stream-frame/⏮️drops-the-first-d98a0f        |
      | remove-stream-frame-realworld       | remove-stream-frame       | ➖remove-stream-frame/✂️drops-the-middle-2d6d53       |
      | replace-dense                       | replace-dense             | ☁️replace-dense/☁️swaps-in-a-two-c688db              |
      | replace-dense-realworld             | replace-dense             | ☁️replace-dense/☁️swaps-in-a-denser-4174e1           |
      | replace-geo-products                | replace-geo-products      | 🗾replace-geo-products/🗺️adds-the-dtm-and-64d5bb      |
      | replace-geo-products-clears         | replace-geo-products      | 🗾replace-geo-products/🧹️clears-the-geo-f4886e        |
      | replace-geo-products-realworld      | replace-geo-products      | 🗾replace-geo-products/🗺️records-a-dtm-6e132a         |
      | replace-job                         | replace-job               | 🏗️replace-job/🎨️advances-the-job-c1e878              |
      | replace-job-realworld               | replace-job               | 🏗️replace-job/🎨️advances-the-555298                  |
      | replace-mesh-result                 | replace-mesh-result       | 🧱replace-mesh-result/🕸️swaps-in-an-f23e71            |
      | replace-mesh-result-realworld       | replace-mesh-result       | 🧱replace-mesh-result/🕸️swaps-the-c43d9c              |
      | replace-qc                          | replace-qc                | 🧾replace-qc/📋️records-a-qc-f5caf4                    |
      | replace-qc-clears                   | replace-qc                | 🧾replace-qc/🧹️clears-the-qc-1d2249                   |
      | replace-qc-realworld                | replace-qc                | 🧾replace-qc/✅️files-a-qc-report-64d222               |
      | replace-sparse                      | replace-sparse            | ⭐replace-sparse/✨️swaps-in-an-6d9ae4                 |
      | replace-sparse-realworld            | replace-sparse            | ⭐replace-sparse/✨️swaps-in-a-re-3cfa6d               |
      | replace-stream-source               | replace-stream-source     | 🔁replace-stream-source/🧹️clears-the-video-143f2b     |
      | replace-stream-source-attaches      | replace-stream-source     | 🔁replace-stream-source/📼️attaches-a-607df8           |
      | replace-stream-source-realworld     | replace-stream-source     | 🔁replace-stream-source/🎥️reingests-the-311c32        |
      | replace-tracks                      | replace-tracks            | 🚂replace-tracks/⏸️replaces-the-d40c68                |
      | replace-tracks-empty                | replace-tracks            | 🚂replace-tracks/🕳️clears-every-760061                |
      | replace-tracks-realworld            | replace-tracks            | 🚂replace-tracks/🏃️swaps-in-two-166265                |
      | replace-trajectory                  | replace-trajectory        | 🛣️replace-trajectory/🧹️clears-the-d2f81a             |
      | replace-trajectory-clears           | replace-trajectory        | 🛣️replace-trajectory/🕳️drops-the-6436a8              |
      | replace-trajectory-realworld        | replace-trajectory        | 🛣️replace-trajectory/🛣️swaps-in-a-three-49b17f       |
      | update-camera-calibration           | update-camera-calibration | 🛠️update-camera-calibration/🔍️refines-the-cam-0eaef0 |
      | update-camera-calibration-realworld | update-camera-calibration | 🛠️update-camera-calibration/🔍️refines-the-9fd25a     |
      | update-dense-params                 | update-dense-params       | 🌁update-dense-params/🔬️raises-the-dense-ddb263       |
      | update-dense-params-realworld       | update-dense-params       | 🌁update-dense-params/🧊️sharpens-the-25044c           |
      | update-feature-params               | update-feature-params     | 🌠update-feature-params/🔎️switches-the-423de9         |
      | update-feature-params-realworld     | update-feature-params     | 🌠update-feature-params/🌟️moves-the-3621f6            |
      | update-geo-params                   | update-geo-params         | 🌐update-geo-params/🌐️enables-georefere-18a68a        |
      | update-geo-params-realworld         | update-geo-params         | 🌐update-geo-params/🌐️halves-the-002a17               |
      | update-ingest-params                | update-ingest-params      | 🥣update-ingest-params/🔍️tightens-the-499c47          |
      | update-ingest-params-realworld      | update-ingest-params      | 🥣update-ingest-params/📥️widens-the-73f33e            |
      | update-match-params                 | update-match-params       | 🪢update-match-params/🌳️switches-the-652d03           |
      | update-match-params-realworld       | update-match-params       | 🪢update-match-params/🌳️switches-to-a-kd-d6fa4b       |
      | update-mesh-params                  | update-mesh-params        | 🕸️update-mesh-params/🔳️doubles-the-c245d5            |
      | update-mesh-params-realworld        | update-mesh-params        | 🕸️update-mesh-params/🔳️halves-the-voxel-21b53d       |
      | update-motion-params                | update-motion-params      | 🏎️update-motion-params/🏃️enables-motion-2444a3       |
      | update-motion-params-realworld      | update-motion-params      | 🏎️update-motion-params/🏃️triples-the-4bb69f          |
      | update-rig-extrinsic                | update-rig-extrinsic      | 🔩update-rig-extrinsic/📍️retunes-the-cam-4ca5a2       |
      | update-rig-extrinsic-realworld      | update-rig-extrinsic      | 🔩update-rig-extrinsic/📍️retunes-the-675f52           |
      | update-sfm-params                   | update-sfm-params         | 🧮update-sfm-params/🎯️switches-the-7f0371             |
      | update-sfm-params-realworld         | update-sfm-params         | 🧮update-sfm-params/🎯️tightens-the-850036             |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the document its own refusal never moved
    Given the committed refusal vector for the <id> kind
      """
      {
        "kind": "<kind>",
        "before": "shared://🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json",
        "mutation": "shared://🧬️mutations/<vector>/🦠️mutation/🔣️.json",
        "after": "shared://🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️.json",
        "code": "<code>"
      }
      """
    When <kind> and then every step of its own computed inverse are applied through undo_remodeling_mutation_json
    Then the document is the committed before-document again, member positions included, and the two implementations agree
    Examples:
      | id                                   | kind                      | vector                                                        | code                                   |
      | add-gcp-observation-missing          | add-gcp-observation       | 🔎add-gcp-observation/🚫️refuses-to-pick-3c0570         | mutation.target-missing                |
      | add-gcp-observation-noop             | add-gcp-observation       | 🔎add-gcp-observation/🔁️warns-that-this-dca661         | mutation.no-op                         |
      | add-stream-frame-kind                | add-stream-frame          | ➕add-stream-frame/🎬️refuses-a-frame-81beea            | mutation.invariant                     |
      | add-stream-frame-missing             | add-stream-frame          | ➕add-stream-frame/🚫️refuses-to-c93e98                 | mutation.target-missing                |
      | add-stream-frame-noop                | add-stream-frame          | ➕add-stream-frame/🔁️warns-that-the-1e8abe             | mutation.no-op                         |
      | change-stream-sync-missing           | change-stream-sync        | ⏱️change-stream-sync/🚫️refuses-to-8095d3              | mutation.target-missing                |
      | change-stream-sync-noop              | change-stream-sync        | ⏱️change-stream-sync/🔁️warns-that-the-a98c13          | mutation.no-op                         |
      | commit-reconstruction-asset          | commit-reconstruction     | 🏁commit-reconstruction/🖼️rejects-an-e9fa51            | mutation.invalid-reconstruction-asset  |
      | commit-reconstruction-mesh           | commit-reconstruction     | 🏁commit-reconstruction/🕸️rejects-an-5d3a60            | mutation.invalid-reconstruction-mesh   |
      | commit-reconstruction-sparse         | commit-reconstruction     | 🏁commit-reconstruction/⭐️rejects-an-2e5568            | mutation.invalid-reconstruction-sparse |
      | create-asset-staging-handle          | create-asset              | 🧷create-asset/🚫️refuses-an-asset-cb0d4b               | mutation.invalid-asset-payload         |
      | create-camera-calibration-duplicate  | create-camera-calibration | 🔭create-camera-calibration/🚫️refuses-a-camera-e92a02  | mutation.duplicate-id                  |
      | create-gcp-duplicate                 | create-gcp                | 🧿create-gcp/🚫️refuses-a-19c1ab                        | mutation.duplicate-id                  |
      | create-rig-extrinsic-duplicate       | create-rig-extrinsic      | ⛓️create-rig-extrinsic/🚫️refuses-a-second-95e04d      | mutation.duplicate-id                  |
      | create-rig-extrinsic-unknown-camera  | create-rig-extrinsic      | ⛓️create-rig-extrinsic/🚫️refuses-a-rig-cb71ba         | mutation.invariant                     |
      | create-stream-duplicate-id           | create-stream             | 🌱create-stream/🔂️rejects-a-6b58da                     | mutation.duplicate-id                  |
      | create-stream-unknown-camera         | create-stream             | 🌱create-stream/👻️rejects-a-stream-aac5c2              | mutation.invariant                     |
      | delete-asset-geo-product             | delete-asset              | 🗞️delete-asset/🗺️refuses-to-5c6f74                    | mutation.referenced                    |
      | delete-asset-missing                 | delete-asset              | 🗞️delete-asset/🚫️refuses-to-c4563a                    | mutation.target-missing                |
      | delete-asset-referenced-frames       | delete-asset              | 🗞️delete-asset/🖼️refuses-to-f9541f                    | mutation.referenced                    |
      | delete-camera-calibration-missing    | delete-camera-calibration | 🚫delete-camera-calibration/🚫️refuses-to-73655a        | mutation.target-missing                |
      | delete-camera-calibration-referenced | delete-camera-calibration | 🚫delete-camera-calibration/⛓️refuses-to-remove-3c8f32 | mutation.referenced                    |
      | delete-gcp-missing                   | delete-gcp                | 🚮delete-gcp/🚫️refuses-to-12366b                       | mutation.target-missing                |
      | delete-rig-extrinsic-missing         | delete-rig-extrinsic      | ✂️delete-rig-extrinsic/🚫️refuses-to-1805df            | mutation.target-missing                |
      | delete-stream-missing                | delete-stream             | 🪓delete-stream/🚫️refuses-to-3c20ff                    | mutation.target-missing                |
      | delete-stream-referenced             | delete-stream             | 🪓delete-stream/⛓️refuses-to-remove-422a37             | mutation.referenced                    |
      | remove-gcp-observation-out-of-range  | remove-gcp-observation    | 🚷remove-gcp-observation/🚫️refuses-an-109cf1           | mutation.target-missing                |
      | remove-stream-frame-out-of-range     | remove-stream-frame       | ➖remove-stream-frame/🚫️refuses-a-frame-e7c374         | mutation.target-missing                |
      | replace-dense-noop                   | replace-dense             | ☁️replace-dense/🔁️warns-that-the-675b6e               | mutation.no-op                         |
      | replace-geo-products-absent          | replace-geo-products      | 🗾replace-geo-products/🚫️refuses-to-clear-b8c54a       | mutation.target-missing                |
      | replace-job-noop                     | replace-job               | 🏗️replace-job/🔁️warns-that-the-bdf2e9                 | mutation.no-op                         |
      | replace-mesh-result-noop             | replace-mesh-result       | 🧱replace-mesh-result/🔁️warns-that-the-b39bab          | mutation.no-op                         |
      | replace-mesh-result-staged           | replace-mesh-result       | 🧱replace-mesh-result/🚫️refuses-a-48f3a6               | mutation.incomplete-mesh               |
      | replace-qc-absent                    | replace-qc                | 🧾replace-qc/🚫️refuses-to-clear-30cbb5                 | mutation.target-missing                |
      | replace-sparse-noop                  | replace-sparse            | ⭐replace-sparse/🔁️warns-that-the-56a3a9               | mutation.no-op                         |
      | replace-stream-source-missing        | replace-stream-source     | 🔁replace-stream-source/🚫️refuses-to-f7f40d            | mutation.target-missing                |
      | replace-tracks-noop                  | replace-tracks            | 🚂replace-tracks/🔁️warns-that-the-8dbf82               | mutation.no-op                         |
      | replace-trajectory-absent            | replace-trajectory        | 🛣️replace-trajectory/🚫️refuses-to-clear-524569        | mutation.target-missing                |
      | update-camera-calibration-missing    | update-camera-calibration | 🛠️update-camera-calibration/🚫️refuses-to-b60a39       | mutation.target-missing                |
      | update-camera-calibration-noop       | update-camera-calibration | 🛠️update-camera-calibration/🔁️warns-that-the-697b4f   | mutation.no-op                         |
      | update-dense-params-noop             | update-dense-params       | 🌁update-dense-params/🔁️warns-that-the-4e65c8          | mutation.no-op                         |
      | update-feature-params-invariant      | update-feature-params     | 🌠update-feature-params/🚫️refuses-a-d82e38             | mutation.invariant                     |
      | update-feature-params-noop           | update-feature-params     | 🌠update-feature-params/🔁️warns-that-the-b6b7dc        | mutation.no-op                         |
      | update-geo-params-invariant          | update-geo-params         | 🌐update-geo-params/🚫️refuses-a-zero-fa917f            | mutation.invariant                     |
      | update-geo-params-noop               | update-geo-params         | 🌐update-geo-params/🔁️warns-that-the-efc6e8            | mutation.no-op                         |
      | update-ingest-params-invariant       | update-ingest-params      | 🥣update-ingest-params/🚫️refuses-an-59752a             | mutation.invariant                     |
      | update-ingest-params-noop            | update-ingest-params      | 🥣update-ingest-params/🔁️warns-that-the-8eaad8         | mutation.no-op                         |
      | update-match-params-invariant        | update-match-params       | 🪢update-match-params/🚫️refuses-a-ratio-65dcb9         | mutation.invariant                     |
      | update-match-params-noop             | update-match-params       | 🪢update-match-params/🔁️warns-that-the-414aae          | mutation.no-op                         |
      | update-mesh-params-noop              | update-mesh-params        | 🕸️update-mesh-params/🔁️warns-that-the-887e9f          | mutation.no-op                         |
      | update-motion-params-noop            | update-motion-params      | 🏎️update-motion-params/🔁️warns-that-the-83ff67        | mutation.no-op                         |
      | update-rig-extrinsic-missing         | update-rig-extrinsic      | 🔩update-rig-extrinsic/🚫️refuses-to-2cfb53             | mutation.target-missing                |
      | update-rig-extrinsic-noop            | update-rig-extrinsic      | 🔩update-rig-extrinsic/🔁️warns-that-the-89422a         | mutation.no-op                         |
      | update-sfm-params-noop               | update-sfm-params         | 🧮update-sfm-params/🔁️warns-that-the-79a92a            | mutation.no-op                         |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the document its case-local refusal never moved
    Given the case-local refusal vector for the <id> kind
      """
      {
        "kind": "<kind>",
        "before": "shared://🏁️commit-reconstruction/⬅️before.json",
        "mutation": "shared://🏁️commit-reconstruction/🦠️mutation.json",
        "after": "shared://🏁️commit-reconstruction/➡️after.json",
        "code": "<code>"
      }
      """
    When <kind> and then every step of its own computed inverse are applied through undo_remodeling_mutation_json
    Then the document is the committed before-document again, member positions included, and the two implementations agree
    Examples:
      | id                    | kind                  | code                                   |
      | commit-reconstruction | commit-reconstruction | mutation.invalid-reconstruction-sparse |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse and reprint the real committed example without passing bytes through
    Given the real committed example this artifact ships
      """
      {
        "kind": "identity-round-trip",
        "carrier": "asset://🎬️demo/🗣️.dsl.semio"
      }
      """
    When it is parsed, printed back to DSL and parsed again through round_trip_remodeling_dsl
    Then both parses agree on one document, and the reprinted text reproduces the committed example byte for byte
