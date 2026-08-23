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
  producing the committed `shared://🎥️bauen-mit-bestand-ausschnitt.mp4` (2.7 MB, same 1200x1080
  `avc1` stream, `nal_length_size=4`, 47 real B-frame-containing samples with non-zero composition
  offsets).

  Unlike several of this wave's reference crates, `mp4` 0.14 genuinely reads AND writes: every
  mutation below is performed for real by `mp4::Mp4Writer` re-muxing a fresh file from typed tracks
  and samples `mp4::Mp4Reader` read out of the real excerpt — confirmed directly against this exact
  fixture before this feature was written, including the degenerate real case of `remove-track`
  leaving zero tracks, which `mp4` still muxes and re-parses cleanly. Every scenario is therefore
  genuinely `@mode-differential`; §6 of the wave brief (reader-only fallback) does not apply here.

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
    Given the real input video shared://🎥️bauen-mit-bestand-ausschnitt.mp4
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
      | set-sample-sync       | {"trackIndex":0,"index":2,"sync":false}                                                                                   |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real video
    Given the real input video shared://🎥️bauen-mit-bestand-ausschnitt.mp4
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
      | set-sample-sync       | {"trackIndex":0,"index":2,"sync":false}                                                                                   |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real video without passing bytes through
    Given the real input video shared://🎥️bauen-mit-bestand-ausschnitt.mp4
    When the video is decoded to the typed snapshot and re-encoded from it alone
    Then the reference implementation and this repository agree on the result
