@capability-semio-v1-animation-mutate
@oracle-semio-animation-python-independent
@comparison-ordered-json-v1
@mutations-semio-v1-animation
Feature: Apply every typed semio ANIMATION mutation to the real committed walk cycle, against an independent Python implementation
  `s.stdio.semio.animation` is a semio-NATIVE format: no third party in any ecosystem reads or
  writes `.dsl.semio`, so the second producer a differential comparison needs is a second
  IMPLEMENTATION. `🐍️component.py` beside this file is that implementation — the envelope, the DSL
  grammar and all thirteen verbs together with their inverses, written in Python from the committed
  specification documents alone
  (`../../🏅️standards/🔖️v1/🪆️subsets/✳️animation/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio`,
  `…/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio` and the semio envelope in
  `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️component.rs`), importing nothing from and
  transliterating nothing of the Rust it judges. It is registered as the oracle
  `semio-animation-python-independent` in `…/✳️animation/🧪️oracle/🔣️.json`; the recorded
  no-oracle decision it replaces is gone, because there is now a reference to compare against.

  The interchange readers the replaced decision surveyed were rejected on the same merits as before
  and nothing here revives them: `gif` exposes per-frame delays and nothing else, and `pygltflib` /
  `gltf-transform` judge glTF's `channel` + `sampler` indirection while this snapshot resolves the
  same model into owned keyframes — so nine of the thirteen kinds would have had nothing to compare
  against. A from-specification second implementation covers all thirteen.

  The document under test is the REAL committed walk cycle, read where the domain keeps it through
  `asset://` and never written to: one named timeline carrying four channels that between them
  exercise every leaf this subset has — a two-keyframe linearly interpolated `translation` channel
  holding `vec3` values, a cubic-spline `rotation` channel holding a `quat`, a stepped `weights`
  channel holding a three-entry weight vector, and a linear channel on the `custom` property
  `opacity` holding a `scalar`. It is the richest `s.stdio.semio.animation` document committed
  anywhere in this artifact; `asset://` resolves against the artifact root, so no other plugin's
  larger `.dsl.semio` is reachable from here, and that limit is stated rather than papered over.

  The `mutate-` and `inverse-` parameters are chosen against the walk cycle's own shape, so a
  plausible wrong codec fails: `insert-timeline` and `insert-channel` land BEFORE existing entries
  so an append-only implementation fails, `remove-channel` deletes the MIDDLE quaternion channel so
  an undo that re-appends rather than re-inserts fails, `set-timeline-name` clears a `Some` name to
  `None` so an implementation that cannot represent the absent name fails, `set-channel-target`
  overwrites the `custom` property with a unit one so an undo that loses the custom NAME fails,
  `remove-keyframe` deletes the FIRST of two keyframes, `set-keyframe-time` moves a fractional
  stamp, and `set-keyframe-value` replaces a weight vector with a different-valued one of the same
  length so a length-only comparison fails.

  `spec-vector-` keeps the evidence this case rested on before the oracle existed: the committed
  `(before, mutation, after)` vector for each kind in this case's own `🧫️fixtures/`, now applied by
  BOTH implementations and checked against the committed after-snapshot by each of them in role.
  Nothing was removed to make room for the oracle.

  `identity-round-trip` carries the BYTE half of the identity law. `.dsl.semio` is a fixed-layout
  record grammar and the committed file was produced by the Rust codec, so an exact re-emission is
  the CORRECT answer here and the wave's must-differ tripwire would be backwards, which is why the
  Rust side asserts `law::carrier_is_exact`. What stops that being a codec agreeing with itself is
  that the Python side reproduces the same 278 bytes from the grammar alone — including the `{v}`
  Display spelling of every `f64`, which prints `0` and `0.5` differently — and the two sides'
  digests of the re-emitted bytes are compared. `✳️animation` exports no pack bridge, so the
  committed `🎒️.pack.semio` twin is NOT read by either side and no claim is made about it —
  one carrier, measured, and the other named as unmeasured.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real committed walk cycle
    Given the real committed animation artifact asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🚶️walk/🖼️assets/🗣️example.dsl.semio
    When the <id> mutation is applied to the walk cycle parsed from it
      """
      <mutation>
      """
    Then the independent implementation and the subject agree on the resulting snapshot
    Examples:
      | id                        | mutation                                                                                                                                                                                                                                                                                    |
      | no-mutation               | {"kind":"no-mutation","params":{}}                                                                                                                                                                                                                                                          |
      | set-snapshot              | {"kind":"set-snapshot","params":{"snapshot":{"schema":"s.stdio.semio.animation","timelines":[{"name":null,"channels":[{"target":{"node":"root","property":{"kind":"translation"}},"interpolation":"linear","keyframes":[{"t":0,"value":{"kind":"vec3","value":{"x":0,"y":0,"z":0}}}]}]}]}}} |
      | insert-timeline           | {"kind":"insert-timeline","params":{"index":0,"timeline":{"name":"wave","channels":[]}}}                                                                                                                                                                                                    |
      | remove-timeline           | {"kind":"remove-timeline","params":{"index":0}}                                                                                                                                                                                                                                             |
      | set-timeline-name         | {"kind":"set-timeline-name","params":{"index":0,"name":null}}                                                                                                                                                                                                                               |
      | insert-channel            | {"kind":"insert-channel","params":{"timelineIndex":0,"index":2,"channel":{"target":{"node":"knee","property":{"kind":"scale"}},"interpolation":"step","keyframes":[{"t":0,"value":{"kind":"vec3","value":{"x":1,"y":1,"z":1}}}]}}}                                                          |
      | remove-channel            | {"kind":"remove-channel","params":{"timelineIndex":0,"index":1}}                                                                                                                                                                                                                            |
      | set-channel-target        | {"kind":"set-channel-target","params":{"timelineIndex":0,"index":3,"target":{"node":"pelvis","property":{"kind":"scale"}}}}                                                                                                                                                                 |
      | set-channel-interpolation | {"kind":"set-channel-interpolation","params":{"timelineIndex":0,"index":1,"interpolation":"step"}}                                                                                                                                                                                          |
      | insert-keyframe           | {"kind":"insert-keyframe","params":{"timelineIndex":0,"channelIndex":0,"index":1,"keyframe":{"t":0.5,"value":{"kind":"vec3","value":{"x":0.5,"y":0,"z":0}}}}}                                                                                                                               |
      | remove-keyframe           | {"kind":"remove-keyframe","params":{"timelineIndex":0,"channelIndex":0,"index":0}}                                                                                                                                                                                                          |
      | set-keyframe-time         | {"kind":"set-keyframe-time","params":{"timelineIndex":0,"channelIndex":1,"index":0,"t":0.25}}                                                                                                                                                                                               |
      | set-keyframe-value        | {"kind":"set-keyframe-value","params":{"timelineIndex":0,"channelIndex":2,"index":0,"value":{"kind":"weights","values":[0.25,0.5,0.75]}}}                                                                                                                                                   |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the real committed walk cycle
    Given the real committed animation artifact asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🚶️walk/🖼️assets/🗣️example.dsl.semio
    When the <id> mutation is applied to the walk cycle parsed from it and each side undoes it with its own computed inverse
      """
      <mutation>
      """
    Then both sides restore the walk cycle and agree on the mutated and the restored snapshot
    Examples:
      | id                        | mutation                                                                                                                                                                                                                                                                                    |
      | no-mutation               | {"kind":"no-mutation","params":{}}                                                                                                                                                                                                                                                          |
      | set-snapshot              | {"kind":"set-snapshot","params":{"snapshot":{"schema":"s.stdio.semio.animation","timelines":[{"name":null,"channels":[{"target":{"node":"root","property":{"kind":"translation"}},"interpolation":"linear","keyframes":[{"t":0,"value":{"kind":"vec3","value":{"x":0,"y":0,"z":0}}}]}]}]}}} |
      | insert-timeline           | {"kind":"insert-timeline","params":{"index":0,"timeline":{"name":"wave","channels":[]}}}                                                                                                                                                                                                    |
      | remove-timeline           | {"kind":"remove-timeline","params":{"index":0}}                                                                                                                                                                                                                                             |
      | set-timeline-name         | {"kind":"set-timeline-name","params":{"index":0,"name":null}}                                                                                                                                                                                                                               |
      | insert-channel            | {"kind":"insert-channel","params":{"timelineIndex":0,"index":2,"channel":{"target":{"node":"knee","property":{"kind":"scale"}},"interpolation":"step","keyframes":[{"t":0,"value":{"kind":"vec3","value":{"x":1,"y":1,"z":1}}}]}}}                                                          |
      | remove-channel            | {"kind":"remove-channel","params":{"timelineIndex":0,"index":1}}                                                                                                                                                                                                                            |
      | set-channel-target        | {"kind":"set-channel-target","params":{"timelineIndex":0,"index":3,"target":{"node":"pelvis","property":{"kind":"scale"}}}}                                                                                                                                                                 |
      | set-channel-interpolation | {"kind":"set-channel-interpolation","params":{"timelineIndex":0,"index":1,"interpolation":"step"}}                                                                                                                                                                                          |
      | insert-keyframe           | {"kind":"insert-keyframe","params":{"timelineIndex":0,"channelIndex":0,"index":1,"keyframe":{"t":0.5,"value":{"kind":"vec3","value":{"x":0.5,"y":0,"z":0}}}}}                                                                                                                               |
      | remove-keyframe           | {"kind":"remove-keyframe","params":{"timelineIndex":0,"channelIndex":0,"index":0}}                                                                                                                                                                                                          |
      | set-keyframe-time         | {"kind":"set-keyframe-time","params":{"timelineIndex":0,"channelIndex":1,"index":0,"t":0.25}}                                                                                                                                                                                               |
      | set-keyframe-value        | {"kind":"set-keyframe-value","params":{"timelineIndex":0,"channelIndex":2,"index":0,"value":{"kind":"weights","values":[0.25,0.5,0.75]}}}                                                                                                                                                   |

  @id-spec-vector
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to its committed specification vector
    Given the committed specification vector local://🦠️<id>.json for the <id> kind
    When both implementations apply the vector's mutation to its before-snapshot
    Then each reaches the committed after-snapshot and the two agree
    Examples:
      | id                        |
      | no-mutation               |
      | set-snapshot              |
      | insert-timeline           |
      | remove-timeline           |
      | set-timeline-name         |
      | insert-channel            |
      | remove-channel            |
      | set-channel-target        |
      | set-channel-interpolation |
      | insert-keyframe           |
      | remove-keyframe           |
      | set-keyframe-time         |
      | set-keyframe-value        |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the committed encoding of the real walk cycle from the parsed snapshot
    Given the real committed animation artifact asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🚶️walk/🖼️assets/🗣️example.dsl.semio
    And the committed specification vector local://🦠️no-mutation.json whose before-snapshot is that artifact decoded
    When each implementation parses the artifact, prints it back and parses the printed text again
    Then both reproduce the committed file byte for byte and agree on the walk cycle and on the digest of what they emitted
