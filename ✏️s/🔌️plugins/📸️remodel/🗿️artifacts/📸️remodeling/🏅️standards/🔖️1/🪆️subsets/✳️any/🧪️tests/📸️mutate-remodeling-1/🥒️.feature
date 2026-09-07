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
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️replace-job/🧪️tests/🎨️advances-the-job-c1e878/📸️snapshot/⬅️before/🔣️.json`,
  shared://🏁️commit-reconstruction/🦠️mutation.json pairs that leaf's committed `job` payload with
  `⭐replace-sparse`'s committed `sparse` payload (a plain point buffer, deliberately NOT a replayable
  staging handle), and shared://🏁️commit-reconstruction/➡️after.json is the before-document unchanged,
  because the documented answer is `mutation.invalid-reconstruction-sparse` and a refused commit must
  leave the scene untouched. Note also that `commit-reconstruction`'s own inverse restores only `job`
  and the six result slots — never `assets` or `durable_artifacts` — so the inverse law holds for this
  refusal vector and would NOT hold for a commit that published new assets; that is a real weakness of
  the kind, recorded here rather than hidden by the vector that dodges it. Its OTHER three guards — invalid-reconstruction-asset, invalid-reconstruction-mesh and the sparse
  guard again on the real-world survey — need no staging state at all and ship as ordinary leaf
  vectors under `🧬️schema/🧬️mutations/🏁commit-reconstruction/🧪️tests/`. This vector's own two scenarios
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
        "before": "asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json",
        "mutation": "asset://🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️.json",
        "after": "asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️.json"
      }
      """
    When <kind> is applied through apply_remodeling_mutation_json
    Then the resulting document is the committed after-document, the mutation moved it, and the two implementations agree
    Examples:
      | id                                  | kind                      | vector                                                       |
      | add-gcp-observation                 | add-gcp-observation       | 🔎add-gcp-observation/🧪️tests/🔎️adds-the-first-05b1b5         |
      | add-gcp-observation-realworld       | add-gcp-observation       | 🔎add-gcp-observation/🧪️tests/🔎️picks-the-south-eb0c4d        |
      | add-stream-frame                    | add-stream-frame          | ➕add-stream-frame/🧪️tests/🎞️appends-a-third-8ac259           |
      | add-stream-frame-realworld          | add-stream-frame          | ➕add-stream-frame/🧪️tests/🎞️appends-an-0c2164                |
      | change-stream-sync                  | change-stream-sync        | ⏱️change-stream-sync/🧪️tests/⏱️shifts-stream-a-5b442c        |
      | change-stream-sync-realworld        | change-stream-sync        | ⏱️change-stream-sync/🧪️tests/⏱️retimes-the-50dd75            |
      | create-asset                        | create-asset              | 🧷create-asset/🧪️tests/🖼️stores-a-new-d56283                  |
      | create-asset-realworld              | create-asset              | 🧷create-asset/🧪️tests/🖼️stores-an-9f39e1                     |
      | create-asset-upsert                 | create-asset              | 🧷create-asset/🧪️tests/♻️overwrites-an-a34b9d                 |
      | create-camera-calibration           | create-camera-calibration | 🔭create-camera-calibration/🧪️tests/📷️adds-the-cam-c-82c8fb   |
      | create-camera-calibration-realworld | create-camera-calibration | 🔭create-camera-calibration/🧪️tests/📷️adds-a-fourth-97e912    |
      | create-gcp                          | create-gcp                | 🧿create-gcp/🧪️tests/📍️adds-gcp-tower-d71a54                  |
      | create-gcp-realworld                | create-gcp                | 🧿create-gcp/🧪️tests/📍️adds-a-quay-7569de                     |
      | create-gcp-unobserved               | create-gcp                | 🧿create-gcp/🧪️tests/🕳️adds-a-control-298de4                  |
      | create-rig-extrinsic                | create-rig-extrinsic      | ⛓️create-rig-extrinsic/🧪️tests/🔗️adds-a-rig-2df5df           |
      | create-rig-extrinsic-realworld      | create-rig-extrinsic      | ⛓️create-rig-extrinsic/🧪️tests/🔗️places-the-0d0b8d           |
      | create-stream                       | create-stream             | 🌱create-stream/🧪️tests/🎥️adds-stream-c-458900                |
      | create-stream-realworld             | create-stream             | 🌱create-stream/🧪️tests/🛰️adds-a-third-61fb5d                 |
      | create-stream-unbound               | create-stream             | 🌱create-stream/🧪️tests/🎞️adds-an-unbound-2b2373              |
      | delete-asset                        | delete-asset              | 🗞️delete-asset/🧪️tests/🧹️drops-the-spare-c6ffb6              |
      | delete-asset-realworld              | delete-asset              | 🗞️delete-asset/🧪️tests/🗑️sweeps-the-503b27                   |
      | delete-camera-calibration           | delete-camera-calibration | 🚫delete-camera-calibration/🧪️tests/🚫️removes-the-cam-f90b89  |
      | delete-camera-calibration-realworld | delete-camera-calibration | 🚫delete-camera-calibration/🧪️tests/🚫️removes-the-40cba4      |
      | delete-gcp                          | delete-gcp                | 🚮delete-gcp/🧪️tests/🚫️removes-gcp-209b7d                     |
      | delete-gcp-realworld                | delete-gcp                | 🚮delete-gcp/🧪️tests/🚮removes-the-south-42cd9e                |
      | delete-gcp-unobserved               | delete-gcp                | 🚮delete-gcp/🧪️tests/🕳️removes-an-8f3868                      |
      | delete-rig-extrinsic                | delete-rig-extrinsic      | ✂️delete-rig-extrinsic/🧪️tests/✂️drops-the-cam-a-a1f8a2      |
      | delete-rig-extrinsic-first          | delete-rig-extrinsic      | ✂️delete-rig-extrinsic/🧪️tests/⏮️unplaces-the-f5b35e         |
      | delete-rig-extrinsic-realworld      | delete-rig-extrinsic      | ✂️delete-rig-extrinsic/🧪️tests/✂️unplaces-the-a39356         |
      | delete-stream                       | delete-stream             | 🪓delete-stream/🧪️tests/⏮️removes-the-first-c0fc2a            |
      | delete-stream-realworld             | delete-stream             | 🪓delete-stream/🧪️tests/🪓removes-the-spare-556d1d             |
      | remove-gcp-observation              | remove-gcp-observation    | 🚷remove-gcp-observation/🧪️tests/🚫️removes-the-only-f82e64    |
      | remove-gcp-observation-first        | remove-gcp-observation    | 🚷remove-gcp-observation/🧪️tests/⏮️drops-the-first-9ebf0b     |
      | remove-gcp-observation-realworld    | remove-gcp-observation    | 🚷remove-gcp-observation/🧪️tests/🚷drops-the-middle-282fb7     |
      | remove-stream-frame                 | remove-stream-frame       | ➖remove-stream-frame/🧪️tests/🚫️removes-the-last-304bdf       |
      | remove-stream-frame-first           | remove-stream-frame       | ➖remove-stream-frame/🧪️tests/⏮️drops-the-first-d98a0f        |
      | remove-stream-frame-realworld       | remove-stream-frame       | ➖remove-stream-frame/🧪️tests/✂️drops-the-middle-2d6d53       |
      | replace-dense                       | replace-dense             | ☁️replace-dense/🧪️tests/☁️swaps-in-a-two-c688db              |
      | replace-dense-realworld             | replace-dense             | ☁️replace-dense/🧪️tests/☁️swaps-in-a-denser-4174e1           |
      | replace-geo-products                | replace-geo-products      | 🗾replace-geo-products/🧪️tests/🗺️adds-the-dtm-and-64d5bb      |
      | replace-geo-products-clears         | replace-geo-products      | 🗾replace-geo-products/🧪️tests/🧹️clears-the-geo-f4886e        |
      | replace-geo-products-realworld      | replace-geo-products      | 🗾replace-geo-products/🧪️tests/🗺️records-a-dtm-6e132a         |
      | replace-job                         | replace-job               | 🏗️replace-job/🧪️tests/🎨️advances-the-job-c1e878              |
      | replace-job-realworld               | replace-job               | 🏗️replace-job/🧪️tests/🎨️advances-the-555298                  |
      | replace-mesh-result                 | replace-mesh-result       | 🧱replace-mesh-result/🧪️tests/🕸️swaps-in-an-f23e71            |
      | replace-mesh-result-realworld       | replace-mesh-result       | 🧱replace-mesh-result/🧪️tests/🕸️swaps-the-c43d9c              |
      | replace-qc                          | replace-qc                | 🧾replace-qc/🧪️tests/📋️records-a-qc-f5caf4                    |
      | replace-qc-clears                   | replace-qc                | 🧾replace-qc/🧪️tests/🧹️clears-the-qc-1d2249                   |
      | replace-qc-realworld                | replace-qc                | 🧾replace-qc/🧪️tests/✅️files-a-qc-report-64d222               |
      | replace-sparse                      | replace-sparse            | ⭐replace-sparse/🧪️tests/✨️swaps-in-an-6d9ae4                 |
      | replace-sparse-realworld            | replace-sparse            | ⭐replace-sparse/🧪️tests/✨️swaps-in-a-re-3cfa6d               |
      | replace-stream-source               | replace-stream-source     | 🔁replace-stream-source/🧪️tests/🧹️clears-the-video-143f2b     |
      | replace-stream-source-attaches      | replace-stream-source     | 🔁replace-stream-source/🧪️tests/📼️attaches-a-607df8           |
      | replace-stream-source-realworld     | replace-stream-source     | 🔁replace-stream-source/🧪️tests/🎥️reingests-the-311c32        |
      | replace-tracks                      | replace-tracks            | 🚂replace-tracks/🧪️tests/⏸️replaces-the-d40c68                |
      | replace-tracks-empty                | replace-tracks            | 🚂replace-tracks/🧪️tests/🕳️clears-every-760061                |
      | replace-tracks-realworld            | replace-tracks            | 🚂replace-tracks/🧪️tests/🏃️swaps-in-two-166265                |
      | replace-trajectory                  | replace-trajectory        | 🛣️replace-trajectory/🧪️tests/🧹️clears-the-d2f81a             |
      | replace-trajectory-clears           | replace-trajectory        | 🛣️replace-trajectory/🧪️tests/🕳️drops-the-6436a8              |
      | replace-trajectory-realworld        | replace-trajectory        | 🛣️replace-trajectory/🧪️tests/🛣️swaps-in-a-three-49b17f       |
      | update-camera-calibration           | update-camera-calibration | 🛠️update-camera-calibration/🧪️tests/🔍️refines-the-cam-0eaef0 |
      | update-camera-calibration-realworld | update-camera-calibration | 🛠️update-camera-calibration/🧪️tests/🔍️refines-the-9fd25a     |
      | update-dense-params                 | update-dense-params       | 🌁update-dense-params/🧪️tests/🔬️raises-the-dense-ddb263       |
      | update-dense-params-realworld       | update-dense-params       | 🌁update-dense-params/🧪️tests/🧊️sharpens-the-25044c           |
      | update-feature-params               | update-feature-params     | 🌠update-feature-params/🧪️tests/🔎️switches-the-423de9         |
      | update-feature-params-realworld     | update-feature-params     | 🌠update-feature-params/🧪️tests/🌟️moves-the-3621f6            |
      | update-geo-params                   | update-geo-params         | 🌐update-geo-params/🧪️tests/🌐️enables-georefere-18a68a        |
      | update-geo-params-realworld         | update-geo-params         | 🌐update-geo-params/🧪️tests/🌐️halves-the-002a17               |
      | update-ingest-params                | update-ingest-params      | 🥣update-ingest-params/🧪️tests/🔍️tightens-the-499c47          |
      | update-ingest-params-realworld      | update-ingest-params      | 🥣update-ingest-params/🧪️tests/📥️widens-the-73f33e            |
      | update-match-params                 | update-match-params       | 🪢update-match-params/🧪️tests/🌳️switches-the-652d03           |
      | update-match-params-realworld       | update-match-params       | 🪢update-match-params/🧪️tests/🌳️switches-to-a-kd-d6fa4b       |
      | update-mesh-params                  | update-mesh-params        | 🕸️update-mesh-params/🧪️tests/🔳️doubles-the-c245d5            |
      | update-mesh-params-realworld        | update-mesh-params        | 🕸️update-mesh-params/🧪️tests/🔳️halves-the-voxel-21b53d       |
      | update-motion-params                | update-motion-params      | 🏎️update-motion-params/🧪️tests/🏃️enables-motion-2444a3       |
      | update-motion-params-realworld      | update-motion-params      | 🏎️update-motion-params/🧪️tests/🏃️triples-the-4bb69f          |
      | update-rig-extrinsic                | update-rig-extrinsic      | 🔩update-rig-extrinsic/🧪️tests/📍️retunes-the-cam-4ca5a2       |
      | update-rig-extrinsic-realworld      | update-rig-extrinsic      | 🔩update-rig-extrinsic/🧪️tests/📍️retunes-the-675f52           |
      | update-sfm-params                   | update-sfm-params         | 🧮update-sfm-params/🧪️tests/🎯️switches-the-7f0371             |
      | update-sfm-params-realworld         | update-sfm-params         | 🧮update-sfm-params/🧪️tests/🎯️tightens-the-850036             |

  @id-mutate
  @level-exhaustive
  @mode-error
  Scenario Outline: Applying <id> is refused exactly as its vector declares
    Given the committed refusal vector for the <id> kind
      """
      {
        "kind": "<kind>",
        "before": "asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json",
        "mutation": "asset://🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️.json",
        "after": "asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️.json",
        "code": "<code>"
      }
      """
    When <kind> is applied through apply_remodeling_mutation_json
    Then the document is left untouched and the declared <code> refusal was raised
    Examples:
      | id                                   | kind                      | vector                                                        | code                                   |
      | add-gcp-observation-missing          | add-gcp-observation       | 🔎add-gcp-observation/🧪️tests/🚫️refuses-to-pick-3c0570         | mutation.target-missing                |
      | add-stream-frame-kind                | add-stream-frame          | ➕add-stream-frame/🧪️tests/🎬️refuses-a-frame-81beea            | mutation.invariant                     |
      | add-stream-frame-missing             | add-stream-frame          | ➕add-stream-frame/🧪️tests/🚫️refuses-to-c93e98                 | mutation.target-missing                |
      | change-stream-sync-missing           | change-stream-sync        | ⏱️change-stream-sync/🧪️tests/🚫️refuses-to-8095d3              | mutation.target-missing                |
      | commit-reconstruction-asset          | commit-reconstruction     | 🏁commit-reconstruction/🧪️tests/🖼️rejects-an-e9fa51            | mutation.invalid-reconstruction-asset  |
      | commit-reconstruction-mesh           | commit-reconstruction     | 🏁commit-reconstruction/🧪️tests/🕸️rejects-an-5d3a60            | mutation.invalid-reconstruction-mesh   |
      | commit-reconstruction-sparse         | commit-reconstruction     | 🏁commit-reconstruction/🧪️tests/⭐️rejects-an-2e5568            | mutation.invalid-reconstruction-sparse |
      | create-asset-staging-handle          | create-asset              | 🧷create-asset/🧪️tests/🚫️refuses-an-asset-cb0d4b               | mutation.invalid-asset-payload         |
      | create-camera-calibration-duplicate  | create-camera-calibration | 🔭create-camera-calibration/🧪️tests/🚫️refuses-a-camera-e92a02  | mutation.duplicate-id                  |
      | create-gcp-duplicate                 | create-gcp                | 🧿create-gcp/🧪️tests/🚫️refuses-a-19c1ab                        | mutation.duplicate-id                  |
      | create-rig-extrinsic-duplicate       | create-rig-extrinsic      | ⛓️create-rig-extrinsic/🧪️tests/🚫️refuses-a-second-95e04d      | mutation.duplicate-id                  |
      | create-rig-extrinsic-unknown-camera  | create-rig-extrinsic      | ⛓️create-rig-extrinsic/🧪️tests/🚫️refuses-a-rig-cb71ba         | mutation.invariant                     |
      | create-stream-duplicate-id           | create-stream             | 🌱create-stream/🧪️tests/🔂️rejects-a-6b58da                     | mutation.duplicate-id                  |
      | create-stream-unknown-camera         | create-stream             | 🌱create-stream/🧪️tests/👻️rejects-a-stream-aac5c2              | mutation.invariant                     |
      | delete-asset-geo-product             | delete-asset              | 🗞️delete-asset/🧪️tests/🗺️refuses-to-5c6f74                    | mutation.referenced                    |
      | delete-asset-missing                 | delete-asset              | 🗞️delete-asset/🧪️tests/🚫️refuses-to-c4563a                    | mutation.target-missing                |
      | delete-asset-referenced-frames       | delete-asset              | 🗞️delete-asset/🧪️tests/🖼️refuses-to-f9541f                    | mutation.referenced                    |
      | delete-camera-calibration-missing    | delete-camera-calibration | 🚫delete-camera-calibration/🧪️tests/🚫️refuses-to-73655a        | mutation.target-missing                |
      | delete-camera-calibration-referenced | delete-camera-calibration | 🚫delete-camera-calibration/🧪️tests/⛓️refuses-to-remove-3c8f32 | mutation.referenced                    |
      | delete-gcp-missing                   | delete-gcp                | 🚮delete-gcp/🧪️tests/🚫️refuses-to-12366b                       | mutation.target-missing                |
      | delete-rig-extrinsic-missing         | delete-rig-extrinsic      | ✂️delete-rig-extrinsic/🧪️tests/🚫️refuses-to-1805df            | mutation.target-missing                |
      | delete-stream-missing                | delete-stream             | 🪓delete-stream/🧪️tests/🚫️refuses-to-3c20ff                    | mutation.target-missing                |
      | delete-stream-referenced             | delete-stream             | 🪓delete-stream/🧪️tests/⛓️refuses-to-remove-422a37             | mutation.referenced                    |
      | remove-gcp-observation-out-of-range  | remove-gcp-observation    | 🚷remove-gcp-observation/🧪️tests/🚫️refuses-an-109cf1           | mutation.target-missing                |
      | remove-stream-frame-out-of-range     | remove-stream-frame       | ➖remove-stream-frame/🧪️tests/🚫️refuses-a-frame-e7c374         | mutation.target-missing                |
      | replace-geo-products-absent          | replace-geo-products      | 🗾replace-geo-products/🧪️tests/🚫️refuses-to-clear-b8c54a       | mutation.target-missing                |
      | replace-mesh-result-staged           | replace-mesh-result       | 🧱replace-mesh-result/🧪️tests/🚫️refuses-a-48f3a6               | mutation.incomplete-mesh               |
      | replace-qc-absent                    | replace-qc                | 🧾replace-qc/🧪️tests/🚫️refuses-to-clear-30cbb5                 | mutation.target-missing                |
      | replace-stream-source-missing        | replace-stream-source     | 🔁replace-stream-source/🧪️tests/🚫️refuses-to-f7f40d            | mutation.target-missing                |
      | replace-trajectory-absent            | replace-trajectory        | 🛣️replace-trajectory/🧪️tests/🚫️refuses-to-clear-524569        | mutation.target-missing                |
      | update-camera-calibration-missing    | update-camera-calibration | 🛠️update-camera-calibration/🧪️tests/🚫️refuses-to-b60a39       | mutation.target-missing                |
      | update-feature-params-invariant      | update-feature-params     | 🌠update-feature-params/🧪️tests/🚫️refuses-a-d82e38             | mutation.invariant                     |
      | update-geo-params-invariant          | update-geo-params         | 🌐update-geo-params/🧪️tests/🚫️refuses-a-zero-fa917f            | mutation.invariant                     |
      | update-ingest-params-invariant       | update-ingest-params      | 🥣update-ingest-params/🧪️tests/🚫️refuses-an-59752a             | mutation.invariant                     |
      | update-match-params-invariant        | update-match-params       | 🪢update-match-params/🧪️tests/🚫️refuses-a-ratio-65dcb9         | mutation.invariant                     |
      | update-rig-extrinsic-missing         | update-rig-extrinsic      | 🔩update-rig-extrinsic/🧪️tests/🚫️refuses-to-2cfb53             | mutation.target-missing                |

  @id-mutate
  @level-exhaustive
  @mode-error
  Scenario Outline: Applying <id> is warned as a no-op and moves nothing
    Given the committed no-op vector for the <id> kind
      """
      {
        "kind": "<kind>",
        "before": "asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json",
        "mutation": "asset://🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️.json",
        "after": "asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️.json",
        "code": "<code>"
      }
      """
    When <kind> is applied through apply_remodeling_mutation_json
    Then the document is left untouched and the declared <code> refusal was raised
    Examples:
      | id                             | kind                      | vector                                                      | code           |
      | add-gcp-observation-noop       | add-gcp-observation       | 🔎add-gcp-observation/🧪️tests/🔁️warns-that-this-dca661       | mutation.no-op |
      | add-stream-frame-noop          | add-stream-frame          | ➕add-stream-frame/🧪️tests/🔁️warns-that-the-1e8abe           | mutation.no-op |
      | change-stream-sync-noop        | change-stream-sync        | ⏱️change-stream-sync/🧪️tests/🔁️warns-that-the-a98c13        | mutation.no-op |
      | replace-dense-noop             | replace-dense             | ☁️replace-dense/🧪️tests/🔁️warns-that-the-675b6e             | mutation.no-op |
      | replace-job-noop               | replace-job               | 🏗️replace-job/🧪️tests/🔁️warns-that-the-bdf2e9               | mutation.no-op |
      | replace-mesh-result-noop       | replace-mesh-result       | 🧱replace-mesh-result/🧪️tests/🔁️warns-that-the-b39bab        | mutation.no-op |
      | replace-sparse-noop            | replace-sparse            | ⭐replace-sparse/🧪️tests/🔁️warns-that-the-56a3a9             | mutation.no-op |
      | replace-tracks-noop            | replace-tracks            | 🚂replace-tracks/🧪️tests/🔁️warns-that-the-8dbf82             | mutation.no-op |
      | update-camera-calibration-noop | update-camera-calibration | 🛠️update-camera-calibration/🧪️tests/🔁️warns-that-the-697b4f | mutation.no-op |
      | update-dense-params-noop       | update-dense-params       | 🌁update-dense-params/🧪️tests/🔁️warns-that-the-4e65c8        | mutation.no-op |
      | update-feature-params-noop     | update-feature-params     | 🌠update-feature-params/🧪️tests/🔁️warns-that-the-b6b7dc      | mutation.no-op |
      | update-geo-params-noop         | update-geo-params         | 🌐update-geo-params/🧪️tests/🔁️warns-that-the-efc6e8          | mutation.no-op |
      | update-ingest-params-noop      | update-ingest-params      | 🥣update-ingest-params/🧪️tests/🔁️warns-that-the-8eaad8       | mutation.no-op |
      | update-match-params-noop       | update-match-params       | 🪢update-match-params/🧪️tests/🔁️warns-that-the-414aae        | mutation.no-op |
      | update-mesh-params-noop        | update-mesh-params        | 🕸️update-mesh-params/🧪️tests/🔁️warns-that-the-887e9f        | mutation.no-op |
      | update-motion-params-noop      | update-motion-params      | 🏎️update-motion-params/🧪️tests/🔁️warns-that-the-83ff67      | mutation.no-op |
      | update-rig-extrinsic-noop      | update-rig-extrinsic      | 🔩update-rig-extrinsic/🧪️tests/🔁️warns-that-the-89422a       | mutation.no-op |
      | update-sfm-params-noop         | update-sfm-params         | 🧮update-sfm-params/🧪️tests/🔁️warns-that-the-79a92a          | mutation.no-op |

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
        "before": "asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json",
        "mutation": "asset://🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️.json",
        "after": "asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️.json"
      }
      """
    When <kind> and then every step of its own computed inverse are applied through undo_remodeling_mutation_json
    Then the document is the committed before-document again, member positions included, and the two implementations agree
    Examples:
      | id                                  | kind                      | vector                                                       |
      | add-gcp-observation                 | add-gcp-observation       | 🔎add-gcp-observation/🧪️tests/🔎️adds-the-first-05b1b5         |
      | add-gcp-observation-realworld       | add-gcp-observation       | 🔎add-gcp-observation/🧪️tests/🔎️picks-the-south-eb0c4d        |
      | add-stream-frame                    | add-stream-frame          | ➕add-stream-frame/🧪️tests/🎞️appends-a-third-8ac259           |
      | add-stream-frame-realworld          | add-stream-frame          | ➕add-stream-frame/🧪️tests/🎞️appends-an-0c2164                |
      | change-stream-sync                  | change-stream-sync        | ⏱️change-stream-sync/🧪️tests/⏱️shifts-stream-a-5b442c        |
      | change-stream-sync-realworld        | change-stream-sync        | ⏱️change-stream-sync/🧪️tests/⏱️retimes-the-50dd75            |
      | create-asset                        | create-asset              | 🧷create-asset/🧪️tests/🖼️stores-a-new-d56283                  |
      | create-asset-realworld              | create-asset              | 🧷create-asset/🧪️tests/🖼️stores-an-9f39e1                     |
      | create-asset-upsert                 | create-asset              | 🧷create-asset/🧪️tests/♻️overwrites-an-a34b9d                 |
      | create-camera-calibration           | create-camera-calibration | 🔭create-camera-calibration/🧪️tests/📷️adds-the-cam-c-82c8fb   |
      | create-camera-calibration-realworld | create-camera-calibration | 🔭create-camera-calibration/🧪️tests/📷️adds-a-fourth-97e912    |
      | create-gcp                          | create-gcp                | 🧿create-gcp/🧪️tests/📍️adds-gcp-tower-d71a54                  |
      | create-gcp-realworld                | create-gcp                | 🧿create-gcp/🧪️tests/📍️adds-a-quay-7569de                     |
      | create-gcp-unobserved               | create-gcp                | 🧿create-gcp/🧪️tests/🕳️adds-a-control-298de4                  |
      | create-rig-extrinsic                | create-rig-extrinsic      | ⛓️create-rig-extrinsic/🧪️tests/🔗️adds-a-rig-2df5df           |
      | create-rig-extrinsic-realworld      | create-rig-extrinsic      | ⛓️create-rig-extrinsic/🧪️tests/🔗️places-the-0d0b8d           |
      | create-stream                       | create-stream             | 🌱create-stream/🧪️tests/🎥️adds-stream-c-458900                |
      | create-stream-realworld             | create-stream             | 🌱create-stream/🧪️tests/🛰️adds-a-third-61fb5d                 |
      | create-stream-unbound               | create-stream             | 🌱create-stream/🧪️tests/🎞️adds-an-unbound-2b2373              |
      | delete-asset                        | delete-asset              | 🗞️delete-asset/🧪️tests/🧹️drops-the-spare-c6ffb6              |
      | delete-asset-realworld              | delete-asset              | 🗞️delete-asset/🧪️tests/🗑️sweeps-the-503b27                   |
      | delete-camera-calibration           | delete-camera-calibration | 🚫delete-camera-calibration/🧪️tests/🚫️removes-the-cam-f90b89  |
      | delete-camera-calibration-realworld | delete-camera-calibration | 🚫delete-camera-calibration/🧪️tests/🚫️removes-the-40cba4      |
      | delete-gcp                          | delete-gcp                | 🚮delete-gcp/🧪️tests/🚫️removes-gcp-209b7d                     |
      | delete-gcp-realworld                | delete-gcp                | 🚮delete-gcp/🧪️tests/🚮removes-the-south-42cd9e                |
      | delete-gcp-unobserved               | delete-gcp                | 🚮delete-gcp/🧪️tests/🕳️removes-an-8f3868                      |
      | delete-rig-extrinsic                | delete-rig-extrinsic      | ✂️delete-rig-extrinsic/🧪️tests/✂️drops-the-cam-a-a1f8a2      |
      | delete-rig-extrinsic-first          | delete-rig-extrinsic      | ✂️delete-rig-extrinsic/🧪️tests/⏮️unplaces-the-f5b35e         |
      | delete-rig-extrinsic-realworld      | delete-rig-extrinsic      | ✂️delete-rig-extrinsic/🧪️tests/✂️unplaces-the-a39356         |
      | delete-stream                       | delete-stream             | 🪓delete-stream/🧪️tests/⏮️removes-the-first-c0fc2a            |
      | delete-stream-realworld             | delete-stream             | 🪓delete-stream/🧪️tests/🪓removes-the-spare-556d1d             |
      | remove-gcp-observation              | remove-gcp-observation    | 🚷remove-gcp-observation/🧪️tests/🚫️removes-the-only-f82e64    |
      | remove-gcp-observation-first        | remove-gcp-observation    | 🚷remove-gcp-observation/🧪️tests/⏮️drops-the-first-9ebf0b     |
      | remove-gcp-observation-realworld    | remove-gcp-observation    | 🚷remove-gcp-observation/🧪️tests/🚷drops-the-middle-282fb7     |
      | remove-stream-frame                 | remove-stream-frame       | ➖remove-stream-frame/🧪️tests/🚫️removes-the-last-304bdf       |
      | remove-stream-frame-first           | remove-stream-frame       | ➖remove-stream-frame/🧪️tests/⏮️drops-the-first-d98a0f        |
      | remove-stream-frame-realworld       | remove-stream-frame       | ➖remove-stream-frame/🧪️tests/✂️drops-the-middle-2d6d53       |
      | replace-dense                       | replace-dense             | ☁️replace-dense/🧪️tests/☁️swaps-in-a-two-c688db              |
      | replace-dense-realworld             | replace-dense             | ☁️replace-dense/🧪️tests/☁️swaps-in-a-denser-4174e1           |
      | replace-geo-products                | replace-geo-products      | 🗾replace-geo-products/🧪️tests/🗺️adds-the-dtm-and-64d5bb      |
      | replace-geo-products-clears         | replace-geo-products      | 🗾replace-geo-products/🧪️tests/🧹️clears-the-geo-f4886e        |
      | replace-geo-products-realworld      | replace-geo-products      | 🗾replace-geo-products/🧪️tests/🗺️records-a-dtm-6e132a         |
      | replace-job                         | replace-job               | 🏗️replace-job/🧪️tests/🎨️advances-the-job-c1e878              |
      | replace-job-realworld               | replace-job               | 🏗️replace-job/🧪️tests/🎨️advances-the-555298                  |
      | replace-mesh-result                 | replace-mesh-result       | 🧱replace-mesh-result/🧪️tests/🕸️swaps-in-an-f23e71            |
      | replace-mesh-result-realworld       | replace-mesh-result       | 🧱replace-mesh-result/🧪️tests/🕸️swaps-the-c43d9c              |
      | replace-qc                          | replace-qc                | 🧾replace-qc/🧪️tests/📋️records-a-qc-f5caf4                    |
      | replace-qc-clears                   | replace-qc                | 🧾replace-qc/🧪️tests/🧹️clears-the-qc-1d2249                   |
      | replace-qc-realworld                | replace-qc                | 🧾replace-qc/🧪️tests/✅️files-a-qc-report-64d222               |
      | replace-sparse                      | replace-sparse            | ⭐replace-sparse/🧪️tests/✨️swaps-in-an-6d9ae4                 |
      | replace-sparse-realworld            | replace-sparse            | ⭐replace-sparse/🧪️tests/✨️swaps-in-a-re-3cfa6d               |
      | replace-stream-source               | replace-stream-source     | 🔁replace-stream-source/🧪️tests/🧹️clears-the-video-143f2b     |
      | replace-stream-source-attaches      | replace-stream-source     | 🔁replace-stream-source/🧪️tests/📼️attaches-a-607df8           |
      | replace-stream-source-realworld     | replace-stream-source     | 🔁replace-stream-source/🧪️tests/🎥️reingests-the-311c32        |
      | replace-tracks                      | replace-tracks            | 🚂replace-tracks/🧪️tests/⏸️replaces-the-d40c68                |
      | replace-tracks-empty                | replace-tracks            | 🚂replace-tracks/🧪️tests/🕳️clears-every-760061                |
      | replace-tracks-realworld            | replace-tracks            | 🚂replace-tracks/🧪️tests/🏃️swaps-in-two-166265                |
      | replace-trajectory                  | replace-trajectory        | 🛣️replace-trajectory/🧪️tests/🧹️clears-the-d2f81a             |
      | replace-trajectory-clears           | replace-trajectory        | 🛣️replace-trajectory/🧪️tests/🕳️drops-the-6436a8              |
      | replace-trajectory-realworld        | replace-trajectory        | 🛣️replace-trajectory/🧪️tests/🛣️swaps-in-a-three-49b17f       |
      | update-camera-calibration           | update-camera-calibration | 🛠️update-camera-calibration/🧪️tests/🔍️refines-the-cam-0eaef0 |
      | update-camera-calibration-realworld | update-camera-calibration | 🛠️update-camera-calibration/🧪️tests/🔍️refines-the-9fd25a     |
      | update-dense-params                 | update-dense-params       | 🌁update-dense-params/🧪️tests/🔬️raises-the-dense-ddb263       |
      | update-dense-params-realworld       | update-dense-params       | 🌁update-dense-params/🧪️tests/🧊️sharpens-the-25044c           |
      | update-feature-params               | update-feature-params     | 🌠update-feature-params/🧪️tests/🔎️switches-the-423de9         |
      | update-feature-params-realworld     | update-feature-params     | 🌠update-feature-params/🧪️tests/🌟️moves-the-3621f6            |
      | update-geo-params                   | update-geo-params         | 🌐update-geo-params/🧪️tests/🌐️enables-georefere-18a68a        |
      | update-geo-params-realworld         | update-geo-params         | 🌐update-geo-params/🧪️tests/🌐️halves-the-002a17               |
      | update-ingest-params                | update-ingest-params      | 🥣update-ingest-params/🧪️tests/🔍️tightens-the-499c47          |
      | update-ingest-params-realworld      | update-ingest-params      | 🥣update-ingest-params/🧪️tests/📥️widens-the-73f33e            |
      | update-match-params                 | update-match-params       | 🪢update-match-params/🧪️tests/🌳️switches-the-652d03           |
      | update-match-params-realworld       | update-match-params       | 🪢update-match-params/🧪️tests/🌳️switches-to-a-kd-d6fa4b       |
      | update-mesh-params                  | update-mesh-params        | 🕸️update-mesh-params/🧪️tests/🔳️doubles-the-c245d5            |
      | update-mesh-params-realworld        | update-mesh-params        | 🕸️update-mesh-params/🧪️tests/🔳️halves-the-voxel-21b53d       |
      | update-motion-params                | update-motion-params      | 🏎️update-motion-params/🧪️tests/🏃️enables-motion-2444a3       |
      | update-motion-params-realworld      | update-motion-params      | 🏎️update-motion-params/🧪️tests/🏃️triples-the-4bb69f          |
      | update-rig-extrinsic                | update-rig-extrinsic      | 🔩update-rig-extrinsic/🧪️tests/📍️retunes-the-cam-4ca5a2       |
      | update-rig-extrinsic-realworld      | update-rig-extrinsic      | 🔩update-rig-extrinsic/🧪️tests/📍️retunes-the-675f52           |
      | update-sfm-params                   | update-sfm-params         | 🧮update-sfm-params/🧪️tests/🎯️switches-the-7f0371             |
      | update-sfm-params-realworld         | update-sfm-params         | 🧮update-sfm-params/🧪️tests/🎯️tightens-the-850036             |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the document its own refusal never moved
    Given the committed refusal vector for the <id> kind
      """
      {
        "kind": "<kind>",
        "before": "asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json",
        "mutation": "asset://🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️.json",
        "after": "asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️.json",
        "code": "<code>"
      }
      """
    When <kind> and then every step of its own computed inverse are applied through undo_remodeling_mutation_json
    Then the document is the committed before-document again, member positions included, and the two implementations agree
    Examples:
      | id                                   | kind                      | vector                                                        | code                                   |
      | add-gcp-observation-missing          | add-gcp-observation       | 🔎add-gcp-observation/🧪️tests/🚫️refuses-to-pick-3c0570         | mutation.target-missing                |
      | add-gcp-observation-noop             | add-gcp-observation       | 🔎add-gcp-observation/🧪️tests/🔁️warns-that-this-dca661         | mutation.no-op                         |
      | add-stream-frame-kind                | add-stream-frame          | ➕add-stream-frame/🧪️tests/🎬️refuses-a-frame-81beea            | mutation.invariant                     |
      | add-stream-frame-missing             | add-stream-frame          | ➕add-stream-frame/🧪️tests/🚫️refuses-to-c93e98                 | mutation.target-missing                |
      | add-stream-frame-noop                | add-stream-frame          | ➕add-stream-frame/🧪️tests/🔁️warns-that-the-1e8abe             | mutation.no-op                         |
      | change-stream-sync-missing           | change-stream-sync        | ⏱️change-stream-sync/🧪️tests/🚫️refuses-to-8095d3              | mutation.target-missing                |
      | change-stream-sync-noop              | change-stream-sync        | ⏱️change-stream-sync/🧪️tests/🔁️warns-that-the-a98c13          | mutation.no-op                         |
      | commit-reconstruction-asset          | commit-reconstruction     | 🏁commit-reconstruction/🧪️tests/🖼️rejects-an-e9fa51            | mutation.invalid-reconstruction-asset  |
      | commit-reconstruction-mesh           | commit-reconstruction     | 🏁commit-reconstruction/🧪️tests/🕸️rejects-an-5d3a60            | mutation.invalid-reconstruction-mesh   |
      | commit-reconstruction-sparse         | commit-reconstruction     | 🏁commit-reconstruction/🧪️tests/⭐️rejects-an-2e5568            | mutation.invalid-reconstruction-sparse |
      | create-asset-staging-handle          | create-asset              | 🧷create-asset/🧪️tests/🚫️refuses-an-asset-cb0d4b               | mutation.invalid-asset-payload         |
      | create-camera-calibration-duplicate  | create-camera-calibration | 🔭create-camera-calibration/🧪️tests/🚫️refuses-a-camera-e92a02  | mutation.duplicate-id                  |
      | create-gcp-duplicate                 | create-gcp                | 🧿create-gcp/🧪️tests/🚫️refuses-a-19c1ab                        | mutation.duplicate-id                  |
      | create-rig-extrinsic-duplicate       | create-rig-extrinsic      | ⛓️create-rig-extrinsic/🧪️tests/🚫️refuses-a-second-95e04d      | mutation.duplicate-id                  |
      | create-rig-extrinsic-unknown-camera  | create-rig-extrinsic      | ⛓️create-rig-extrinsic/🧪️tests/🚫️refuses-a-rig-cb71ba         | mutation.invariant                     |
      | create-stream-duplicate-id           | create-stream             | 🌱create-stream/🧪️tests/🔂️rejects-a-6b58da                     | mutation.duplicate-id                  |
      | create-stream-unknown-camera         | create-stream             | 🌱create-stream/🧪️tests/👻️rejects-a-stream-aac5c2              | mutation.invariant                     |
      | delete-asset-geo-product             | delete-asset              | 🗞️delete-asset/🧪️tests/🗺️refuses-to-5c6f74                    | mutation.referenced                    |
      | delete-asset-missing                 | delete-asset              | 🗞️delete-asset/🧪️tests/🚫️refuses-to-c4563a                    | mutation.target-missing                |
      | delete-asset-referenced-frames       | delete-asset              | 🗞️delete-asset/🧪️tests/🖼️refuses-to-f9541f                    | mutation.referenced                    |
      | delete-camera-calibration-missing    | delete-camera-calibration | 🚫delete-camera-calibration/🧪️tests/🚫️refuses-to-73655a        | mutation.target-missing                |
      | delete-camera-calibration-referenced | delete-camera-calibration | 🚫delete-camera-calibration/🧪️tests/⛓️refuses-to-remove-3c8f32 | mutation.referenced                    |
      | delete-gcp-missing                   | delete-gcp                | 🚮delete-gcp/🧪️tests/🚫️refuses-to-12366b                       | mutation.target-missing                |
      | delete-rig-extrinsic-missing         | delete-rig-extrinsic      | ✂️delete-rig-extrinsic/🧪️tests/🚫️refuses-to-1805df            | mutation.target-missing                |
      | delete-stream-missing                | delete-stream             | 🪓delete-stream/🧪️tests/🚫️refuses-to-3c20ff                    | mutation.target-missing                |
      | delete-stream-referenced             | delete-stream             | 🪓delete-stream/🧪️tests/⛓️refuses-to-remove-422a37             | mutation.referenced                    |
      | remove-gcp-observation-out-of-range  | remove-gcp-observation    | 🚷remove-gcp-observation/🧪️tests/🚫️refuses-an-109cf1           | mutation.target-missing                |
      | remove-stream-frame-out-of-range     | remove-stream-frame       | ➖remove-stream-frame/🧪️tests/🚫️refuses-a-frame-e7c374         | mutation.target-missing                |
      | replace-dense-noop                   | replace-dense             | ☁️replace-dense/🧪️tests/🔁️warns-that-the-675b6e               | mutation.no-op                         |
      | replace-geo-products-absent          | replace-geo-products      | 🗾replace-geo-products/🧪️tests/🚫️refuses-to-clear-b8c54a       | mutation.target-missing                |
      | replace-job-noop                     | replace-job               | 🏗️replace-job/🧪️tests/🔁️warns-that-the-bdf2e9                 | mutation.no-op                         |
      | replace-mesh-result-noop             | replace-mesh-result       | 🧱replace-mesh-result/🧪️tests/🔁️warns-that-the-b39bab          | mutation.no-op                         |
      | replace-mesh-result-staged           | replace-mesh-result       | 🧱replace-mesh-result/🧪️tests/🚫️refuses-a-48f3a6               | mutation.incomplete-mesh               |
      | replace-qc-absent                    | replace-qc                | 🧾replace-qc/🧪️tests/🚫️refuses-to-clear-30cbb5                 | mutation.target-missing                |
      | replace-sparse-noop                  | replace-sparse            | ⭐replace-sparse/🧪️tests/🔁️warns-that-the-56a3a9               | mutation.no-op                         |
      | replace-stream-source-missing        | replace-stream-source     | 🔁replace-stream-source/🧪️tests/🚫️refuses-to-f7f40d            | mutation.target-missing                |
      | replace-tracks-noop                  | replace-tracks            | 🚂replace-tracks/🧪️tests/🔁️warns-that-the-8dbf82               | mutation.no-op                         |
      | replace-trajectory-absent            | replace-trajectory        | 🛣️replace-trajectory/🧪️tests/🚫️refuses-to-clear-524569        | mutation.target-missing                |
      | update-camera-calibration-missing    | update-camera-calibration | 🛠️update-camera-calibration/🧪️tests/🚫️refuses-to-b60a39       | mutation.target-missing                |
      | update-camera-calibration-noop       | update-camera-calibration | 🛠️update-camera-calibration/🧪️tests/🔁️warns-that-the-697b4f   | mutation.no-op                         |
      | update-dense-params-noop             | update-dense-params       | 🌁update-dense-params/🧪️tests/🔁️warns-that-the-4e65c8          | mutation.no-op                         |
      | update-feature-params-invariant      | update-feature-params     | 🌠update-feature-params/🧪️tests/🚫️refuses-a-d82e38             | mutation.invariant                     |
      | update-feature-params-noop           | update-feature-params     | 🌠update-feature-params/🧪️tests/🔁️warns-that-the-b6b7dc        | mutation.no-op                         |
      | update-geo-params-invariant          | update-geo-params         | 🌐update-geo-params/🧪️tests/🚫️refuses-a-zero-fa917f            | mutation.invariant                     |
      | update-geo-params-noop               | update-geo-params         | 🌐update-geo-params/🧪️tests/🔁️warns-that-the-efc6e8            | mutation.no-op                         |
      | update-ingest-params-invariant       | update-ingest-params      | 🥣update-ingest-params/🧪️tests/🚫️refuses-an-59752a             | mutation.invariant                     |
      | update-ingest-params-noop            | update-ingest-params      | 🥣update-ingest-params/🧪️tests/🔁️warns-that-the-8eaad8         | mutation.no-op                         |
      | update-match-params-invariant        | update-match-params       | 🪢update-match-params/🧪️tests/🚫️refuses-a-ratio-65dcb9         | mutation.invariant                     |
      | update-match-params-noop             | update-match-params       | 🪢update-match-params/🧪️tests/🔁️warns-that-the-414aae          | mutation.no-op                         |
      | update-mesh-params-noop              | update-mesh-params        | 🕸️update-mesh-params/🧪️tests/🔁️warns-that-the-887e9f          | mutation.no-op                         |
      | update-motion-params-noop            | update-motion-params      | 🏎️update-motion-params/🧪️tests/🔁️warns-that-the-83ff67        | mutation.no-op                         |
      | update-rig-extrinsic-missing         | update-rig-extrinsic      | 🔩update-rig-extrinsic/🧪️tests/🚫️refuses-to-2cfb53             | mutation.target-missing                |
      | update-rig-extrinsic-noop            | update-rig-extrinsic      | 🔩update-rig-extrinsic/🧪️tests/🔁️warns-that-the-89422a         | mutation.no-op                         |
      | update-sfm-params-noop               | update-sfm-params         | 🧮update-sfm-params/🧪️tests/🔁️warns-that-the-79a92a            | mutation.no-op                         |

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
        "carrier": "asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio"
      }
      """
    When it is parsed, printed back to DSL and parsed again through round_trip_remodeling_dsl
    Then both parses agree on one document, and the reprinted text reproduces the committed example byte for byte
