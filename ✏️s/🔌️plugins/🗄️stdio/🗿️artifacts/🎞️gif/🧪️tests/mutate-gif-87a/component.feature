@capability-gif-87a-mutate
@oracle-gif-87a-mutate
@comparison-semantic-raster-v1
@mutations-gif-87a-any
Feature: Apply every typed GIF87a mutation to a real derived document
  The input is a genuine GIF87a file — magic bytes literally "GIF87a", zero Graphic Control
  Extensions — because this subset's own vocabulary has no frame delay, disposal, transparency,
  comment or application-extension concept: GIF87a itself has none of those. It was derived ONCE,
  not fabricated, from the real animated ../../🏅️standards/🔖️87a/🪆️subsets/✳️any/📚️examples/💃️dancing/🖼️assets/🖼️dancing.gif
  fixture (itself a real GIF89a file mis-filed under the 87a example directory — see the ticket
  writeup for that finding) using the `gif` reference crate: three of its real frames (indices 0,
  20 and 40) were decoded with the crate's own indexed-colour reader, a genuine 16x16 rectangle of
  real, already-decoded palette indices was cropped out of each, and frame 0's real 256-colour
  local table was promoted to this file's own Global Color Table so a fourth, inserted image can
  reference it without carrying its own. Every scenario copies the immutable fixture into the case
  work directory before touching it; the fixture itself is never written to.

  All twelve declared kinds move the compared projection, and the oracle FAILS any scenario whose
  kind leaves it untouched — there is no exemption list. Getting there took three fixes to what the
  reference crate hands back, each recorded against its source in the subset's own oracle module:

    – `gif::Encoder`'s `write_screen_desc` emits `b"GIF89a"`, then a hard-coded `0` background index
      and a hard-coded `0` pixel-aspect-ratio, and offers no setter for any of them. All three are
      patched back onto its output at their fixed Logical Screen Descriptor offsets. Before that,
      set-background-color-index and set-pixel-aspect-ratio were accepted and silently discarded.
    – `gif::Decoder` de-interlaces every image on read and then reports `Frame::interlaced` as false
      regardless of what the file said, so the interlace flag is read off the Image Descriptor's own
      packed byte instead. Trusting the decoder made set-image-interlace invisible: the round trip
      erased both the flag and the row permutation and landed back where it started.
    – The projection is this subset's own, not the shared raster one, which reports screen geometry,
      per-frame rectangles and an opaque-sample count only — leaving the Global Color Table, both
      screen scalars, the interlace flag and the raw index buffers outside the compared surface.
    – `gif::Encoder::new` documents that an empty global palette means no Global Color Table, then
      `write_global_palette` sets the table-present flag unconditionally and writes two padded
      all-zero entries (gif 0.14.2 `src/encoder.rs:183-195`, `303-311`). `oracle_encode` clears the
      flag and drops those six bytes; before that, `set-snapshot {"gct": null}` came back with a
      phantom two-colour table and diverged from a subject that correctly wrote none.

  ⚠️ KNOWN OPEN DIVERGENCE — `mutate-set-global-color-table` (parity 24/25, 2026-08-24). The row
  installs a four-colour Global Color Table; image 0 carries no Local Color Table and its indices
  run to 255, so after the edit those pixels index past the active table. The reference writer emits
  the file anyway and derives the LZW minimum code size from the maximum index actually present
  (`gif` 0.14.2 `src/encoder.rs:448-450`, matching GIF89a Appendix F "the minimum number of bits
  required to represent the set of actual pixel values"). This repository's `encode_gif` refuses,
  and `encode_gif_rejects_index_past_color_table`
  (`../../🏅️standards/🔖️87a/🪆️subsets/✳️any/🚪️io/🦀️component.rs`) pins that refusal as deliberate.
  Neither side is a coding error: the declared kind says "replace the global colour table" and says
  nothing about the images that resolve through it, so both sides are extrapolating. Resolving it is
  an owner decision on what a table-shrinking edit means — either the codec writes out-of-range
  indices and sizes LZW from the data (which means retiring that pinned test), or the vocabulary
  refuses a shrink that orphans indices and the oracle mirrors that refusal. NOTHING here is to be
  relaxed to make the row pass: not the profile, not the row's parameters, not the fixture.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real document
    Given the real GIF87a input document shared://🖼️dancing-87a.gif
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id | params |
      | no-mutation | {} |
      | set-snapshot | {"snapshot":{"width":4,"height":4,"gct":null,"backgroundColorIndex":0,"images":[{"left":0,"top":0,"width":4,"height":4,"interlace":false,"lct":{"sorted":false,"colors":[{"r":50,"g":205,"b":50},{"r":50,"g":205,"b":50}]},"indices":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]}]}} |
      | set-screen-size | {"width":20,"height":20} |
      | set-global-color-table | {"gct":{"colors":[{"r":193,"g":108,"b":82},{"r":206,"g":161,"b":101},{"r":217,"g":120,"b":137},{"r":193,"g":108,"b":82}]}} |
      | set-background-color-index | {"index":1} |
      | set-pixel-aspect-ratio | {"ratio":2} |
      | insert-image | {"index":1,"image":{"left":0,"top":0,"width":4,"height":4,"interlace":false,"lct":null,"indices":[74,74,74,73,16,74,74,75,16,74,75,75,16,14,75,75]}} |
      | remove-image | {"index":1} |
      | move-image | {"from":0,"to":2} |
      | set-image-geometry | {"index":0,"left":2,"top":2,"width":16,"height":16} |
      | set-image-pixels | {"index":0,"indices":[15,15,17,17,17,143,254,254,128,128,135,131,131,133,133,133,15,15,17,17,70,221,254,254,128,128,131,131,131,131,61,61,15,15,15,17,254,254,254,127,128,128,131,131,131,61,61,61,15,15,15,146,221,220,126,127,128,129,131,131,131,61,61,61,15,16,16,254,221,220,126,127,128,129,130,131,64,47,47,47,72,16,146,223,220,125,126,127,129,47,47,31,31,32,32,32,72,16,66,223,220,125,126,47,27,28,32,32,32,32,32,32,72,16,220,223,220,112,26,27,26,32,32,32,32,33,33,39,72,16,223,220,112,26,26,26,32,32,32,33,33,33,39,39,72,16,223,112,26,26,26,25,32,32,33,33,33,39,39,39,52,64,113,5,26,26,25,25,25,33,33,33,33,39,39,40,52,16,5,26,26,25,25,25,24,33,33,45,45,39,40,40,16,5,5,26,23,25,25,24,24,45,45,44,43,43,41,41,18,5,22,23,25,25,24,24,45,45,44,44,43,42,42,41,5,5,22,23,25,25,24,45,45,44,44,43,43,42,42,41,5,21,22,23,25,24,45,45,45,44,44,43,42,42,42,41]} |
      | set-image-interlace | {"index":0,"interlace":true} |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the original document
    Given the real GIF87a input document shared://🖼️dancing-87a.gif
    When the <id> mutation is applied and then its own inverse, computed from the pre-mutation document, is applied on top
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection of the restored document
    Examples:
      | id | params |
      | no-mutation | {} |
      | set-snapshot | {"snapshot":{"width":4,"height":4,"gct":null,"backgroundColorIndex":0,"images":[{"left":0,"top":0,"width":4,"height":4,"interlace":false,"lct":{"sorted":false,"colors":[{"r":50,"g":205,"b":50},{"r":50,"g":205,"b":50}]},"indices":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]}]}} |
      | set-screen-size | {"width":20,"height":20} |
      | set-global-color-table | {"gct":{"colors":[{"r":193,"g":108,"b":82},{"r":206,"g":161,"b":101},{"r":217,"g":120,"b":137},{"r":193,"g":108,"b":82}]}} |
      | set-background-color-index | {"index":1} |
      | set-pixel-aspect-ratio | {"ratio":2} |
      | insert-image | {"index":1,"image":{"left":0,"top":0,"width":4,"height":4,"interlace":false,"lct":null,"indices":[74,74,74,73,16,74,74,75,16,74,75,75,16,14,75,75]}} |
      | remove-image | {"index":1} |
      | move-image | {"from":0,"to":2} |
      | set-image-geometry | {"index":0,"left":2,"top":2,"width":16,"height":16} |
      | set-image-pixels | {"index":0,"indices":[15,15,17,17,17,143,254,254,128,128,135,131,131,133,133,133,15,15,17,17,70,221,254,254,128,128,131,131,131,131,61,61,15,15,15,17,254,254,254,127,128,128,131,131,131,61,61,61,15,15,15,146,221,220,126,127,128,129,131,131,131,61,61,61,15,16,16,254,221,220,126,127,128,129,130,131,64,47,47,47,72,16,146,223,220,125,126,127,129,47,47,31,31,32,32,32,72,16,66,223,220,125,126,47,27,28,32,32,32,32,32,32,72,16,220,223,220,112,26,27,26,32,32,32,32,33,33,39,72,16,223,220,112,26,26,26,32,32,32,33,33,33,39,39,72,16,223,112,26,26,26,25,32,32,33,33,33,39,39,39,52,64,113,5,26,26,25,25,25,33,33,33,33,39,39,40,52,16,5,26,26,25,25,25,24,33,33,45,45,39,40,40,16,5,5,26,23,25,25,24,24,45,45,44,43,43,41,41,18,5,22,23,25,25,24,24,45,45,44,44,43,42,42,41,5,5,22,23,25,25,24,45,45,44,44,43,43,42,42,41,5,21,22,23,25,24,45,45,45,44,44,43,42,42,42,41]} |
      | set-image-interlace | {"index":0,"interlace":true} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real document without passing bytes through
    Given the real GIF87a input document shared://🖼️dancing-87a.gif
    When the document is fully parsed into the typed snapshot and re-encoded from that snapshot alone, never by copying or splicing the source bytes
    Then the re-encoded bytes are not bit-identical to the input and project to the same semantic content the input does
