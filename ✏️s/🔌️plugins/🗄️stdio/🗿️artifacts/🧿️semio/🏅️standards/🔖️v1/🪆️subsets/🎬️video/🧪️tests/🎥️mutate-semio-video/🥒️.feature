@capability-semio-v1-video-mutate
@oracle-semio-video-python-independent
@comparison-ordered-json-v1
@mutations-semio-v1-video
Feature: Apply every typed semio VIDEO mutation to a real recording, against an independent Python implementation
  `s.stdio.semio.video` is a semio-NATIVE format: no third party in any ecosystem reads or writes
  `.dsl.semio`, so the second producer a differential comparison needs is a second IMPLEMENTATION.
  `🐍️component.py` beside this file is that implementation — the envelope, the DSL grammar and all
  nine verbs together with their inverses, written in Python from the committed specification
  documents alone (`../../🏅️standards/🔖️v1/🪆️subsets/🎬️video/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio`,
  `…/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio` and the semio envelope in
  `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs`), importing nothing from and
  transliterating nothing of the Rust it judges. It is registered as the oracle
  `semio-video-python-independent` in `…/🎬️video/🔮️oracle/🔣️.json`; the recorded no-oracle
  decision it replaces is gone, because there is now a reference to compare against.

  The container libraries the replaced decision surveyed were rejected on the same merits as before
  and nothing here revives them: `mp4`, `riff`, `pymp4` and `mp4box.js` read and write CONTAINERS,
  while this snapshot is deliberately container-neutral and payload-OPAQUE — a sample's `data` is an
  uninterpreted byte run, a stream's `rate` an exact rational rather than a timescale/duration pair,
  and a stream carries no track id, no edit list and no codec-configuration record. Projecting these
  verbs through a container write would invent fields the snapshot does not carry. A
  from-specification second implementation judges the vocabulary that actually exists.

  🎬️ **The document under test is a real recording.** The richest `s.stdio.semio.video` document
  committed anywhere in this artifact is the two-stream two-sample demo clip — 172 bytes, which is a
  fixture, not a recording. So the document every mutation row below runs on was derived ONCE — by
  `🐍️derive-video-fixture.py` in the ticket folder — from two real committed recordings of the SAME
  real source, the "Bauen mit Bestand" presentation excerpt. The `V` stream is the real
  `../../../📼️avi/🧫️fixtures/📼️bauen-mit-bestand-mjpeg.avi`, read with a purpose-written RIFF/AVI
  reader: its real `strh` four-cc `MJPG`, its real `avih` frame size 480×432, its real 15/1
  scale-rate pair and the first eight real `00dc` frame chunks of the real `movi` list, each carrying
  its real JPEG bytes — 9 345 to 12 400 bytes apiece — and a real timestamp in the file's own frame
  ticks. The `A` stream is the real
  `../../../🎵️mp3/🧫️fixtures/🎵️bauen-mit-bestand-ausschnitt.mp3`, read with a purpose-written MPEG-1
  Layer III frame reader: its real 44 100 Hz sample rate and its first twenty-four real audio frames,
  each carrying its real frame bytes and a real timestamp in the layer's own 1 152-sample granule.
  The result is 220 106 bytes, against 172 for the clip the case used to rest on. Both the `V` and
  the `A` arm of the `kind` vocabulary are therefore carried by the artifact itself, as before, and
  every MJPEG frame is a key frame by construction of the codec, which is what the `key` flag
  records. Neither reader speaks a semio envelope, which is why they are the source of the ARTIFACT
  and never the oracle.

  The `mutate-` and `inverse-` parameters are chosen against the recording's own shape, so a
  plausible wrong codec fails: `insert-stream` puts a subtitle track AHEAD of the video one so an
  append-only implementation fails, `remove-stream` deletes the FIRST track — the eight-frame video
  one, so its inverse has to put all eight real JPEG payloads back in order — `insert-sample` lands
  in the MIDDLE of the audio track's twenty-four frames, `remove-sample` deletes the video track's
  first real frame, `set-sample-data` rewrites the fifth real frame's opaque payload without touching
  its stamps, and `set-sample-flags` clears the key flag on the first real frame while moving its
  stamp, so an implementation that writes only one of the two fails.

  `spec-vector-` keeps the evidence this case rested on before the oracle existed: the committed
  `(before, mutation, after)` vector for each kind in this case's own `🧫️fixtures/`, now applied by
  BOTH implementations and checked against the committed after-snapshot by each of them in role.
  Nothing was removed to make room for the oracle.

  `identity-round-trip` carries the BYTE half of the identity law, in both directions, over BOTH
  documents. `.dsl.semio` is a fixed-layout record grammar, so an exact re-emission is the CORRECT
  answer here and the wave's must-differ tripwire would be backwards, which is why the Rust side
  asserts `law::carrier_is_exact`. The clip's 172 bytes were produced by the RUST codec and the
  Python side reproduces them from the grammar alone — the clip is kept for exactly that reason, and
  because it is also the before-snapshot every committed specification vector starts from, a tie both
  sides still assert — while the recording's 220 106 bytes were produced by the PYTHON implementation
  and the Rust codec has to reproduce THOSE, 32 real coded frames among them. `🎬️video` exports no
  pack bridge, so no `.pack.semio` twin is read by either side and no claim is made about one — one
  carrier, measured, and the other named as unmeasured.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real recording
    Given the real recording local://🏚️bauen-mit-bestand-ausschnitt/🗣️.dsl.semio
    When the <id> mutation is applied to the recording parsed from it
      """
      <mutation>
      """
    Then the independent implementation and the subject agree on the resulting snapshot
    Examples:
      | id               | mutation |
      | set-snapshot     | {"kind":"set-snapshot","params":{"snapshot":{"schema":"stdio.semio.video","streams":[{"kind":"S","codec":"srt","width":0,"height":0,"rate":{"num":1,"den":1},"samples":[{"pts":0,"key":true,"data":"4261756572"}]}]}}} |
      | insert-stream    | {"kind":"insert-stream","params":{"index":0,"stream":{"kind":"S","codec":"srt","width":0,"height":0,"rate":{"num":1,"den":1},"samples":[]}}} |
      | remove-stream    | {"kind":"remove-stream","params":{"index":0}} |
      | set-stream-meta  | {"kind":"set-stream-meta","params":{"index":0,"kind":"V","codec":"vp9","width":1280,"height":720,"rate":{"num":60,"den":1}}} |
      | insert-sample    | {"kind":"insert-sample","params":{"streamIndex":1,"index":12,"sample":{"pts":13248,"key":true,"data":"fffbe044"}}} |
      | remove-sample    | {"kind":"remove-sample","params":{"streamIndex":0,"index":0}} |
      | set-sample-data  | {"kind":"set-sample-data","params":{"streamIndex":0,"index":4,"data":"ffd8ffe000104a464946"}} |
      | set-sample-flags | {"kind":"set-sample-flags","params":{"streamIndex":0,"index":0,"pts":500,"key":false}} |

  @id-no-mutation-baseline-mutate
  @level-exhaustive
  @mode-differential
  Scenario: Apply no-mutation to the real recording
    Given the real recording local://🏚️bauen-mit-bestand-ausschnitt/🗣️.dsl.semio
    When the no-mutation mutation is applied to the recording parsed from it
      """
      {"kind":"no-mutation","params":{}}
      """
    Then the independent implementation and the subject agree on the resulting snapshot

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the real recording
    Given the real recording local://🏚️bauen-mit-bestand-ausschnitt/🗣️.dsl.semio
    When the <id> mutation is applied to the recording parsed from it and each side undoes it with its own computed inverse
      """
      <mutation>
      """
    Then both sides restore the recording and agree on the mutated and the restored snapshot
    Examples:
      | id               | mutation |
      | set-snapshot     | {"kind":"set-snapshot","params":{"snapshot":{"schema":"stdio.semio.video","streams":[{"kind":"S","codec":"srt","width":0,"height":0,"rate":{"num":1,"den":1},"samples":[{"pts":0,"key":true,"data":"4261756572"}]}]}}} |
      | insert-stream    | {"kind":"insert-stream","params":{"index":0,"stream":{"kind":"S","codec":"srt","width":0,"height":0,"rate":{"num":1,"den":1},"samples":[]}}} |
      | remove-stream    | {"kind":"remove-stream","params":{"index":0}} |
      | set-stream-meta  | {"kind":"set-stream-meta","params":{"index":0,"kind":"V","codec":"vp9","width":1280,"height":720,"rate":{"num":60,"den":1}}} |
      | insert-sample    | {"kind":"insert-sample","params":{"streamIndex":1,"index":12,"sample":{"pts":13248,"key":true,"data":"fffbe044"}}} |
      | remove-sample    | {"kind":"remove-sample","params":{"streamIndex":0,"index":0}} |
      | set-sample-data  | {"kind":"set-sample-data","params":{"streamIndex":0,"index":4,"data":"ffd8ffe000104a464946"}} |
      | set-sample-flags | {"kind":"set-sample-flags","params":{"streamIndex":0,"index":0,"pts":500,"key":false}} |

  @id-no-mutation-baseline-inverse
  @level-exhaustive
  @mode-differential
  Scenario: Undoing no-mutation restores the real recording
    Given the real recording local://🏚️bauen-mit-bestand-ausschnitt/🗣️.dsl.semio
    When the no-mutation mutation is applied to the recording parsed from it and each side undoes it with its own computed inverse
      """
      {"kind":"no-mutation","params":{}}
      """
    Then both sides restore the recording and agree on the mutated and the restored snapshot

  @id-spec-vector
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to its committed specification vector
    Given the committed specification vector local://<fixture>/🦠️mutation/🔣️.json for the <id> kind
    When both implementations apply the vector's mutation to its before-snapshot
    Then each reaches the committed after-snapshot and the two agree
    Examples:
      | id | fixture |
      | no-mutation | ⏸️no-mutation |
      | set-snapshot | 📸️set-snapshot |
      | insert-stream | 🎥️insert-stream |
      | remove-stream | 🗑️remove-stream |
      | set-stream-meta | 📋️set-stream-meta |
      | insert-sample | ➕️insert-sample |
      | remove-sample | 🚮️remove-sample |
      | set-sample-data | 📀️set-sample-data |
      | set-sample-flags | 🚩️set-sample-flags |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the committed encodings of the demo clip and of the real recording
    Given the real committed video artifact asset://📚️examples/🎥️clip/🖼️assets/🗣️.dsl.semio
    And the committed specification vector local://⏸️no-mutation/🦠️mutation/🔣️.json whose before-snapshot is that artifact decoded
    And the real recording local://🏚️bauen-mit-bestand-ausschnitt/🗣️.dsl.semio
    When each implementation parses both artifacts, prints them back and parses the printed text again
    Then both reproduce the two files byte for byte and agree on both documents and on the digests of what they emitted
