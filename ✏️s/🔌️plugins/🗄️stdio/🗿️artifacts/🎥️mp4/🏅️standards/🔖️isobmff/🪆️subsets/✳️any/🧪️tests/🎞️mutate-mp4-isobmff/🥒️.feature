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
  exercise here: the only real track kind available is video, so `insert-track` inserts a second
  VIDEO track, never a fabricated audio track. The excerpt
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

  Every Examples `params` cell is exactly the leaf's wire payload — its `payload_value()`, camelCase,
  no aggregate tag — decoded by the subject through the derive-generated `from_payload_value` and
  read by the `mp4` oracle by the same field names. A row therefore states a whole track or movie
  instead of asking the harness to clone one: `insert-track` inserts, and `set-snapshot` installs
  under its own `ftyp`, the 16x16 two-sample AVC track of this subset's committed `set-snapshot`
  specification vector — a verbatim duplicate of the real track would put 2.7 MB of sample bytes
  into one cell. The oracle refuses what `mp4` 0.14 cannot write (a non-AVC sample entry, more than
  one SPS or PPS) rather than approximating it.

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
      | id | params |
      | set-snapshot | {"snapshot":{"schema":"stdio.mp4","ftyp":{"majorBrand":"isom","minorVersion":42,"compatibleBrands":["isom","iso2","avc1","mp41"]},"movie":{"creationTime":0,"modificationTime":0,"timescale":1000,"duration":2000,"rate":65536,"volume":256,"matrix":[65536,0,0,0,65536,0,0,0,1073741824],"nextTrackId":2,"title":null,"encoder":null},"tracks":[{"trackId":1,"timescale":1000,"codec":{"sps":[[103,66,0,10]],"pps":[[104,206,56,128]],"nalLengthSize":4,"extension":null},"width":16,"height":16,"metadata":{"creationTime":0,"modificationTime":0,"flags":3,"duration":2000,"layer":0,"alternateGroup":0,"volume":0,"matrix":[65536,0,0,0,65536,0,0,0,1073741824],"mediaDuration":2000,"mediaCreationTime":0,"mediaModificationTime":0,"language":"und","quality":0,"handlerName":"VideoHandler","edits":[],"visual":{"dataReferenceIndex":1,"version":0,"revisionLevel":0,"vendor":0,"temporalQuality":0,"spatialQuality":0,"horizontalResolution":4718592,"verticalResolution":4718592,"frameCount":1,"compressorName":"","depth":24,"colorTableId":-1},"color":null,"pixelAspectRatio":null,"bitrate":null},"chunkSampleCounts":[2],"samples":[{"data":[0,0,0,1,101],"duration":1000,"ctsOffset":0,"sync":true},{"data":[0,0,0,1,97],"duration":1000,"ctsOffset":0,"sync":true}]}]}} |
      | set-ftyp | {"ftyp":{"majorBrand":"mp42","minorVersion":1,"compatibleBrands":["mp42","isom"]}} |
      | insert-track | {"index":1,"track":{"trackId":2,"timescale":1000,"codec":{"sps":[[103,66,0,10]],"pps":[[104,206,56,128]],"nalLengthSize":4,"extension":null},"width":16,"height":16,"metadata":{"creationTime":0,"modificationTime":0,"flags":3,"duration":2000,"layer":0,"alternateGroup":0,"volume":0,"matrix":[65536,0,0,0,65536,0,0,0,1073741824],"mediaDuration":2000,"mediaCreationTime":0,"mediaModificationTime":0,"language":"und","quality":0,"handlerName":"VideoHandler","edits":[],"visual":{"dataReferenceIndex":1,"version":0,"revisionLevel":0,"vendor":0,"temporalQuality":0,"spatialQuality":0,"horizontalResolution":4718592,"verticalResolution":4718592,"frameCount":1,"compressorName":"","depth":24,"colorTableId":-1},"color":null,"pixelAspectRatio":null,"bitrate":null},"chunkSampleCounts":[2],"samples":[{"data":[0,0,0,1,101],"duration":1000,"ctsOffset":0,"sync":true},{"data":[0,0,0,1,97],"duration":1000,"ctsOffset":0,"sync":true}]}} |
      | remove-track | {"index":0} |
      | set-track-dimensions | {"trackIndex":0,"width":640,"height":480} |
      | set-track-codec | {"trackIndex":0,"codec":{"sps":[[103,66,0,30,140,141,64]],"pps":[[104,206,60,128]],"nalLengthSize":4,"extension":null}} |
      | insert-sample | {"trackIndex":0,"index":10,"sample":{"data":[0,0,0,4,101,1,2,3],"duration":512,"ctsOffset":0,"sync":false}} |
      | remove-sample | {"trackIndex":0,"index":10} |
      | set-sample-sync | {"trackIndex":0,"index":27,"sync":false} |

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
      | id | params |
      | set-snapshot | {"snapshot":{"schema":"stdio.mp4","ftyp":{"majorBrand":"isom","minorVersion":42,"compatibleBrands":["isom","iso2","avc1","mp41"]},"movie":{"creationTime":0,"modificationTime":0,"timescale":1000,"duration":2000,"rate":65536,"volume":256,"matrix":[65536,0,0,0,65536,0,0,0,1073741824],"nextTrackId":2,"title":null,"encoder":null},"tracks":[{"trackId":1,"timescale":1000,"codec":{"sps":[[103,66,0,10]],"pps":[[104,206,56,128]],"nalLengthSize":4,"extension":null},"width":16,"height":16,"metadata":{"creationTime":0,"modificationTime":0,"flags":3,"duration":2000,"layer":0,"alternateGroup":0,"volume":0,"matrix":[65536,0,0,0,65536,0,0,0,1073741824],"mediaDuration":2000,"mediaCreationTime":0,"mediaModificationTime":0,"language":"und","quality":0,"handlerName":"VideoHandler","edits":[],"visual":{"dataReferenceIndex":1,"version":0,"revisionLevel":0,"vendor":0,"temporalQuality":0,"spatialQuality":0,"horizontalResolution":4718592,"verticalResolution":4718592,"frameCount":1,"compressorName":"","depth":24,"colorTableId":-1},"color":null,"pixelAspectRatio":null,"bitrate":null},"chunkSampleCounts":[2],"samples":[{"data":[0,0,0,1,101],"duration":1000,"ctsOffset":0,"sync":true},{"data":[0,0,0,1,97],"duration":1000,"ctsOffset":0,"sync":true}]}]}} |
      | set-ftyp | {"ftyp":{"majorBrand":"mp42","minorVersion":1,"compatibleBrands":["mp42","isom"]}} |
      | insert-track | {"index":1,"track":{"trackId":2,"timescale":1000,"codec":{"sps":[[103,66,0,10]],"pps":[[104,206,56,128]],"nalLengthSize":4,"extension":null},"width":16,"height":16,"metadata":{"creationTime":0,"modificationTime":0,"flags":3,"duration":2000,"layer":0,"alternateGroup":0,"volume":0,"matrix":[65536,0,0,0,65536,0,0,0,1073741824],"mediaDuration":2000,"mediaCreationTime":0,"mediaModificationTime":0,"language":"und","quality":0,"handlerName":"VideoHandler","edits":[],"visual":{"dataReferenceIndex":1,"version":0,"revisionLevel":0,"vendor":0,"temporalQuality":0,"spatialQuality":0,"horizontalResolution":4718592,"verticalResolution":4718592,"frameCount":1,"compressorName":"","depth":24,"colorTableId":-1},"color":null,"pixelAspectRatio":null,"bitrate":null},"chunkSampleCounts":[2],"samples":[{"data":[0,0,0,1,101],"duration":1000,"ctsOffset":0,"sync":true},{"data":[0,0,0,1,97],"duration":1000,"ctsOffset":0,"sync":true}]}} |
      | remove-track | {"index":0} |
      | set-track-dimensions | {"trackIndex":0,"width":640,"height":480} |
      | set-track-codec | {"trackIndex":0,"codec":{"sps":[[103,66,0,30,140,141,64]],"pps":[[104,206,60,128]],"nalLengthSize":4,"extension":null}} |
      | insert-sample | {"trackIndex":0,"index":10,"sample":{"data":[0,0,0,4,101,1,2,3],"duration":512,"ctsOffset":0,"sync":false}} |
      | remove-sample | {"trackIndex":0,"index":10} |
      | set-sample-sync | {"trackIndex":0,"index":27,"sync":false} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real video from the typed model alone
    Given the real input video shared://🎬️.mp4
    When the video is decoded to the typed snapshot and re-encoded from it alone
    Then the reference implementation and this repository agree on the result
