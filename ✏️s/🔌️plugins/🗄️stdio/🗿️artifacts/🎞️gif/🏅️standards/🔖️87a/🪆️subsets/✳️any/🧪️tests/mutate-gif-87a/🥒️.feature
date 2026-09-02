@capability-gif-87a-mutate
@oracle-gif-87a-any-mutate-reader
@comparison-semantic-raster-v1
@mutations-gif-87a-any
Feature: Apply every typed GIF87a mutation to a real derived document
  The input is a genuine GIF87a file — magic bytes literally "GIF87a", zero Graphic Control
  Extensions — because this subset's own vocabulary has no frame delay, disposal, transparency,
  comment or application-extension concept: GIF87a itself has none of those. It was derived ONCE,
  not fabricated, from the real animated ../../🏅️standards/🔖️87a/🪆️subsets/✳️any/📚️examples/💃️dancing/🖼️assets/🖼️dancing.gif
  fixture (itself a real GIF89a file mis-filed under the 87a example directory — see the ticket
  writeup for that finding) using the `gif` reference crate: three of its real frames (indices 0,
  20 and 40) were decoded with the crate's own indexed-colour reader, genuine rectangles of real,
  already-decoded palette indices were cropped out of each — 400×400 out of frame 0 at (200,200),
  400×400 out of frame 20 at (50,150) and 32×32 out of frame 40 at (120,180) — and frame 0's real
  256-colour local table was promoted to this file's own Global Color Table so a fourth, inserted
  image can reference it without carrying its own. The result is `shared://🧪️dancing-87a-large/🖼️.gif`,
  117 704 bytes over a 400×400 logical screen, against 2 936 bytes over a 16×16 one for the
  derivation this case used to rest on. Every scenario copies the immutable fixture into the case
  work directory before touching it; the fixture itself is never written to.

  The 16×16 derivation is NOT gone: `identity-round-trip` still reads it beside the large one,
  because it is the smallest genuine GIF87a committed here and the only one whose entire index buffer
  a scenario could still name literally — and nothing it proved was given up to make room.

  Two `Examples` parameters are aimed by that size difference, and are stated rather than left to be
  discovered. `set-image-pixels` addresses image 2, the 32×32 crop, because the verb takes the WHOLE
  index buffer of the image it rewrites and image 0's is 160 000 entries; the 1 024 indices it does
  carry are a real 32×32 crop of a fourth real frame (30, at (200,300)), so the row still overwrites
  real pixels with different real pixels. `set-image-geometry` moves image 0's origin while declaring
  its real 400×400 extent, so the row moves the placement without claiming an extent the buffer does
  not have.

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

  ⚠️ KNOWN OPEN DIVERGENCE — `mutate-set-global-color-table` (this case's parity ratio is recorded
  in the ticket, not here). The row
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
    Given the real GIF87a input document shared://🧪️dancing-87a-large/🖼️.gif
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id | params |
      | no-mutation | {} |
      | set-snapshot | {"snapshot":{"width":4,"height":4,"gct":null,"backgroundColorIndex":0,"images":[{"left":0,"top":0,"width":4,"height":4,"interlace":false,"lct":{"sorted":false,"colors":[{"r":50,"g":205,"b":50},{"r":50,"g":205,"b":50}]},"indices":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]}]}} |
      | set-screen-size | {"width":420,"height":410} |
      | set-global-color-table | {"gct":{"colors":[{"r":193,"g":108,"b":82},{"r":206,"g":161,"b":101},{"r":217,"g":120,"b":137},{"r":193,"g":108,"b":82}]}} |
      | set-background-color-index | {"index":1} |
      | set-pixel-aspect-ratio | {"ratio":2} |
      | insert-image | {"index":1,"image":{"left":0,"top":0,"width":4,"height":4,"interlace":false,"lct":null,"indices":[74,74,74,73,16,74,74,75,16,74,75,75,16,14,75,75]}} |
      | remove-image | {"index":1} |
      | move-image | {"from":0,"to":2} |
      | set-image-geometry | {"index":0,"left":2,"top":2,"width":400,"height":400} |
      | set-image-pixels | {"index":2,"indices":[82,82,82,82,82,82,82,80,78,78,77,77,149,84,84,84,84,84,84,84,113,112,45,44,43,70,146,70,70,71,71,72,84,84,82,82,82,82,82,78,78,77,77,151,85,85,85,85,85,85,85,113,125,220,125,47,47,147,146,146,70,71,71,72,84,84,84,82,82,82,78,77,77,77,151,151,85,85,85,85,85,85,74,126,125,220,221,223,139,147,146,146,70,71,71,72,85,84,84,84,84,77,77,77,151,151,152,86,86,86,86,85,85,85,127,126,220,220,221,221,141,147,146,146,146,71,71,72,85,85,85,84,77,152,152,151,151,152,86,86,88,89,89,89,89,129,127,254,220,221,221,249,141,145,146,146,146,71,71,72,89,89,89,89,152,152,152,152,153,87,88,88,89,89,89,89,89,128,127,254,254,221,221,251,144,144,145,146,66,67,67,72,89,89,89,152,152,152,153,89,88,88,88,89,89,89,89,89,133,128,254,254,254,249,221,137,143,144,145,66,66,67,67,72,89,89,152,152,153,89,92,88,88,88,88,89,89,89,89,89,131,128,254,254,254,249,249,137,143,144,145,145,66,67,67,68,89,92,153,89,92,87,88,88,88,88,90,89,89,92,92,133,135,128,254,254,254,249,249,137,143,143,144,65,66,66,67,67,89,92,103,103,103,102,88,90,90,90,90,91,92,92,92,131,135,128,254,254,254,249,249,136,134,143,144,145,65,66,67,52,17,103,101,102,102,102,90,90,90,90,91,91,91,91,92,131,135,136,254,254,254,249,248,251,134,143,143,144,65,65,67,20,11,11,84,102,101,102,90,90,91,91,91,91,94,94,113,131,135,136,254,254,254,250,250,250,137,133,143,143,64,65,68,5,11,11,11,11,84,102,96,96,96,96,96,95,95,95,113,134,136,136,254,254,250,250,250,250,137,134,143,143,64,64,18,5,8,8,11,11,11,11,11,84,84,89,96,92,92,94,144,134,136,136,136,250,250,250,250,250,136,134,133,133,143,64,3,5,8,8,8,11,11,11,11,11,13,13,13,80,82,82,131,133,134,136,136,136,250,251,251,251,136,136,134,133,133,67,6,5,8,8,8,10,11,11,11,10,12,12,12,12,82,82,66,143,134,137,136,136,136,251,251,251,251,136,135,133,133,65,6,5,8,8,8,8,10,10,10,10,12,12,12,12,12,82,71,143,134,137,137,137,137,137,160,160,160,136,135,131,133,68,6,5,3,3,3,9,10,10,10,10,12,12,12,12,12,82,72,143,143,137,137,137,139,139,139,139,139,251,136,135,131,68,6,20,3,3,3,255,9,10,10,10,12,12,12,12,12,12,82,144,143,137,137,139,139,139,139,158,158,158,254,135,131,67,7,20,3,3,255,255,9,9,10,10,12,12,12,12,12,12,82,145,143,143,140,139,139,139,139,158,158,158,251,128,128,67,7,20,3,3,255,255,2,9,9,10,10,12,12,12,12,12,12,71,144,143,143,140,139,139,139,158,158,158,158,251,128,67,7,20,3,255,255,2,2,9,9,9,10,12,12,12,12,12,12,72,145,144,144,140,140,140,140,139,139,139,157,156,160,133,7,20,3,255,255,2,2,2,9,9,9,12,12,12,12,12,12,84,146,145,145,141,141,141,141,141,141,141,156,156,156,139,18,19,255,3,255,2,2,2,4,9,9,10,12,12,12,12,12,28,72,146,145,141,141,141,141,141,141,141,141,141,153,153,74,19,255,3,3,2,2,2,4,9,9,9,12,12,12,12,12,12,28,72,146,147,147,141,141,141,141,149,149,149,149,150,74,19,2,255,3,3,3,2,4,9,9,9,9,27,27,27,12,12,28,29,146,146,146,147,147,149,149,149,149,149,150,150,150,19,2,2,255,255,4,4,2,4,9,9,9,27,27,27,27,28,28,29,15,70,146,146,148,148,148,149,149,149,150,150,150,74,2,2,2,255,255,3,22,5,4,9,9,9,27,27,27,28,28,28,29,15,70,70,148,148,148,148,76,76,76,76,151,151,2,2,2,2,255,255,255,3,5,5,4,27,27,27,27,28,28,28,29,30,25,70,74,74,74,74,76,76,77,77,77,77,2,2,2,2,2,255,255,255,255,5,5,5,26,26,27,27,28,28,28,27,92,170,148,74,74,74,74,78,78,78,78,78,2,2,2,2,2,2,255,255,255,255,255,255,255,255,255,255,255,255,255,89,179,180,240,93,15,15,74,78,78,78,78,78,2,2,2,2,2,2,2,255,255,255,255,255,255,255,255,255,255,255,89,179,180,180,240,240,239,113,15,15,80,80,80,80]} |
      | set-image-interlace | {"index":0,"interlace":true} |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the original document
    Given the real GIF87a input document shared://🧪️dancing-87a-large/🖼️.gif
    When the <id> mutation is applied and then its own inverse, computed from the pre-mutation document, is applied on top
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection of the restored document
    Examples:
      | id | params |
      | no-mutation | {} |
      | set-snapshot | {"snapshot":{"width":4,"height":4,"gct":null,"backgroundColorIndex":0,"images":[{"left":0,"top":0,"width":4,"height":4,"interlace":false,"lct":{"sorted":false,"colors":[{"r":50,"g":205,"b":50},{"r":50,"g":205,"b":50}]},"indices":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]}]}} |
      | set-screen-size | {"width":420,"height":410} |
      | set-global-color-table | {"gct":{"colors":[{"r":193,"g":108,"b":82},{"r":206,"g":161,"b":101},{"r":217,"g":120,"b":137},{"r":193,"g":108,"b":82}]}} |
      | set-background-color-index | {"index":1} |
      | set-pixel-aspect-ratio | {"ratio":2} |
      | insert-image | {"index":1,"image":{"left":0,"top":0,"width":4,"height":4,"interlace":false,"lct":null,"indices":[74,74,74,73,16,74,74,75,16,74,75,75,16,14,75,75]}} |
      | remove-image | {"index":1} |
      | move-image | {"from":0,"to":2} |
      | set-image-geometry | {"index":0,"left":2,"top":2,"width":400,"height":400} |
      | set-image-pixels | {"index":2,"indices":[82,82,82,82,82,82,82,80,78,78,77,77,149,84,84,84,84,84,84,84,113,112,45,44,43,70,146,70,70,71,71,72,84,84,82,82,82,82,82,78,78,77,77,151,85,85,85,85,85,85,85,113,125,220,125,47,47,147,146,146,70,71,71,72,84,84,84,82,82,82,78,77,77,77,151,151,85,85,85,85,85,85,74,126,125,220,221,223,139,147,146,146,70,71,71,72,85,84,84,84,84,77,77,77,151,151,152,86,86,86,86,85,85,85,127,126,220,220,221,221,141,147,146,146,146,71,71,72,85,85,85,84,77,152,152,151,151,152,86,86,88,89,89,89,89,129,127,254,220,221,221,249,141,145,146,146,146,71,71,72,89,89,89,89,152,152,152,152,153,87,88,88,89,89,89,89,89,128,127,254,254,221,221,251,144,144,145,146,66,67,67,72,89,89,89,152,152,152,153,89,88,88,88,89,89,89,89,89,133,128,254,254,254,249,221,137,143,144,145,66,66,67,67,72,89,89,152,152,153,89,92,88,88,88,88,89,89,89,89,89,131,128,254,254,254,249,249,137,143,144,145,145,66,67,67,68,89,92,153,89,92,87,88,88,88,88,90,89,89,92,92,133,135,128,254,254,254,249,249,137,143,143,144,65,66,66,67,67,89,92,103,103,103,102,88,90,90,90,90,91,92,92,92,131,135,128,254,254,254,249,249,136,134,143,144,145,65,66,67,52,17,103,101,102,102,102,90,90,90,90,91,91,91,91,92,131,135,136,254,254,254,249,248,251,134,143,143,144,65,65,67,20,11,11,84,102,101,102,90,90,91,91,91,91,94,94,113,131,135,136,254,254,254,250,250,250,137,133,143,143,64,65,68,5,11,11,11,11,84,102,96,96,96,96,96,95,95,95,113,134,136,136,254,254,250,250,250,250,137,134,143,143,64,64,18,5,8,8,11,11,11,11,11,84,84,89,96,92,92,94,144,134,136,136,136,250,250,250,250,250,136,134,133,133,143,64,3,5,8,8,8,11,11,11,11,11,13,13,13,80,82,82,131,133,134,136,136,136,250,251,251,251,136,136,134,133,133,67,6,5,8,8,8,10,11,11,11,10,12,12,12,12,82,82,66,143,134,137,136,136,136,251,251,251,251,136,135,133,133,65,6,5,8,8,8,8,10,10,10,10,12,12,12,12,12,82,71,143,134,137,137,137,137,137,160,160,160,136,135,131,133,68,6,5,3,3,3,9,10,10,10,10,12,12,12,12,12,82,72,143,143,137,137,137,139,139,139,139,139,251,136,135,131,68,6,20,3,3,3,255,9,10,10,10,12,12,12,12,12,12,82,144,143,137,137,139,139,139,139,158,158,158,254,135,131,67,7,20,3,3,255,255,9,9,10,10,12,12,12,12,12,12,82,145,143,143,140,139,139,139,139,158,158,158,251,128,128,67,7,20,3,3,255,255,2,9,9,10,10,12,12,12,12,12,12,71,144,143,143,140,139,139,139,158,158,158,158,251,128,67,7,20,3,255,255,2,2,9,9,9,10,12,12,12,12,12,12,72,145,144,144,140,140,140,140,139,139,139,157,156,160,133,7,20,3,255,255,2,2,2,9,9,9,12,12,12,12,12,12,84,146,145,145,141,141,141,141,141,141,141,156,156,156,139,18,19,255,3,255,2,2,2,4,9,9,10,12,12,12,12,12,28,72,146,145,141,141,141,141,141,141,141,141,141,153,153,74,19,255,3,3,2,2,2,4,9,9,9,12,12,12,12,12,12,28,72,146,147,147,141,141,141,141,149,149,149,149,150,74,19,2,255,3,3,3,2,4,9,9,9,9,27,27,27,12,12,28,29,146,146,146,147,147,149,149,149,149,149,150,150,150,19,2,2,255,255,4,4,2,4,9,9,9,27,27,27,27,28,28,29,15,70,146,146,148,148,148,149,149,149,150,150,150,74,2,2,2,255,255,3,22,5,4,9,9,9,27,27,27,28,28,28,29,15,70,70,148,148,148,148,76,76,76,76,151,151,2,2,2,2,255,255,255,3,5,5,4,27,27,27,27,28,28,28,29,30,25,70,74,74,74,74,76,76,77,77,77,77,2,2,2,2,2,255,255,255,255,5,5,5,26,26,27,27,28,28,28,27,92,170,148,74,74,74,74,78,78,78,78,78,2,2,2,2,2,2,255,255,255,255,255,255,255,255,255,255,255,255,255,89,179,180,240,93,15,15,74,78,78,78,78,78,2,2,2,2,2,2,2,255,255,255,255,255,255,255,255,255,255,255,89,179,180,180,240,240,239,113,15,15,80,80,80,80]} |
      | set-image-interlace | {"index":0,"interlace":true} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode both real documents without passing bytes through
    Given the real GIF87a input document shared://🧪️dancing-87a-large/🖼️.gif
    And the 16x16 derivation this case used to rest on shared://🧪️dancing-87a/🖼️.gif
    When each document is fully parsed into the typed snapshot and re-encoded from that snapshot alone, never by copying or splicing the source bytes
    Then the re-encoded bytes of each are not bit-identical to its input and project to the same semantic content that input does
