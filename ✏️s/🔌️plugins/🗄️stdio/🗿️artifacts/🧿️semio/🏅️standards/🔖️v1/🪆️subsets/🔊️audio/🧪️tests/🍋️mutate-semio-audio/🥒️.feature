@capability-semio-v1-audio-mutate
@oracle-semio-audio-python-independent
@comparison-ordered-json-v1
@mutations-semio-v1-audio
Feature: Apply every typed semio AUDIO mutation to a real recording, against an independent Python implementation
  `s.stdio.semio.audio` is a semio-NATIVE format: no third party in any ecosystem reads or writes
  `.dsl.semio`, so the second producer a differential comparison needs is a second IMPLEMENTATION.
  `🐍️component.py` beside this file is that implementation — the envelope, the DSL grammar and all
  ten verbs together with their inverses, written in Python from the committed specification
  documents alone (`../../🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio`,
  `…/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio` and the semio envelope in
  `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️component.rs`), importing nothing from and
  transliterating nothing of the Rust it judges. It is registered as the oracle
  `semio-audio-python-independent` in `…/🔊️audio/🔮️oracle/🔣️.json`; the recorded no-oracle
  decision it replaces is gone, because there is now a reference to compare against.

  The candidate third-party libraries the replaced decision surveyed were rejected on the same
  merits as before and nothing here revives them: `hound`, `soundfile` and `node-wav` speak WAV
  containers, which model no `LIST INFO` chunk and therefore strand `insert-tag`, `remove-tag` and
  `set-tag-value` with nothing to compare against, and `SemioAudioFormat` records the ORIGINAL
  encoding the always-`f32` samples were decoded FROM, which no WAV writer can represent
  independently of its own storage. A from-specification second implementation covers all ten kinds
  instead of four, which is why that route was taken.

  🎤️ **The document under test is a real recording.** The richest `s.stdio.semio.audio` document
  committed anywhere in this artifact is the two-channel four-sample reference tone — 226 bytes,
  which is a fixture, not a recording. So the document every mutation row below runs on was derived
  ONCE — by `🐍️derive-audio-fixture.py` in the ticket folder — from two real committed recordings of
  the SAME real source, the "Bauen mit Bestand" presentation excerpt: the first real second of
  `../../../🔊️wav/🧫️fixtures/🔊️bauen-mit-bestand-ausschnitt.wav`, read with Python's own stdlib
  `wave` module — 8 000 real 16-bit PCM frames at the file's own 8 000 Hz, each scaled by 2⁻¹⁵, which
  is exact in binary32, with no resampling and no filtering — and the four real ID3v2.3 frames
  (`TSSE`, `TIT2`, `TPE1`, `TLEN`) of
  `../../../🎵️mp3/🧫️fixtures/🎵️bauen-mit-bestand-ausschnitt.mp3`, in the file's own frame order. The
  declared `format` is the wav's own `pcm16`, an arm no committed example carried. The result is
  72 341 bytes, against 226 for the tone the case used to rest on. Neither reader speaks a semio
  envelope, which is why they are the source of the ARTIFACT and never the oracle.

  A limit of the source, stated rather than papered over: the committed recording is MONO, so the
  document carries one channel where the tone carried two. `remove-channel` therefore empties the
  channel pool rather than deleting one of two, `set-channel-samples` addresses channel 0 — the real
  8 000-sample track, which it replaces with four of its own real samples, so the inverse has to put
  all 8 000 back — and `insert-channel` still lands at index 1, appending beside the real track. No
  stereo image was fabricated to keep the old indices.

  The remaining `mutate-` and `inverse-` parameters are chosen against the recording's own shape, so
  a plausible wrong codec fails: `insert-tag` puts a tag AHEAD of the real `TSSE` frame so an append
  fails, `remove-tag` deletes that real first frame, `set-tag-value` rewrites the real `TIT2` title
  at index 1 so a write that reached the wrong tag fails, `set-sample-rate` moves off the file's own
  8 000 Hz, and `set-format` moves to `pcm24` — a width the samples themselves cannot betray, since
  they stay `f32` in the model, so an implementation that inferred the format from the payload fails.

  `spec-vector-` keeps the evidence this case rested on before the oracle existed: the committed
  `(before, mutation, after)` vector for each kind in this case's own `🧫️fixtures/`, now applied by
  BOTH implementations and checked against the committed after-snapshot by each of them in role.
  Nothing was removed to make room for the oracle.

  `identity-round-trip` carries the BYTE half of the identity law, in both directions, over BOTH
  documents. `.dsl.semio` is a fixed-layout record grammar, so an exact re-emission is the CORRECT
  answer here and the wave's must-differ tripwire would be backwards, which is why the Rust side
  asserts `law::carrier_is_exact`. The tone's 226 bytes were produced by the RUST codec and the
  Python side reproduces them from the grammar alone — the tone is kept for exactly that reason, and
  because it is also the before-snapshot every committed specification vector starts from, a tie
  both sides still assert — while the recording's 72 341 bytes were produced by the PYTHON
  implementation and the Rust codec has to reproduce THOSE, 8 000 real binary32 samples among them.
  `🔊️audio` exports no pack bridge, so no `.pack.semio` twin is read by either side and no claim is
  made about one — one carrier, measured, and the other named as unmeasured.

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
      | id                  | mutation |
      | set-snapshot        | {"kind":"set-snapshot","params":{"snapshot":{"schema":"stdio.semio.audio","sampleRate":44100,"format":"f32","channels":[{"samples":[0.0,1.0]}],"tags":[{"key":"TIT2","value":"Bauen mit Bestand (Ausschnitt)"}]}}} |
      | set-sample-rate     | {"kind":"set-sample-rate","params":{"sampleRate":48000}} |
      | set-format          | {"kind":"set-format","params":{"format":"pcm24"}} |
      | insert-channel      | {"kind":"insert-channel","params":{"index":1,"channel":{"samples":[-1.0,-0.8359375,-0.71875,-0.71875]}}} |
      | remove-channel      | {"kind":"remove-channel","params":{"index":0}} |
      | set-channel-samples | {"kind":"set-channel-samples","params":{"index":0,"samples":[-1.0,-0.8359375,-0.71875,-0.71875]}} |
      | insert-tag          | {"kind":"insert-tag","params":{"index":0,"tag":{"key":"TALB","value":"33. Projektetage"}}} |
      | remove-tag          | {"kind":"remove-tag","params":{"index":0}} |
      | set-tag-value       | {"kind":"set-tag-value","params":{"index":1,"value":"Bauen mit Bestand, Ausschnitt 1"}} |

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
      | id                  | mutation |
      | set-snapshot        | {"kind":"set-snapshot","params":{"snapshot":{"schema":"stdio.semio.audio","sampleRate":44100,"format":"f32","channels":[{"samples":[0.0,1.0]}],"tags":[{"key":"TIT2","value":"Bauen mit Bestand (Ausschnitt)"}]}}} |
      | set-sample-rate     | {"kind":"set-sample-rate","params":{"sampleRate":48000}} |
      | set-format          | {"kind":"set-format","params":{"format":"pcm24"}} |
      | insert-channel      | {"kind":"insert-channel","params":{"index":1,"channel":{"samples":[-1.0,-0.8359375,-0.71875,-0.71875]}}} |
      | remove-channel      | {"kind":"remove-channel","params":{"index":0}} |
      | set-channel-samples | {"kind":"set-channel-samples","params":{"index":0,"samples":[-1.0,-0.8359375,-0.71875,-0.71875]}} |
      | insert-tag          | {"kind":"insert-tag","params":{"index":0,"tag":{"key":"TALB","value":"33. Projektetage"}}} |
      | remove-tag          | {"kind":"remove-tag","params":{"index":0}} |
      | set-tag-value       | {"kind":"set-tag-value","params":{"index":1,"value":"Bauen mit Bestand, Ausschnitt 1"}} |

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
    Given the committed specification vector local://🧫️<id>/🦠️mutation/🔣️.json for the <id> kind
    When both implementations apply the vector's mutation to its before-snapshot
    Then each reaches the committed after-snapshot and the two agree
    Examples:
      | id                  |
      | no-mutation         |
      | set-snapshot        |
      | set-sample-rate     |
      | set-format          |
      | insert-channel      |
      | remove-channel      |
      | set-channel-samples |
      | insert-tag          |
      | remove-tag          |
      | set-tag-value       |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the committed encodings of the reference tone and of the real recording
    Given the real committed audio artifact asset://📚️examples/🎵️tone/🖼️assets/🗣️.dsl.semio
    And the committed specification vector local://⏸️no-mutation/🦠️mutation/🔣️.json whose before-snapshot is that artifact decoded
    And the real recording local://🏚️bauen-mit-bestand-ausschnitt/🗣️.dsl.semio
    When each implementation parses both artifacts, prints them back and parses the printed text again
    Then both reproduce the two files byte for byte and agree on both documents and on the digests of what they emitted
