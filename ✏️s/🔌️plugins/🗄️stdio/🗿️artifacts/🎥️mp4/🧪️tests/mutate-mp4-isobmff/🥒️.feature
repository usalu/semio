@capability-mp4-isobmff-mutate
@oracle-mp4-isobmff-mutate
@comparison-semantic-mp4-mutate-v1
@mutations-mp4-isobmff-any
Feature: Apply every typed ISO-BMFF mutation to a real-world video
  The input is a real ~1.5s, 1200x1080, 47-frame H.264 excerpt of the repository's only real camera
  footage, not a synthetic fixture. Provenance: `ffprobe` confirms the full 16 MB source
  (`♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🌐️public/🎥️bauen-mit-bestand.mp4`) carries exactly
  ONE stream (`codec_type=video`, `codec_name=h264`) and NO audio stream at all — checked directly on
  that file, not assumed from an earlier report. That bounds what `InsertTrack`/`RemoveTrack` can
  exercise here: the only real track available to duplicate or remove is the video track itself, so
  `insert-track` inserts a real second video track (a verbatim structural duplicate of the real
  track, every sample byte-identical to the original), never a fabricated audio track. The excerpt
  was derived ONCE with a real stream copy — no re-encoded pixels, every sample a genuine slice of
  the original encoded bitstream — and committed here rather than reading the 16 MB original via
  `asset://` on every scenario, with the exact derivation command: `ffmpeg -i
  "♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🌐️public/🎥️bauen-mit-bestand.mp4" -t 1.5 -c copy
  -movflags +faststart 🎥️bauen-mit-bestand-ausschnitt.mp4`,
  producing the committed `shared://🎬️.mp4` (2.7 MB, same 1200x1080
  `avc1` stream, `nal_length_size=4`, 47 real B-frame-containing samples with non-zero composition
  offsets).

  Unlike several of this wave's reference crates, `mp4` 0.14 genuinely reads AND writes: every
  mutation below is performed for real by `mp4::Mp4Writer` re-muxing a fresh file from typed tracks
  and samples `mp4::Mp4Reader` read out of the real excerpt — confirmed directly against this exact
  fixture before this feature was written, including the degenerate real case of `remove-track`
  leaving zero tracks, which `mp4` still muxes and re-parses cleanly. Every scenario is therefore
  genuinely `@mode-differential`; §6 of the wave brief (reader-only fallback) does not apply here.

  ⚠️ The fixture's own `stss` box lists exactly TWO sync samples, at 1-based sample ids 1 and 28 —
  read straight out of the committed bytes, not assumed. That is what the `set-sample-sync` row
  addresses: it clears the flag on 0-based sample 27, the second real key frame. The row used to
  name index 2, which is not a key frame, so it set an already-false flag to false and the scenario
  passed without a mutation ever happening. The observability law added in this wave is what
  surfaced it.

  ⚠️ The two halves of the identity scenario assert OPPOSITE byte laws, and both are asserted rather
  than skipped. The oracle half re-muxes with `mp4` 0.14's `Mp4Writer`, a second writer with its own
  box order and `mdat` layout, so it asserts the no-byte-pass-through tripwire. The subject half
  asserts the tripwire's documented mirror, `carrier_is_exact`: `Mp4Snapshot` carries no raw-byte
  escape hatch — every `mvhd`/`tkhd`/`mdhd` field, every edit list entry, the visual sample entry,
  `colr`/`pasp`/`btrt`, the `avcC` extension and the `stsc`/`stco` chunk grouping are typed fields —
  and this repository's `encode_mp4` rebuilds the entire `moov` from them into one deterministic
  normal form (`ftyp`, `moov`, canonical empty `free`, `mdat`) which is already this ffmpeg
  `-c copy -movflags +faststart` fixture's own layout. Demanding that our writer move the bytes
  would demand that a lossless container codec lose something. The artifact holds itself to the
  exact-bytes claim outside this case too, on the FULL recording rather than this excerpt
  (`🚪️io/🦀️component.rs::exact_bauen_mit_bestand_fixture_round_trips_byte_for_byte`), and the ten
  `mutate-*` rows below are what prove a real parse happened: they drive the same decode/encode
  pipeline and every one of them moves both the bytes and the compared projection.

  Every scenario copies the immutable fixture into the case work directory before touching it; the
  committed fixture is never written to. Both the oracle's and the subject's results are read back by
  the SAME independent `mp4`-backed projection (`ftyp`, per-track geometry/codec digest, and every
  sample's duration/composition-time-offset/sync-flag/payload digest) before the
  `semantic-mp4-mutate-v1` profile compares them — never against each other's own writing. H.264
  samples are container-typed and payload-opaque and this format is lossless at the container level,
  so the comparison is exact structural identity, not the lossy raster oracles' bucket/histogram
  approximation; a digest stands in for a sample's raw payload only because a single real sample runs
  tens of kilobytes.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real video
    Given the real input video shared://🎬️.mp4
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                    | params                                                                                                                     |
      | no-mutation           | {}                                                                                                                         |
      | set-snapshot          | {"ftyp":{"majorBrand":"isom","minorVersion":42,"compatibleBrands":["isom","iso2","avc1","mp41"]}}                         |
      | set-ftyp              | {"majorBrand":"mp42","minorVersion":1,"compatibleBrands":["mp42","isom"]}                                                 |
      | insert-track          | {"index":1}                                                                                                                |
      | remove-track          | {"index":0}                                                                                                                |
      | set-track-dimensions  | {"trackIndex":0,"width":640,"height":480}                                                                                 |
      | set-track-codec       | {"trackIndex":0,"sps":[103,66,0,30,140,141,64],"pps":[104,206,60,128]}                                                    |
      | insert-sample         | {"trackIndex":0,"index":10,"sample":{"data":[0,0,0,4,101,1,2,3],"duration":512,"ctsOffset":0,"sync":false}}               |
      | remove-sample         | {"trackIndex":0,"index":10}                                                                                               |
      | set-sample-sync       | {"trackIndex":0,"index":27,"sync":false}                                                                                   |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real video
    Given the real input video shared://🎬️.mp4
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the mutation's inverse is applied
    Then the video is restored to its original semantic projection
    Examples:
      | id                    | params                                                                                                                     |
      | no-mutation           | {}                                                                                                                         |
      | set-snapshot          | {"ftyp":{"majorBrand":"isom","minorVersion":42,"compatibleBrands":["isom","iso2","avc1","mp41"]}}                         |
      | set-ftyp              | {"majorBrand":"mp42","minorVersion":1,"compatibleBrands":["mp42","isom"]}                                                 |
      | insert-track          | {"index":1}                                                                                                                |
      | remove-track          | {"index":0}                                                                                                                |
      | set-track-dimensions  | {"trackIndex":0,"width":640,"height":480}                                                                                 |
      | set-track-codec       | {"trackIndex":0,"sps":[103,66,0,30,140,141,64],"pps":[104,206,60,128]}                                                    |
      | insert-sample         | {"trackIndex":0,"index":10,"sample":{"data":[0,0,0,4,101,1,2,3],"duration":512,"ctsOffset":0,"sync":false}}               |
      | remove-sample         | {"trackIndex":0,"index":10}                                                                                               |
      | set-sample-sync       | {"trackIndex":0,"index":27,"sync":false}                                                                                   |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real video from the typed model alone
    Given the real input video shared://🎬️.mp4
    When the video is decoded to the typed snapshot and re-encoded from it alone
    Then the reference implementation and this repository agree on the result
