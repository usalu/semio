@capability-puzzle-2d-1-mutate
@oracle-puzzle-2d-graphology-graph
@comparison-ordered-json-v1
Feature: Answer the same puzzle2d vectors a second time, in a second ecosystem, with three npm libraries
  `🕸️third-party-puzzle-2d-1` answers this artifact's topology, payload-shape and diff questions with
  `networkx`, `shapely`, `jsonschema` and `jsonpatch` on CPython. This case answers three of the same
  four questions again, in bun, with libraries from an unrelated ecosystem written by unrelated
  authors: `graphology` 0.26 (MIT), the npm `jsonschema` 1.5 (MIT) and `fast-json-patch` 3.1 (MIT).

  Two independent engine families agreeing is the whole point. A single graph library that got
  cascade-on-delete subtly wrong would look exactly like agreement; two, on two runtimes, do not.
  On its first run the two ecosystems agreed to the vector on twenty-four leaf-schema
  disagreements, which is what makes those findings credible rather than an artefact of one
  validator.

  **`graphology` speaks the TOPOLOGY**, in the same bipartite encoding: NODE and HANDLE vertices,
  `owns` edges (node → handle), `wire` edges (handle → handle) keyed by the board's edge id.
  `Graph.dropNode` severs a vertex's incident edges because that is what a graph is, and
  `Graph.export` — graphology's own serializer — is what the comparison is made over, so nothing
  here restates the cascade it is testing. Its one honest limitation is recorded in the adapter:
  graphology's core has no vertex RELABEL, so a `replace-node-handle` that re-identifies a port is
  expressed as drop-and-reattach, and the Python half — whose library does have `relabel_nodes` —
  is what adjudicates that carry-over independently.

  **The npm `jsonschema` speaks the PAYLOAD SHAPE.** `ajv` was the obvious choice and is DECLINED,
  for a reason this repository enforces mechanically rather than by taste: `ajv` is declared
  `production-runtime` by five packages in this tree, and `verify dependencies literal-external`
  counts an oracle package production can reach as an `oracle-conflict`. An oracle that production
  can reach is this repository comparing itself with itself. `jsonschema` carries the same draft-07
  role with no production reachability, and the registration records the substitution.

  **`fast-json-patch` speaks the DIFF**, on the same terms as the Python half: derive RFC 6902 from
  `(before, after)`, apply it back to reproduce the after-snapshot, and hold the typed
  `Puzzle2dDiff` to the members those op paths reach and to a `patched` set that is exactly the
  records RFC 6902 needs an operation for. Equality is asked of the library itself — an empty patch
  between the reproduction and the committed file — because `ordered-json-v1` holds array order
  significant and key order never, which a string comparison gets wrong.

  🚫️ GEOMETRY IS DELIBERATELY NOT RE-ANSWERED HERE. `shapely` sits on GEOS and its affine algebra is
  strictly richer than anything this ecosystem offers for the same job; `@flatten-js/core` was tried
  and needs manual matrix composition for the anchor-relative scale shapely does in one call. A
  second, weaker geometry check would add an engine to the count without adding evidence.

  🧫️ VECTORS ARE DISCOVERED, NEVER LISTED — the same one declared fixture, the same run-time walk,
  for the same reason: the sibling case's hand-written `Examples` table pointed at four directories
  that no longer existed while still reading green from cache.

  🚧️ ONLY THE ORACLE HALF OF THIS CASE RUNS. This repository ships TypeScript TYPES for
  `s.puzzle.2d` and no TypeScript reducer, so there is no TypeScript subject to compare against; the
  subject half of this vocabulary is Rust and lives in `◻️mutate-puzzle-2d-1`.

  @id-graph-cascade
  @level-long
  @mode-differential
  Scenario: graphology reproduces every topological kind and the ownership invariant on every board
    Given every committed vector under asset://🧬️schema/🧬️mutations/🔣️.json
    When each board is rebuilt as a graphology multi-directed graph over node and handle vertices
    Then every handle vertex is owned by exactly one node and every wire joins two handle vertices
    And the graph graphology computes for each applied topological kind equals the graph of the committed after-snapshot
    And a rejected or no-op vector leaves the graph untouched

  @id-kind-compatibility
  @level-long
  @mode-differential
  Scenario: graphology reproduces the kind-compatibility relation and holds it to one edge per ordered pair
    Given every committed vector under asset://🧬️schema/🧬️mutations/🔣️.json
    When the meta relation is rebuilt as a graphology directed graph over kind labels
    Then the relation carries exactly as many distinct ordered pairs as it declares records
    And connect-kind-compatibility and disconnect-kind-compatibility reproduce the committed after-snapshot's relation
    And no other kind moves the relation

  @id-payload-schemas
  @level-long
  @mode-conformance
  Scenario: the npm jsonschema accepts every committed payload against its own leaf schema and rejects an undeclared member
    Given every committed vector under asset://🧬️schema/🧬️mutations/🔣️.json
    When each payload is validated against its leaf 🧬️.schema.json
    Then the committed payload carries its kind's own internally tagged discriminator
    And the validator accepts the committed payload with no error
    And the validator rejects the same payload once a member the schema does not declare is added

  @id-diff-reproduction
  @level-long
  @mode-differential
  Scenario: fast-json-patch reproduces every after-snapshot and holds the typed diff to its operations
    Given every committed vector under asset://🧬️schema/🧬️mutations/🔣️.json
    When an RFC 6902 patch is derived from the before and after snapshots by fast-json-patch
    Then applying that patch to the before-snapshot reproduces the committed after-snapshot exactly
    And a rejected or no-op vector yields no operation
    And the members the patch touches are exactly the members the typed Puzzle2dDiff declares
    And the typed diff's added, removed and patched sets are exactly what fast-json-patch needs operations for
