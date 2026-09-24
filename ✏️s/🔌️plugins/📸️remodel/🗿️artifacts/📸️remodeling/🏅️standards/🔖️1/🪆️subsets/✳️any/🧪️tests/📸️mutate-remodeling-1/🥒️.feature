@capability-remodeling-1-mutate
@oracle-remodeling-1-python-independent
@comparison-ordered-json-v1
@mutations-remodeling-1-any
Feature: Apply every typed remodeling-scene mutation to its committed specification vector and against an independent Python implementation
  `s.remodel.remodeling` is a semio-NATIVE artifact, and the document is a reconstruction
  PROJECT — streams, calibrations, ground control points, the eight parameter blocks a pipeline runs
  under, the durable content leaves a run publishes, and the engine-owned results — not a point cloud
  or a mesh file. A reader of COLMAP, LAS or
  PLY output would therefore be judging a different artifact, and nothing reads `.dsl.semio`. That is
  recorded as the `remodeling-mutation-semantics` no-oracle decision in
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracles/🔣️.json`. That decision is narrowed to an empty
  `capabilities` list rather than deleted (it already was, by a prior shard of this same ticket),
  because its own investigation remains the honest record of what was checked; a dated note is
  appended recording that the `asset://` blocker it named is now resolved.

  🐍️ `🐍️component.py` beside this file is the second IMPLEMENTATION that decision named as the
  remaining debt: all 36 of this vocabulary's kinds, `commit-reconstruction` included as an ordinary
  kind with its refusals, its no-op, its forward application and its inverse — written in Python from this subset's own
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
  they make every one of the 36 kinds' inverses a single step of this same vocabulary that restores
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

  📦️ DURABLE CONTENT is ordinary document state. A reconstruction run publishes every sparse cloud,
  mesh and raster it produced as bounded 4 KiB base64 leaves into `durableArtifacts` through
  `append-content`, whose exact inverse is `remove-content`; leaves are contiguous, immutable once
  stored, kept to one kind and presentation per entry and bounded by that kind's envelope, and an
  append that only repeats stored leaves is warned as a no-op. `commit-reconstruction` is then a PURE
  document mutation: it replaces the sparse, trajectory, geo and QC results (and the mesh when one is
  given) and binds or unbinds assets, naming content by id only — a `remodeling-content:<id>|<leaves>`
  sparse buffer, a `remodeling-mesh-content:<leaves>` mesh child, or an asset binding's `contentId`.
  Each named handle must address content the BEFORE-document already stores complete, which is what
  its three refusal vectors under `🧬️schema/🧬️mutations/🏁commit-reconstruction/` pin; a commit that
  changes neither a result nor a binding is a no-op, and its inverse is one commit carrying the BASE
  result lanes and the BASE binding of every asset id it named. Both halves decide all of that from the
  committed `(before, mutation, after)` triple alone.

  📄️ `commit-reconstruction` additionally keeps one shared vector in this owner's fixtures:
  shared://🏁️commit-reconstruction/⬅️before.json is the two-stream unit scene every toy vector
  starts from, shared://🏁️commit-reconstruction/🦠️mutation.json commits a sparse cloud naming content
  that scene never published, and shared://🏁️commit-reconstruction/➡️after.json is the before-document unchanged,
  because the documented answer is `mutation.invalid-reconstruction-sparse` and a refused commit must
  leave the scene untouched. Its two scenarios address that vector by the same doc-string mechanism as
  every other row — three `shared://` URIs in place of the triple a `<vector>` builds — so both halves
  resolve it at run time and both halves answer for it, the reference deriving the refusal from the
  payload and the base document rather than adopting production's verdict.

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
    Given the committed specification vector <id>
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
      | id                       | kind                      | vector                                               |
      | adds-the-first-05b1b5    | add-gcp-observation       | 🔎add-gcp-observation/🔎️adds-the-first-05b1b5         |
      | picks-the-south-eb0c4d   | add-gcp-observation       | 🔎add-gcp-observation/🔎️picks-the-south-eb0c4d        |
      | appends-a-third-8ac259   | add-stream-frame          | ➕add-stream-frame/🎞️appends-a-third-8ac259           |
      | appends-an-0c2164        | add-stream-frame          | ➕add-stream-frame/🎞️appends-an-0c2164                |
      | appends-sparse-leaves    | append-content            | 📦append-content/🧱️appends-sparse-leaves              |
      | shifts-stream-a-5b442c   | change-stream-sync        | ⏱️change-stream-sync/⏱️shifts-stream-a-5b442c        |
      | retimes-the-50dd75       | change-stream-sync        | ⏱️change-stream-sync/⏱️retimes-the-50dd75            |
      | stores-a-new-d56283      | create-asset              | 🧷create-asset/🖼️stores-a-new-d56283                  |
      | stores-an-9f39e1         | create-asset              | 🧷create-asset/🖼️stores-an-9f39e1                     |
      | overwrites-an-a34b9d     | create-asset              | 🧷create-asset/♻️overwrites-an-a34b9d                 |
      | adds-the-cam-c-82c8fb    | create-camera-calibration | 🔭create-camera-calibration/📷️adds-the-cam-c-82c8fb   |
      | adds-a-fourth-97e912     | create-camera-calibration | 🔭create-camera-calibration/📷️adds-a-fourth-97e912    |
      | adds-gcp-tower-d71a54    | create-gcp                | 🧿create-gcp/📍️adds-gcp-tower-d71a54                  |
      | adds-a-quay-7569de       | create-gcp                | 🧿create-gcp/📍️adds-a-quay-7569de                     |
      | adds-a-control-298de4    | create-gcp                | 🧿create-gcp/🕳️adds-a-control-298de4                  |
      | adds-a-rig-2df5df        | create-rig-extrinsic      | ⛓️create-rig-extrinsic/🔗️adds-a-rig-2df5df           |
      | places-the-0d0b8d        | create-rig-extrinsic      | ⛓️create-rig-extrinsic/🔗️places-the-0d0b8d           |
      | adds-stream-c-458900     | create-stream             | 🌱create-stream/🎥️adds-stream-c-458900                |
      | adds-a-third-61fb5d      | create-stream             | 🌱create-stream/🛰️adds-a-third-61fb5d                 |
      | adds-an-unbound-2b2373   | create-stream             | 🌱create-stream/🎞️adds-an-unbound-2b2373              |
      | drops-the-spare-c6ffb6   | delete-asset              | 🗞️delete-asset/🧹️drops-the-spare-c6ffb6              |
      | sweeps-the-503b27        | delete-asset              | 🗞️delete-asset/🗑️sweeps-the-503b27                   |
      | removes-the-cam-f90b89   | delete-camera-calibration | 🚫delete-camera-calibration/🚫️removes-the-cam-f90b89  |
      | removes-the-40cba4       | delete-camera-calibration | 🚫delete-camera-calibration/🚫️removes-the-40cba4      |
      | removes-gcp-209b7d       | delete-gcp                | 🚮delete-gcp/🚫️removes-gcp-209b7d                     |
      | removes-the-south-42cd9e | delete-gcp                | 🚮delete-gcp/🚮removes-the-south-42cd9e                |
      | removes-an-8f3868        | delete-gcp                | 🚮delete-gcp/🕳️removes-an-8f3868                      |
      | drops-the-cam-a-a1f8a2   | delete-rig-extrinsic      | ✂️delete-rig-extrinsic/✂️drops-the-cam-a-a1f8a2      |
      | unplaces-the-f5b35e      | delete-rig-extrinsic      | ✂️delete-rig-extrinsic/⏮️unplaces-the-f5b35e         |
      | unplaces-the-a39356      | delete-rig-extrinsic      | ✂️delete-rig-extrinsic/✂️unplaces-the-a39356         |
      | removes-the-first-c0fc2a | delete-stream             | 🪓delete-stream/⏮️removes-the-first-c0fc2a            |
      | removes-the-spare-556d1d | delete-stream             | 🪓delete-stream/🪓removes-the-spare-556d1d             |
      | drops-the-content        | remove-content            | 🔪remove-content/🗑️drops-the-content                  |
      | removes-the-only-f82e64  | remove-gcp-observation    | 🚷remove-gcp-observation/🚫️removes-the-only-f82e64    |
      | drops-the-first-9ebf0b   | remove-gcp-observation    | 🚷remove-gcp-observation/⏮️drops-the-first-9ebf0b     |
      | drops-the-middle-282fb7  | remove-gcp-observation    | 🚷remove-gcp-observation/🚷drops-the-middle-282fb7     |
      | removes-the-last-304bdf  | remove-stream-frame       | ➖remove-stream-frame/🚫️removes-the-last-304bdf       |
      | drops-the-first-d98a0f   | remove-stream-frame       | ➖remove-stream-frame/⏮️drops-the-first-d98a0f        |
      | drops-the-middle-2d6d53  | remove-stream-frame       | ➖remove-stream-frame/✂️drops-the-middle-2d6d53       |
      | swaps-in-a-two-c688db    | replace-dense             | ☁️replace-dense/☁️swaps-in-a-two-c688db              |
      | swaps-in-a-denser-4174e1 | replace-dense             | ☁️replace-dense/☁️swaps-in-a-denser-4174e1           |
      | adds-the-dtm-and-64d5bb  | replace-geo-products      | 🗾replace-geo-products/🗺️adds-the-dtm-and-64d5bb      |
      | clears-the-geo-f4886e    | replace-geo-products      | 🗾replace-geo-products/🧹️clears-the-geo-f4886e        |
      | records-a-dtm-6e132a     | replace-geo-products      | 🗾replace-geo-products/🗺️records-a-dtm-6e132a         |
      | swaps-in-an-f23e71       | replace-mesh-result       | 🧱replace-mesh-result/🕸️swaps-in-an-f23e71            |
      | swaps-the-c43d9c         | replace-mesh-result       | 🧱replace-mesh-result/🕸️swaps-the-c43d9c              |
      | records-a-qc-f5caf4      | replace-qc                | 🧾replace-qc/📋️records-a-qc-f5caf4                    |
      | clears-the-qc-1d2249     | replace-qc                | 🧾replace-qc/🧹️clears-the-qc-1d2249                   |
      | files-a-qc-report-64d222 | replace-qc                | 🧾replace-qc/✅️files-a-qc-report-64d222               |
      | swaps-in-an-6d9ae4       | replace-sparse            | ⭐replace-sparse/✨️swaps-in-an-6d9ae4                 |
      | swaps-in-a-re-3cfa6d     | replace-sparse            | ⭐replace-sparse/✨️swaps-in-a-re-3cfa6d               |
      | clears-the-video-143f2b  | replace-stream-source     | 🔁replace-stream-source/🧹️clears-the-video-143f2b     |
      | attaches-a-607df8        | replace-stream-source     | 🔁replace-stream-source/📼️attaches-a-607df8           |
      | reingests-the-311c32     | replace-stream-source     | 🔁replace-stream-source/🎥️reingests-the-311c32        |
      | replaces-the-d40c68      | replace-tracks            | 🚂replace-tracks/⏸️replaces-the-d40c68                |
      | clears-every-760061      | replace-tracks            | 🚂replace-tracks/🕳️clears-every-760061                |
      | swaps-in-two-166265      | replace-tracks            | 🚂replace-tracks/🏃️swaps-in-two-166265                |
      | clears-the-d2f81a        | replace-trajectory        | 🛣️replace-trajectory/🧹️clears-the-d2f81a             |
      | drops-the-6436a8         | replace-trajectory        | 🛣️replace-trajectory/🕳️drops-the-6436a8              |
      | swaps-in-a-three-49b17f  | replace-trajectory        | 🛣️replace-trajectory/🛣️swaps-in-a-three-49b17f       |
      | refines-the-cam-0eaef0   | update-camera-calibration | 🛠️update-camera-calibration/🔍️refines-the-cam-0eaef0 |
      | refines-the-9fd25a       | update-camera-calibration | 🛠️update-camera-calibration/🔍️refines-the-9fd25a     |
      | raises-the-dense-ddb263  | update-dense-params       | 🌁update-dense-params/🔬️raises-the-dense-ddb263       |
      | sharpens-the-25044c      | update-dense-params       | 🌁update-dense-params/🧊️sharpens-the-25044c           |
      | switches-the-423de9      | update-feature-params     | 🌠update-feature-params/🔎️switches-the-423de9         |
      | moves-the-3621f6         | update-feature-params     | 🌠update-feature-params/🌟️moves-the-3621f6            |
      | enables-georefere-18a68a | update-geo-params         | 🌐update-geo-params/🌐️enables-georefere-18a68a        |
      | halves-the-002a17        | update-geo-params         | 🌐update-geo-params/🌐️halves-the-002a17               |
      | tightens-the-499c47      | update-ingest-params      | 🥣update-ingest-params/🔍️tightens-the-499c47          |
      | widens-the-73f33e        | update-ingest-params      | 🥣update-ingest-params/📥️widens-the-73f33e            |
      | switches-the-652d03      | update-match-params       | 🪢update-match-params/🌳️switches-the-652d03           |
      | switches-to-a-kd-d6fa4b  | update-match-params       | 🪢update-match-params/🌳️switches-to-a-kd-d6fa4b       |
      | doubles-the-c245d5       | update-mesh-params        | 🕸️update-mesh-params/🔳️doubles-the-c245d5            |
      | halves-the-voxel-21b53d  | update-mesh-params        | 🕸️update-mesh-params/🔳️halves-the-voxel-21b53d       |
      | enables-motion-2444a3    | update-motion-params      | 🏎️update-motion-params/🏃️enables-motion-2444a3       |
      | triples-the-4bb69f       | update-motion-params      | 🏎️update-motion-params/🏃️triples-the-4bb69f          |
      | retunes-the-cam-4ca5a2   | update-rig-extrinsic      | 🔩update-rig-extrinsic/📍️retunes-the-cam-4ca5a2       |
      | retunes-the-675f52       | update-rig-extrinsic      | 🔩update-rig-extrinsic/📍️retunes-the-675f52           |
      | switches-the-7f0371      | update-sfm-params         | 🧮update-sfm-params/🎯️switches-the-7f0371             |
      | tightens-the-850036      | update-sfm-params         | 🧮update-sfm-params/🎯️tightens-the-850036             |

  @id-mutate
  @level-exhaustive
  @mode-error
  Scenario Outline: Applying <id> is refused exactly as its vector declares
    Given the committed refusal vector <id>
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
      | id                       | kind                      | vector                                                | code                                   |
      | refuses-to-pick-3c0570   | add-gcp-observation       | 🔎add-gcp-observation/🚫️refuses-to-pick-3c0570         | mutation.target-missing                |
      | refuses-a-frame-81beea   | add-stream-frame          | ➕add-stream-frame/🎬️refuses-a-frame-81beea            | mutation.invariant                     |
      | refuses-to-c93e98        | add-stream-frame          | ➕add-stream-frame/🚫️refuses-to-c93e98                 | mutation.target-missing                |
      | refuses-a-gap            | append-content            | 📦append-content/🚫️refuses-a-gap                       | mutation.content-gap                   |
      | refuses-to-8095d3        | change-stream-sync        | ⏱️change-stream-sync/🚫️refuses-to-8095d3              | mutation.target-missing                |
      | rejects-an-e9fa51        | commit-reconstruction     | 🏁commit-reconstruction/🖼️rejects-an-e9fa51            | mutation.invalid-reconstruction-asset  |
      | rejects-an-5d3a60        | commit-reconstruction     | 🏁commit-reconstruction/🕸️rejects-an-5d3a60            | mutation.invalid-reconstruction-mesh   |
      | rejects-an-2e5568        | commit-reconstruction     | 🏁commit-reconstruction/⭐️rejects-an-2e5568            | mutation.invalid-reconstruction-sparse |
      | refuses-an-asset-cb0d4b  | create-asset              | 🧷create-asset/🚫️refuses-an-asset-cb0d4b               | mutation.invalid-asset-payload         |
      | refuses-a-camera-e92a02  | create-camera-calibration | 🔭create-camera-calibration/🚫️refuses-a-camera-e92a02  | mutation.duplicate-id                  |
      | refuses-a-19c1ab         | create-gcp                | 🧿create-gcp/🚫️refuses-a-19c1ab                        | mutation.duplicate-id                  |
      | refuses-a-second-95e04d  | create-rig-extrinsic      | ⛓️create-rig-extrinsic/🚫️refuses-a-second-95e04d      | mutation.duplicate-id                  |
      | refuses-a-rig-cb71ba     | create-rig-extrinsic      | ⛓️create-rig-extrinsic/🚫️refuses-a-rig-cb71ba         | mutation.invariant                     |
      | rejects-a-6b58da         | create-stream             | 🌱create-stream/🔂️rejects-a-6b58da                     | mutation.duplicate-id                  |
      | rejects-a-stream-aac5c2  | create-stream             | 🌱create-stream/👻️rejects-a-stream-aac5c2              | mutation.invariant                     |
      | refuses-to-5c6f74        | delete-asset              | 🗞️delete-asset/🗺️refuses-to-5c6f74                    | mutation.referenced                    |
      | refuses-to-c4563a        | delete-asset              | 🗞️delete-asset/🚫️refuses-to-c4563a                    | mutation.target-missing                |
      | refuses-to-f9541f        | delete-asset              | 🗞️delete-asset/🖼️refuses-to-f9541f                    | mutation.referenced                    |
      | refuses-to-73655a        | delete-camera-calibration | 🚫delete-camera-calibration/🚫️refuses-to-73655a        | mutation.target-missing                |
      | refuses-to-remove-3c8f32 | delete-camera-calibration | 🚫delete-camera-calibration/⛓️refuses-to-remove-3c8f32 | mutation.referenced                    |
      | refuses-to-12366b        | delete-gcp                | 🚮delete-gcp/🚫️refuses-to-12366b                       | mutation.target-missing                |
      | refuses-to-1805df        | delete-rig-extrinsic      | ✂️delete-rig-extrinsic/🚫️refuses-to-1805df            | mutation.target-missing                |
      | refuses-to-3c20ff        | delete-stream             | 🪓delete-stream/🚫️refuses-to-3c20ff                    | mutation.target-missing                |
      | refuses-to-remove-422a37 | delete-stream             | 🪓delete-stream/⛓️refuses-to-remove-422a37             | mutation.referenced                    |
      | refuses-missing-content  | remove-content            | 🔪remove-content/🚫️refuses-missing-content             | mutation.target-missing                |
      | refuses-an-109cf1        | remove-gcp-observation    | 🚷remove-gcp-observation/🚫️refuses-an-109cf1           | mutation.target-missing                |
      | refuses-a-frame-e7c374   | remove-stream-frame       | ➖remove-stream-frame/🚫️refuses-a-frame-e7c374         | mutation.target-missing                |
      | refuses-to-clear-b8c54a  | replace-geo-products      | 🗾replace-geo-products/🚫️refuses-to-clear-b8c54a       | mutation.target-missing                |
      | refuses-a-48f3a6         | replace-mesh-result       | 🧱replace-mesh-result/🚫️refuses-a-48f3a6               | mutation.incomplete-mesh               |
      | refuses-to-clear-30cbb5  | replace-qc                | 🧾replace-qc/🚫️refuses-to-clear-30cbb5                 | mutation.target-missing                |
      | refuses-to-f7f40d        | replace-stream-source     | 🔁replace-stream-source/🚫️refuses-to-f7f40d            | mutation.target-missing                |
      | refuses-to-clear-524569  | replace-trajectory        | 🛣️replace-trajectory/🚫️refuses-to-clear-524569        | mutation.target-missing                |
      | refuses-to-b60a39        | update-camera-calibration | 🛠️update-camera-calibration/🚫️refuses-to-b60a39       | mutation.target-missing                |
      | refuses-a-d82e38         | update-feature-params     | 🌠update-feature-params/🚫️refuses-a-d82e38             | mutation.invariant                     |
      | refuses-a-zero-fa917f    | update-geo-params         | 🌐update-geo-params/🚫️refuses-a-zero-fa917f            | mutation.invariant                     |
      | refuses-an-59752a        | update-ingest-params      | 🥣update-ingest-params/🚫️refuses-an-59752a             | mutation.invariant                     |
      | refuses-a-ratio-65dcb9   | update-match-params       | 🪢update-match-params/🚫️refuses-a-ratio-65dcb9         | mutation.invariant                     |
      | refuses-to-2cfb53        | update-rig-extrinsic      | 🔩update-rig-extrinsic/🚫️refuses-to-2cfb53             | mutation.target-missing                |

  @id-mutate
  @level-exhaustive
  @mode-error
  Scenario Outline: Applying <id> is warned as a no-op and moves nothing
    Given the committed no-op vector <id>
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
      | id                          | kind                      | vector                                              | code           |
      | warns-that-this-dca661      | add-gcp-observation       | 🔎add-gcp-observation/🔁️warns-that-this-dca661       | mutation.no-op |
      | warns-that-the-1e8abe       | add-stream-frame          | ➕add-stream-frame/🔁️warns-that-the-1e8abe           | mutation.no-op |
      | warns-that-the-leaves-exist | append-content            | 📦append-content/🔁️warns-that-the-leaves-exist       | mutation.no-op |
      | warns-that-the-a98c13       | change-stream-sync        | ⏱️change-stream-sync/🔁️warns-that-the-a98c13        | mutation.no-op |
      | warns-that-the-675b6e       | replace-dense             | ☁️replace-dense/🔁️warns-that-the-675b6e             | mutation.no-op |
      | warns-that-the-b39bab       | replace-mesh-result       | 🧱replace-mesh-result/🔁️warns-that-the-b39bab        | mutation.no-op |
      | warns-that-the-56a3a9       | replace-sparse            | ⭐replace-sparse/🔁️warns-that-the-56a3a9             | mutation.no-op |
      | warns-that-the-8dbf82       | replace-tracks            | 🚂replace-tracks/🔁️warns-that-the-8dbf82             | mutation.no-op |
      | warns-that-the-697b4f       | update-camera-calibration | 🛠️update-camera-calibration/🔁️warns-that-the-697b4f | mutation.no-op |
      | warns-that-the-4e65c8       | update-dense-params       | 🌁update-dense-params/🔁️warns-that-the-4e65c8        | mutation.no-op |
      | warns-that-the-b6b7dc       | update-feature-params     | 🌠update-feature-params/🔁️warns-that-the-b6b7dc      | mutation.no-op |
      | warns-that-the-efc6e8       | update-geo-params         | 🌐update-geo-params/🔁️warns-that-the-efc6e8          | mutation.no-op |
      | warns-that-the-8eaad8       | update-ingest-params      | 🥣update-ingest-params/🔁️warns-that-the-8eaad8       | mutation.no-op |
      | warns-that-the-414aae       | update-match-params       | 🪢update-match-params/🔁️warns-that-the-414aae        | mutation.no-op |
      | warns-that-the-887e9f       | update-mesh-params        | 🕸️update-mesh-params/🔁️warns-that-the-887e9f        | mutation.no-op |
      | warns-that-the-83ff67       | update-motion-params      | 🏎️update-motion-params/🔁️warns-that-the-83ff67      | mutation.no-op |
      | warns-that-the-89422a       | update-rig-extrinsic      | 🔩update-rig-extrinsic/🔁️warns-that-the-89422a       | mutation.no-op |
      | warns-that-the-79a92a       | update-sfm-params         | 🧮update-sfm-params/🔁️warns-that-the-79a92a          | mutation.no-op |

  @id-mutate
  @level-exhaustive
  @mode-error
  Scenario Outline: Applying <id> is refused by the shared commit vector it declares
    Given the case-local refusal vector <id>
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
    Given the committed specification vector <id>
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
      | id                       | kind                      | vector                                               |
      | adds-the-first-05b1b5    | add-gcp-observation       | 🔎add-gcp-observation/🔎️adds-the-first-05b1b5         |
      | picks-the-south-eb0c4d   | add-gcp-observation       | 🔎add-gcp-observation/🔎️picks-the-south-eb0c4d        |
      | appends-a-third-8ac259   | add-stream-frame          | ➕add-stream-frame/🎞️appends-a-third-8ac259           |
      | appends-an-0c2164        | add-stream-frame          | ➕add-stream-frame/🎞️appends-an-0c2164                |
      | appends-sparse-leaves    | append-content            | 📦append-content/🧱️appends-sparse-leaves              |
      | shifts-stream-a-5b442c   | change-stream-sync        | ⏱️change-stream-sync/⏱️shifts-stream-a-5b442c        |
      | retimes-the-50dd75       | change-stream-sync        | ⏱️change-stream-sync/⏱️retimes-the-50dd75            |
      | stores-a-new-d56283      | create-asset              | 🧷create-asset/🖼️stores-a-new-d56283                  |
      | stores-an-9f39e1         | create-asset              | 🧷create-asset/🖼️stores-an-9f39e1                     |
      | overwrites-an-a34b9d     | create-asset              | 🧷create-asset/♻️overwrites-an-a34b9d                 |
      | adds-the-cam-c-82c8fb    | create-camera-calibration | 🔭create-camera-calibration/📷️adds-the-cam-c-82c8fb   |
      | adds-a-fourth-97e912     | create-camera-calibration | 🔭create-camera-calibration/📷️adds-a-fourth-97e912    |
      | adds-gcp-tower-d71a54    | create-gcp                | 🧿create-gcp/📍️adds-gcp-tower-d71a54                  |
      | adds-a-quay-7569de       | create-gcp                | 🧿create-gcp/📍️adds-a-quay-7569de                     |
      | adds-a-control-298de4    | create-gcp                | 🧿create-gcp/🕳️adds-a-control-298de4                  |
      | adds-a-rig-2df5df        | create-rig-extrinsic      | ⛓️create-rig-extrinsic/🔗️adds-a-rig-2df5df           |
      | places-the-0d0b8d        | create-rig-extrinsic      | ⛓️create-rig-extrinsic/🔗️places-the-0d0b8d           |
      | adds-stream-c-458900     | create-stream             | 🌱create-stream/🎥️adds-stream-c-458900                |
      | adds-a-third-61fb5d      | create-stream             | 🌱create-stream/🛰️adds-a-third-61fb5d                 |
      | adds-an-unbound-2b2373   | create-stream             | 🌱create-stream/🎞️adds-an-unbound-2b2373              |
      | drops-the-spare-c6ffb6   | delete-asset              | 🗞️delete-asset/🧹️drops-the-spare-c6ffb6              |
      | sweeps-the-503b27        | delete-asset              | 🗞️delete-asset/🗑️sweeps-the-503b27                   |
      | removes-the-cam-f90b89   | delete-camera-calibration | 🚫delete-camera-calibration/🚫️removes-the-cam-f90b89  |
      | removes-the-40cba4       | delete-camera-calibration | 🚫delete-camera-calibration/🚫️removes-the-40cba4      |
      | removes-gcp-209b7d       | delete-gcp                | 🚮delete-gcp/🚫️removes-gcp-209b7d                     |
      | removes-the-south-42cd9e | delete-gcp                | 🚮delete-gcp/🚮removes-the-south-42cd9e                |
      | removes-an-8f3868        | delete-gcp                | 🚮delete-gcp/🕳️removes-an-8f3868                      |
      | drops-the-cam-a-a1f8a2   | delete-rig-extrinsic      | ✂️delete-rig-extrinsic/✂️drops-the-cam-a-a1f8a2      |
      | unplaces-the-f5b35e      | delete-rig-extrinsic      | ✂️delete-rig-extrinsic/⏮️unplaces-the-f5b35e         |
      | unplaces-the-a39356      | delete-rig-extrinsic      | ✂️delete-rig-extrinsic/✂️unplaces-the-a39356         |
      | removes-the-first-c0fc2a | delete-stream             | 🪓delete-stream/⏮️removes-the-first-c0fc2a            |
      | removes-the-spare-556d1d | delete-stream             | 🪓delete-stream/🪓removes-the-spare-556d1d             |
      | drops-the-content        | remove-content            | 🔪remove-content/🗑️drops-the-content                  |
      | removes-the-only-f82e64  | remove-gcp-observation    | 🚷remove-gcp-observation/🚫️removes-the-only-f82e64    |
      | drops-the-first-9ebf0b   | remove-gcp-observation    | 🚷remove-gcp-observation/⏮️drops-the-first-9ebf0b     |
      | drops-the-middle-282fb7  | remove-gcp-observation    | 🚷remove-gcp-observation/🚷drops-the-middle-282fb7     |
      | removes-the-last-304bdf  | remove-stream-frame       | ➖remove-stream-frame/🚫️removes-the-last-304bdf       |
      | drops-the-first-d98a0f   | remove-stream-frame       | ➖remove-stream-frame/⏮️drops-the-first-d98a0f        |
      | drops-the-middle-2d6d53  | remove-stream-frame       | ➖remove-stream-frame/✂️drops-the-middle-2d6d53       |
      | swaps-in-a-two-c688db    | replace-dense             | ☁️replace-dense/☁️swaps-in-a-two-c688db              |
      | swaps-in-a-denser-4174e1 | replace-dense             | ☁️replace-dense/☁️swaps-in-a-denser-4174e1           |
      | adds-the-dtm-and-64d5bb  | replace-geo-products      | 🗾replace-geo-products/🗺️adds-the-dtm-and-64d5bb      |
      | clears-the-geo-f4886e    | replace-geo-products      | 🗾replace-geo-products/🧹️clears-the-geo-f4886e        |
      | records-a-dtm-6e132a     | replace-geo-products      | 🗾replace-geo-products/🗺️records-a-dtm-6e132a         |
      | swaps-in-an-f23e71       | replace-mesh-result       | 🧱replace-mesh-result/🕸️swaps-in-an-f23e71            |
      | swaps-the-c43d9c         | replace-mesh-result       | 🧱replace-mesh-result/🕸️swaps-the-c43d9c              |
      | records-a-qc-f5caf4      | replace-qc                | 🧾replace-qc/📋️records-a-qc-f5caf4                    |
      | clears-the-qc-1d2249     | replace-qc                | 🧾replace-qc/🧹️clears-the-qc-1d2249                   |
      | files-a-qc-report-64d222 | replace-qc                | 🧾replace-qc/✅️files-a-qc-report-64d222               |
      | swaps-in-an-6d9ae4       | replace-sparse            | ⭐replace-sparse/✨️swaps-in-an-6d9ae4                 |
      | swaps-in-a-re-3cfa6d     | replace-sparse            | ⭐replace-sparse/✨️swaps-in-a-re-3cfa6d               |
      | clears-the-video-143f2b  | replace-stream-source     | 🔁replace-stream-source/🧹️clears-the-video-143f2b     |
      | attaches-a-607df8        | replace-stream-source     | 🔁replace-stream-source/📼️attaches-a-607df8           |
      | reingests-the-311c32     | replace-stream-source     | 🔁replace-stream-source/🎥️reingests-the-311c32        |
      | replaces-the-d40c68      | replace-tracks            | 🚂replace-tracks/⏸️replaces-the-d40c68                |
      | clears-every-760061      | replace-tracks            | 🚂replace-tracks/🕳️clears-every-760061                |
      | swaps-in-two-166265      | replace-tracks            | 🚂replace-tracks/🏃️swaps-in-two-166265                |
      | clears-the-d2f81a        | replace-trajectory        | 🛣️replace-trajectory/🧹️clears-the-d2f81a             |
      | drops-the-6436a8         | replace-trajectory        | 🛣️replace-trajectory/🕳️drops-the-6436a8              |
      | swaps-in-a-three-49b17f  | replace-trajectory        | 🛣️replace-trajectory/🛣️swaps-in-a-three-49b17f       |
      | refines-the-cam-0eaef0   | update-camera-calibration | 🛠️update-camera-calibration/🔍️refines-the-cam-0eaef0 |
      | refines-the-9fd25a       | update-camera-calibration | 🛠️update-camera-calibration/🔍️refines-the-9fd25a     |
      | raises-the-dense-ddb263  | update-dense-params       | 🌁update-dense-params/🔬️raises-the-dense-ddb263       |
      | sharpens-the-25044c      | update-dense-params       | 🌁update-dense-params/🧊️sharpens-the-25044c           |
      | switches-the-423de9      | update-feature-params     | 🌠update-feature-params/🔎️switches-the-423de9         |
      | moves-the-3621f6         | update-feature-params     | 🌠update-feature-params/🌟️moves-the-3621f6            |
      | enables-georefere-18a68a | update-geo-params         | 🌐update-geo-params/🌐️enables-georefere-18a68a        |
      | halves-the-002a17        | update-geo-params         | 🌐update-geo-params/🌐️halves-the-002a17               |
      | tightens-the-499c47      | update-ingest-params      | 🥣update-ingest-params/🔍️tightens-the-499c47          |
      | widens-the-73f33e        | update-ingest-params      | 🥣update-ingest-params/📥️widens-the-73f33e            |
      | switches-the-652d03      | update-match-params       | 🪢update-match-params/🌳️switches-the-652d03           |
      | switches-to-a-kd-d6fa4b  | update-match-params       | 🪢update-match-params/🌳️switches-to-a-kd-d6fa4b       |
      | doubles-the-c245d5       | update-mesh-params        | 🕸️update-mesh-params/🔳️doubles-the-c245d5            |
      | halves-the-voxel-21b53d  | update-mesh-params        | 🕸️update-mesh-params/🔳️halves-the-voxel-21b53d       |
      | enables-motion-2444a3    | update-motion-params      | 🏎️update-motion-params/🏃️enables-motion-2444a3       |
      | triples-the-4bb69f       | update-motion-params      | 🏎️update-motion-params/🏃️triples-the-4bb69f          |
      | retunes-the-cam-4ca5a2   | update-rig-extrinsic      | 🔩update-rig-extrinsic/📍️retunes-the-cam-4ca5a2       |
      | retunes-the-675f52       | update-rig-extrinsic      | 🔩update-rig-extrinsic/📍️retunes-the-675f52           |
      | switches-the-7f0371      | update-sfm-params         | 🧮update-sfm-params/🎯️switches-the-7f0371             |
      | tightens-the-850036      | update-sfm-params         | 🧮update-sfm-params/🎯️tightens-the-850036             |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the document its own refusal never moved
    Given the committed refusal vector <id>
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
      | id                          | kind                      | vector                                                | code                                   |
      | refuses-to-pick-3c0570      | add-gcp-observation       | 🔎add-gcp-observation/🚫️refuses-to-pick-3c0570         | mutation.target-missing                |
      | warns-that-this-dca661      | add-gcp-observation       | 🔎add-gcp-observation/🔁️warns-that-this-dca661         | mutation.no-op                         |
      | refuses-a-frame-81beea      | add-stream-frame          | ➕add-stream-frame/🎬️refuses-a-frame-81beea            | mutation.invariant                     |
      | refuses-to-c93e98           | add-stream-frame          | ➕add-stream-frame/🚫️refuses-to-c93e98                 | mutation.target-missing                |
      | warns-that-the-1e8abe       | add-stream-frame          | ➕add-stream-frame/🔁️warns-that-the-1e8abe             | mutation.no-op                         |
      | refuses-a-gap               | append-content            | 📦append-content/🚫️refuses-a-gap                       | mutation.content-gap                   |
      | warns-that-the-leaves-exist | append-content            | 📦append-content/🔁️warns-that-the-leaves-exist         | mutation.no-op                         |
      | refuses-to-8095d3           | change-stream-sync        | ⏱️change-stream-sync/🚫️refuses-to-8095d3              | mutation.target-missing                |
      | warns-that-the-a98c13       | change-stream-sync        | ⏱️change-stream-sync/🔁️warns-that-the-a98c13          | mutation.no-op                         |
      | rejects-an-e9fa51           | commit-reconstruction     | 🏁commit-reconstruction/🖼️rejects-an-e9fa51            | mutation.invalid-reconstruction-asset  |
      | rejects-an-5d3a60           | commit-reconstruction     | 🏁commit-reconstruction/🕸️rejects-an-5d3a60            | mutation.invalid-reconstruction-mesh   |
      | rejects-an-2e5568           | commit-reconstruction     | 🏁commit-reconstruction/⭐️rejects-an-2e5568            | mutation.invalid-reconstruction-sparse |
      | refuses-an-asset-cb0d4b     | create-asset              | 🧷create-asset/🚫️refuses-an-asset-cb0d4b               | mutation.invalid-asset-payload         |
      | refuses-a-camera-e92a02     | create-camera-calibration | 🔭create-camera-calibration/🚫️refuses-a-camera-e92a02  | mutation.duplicate-id                  |
      | refuses-a-19c1ab            | create-gcp                | 🧿create-gcp/🚫️refuses-a-19c1ab                        | mutation.duplicate-id                  |
      | refuses-a-second-95e04d     | create-rig-extrinsic      | ⛓️create-rig-extrinsic/🚫️refuses-a-second-95e04d      | mutation.duplicate-id                  |
      | refuses-a-rig-cb71ba        | create-rig-extrinsic      | ⛓️create-rig-extrinsic/🚫️refuses-a-rig-cb71ba         | mutation.invariant                     |
      | rejects-a-6b58da            | create-stream             | 🌱create-stream/🔂️rejects-a-6b58da                     | mutation.duplicate-id                  |
      | rejects-a-stream-aac5c2     | create-stream             | 🌱create-stream/👻️rejects-a-stream-aac5c2              | mutation.invariant                     |
      | refuses-to-5c6f74           | delete-asset              | 🗞️delete-asset/🗺️refuses-to-5c6f74                    | mutation.referenced                    |
      | refuses-to-c4563a           | delete-asset              | 🗞️delete-asset/🚫️refuses-to-c4563a                    | mutation.target-missing                |
      | refuses-to-f9541f           | delete-asset              | 🗞️delete-asset/🖼️refuses-to-f9541f                    | mutation.referenced                    |
      | refuses-to-73655a           | delete-camera-calibration | 🚫delete-camera-calibration/🚫️refuses-to-73655a        | mutation.target-missing                |
      | refuses-to-remove-3c8f32    | delete-camera-calibration | 🚫delete-camera-calibration/⛓️refuses-to-remove-3c8f32 | mutation.referenced                    |
      | refuses-to-12366b           | delete-gcp                | 🚮delete-gcp/🚫️refuses-to-12366b                       | mutation.target-missing                |
      | refuses-to-1805df           | delete-rig-extrinsic      | ✂️delete-rig-extrinsic/🚫️refuses-to-1805df            | mutation.target-missing                |
      | refuses-to-3c20ff           | delete-stream             | 🪓delete-stream/🚫️refuses-to-3c20ff                    | mutation.target-missing                |
      | refuses-to-remove-422a37    | delete-stream             | 🪓delete-stream/⛓️refuses-to-remove-422a37             | mutation.referenced                    |
      | refuses-missing-content     | remove-content            | 🔪remove-content/🚫️refuses-missing-content             | mutation.target-missing                |
      | refuses-an-109cf1           | remove-gcp-observation    | 🚷remove-gcp-observation/🚫️refuses-an-109cf1           | mutation.target-missing                |
      | refuses-a-frame-e7c374      | remove-stream-frame       | ➖remove-stream-frame/🚫️refuses-a-frame-e7c374         | mutation.target-missing                |
      | warns-that-the-675b6e       | replace-dense             | ☁️replace-dense/🔁️warns-that-the-675b6e               | mutation.no-op                         |
      | refuses-to-clear-b8c54a     | replace-geo-products      | 🗾replace-geo-products/🚫️refuses-to-clear-b8c54a       | mutation.target-missing                |
      | warns-that-the-b39bab       | replace-mesh-result       | 🧱replace-mesh-result/🔁️warns-that-the-b39bab          | mutation.no-op                         |
      | refuses-a-48f3a6            | replace-mesh-result       | 🧱replace-mesh-result/🚫️refuses-a-48f3a6               | mutation.incomplete-mesh               |
      | refuses-to-clear-30cbb5     | replace-qc                | 🧾replace-qc/🚫️refuses-to-clear-30cbb5                 | mutation.target-missing                |
      | warns-that-the-56a3a9       | replace-sparse            | ⭐replace-sparse/🔁️warns-that-the-56a3a9               | mutation.no-op                         |
      | refuses-to-f7f40d           | replace-stream-source     | 🔁replace-stream-source/🚫️refuses-to-f7f40d            | mutation.target-missing                |
      | warns-that-the-8dbf82       | replace-tracks            | 🚂replace-tracks/🔁️warns-that-the-8dbf82               | mutation.no-op                         |
      | refuses-to-clear-524569     | replace-trajectory        | 🛣️replace-trajectory/🚫️refuses-to-clear-524569        | mutation.target-missing                |
      | refuses-to-b60a39           | update-camera-calibration | 🛠️update-camera-calibration/🚫️refuses-to-b60a39       | mutation.target-missing                |
      | warns-that-the-697b4f       | update-camera-calibration | 🛠️update-camera-calibration/🔁️warns-that-the-697b4f   | mutation.no-op                         |
      | warns-that-the-4e65c8       | update-dense-params       | 🌁update-dense-params/🔁️warns-that-the-4e65c8          | mutation.no-op                         |
      | refuses-a-d82e38            | update-feature-params     | 🌠update-feature-params/🚫️refuses-a-d82e38             | mutation.invariant                     |
      | warns-that-the-b6b7dc       | update-feature-params     | 🌠update-feature-params/🔁️warns-that-the-b6b7dc        | mutation.no-op                         |
      | refuses-a-zero-fa917f       | update-geo-params         | 🌐update-geo-params/🚫️refuses-a-zero-fa917f            | mutation.invariant                     |
      | warns-that-the-efc6e8       | update-geo-params         | 🌐update-geo-params/🔁️warns-that-the-efc6e8            | mutation.no-op                         |
      | refuses-an-59752a           | update-ingest-params      | 🥣update-ingest-params/🚫️refuses-an-59752a             | mutation.invariant                     |
      | warns-that-the-8eaad8       | update-ingest-params      | 🥣update-ingest-params/🔁️warns-that-the-8eaad8         | mutation.no-op                         |
      | refuses-a-ratio-65dcb9      | update-match-params       | 🪢update-match-params/🚫️refuses-a-ratio-65dcb9         | mutation.invariant                     |
      | warns-that-the-414aae       | update-match-params       | 🪢update-match-params/🔁️warns-that-the-414aae          | mutation.no-op                         |
      | warns-that-the-887e9f       | update-mesh-params        | 🕸️update-mesh-params/🔁️warns-that-the-887e9f          | mutation.no-op                         |
      | warns-that-the-83ff67       | update-motion-params      | 🏎️update-motion-params/🔁️warns-that-the-83ff67        | mutation.no-op                         |
      | refuses-to-2cfb53           | update-rig-extrinsic      | 🔩update-rig-extrinsic/🚫️refuses-to-2cfb53             | mutation.target-missing                |
      | warns-that-the-89422a       | update-rig-extrinsic      | 🔩update-rig-extrinsic/🔁️warns-that-the-89422a         | mutation.no-op                         |
      | warns-that-the-79a92a       | update-sfm-params         | 🧮update-sfm-params/🔁️warns-that-the-79a92a            | mutation.no-op                         |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the document its case-local refusal never moved
    Given the case-local refusal vector <id>
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
