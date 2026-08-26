@capability-remodel-1-mutate
@no-oracle-remodel-mutation-semantics
@comparison-ordered-json-v1
@mutations-remodel-1-any
Feature: Apply every typed remodel-scene mutation to its committed specification vector
  `s.remodel.remodel` is a semio-NATIVE artifact, and the document is a reconstruction
  JOB — streams, calibrations, ground control points, the eight parameter blocks a pipeline runs
  under, and the engine-owned results — not a point cloud or a mesh file. A reader of COLMAP, LAS or
  PLY output would therefore be judging a different artifact, and nothing reads `.dsl.semio`. That is
  recorded as the `remodel-mutation-semantics` no-oracle decision in
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`, whose substitutes are the
  committed per-kind specification vectors plus the inverse law. This case re-exercises those SAME
  committed bytes through `apply_remodel_mutation_json`/`undo_remodel_mutation_json`.

  ⚠️ THIS NO-ORACLE DECISION IS A DEBT, NOT A VERDICT, and is recorded as one. Declining a third-party
  LIBRARY is a different judgement from declining a SECOND IMPLEMENTATION, and only the first was ever
  made here. `mutate-cad-1` and `mutate-lowpoly-1` took Python second
  implementations over this same `.dsl.semio` carrier in this wave, so the same is writable for this
  subset from `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️component.json`, the rules of
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` and the
  committed vectors this feature already replays. What blocks it TODAY is stated in the decision and
  is one edit: this case's vectors are not declared as `asset://` fixtures — the `Examples` table
  carries the payloads inline and the adapter reads the committed files through `include_str!` — so
  the plan pins none of their digests and a Python reference cannot read them at all. Separately, `identity-round-trip` would still be refused: this subset's committed
  snapshot text grammar is the repository-wide placeholder `payload = OCTET+`, whose header production
  declares `"schema" SP "stdio.json"` against an artifact whose own first line says otherwise.
  Until that is done, every assertion below still lives in the SUBJECT role, and the ceiling is the
  one this decision has always had: no second producer runs beside it, so a mistake shared by the
  handcrafted vector and the production code passes unseen.

  The 35 kinds are three families with three different laws, and the Examples rows are chosen to
  separate them. The referential family creates, deletes and changes members of five pools that point
  at each other, so `delete-stream` CASCADES into the GCP observation that named it and `delete-gcp`
  cascades into its own observations — undoing either has to bring the cascaded rows back, not merely
  the deleted record. The `update-*-params` family replaces one of the eight `ReconstructionParams`
  sub-records WHOLE, because the fields inside one are only meaningful together; its vectors move two
  or three fields at once for that reason. The `replace-*` family swaps engine-owned result payloads
  that are large and opaque to a hand-written literal — a four-point sparse cloud, a textured mesh, a
  QC report — which is why the adapter decodes the committed payload rather than restating it.

  `commit-reconstruction` is the one kind with NO committed leaf vector, and the reason is structural
  rather than an oversight: its diff reads process-global staging state
  (`commit_staged_remodel_reconstruction`, `durable_staged_remodel_asset`) that a
  `(before, mutation, after)` triple cannot carry. This case exercises it through its own documented
  refusal path instead, using a vector assembled ONCE from committed sibling content and kept in this
  case's own fixtures — local://commit-reconstruction-before.json is a byte copy of
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️replace-job/🧪️tests/advances-the-job-to-texturing/📸️snapshot/⬅️before/🔣️component.json`,
  local://commit-reconstruction-mutation.json pairs that leaf's committed `job` payload with
  `⭐replace-sparse`'s committed `sparse` payload (a plain point buffer, deliberately NOT a replayable
  staging handle), and local://commit-reconstruction-after.json is the before-document unchanged,
  because the documented answer is `mutation.invalid-reconstruction-sparse` and a refused commit must
  leave the scene untouched. Note also that `commit-reconstruction`'s own inverse restores only `job`
  and the six result slots — never `assets` or `durable_artifacts` — so the inverse law holds for this
  refusal vector and would NOT hold for a commit that published new assets; that is a real weakness of
  the kind, recorded here rather than hidden by the vector that dodges it.

  Because this case records a no-oracle decision the runner executes NO oracle role, so every
  assertion below lives in the subject handler, which compares against the committed after-document
  through the shared `⚖️law` module and fails with the first divergence named by JSON path.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Applying <id> reaches its committed after-document
    Given the committed before-document and mutation payload of the <id> specification vector
    When <id> is applied through apply_remodel_mutation_json
    Then the resulting document is the committed after-document, and the mutation moved it
    Examples:
      | id                        |
      | create-stream             |
      | delete-stream             |
      | change-stream-sync        |
      | add-stream-frame          |
      | remove-stream-frame       |
      | replace-stream-source     |
      | create-asset              |
      | delete-asset              |
      | create-camera-calibration |
      | update-camera-calibration |
      | delete-camera-calibration |
      | create-rig-extrinsic      |
      | delete-rig-extrinsic      |
      | update-rig-extrinsic      |
      | create-gcp                |
      | delete-gcp                |
      | add-gcp-observation       |
      | remove-gcp-observation    |
      | update-ingest-params      |
      | update-feature-params     |
      | update-match-params       |
      | update-sfm-params         |
      | update-dense-params       |
      | update-mesh-params        |
      | update-motion-params      |
      | update-geo-params         |
      | replace-job               |
      | replace-sparse            |
      | replace-dense             |
      | replace-mesh-result       |
      | replace-trajectory        |
      | replace-tracks            |
      | replace-geo-products      |
      | replace-qc                |

  @id-mutate
  @level-exhaustive
  @mode-error
  Scenario Outline: Applying <id> is refused exactly as its vector declares
    Given the committed before-document and mutation payload of the <id> specification vector
    When <id> is applied through apply_remodel_mutation_json
    Then the document is left untouched and the declared <code> refusal was raised
    Examples:
      | id                        | code                                   |
      | commit-reconstruction     | mutation.invalid-reconstruction-sparse |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores its committed before-document
    Given the committed before-document and mutation payload of the <id> specification vector
    When <id> and then every step of its own computed inverse are applied through undo_remodel_mutation_json
    Then the document is the committed before-document again, member positions included
    Examples:
      | id                        |
      | create-stream             |
      | delete-stream             |
      | change-stream-sync        |
      | add-stream-frame          |
      | remove-stream-frame       |
      | replace-stream-source     |
      | create-asset              |
      | delete-asset              |
      | create-camera-calibration |
      | update-camera-calibration |
      | delete-camera-calibration |
      | create-rig-extrinsic      |
      | delete-rig-extrinsic      |
      | update-rig-extrinsic      |
      | create-gcp                |
      | delete-gcp                |
      | add-gcp-observation       |
      | remove-gcp-observation    |
      | update-ingest-params      |
      | update-feature-params     |
      | update-match-params       |
      | update-sfm-params         |
      | update-dense-params       |
      | update-mesh-params        |
      | update-motion-params      |
      | update-geo-params         |
      | replace-job               |
      | replace-sparse            |
      | replace-dense             |
      | replace-mesh-result       |
      | replace-trajectory        |
      | replace-tracks            |
      | replace-geo-products      |
      | replace-qc                |
      | commit-reconstruction     |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse and reprint the real committed example without passing bytes through
    Given the real committed example asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    When it is parsed, printed back to DSL and parsed again through round_trip_remodel_dsl
    Then both parses agree on one document, and the reprinted text reproduces the committed example byte for byte
