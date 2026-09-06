@capability-puzzle-2d-1-mutate
@oracle-puzzle-2d-networkx-graph
@comparison-ordered-json-v1
Feature: Answer every committed puzzle2d vector with four third-party Python libraries that have never seen this repository
  The sibling case `◻️mutate-puzzle-2d-1` compares this repository's Rust with a SECOND
  IMPLEMENTATION written inside this repository. A second implementation is useful evidence and it
  is never independent evidence. This case supplies the independent half, and it re-implements no
  verb: for every committed vector it rebuilds the same fact in a DIFFERENT data structure and lets
  a library state the answer.

  **`networkx` (BSD-3-Clause) speaks the TOPOLOGY.** The board becomes a `MultiDiGraph` whose
  vertices are NODES and HANDLES, with `owns` edges (node → handle) and `wire` edges
  (handle → handle, keyed by the board's edge id). The two cascades this artifact is defined by —
  removing a handle severs the wire attached to it, deleting a node severs every wire on any of its
  handles — are then not computed by the adapter at all: they are what `Graph.remove_node` does to a
  vertex's incident edges, and `networkx.utils.graphs_equal` is what decides whether the committed
  after-snapshot agrees. This answers, rather than repeats, this subset's own earlier survey note
  that "GraphML, DOT and GEXF all join node to node and none of them can express an edge whose
  endpoints are ports OWNED BY a node": that objection is about carrier FORMATS. Model the port as a
  VERTEX and the two-level connectivity becomes ordinary graph incidence, which a graph ALGORITHM
  library speaks natively. The kind-compatibility relation gets a second, separate `DiGraph` over
  kind labels, where the library's own one-edge-per-ordered-pair rule enforces the relation's
  uniqueness and `networkx.isolates` derives which kind labels survive a disconnection.

  **`shapely` 2 (BSD-3-Clause, on GEOS) speaks the GEOMETRY.** A node's footprint becomes a real
  `Point.buffer` or `box`; `move-node` becomes `affinity.translate`, `scale-node` becomes
  `affinity.scale(origin=…)` about the node's own position, `change-node-anchor` becomes a claim of
  geometric invariance, and `replace-node-geometry`'s null-dropping circle→rectangle rebuild is held
  to the area its own arguments imply (πr², or width × height) and to a centroid that did not move.
  Every other kind is required to leave every footprint on the board exactly where it was.

  **`jsonschema` (MIT) speaks the PAYLOAD SHAPE.** Every committed `🦠️mutation`, with its
  discriminator removed, is validated against its own leaf `🧬️.schema.json`, and every leaf schema
  is additionally handed a member it does not declare — so an accepted payload proves the validator
  ran rather than that the schema was permissive.

  **`jsonpatch` (Modified BSD), corroborated by `deepdiff` (MIT), speaks the DIFF.** The committed
  `🔺️diff` is a TYPED `Puzzle2dDiff` with per-collection `added`/`removed`/`patched`, not RFC 6902.
  So an RFC 6902 patch is DERIVED from `(before, after)` by `jsonpatch.make_patch`, round-tripped
  through `apply` to reproduce the after-snapshot exactly, and then held against the typed diff: the
  members its op paths reach must be exactly the members the typed diff declares, and a record
  belongs in `patched` exactly when RFC 6902 needs at least one operation to turn its before-shape
  into its after-shape. `deepdiff`, a structurally unrelated algorithm, must agree with `jsonpatch`
  about whether the document moved at all.

  🧫️ VECTORS ARE DISCOVERED, NEVER LISTED. The one fixture this case declares is the mutation
  vocabulary's own `🔣️.json`; every scenario directory beneath its parent is found at run time. That
  is not convenience: this artifact's scenario directories are authored and renamed continuously,
  and the sibling case's hand-written `Examples` table pointed at four directories that no longer
  existed while still reading green from cache. A table cannot go stale if there is no table.

  🚧️ WHAT THESE LIBRARIES DO NOT ADJUDICATE, stated rather than implied: the anchor ENUM
  (`fixed`/`derived` is a board concept, not a geometric one), a handle's `angle` (no library here
  has a polar-on-owner notion), which of three refusal rules makes a `replace-node-handle` vector a
  no-op, the MEANING of a kind label, and the ORDER of a node's handle list (graph incidence is
  unordered). Those stay with the second implementation and the committed vectors, and the
  registration says so.

  🚧️ ONE MORE CEILING. Only the ORACLE half of this case runs: this repository ships no Python
  implementation of `s.puzzle.2d`, so no subject is dispatched here. What this case establishes is
  that four independent libraries agree with the committed vectors — the same class of check that
  found this vocabulary's leaf-schema drift on its first run — not this repository's codec against a
  second producer. `◻️mutate-puzzle-2d-1` owns that half.

  @id-graph-cascade
  @level-long
  @mode-differential
  Scenario: networkx reproduces every topological kind and the ownership invariant on every board
    Given every committed vector under asset://🧬️schema/🧬️mutations/🔣️.json
    When each board is rebuilt as a networkx MultiDiGraph over node and handle vertices
    Then every handle vertex is owned by exactly one node and every wire joins two handle vertices
    And the graph networkx computes for each applied topological kind equals the graph of the committed after-snapshot
    And a rejected or no-op vector leaves the graph untouched

  @id-kind-compatibility
  @level-long
  @mode-differential
  Scenario: networkx reproduces the kind-compatibility relation and holds it to one edge per ordered pair
    Given every committed vector under asset://🧬️schema/🧬️mutations/🔣️.json
    When the meta relation is rebuilt as a networkx DiGraph over kind labels
    Then the relation carries exactly as many distinct ordered pairs as it declares records
    And connect-kind-compatibility and disconnect-kind-compatibility reproduce the committed after-snapshot's relation
    And no other kind moves the relation

  @id-geometry-transforms
  @level-long
  @mode-differential
  Scenario: shapely reproduces position, scale, anchor invariance and the null-dropping extent rebuild
    Given every committed vector under asset://🧬️schema/🧬️mutations/🔣️.json
    When each node's footprint is built as a shapely Point buffer or box at the node's own scale
    Then move-node equals shapely's translate of the before footprint and preserves its area
    And scale-node equals shapely's scale about the node's own position and multiplies the area by its square
    And change-node-anchor leaves the footprint geometrically equal
    And replace-node-geometry rebuilds an extent whose area its arguments imply and whose centroid did not move
    And every other kind leaves every footprint on the board exactly where it was

  @id-payload-schemas
  @level-long
  @mode-conformance
  Scenario: jsonschema accepts every committed payload against its own leaf schema and rejects an undeclared member
    Given every committed vector under asset://🧬️schema/🧬️mutations/🔣️.json
    When each payload is validated against its leaf 🧬️.schema.json by the draft the schema itself names
    Then the committed payload carries its kind's own internally tagged discriminator
    And the validator accepts the committed payload with no error
    And the validator rejects the same payload once a member the schema does not declare is added

  @id-diff-reproduction
  @level-long
  @mode-differential
  Scenario: jsonpatch reproduces every after-snapshot and deepdiff corroborates whether the document moved
    Given every committed vector under asset://🧬️schema/🧬️mutations/🔣️.json
    When an RFC 6902 patch is derived from the before and after snapshots by jsonpatch
    Then applying that patch to the before-snapshot reproduces the committed after-snapshot exactly
    And deepdiff and jsonpatch agree on whether the document moved at all
    And a rejected or no-op vector yields no operation
    And the members the patch touches are exactly the members the typed Puzzle2dDiff declares
    And the typed diff's added, removed and patched sets are exactly what jsonpatch needs operations for
