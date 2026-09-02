@capability-semio-v1-value-mutate
@oracle-semio-value-python-independent
@comparison-ordered-json-v1
@mutations-semio-v1-value
Feature: Apply every typed semio VALUE mutation to a real 424 KB building model, against an independent Python implementation
  `stdio.semio.value` is a semio-NATIVE format: no third party in any ecosystem reads or writes
  `.dsl.semio` or `.pack.semio`, so the second producer a differential comparison needs is a second
  IMPLEMENTATION. `🐍️component.py` beside this file is that implementation — the envelope, the
  recursive `[hex(schema),<value>,[<node>,…]]` DSL grammar, the tag-prefixed `SemioValue` production
  and all nine verbs with their inverses, written in Python from the committed specification
  documents alone
  (`../../🏅️standards/🔖️v1/🪆️subsets/✳️value/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio`,
  `…/📸️snapshot/💾️binary/📡️component.protocol.semio`, `…/🧬️mutations/🔣️.json`, the committed
  `(before, mutation, after)` vectors, and the semio envelope region of
  `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️component.rs`), importing nothing from and
  transliterating nothing of the Rust it judges. It is registered as the oracle
  `semio-value-python-independent` in `…/✳️value/🧪️oracle/🔣️.json`; the recorded no-oracle
  decision it replaces is gone, because there is now a reference to compare against. That decision
  had named `json` 0.12 as the nearest candidate and declined it — a generic JSON DOM has no
  analogue for `Bytes`, `Ref`, lexeme-preserving numbers or the id-keyed node graph, so it could
  never have been the reference for this vocabulary. Python's `json` module is used here for what it
  IS authoritative about: reading the real source document once, which is a different job.

  🌲️ **The document under test is a real one.** The richest `stdio.semio.value` document committed
  anywhere in this artifact is the six-member demo graph, which is a fixture, not a data set. So the
  document this case mutates was derived ONCE — by `🐍️derive-value-fixture.py` in the ticket folder,
  and re-derived on every run by `payload-fidelity` — from the real committed
  `../../../🔣️json/🧫️fixtures/🔣️hexagonal-cut-concrete-forest-left.model.json`, 424 392 bytes of
  `spatial.modelspace` building geometry across four sub-models, 468 vertices, 416 edges and 25
  solids. Its source is committed beside this case as
  `local://🔣️.json`, and the derivation is a faithful
  transcription read by Python's own `json` module with `parse_int`/`parse_float` hooks, so every
  numeric SOURCE LEXEME — `4.44089209850063e-16` included — survives verbatim into `Int`/`Float`
  instead of being rounded through a native numeric type. One documented restructuring: each
  sub-model's `objects` array is lifted into a graph NODE keyed by `<model id>#objects` with a `Ref`
  left where it stood, so the `Ref`/`nodes` layer carries real content while the deep
  `models → model → geometry → vertices → position` tree stays inline where a `SemioValuePath` can
  reach it. The result is 433 262 bytes of DSL and 433 268 of pack, against 211 and 217 for the demo
  graph the case used to rest on — and, for the first time, this case makes a BYTE claim at all,
  which the previous version could not because the subset exported no DSL or pack bridge.

  The `mutate-` and `inverse-` parameters are chosen against the document's own shape, so a plausible
  wrong codec fails: `set-value` walks eight path segments through key and index descent alternately
  to reach one coordinate of one vertex and retypes it, `set-map-entry` overwrites an existing member
  IN PLACE deep inside a sub-model, `remove-map-entry` drops the MIDDLE member of the root map — the
  one case whose undo cannot be a single verb, because `set-map-entry` appends an absent key, so the
  inverse has to remove and rewrite everything that followed it in order — `insert-list-item` and
  `remove-list-item` open and close a hole at the head of a 71-element vertex list, `set-node`
  replaces the value of a graph node the root still refers to, and `remove-node` detaches the last
  node.

  `spec-vector-` keeps the evidence this case rested on before the oracle existed: the committed
  `(before, mutation, after)` vector for each kind — eight beside this case and `set-snapshot`'s
  under its own committed leaf — now applied AND undone by BOTH implementations and checked against
  the committed after- and before-snapshots by each of them in role. Nothing was removed to make room
  for the oracle.

  `payload-fidelity` is the second half of the provenance claim and the one place a second RFC 8259
  reader does the work: the oracle re-reads the committed source with Python's `json` module and
  requires the derived document to still carry exactly what it finds, and the subject re-reads it
  with this repository's own `stdio.json` codec and requires the same. The derivation rule both sides
  implement is stated here and nowhere else, so neither adapter holds a private version of it: an
  object becomes a `map` in member order, an array a `list`, a string a `str`, `true`/`false` a
  `bool`, `null` a `null`, and a number becomes a `float` when its LEXEME carries a `.`, an `e` or an
  `E` and an `int` otherwise — both keeping the source lexeme verbatim. A drifted fixture, or a
  disagreement between the two readers about a number's spelling, is a red scenario rather than a
  silent one.

  `identity-round-trip` carries the BYTE half of the identity law, in both directions. `.dsl.semio`
  is a fixed-layout recursive grammar and `.pack.semio` is the same body under a binary envelope, so
  an exact re-emission is the CORRECT answer here and the wave's must-differ tripwire would be
  backwards, which is why the Rust side asserts `law::carrier_is_exact`. What stops that being a
  codec agreeing with itself is that the demo graph's two encodings were written by the RUST codec
  and the Python side reproduces them byte for byte from the grammar alone, while the building
  model's two encodings were written by the PYTHON implementation and the Rust codec has to reproduce
  THOSE.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real building model
    Given the real building model local://🧪️hexagonal-cut-concrete-forest/🗣️.dsl.semio
    When the <id> mutation is applied to the document parsed from it
      """
      <mutation>
      """
    Then the independent implementation and the subject agree on the resulting document
    Examples:
      | id               | mutation |
      | no-mutation      | {"mutation":"noMutation"} |
      | set-snapshot     | {"mutation":"setSnapshot","snapshot":{"schema":"stdio.semio.value","root":{"kind":"map","entries":[{"key":"schema","value":{"kind":"str","value":"spatial.modelspace"}},{"key":"revision","value":{"kind":"int","lexeme":"5"}},{"key":"models","value":{"kind":"list","items":[{"kind":"ref","id":{"value":"spatial.shape#objects"}}]}}]},"nodes":[{"id":{"value":"spatial.shape#objects"},"value":{"kind":"bytes","value":[222,173,190,239]}}]}} |
      | set-value        | {"mutation":"setValue","path":[{"kind":"key","key":"models"},{"kind":"index","index":0},{"kind":"key","key":"model"},{"kind":"key","key":"geometry"},{"kind":"key","key":"vertices"},{"kind":"index","index":0},{"kind":"key","key":"position"},{"kind":"index","index":1}],"value":{"kind":"int","lexeme":"5"}} |
      | set-map-entry    | {"mutation":"setMapEntry","path":[{"kind":"key","key":"models"},{"kind":"index","index":0},{"kind":"key","key":"model"}],"key":"revision","value":{"kind":"float","lexeme":"1.5"}} |
      | remove-map-entry | {"mutation":"removeMapEntry","path":[],"key":"revision"} |
      | insert-list-item | {"mutation":"insertListItem","path":[{"kind":"key","key":"models"},{"kind":"index","index":0},{"kind":"key","key":"model"},{"kind":"key","key":"geometry"},{"kind":"key","key":"vertices"}],"index":0,"value":{"kind":"str","value":"placeholder"}} |
      | remove-list-item | {"mutation":"removeListItem","path":[{"kind":"key","key":"models"},{"kind":"index","index":0},{"kind":"key","key":"model"},{"kind":"key","key":"geometry"},{"kind":"key","key":"vertices"}],"index":0} |
      | set-node         | {"mutation":"setNode","id":{"value":"aec.building.energy#objects"},"value":{"kind":"bytes","value":[0,1,2,255]}} |
      | remove-node      | {"mutation":"removeNode","id":{"value":"aec.building.structure.classic#objects"}} |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the real building model
    Given the real building model local://🧪️hexagonal-cut-concrete-forest/🗣️.dsl.semio
    When the <id> mutation is applied to the document parsed from it and each side undoes it with its own computed inverse
      """
      <mutation>
      """
    Then both sides restore the building model and agree on the mutated and the restored document
    Examples:
      | id               | mutation |
      | no-mutation      | {"mutation":"noMutation"} |
      | set-snapshot     | {"mutation":"setSnapshot","snapshot":{"schema":"stdio.semio.value","root":{"kind":"map","entries":[{"key":"schema","value":{"kind":"str","value":"spatial.modelspace"}},{"key":"revision","value":{"kind":"int","lexeme":"5"}},{"key":"models","value":{"kind":"list","items":[{"kind":"ref","id":{"value":"spatial.shape#objects"}}]}}]},"nodes":[{"id":{"value":"spatial.shape#objects"},"value":{"kind":"bytes","value":[222,173,190,239]}}]}} |
      | set-value        | {"mutation":"setValue","path":[{"kind":"key","key":"models"},{"kind":"index","index":0},{"kind":"key","key":"model"},{"kind":"key","key":"geometry"},{"kind":"key","key":"vertices"},{"kind":"index","index":0},{"kind":"key","key":"position"},{"kind":"index","index":1}],"value":{"kind":"int","lexeme":"5"}} |
      | set-map-entry    | {"mutation":"setMapEntry","path":[{"kind":"key","key":"models"},{"kind":"index","index":0},{"kind":"key","key":"model"}],"key":"revision","value":{"kind":"float","lexeme":"1.5"}} |
      | remove-map-entry | {"mutation":"removeMapEntry","path":[],"key":"revision"} |
      | insert-list-item | {"mutation":"insertListItem","path":[{"kind":"key","key":"models"},{"kind":"index","index":0},{"kind":"key","key":"model"},{"kind":"key","key":"geometry"},{"kind":"key","key":"vertices"}],"index":0,"value":{"kind":"str","value":"placeholder"}} |
      | remove-list-item | {"mutation":"removeListItem","path":[{"kind":"key","key":"models"},{"kind":"index","index":0},{"kind":"key","key":"model"},{"kind":"key","key":"geometry"},{"kind":"key","key":"vertices"}],"index":0} |
      | set-node         | {"mutation":"setNode","id":{"value":"aec.building.energy#objects"},"value":{"kind":"bytes","value":[0,1,2,255]}} |
      | remove-node      | {"mutation":"removeNode","id":{"value":"aec.building.structure.classic#objects"}} |

  @id-spec-vector
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply and undo <id> on its committed specification vector
    Given the committed before-snapshot <before>
    And the committed mutation payload <mutation>
    And the committed after-snapshot <after>
    When both implementations apply the committed mutation to the committed before-snapshot and undo it again
    Then each reaches the committed after-snapshot, each returns to the before-snapshot, and the two agree
    Examples:
      | id               | before                                                                                                                                                     | mutation                                                                                                                                                | after                                                                                                                                                     |
      | no-mutation      | local://⬅️before/🔣️.json                                                                                                                                      | local://no-mutation.mutation.json                                                                                                                       | local://⬅️before/🔣️.json                                                                                                                                     |
      | set-snapshot     | asset://🏅️standards/🔖️v1/🪆️subsets/✳️value/🧬️schema/🧬️mutations/📄set-snapshot/🧪️tests/retypes-a-map-member-and-repoints-a-graph-node/📸️snapshot/⬅️before/🔣️.json | asset://🏅️standards/🔖️v1/🪆️subsets/✳️value/🧬️schema/🧬️mutations/📄set-snapshot/🧪️tests/retypes-a-map-member-and-repoints-a-graph-node/🦠️mutation/🔣️.json | asset://🏅️standards/🔖️v1/🪆️subsets/✳️value/🧬️schema/🧬️mutations/📄set-snapshot/🧪️tests/retypes-a-map-member-and-repoints-a-graph-node/📸️snapshot/➡️after/🔣️.json |
      | set-value        | local://⬅️before/🔣️.json                                                                                                                                      | local://set-value.mutation.json                                                                                                                         | local://set-value.after.json                                                                                                                              |
      | set-map-entry    | local://⬅️before/🔣️.json                                                                                                                                      | local://set-map-entry.mutation.json                                                                                                                     | local://set-map-entry.after.json                                                                                                                          |
      | remove-map-entry | local://⬅️before/🔣️.json                                                                                                                                      | local://remove-map-entry.mutation.json                                                                                                                  | local://remove-map-entry.after.json                                                                                                                       |
      | insert-list-item | local://⬅️before/🔣️.json                                                                                                                                      | local://insert-list-item.mutation.json                                                                                                                  | local://insert-list-item.after.json                                                                                                                       |
      | remove-list-item | local://⬅️before/🔣️.json                                                                                                                                      | local://remove-list-item.mutation.json                                                                                                                  | local://remove-list-item.after.json                                                                                                                       |
      | set-node         | local://⬅️before/🔣️.json                                                                                                                                      | local://set-node.mutation.json                                                                                                                          | local://set-node.after.json                                                                                                                               |
      | remove-node      | local://⬅️before/🔣️.json                                                                                                                                      | local://remove-node.mutation.json                                                                                                                       | local://remove-node.after.json                                                                                                                            |

  @id-payload-fidelity
  @level-exhaustive
  @mode-differential
  Scenario: The derived building document still carries exactly what the real JSON carries
    Given the real committed source local://🔣️.json
    And the value document derived from it local://🧪️hexagonal-cut-concrete-forest/🗣️.dsl.semio
    When each implementation re-reads the source with its own RFC 8259 parser and rebuilds the document from it
    Then the rebuilt document equals the committed derived document and the two implementations agree on every member

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit both encodings of the demo graph and of the real building model from the parsed documents
    Given the real committed text artifact asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🕸️graph/🖼️assets/🗣️.dsl.semio
    And its committed binary twin asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🕸️graph/🖼️assets/🎒️.pack.semio
    And the real building model local://🧪️hexagonal-cut-concrete-forest/🗣️.dsl.semio
    And its binary twin local://🧪️hexagonal-cut-concrete-forest/🎒️.pack.semio
    When each implementation parses all four files, prints the two documents back and re-encodes both packs
    Then all four files are reproduced byte for byte and the two implementations agree on the documents and on the digests of what they emitted
