@capability-gif-89a-mutate
@oracle-gif-89a-any-mutate-reader
@comparison-semantic-raster-v1
@mutations-gif-89a-any
Feature: Apply every typed GIF 89a mutation to a real-world animation
  The input is a real 4.4 MB, 800x800, 54-frame animated GIF89a produced by ScreenToGif — not a
  synthetic fixture — carrying a real comment extension, a real NETSCAPE2.0 loop extension and
  per-frame local color tables (no global color table at all). It currently lives under the 87a
  subset's own example directory (📚️examples/💃️dancing), read from there via asset:// rather than
  moved, since 89a shares no example tree of its own and this is the richest real animation already
  committed. Every scenario copies it into the case work directory before touching it; the committed
  asset is never written to. The reference implementation is used only by the test oracle, and its
  result is read back by an independent decoder before projection.

  All 21 declared kinds move the compared projection and the oracle FAILS any scenario whose kind
  leaves it untouched — there is no exemption list. Two things had to be true for that:

    – The NETSCAPE2.0 extension the animation carries is the loop-count axis, modelled as
      `loopCount` and deliberately not an `appExtensions` entry, so the file has no application
      extension for remove-app-extension to remove. That row is exercised on the real document after
      the reference implementation has inserted a named target first — the same arrange step the
      OOXML conformance cases and the PNG case use for their own removal kinds. Without it the row
      addressed nothing and passed for that reason.
    – `gif::Decoder` de-interlaces every frame on read and then reports `Frame::interlaced` as false
      regardless of what the file said, while `Encoder::write_frame` writes the buffer verbatim and
      only flips the descriptor bit. Reading the flag back off the Image Descriptor's own packed
      byte, and re-interleaving the rows on encode, is what makes set-frame-interlace visible:
      trusting the decoder meant the round trip erased both the flag and the row permutation and
      landed back exactly where it started.

  Every Examples `params` cell is exactly the leaf's wire payload — its `payload_value()`, camelCase,
  no aggregate tag — decoded by the subject through the derive-generated `from_payload_value` and
  read by the `gif` oracle by the same field names; a colour table is `{sorted, colors}` and an
  application extension's identifier and authentication code are their raw bytes. A row states
  every frame it hands over instead of naming one to clone: `insert-frame` inserts a real 4x4 crop of
  frame 0 at (398,398), re-indexed onto the four colours it uses. `set-frame-pixels` carries a frame's
  WHOLE index buffer, and every frame of this animation holds at least 122 470 indices, so that kind
  runs in its own outlines on the committed 8x6, three-frame GIF89a
  (`shared://🧱️set-frame-pixels-applied/⬅️before.gif`) with that recipe's own replacement indices.

  ⚠️ TWO KNOWN OPEN DIVERGENCES (this case's parity ratio is recorded in the ticket, not here),
  both the same disagreement: a mutation
  edits one field and leaves a dependent one behind, and the two implementations resolve the
  resulting inconsistency differently. Neither is a coding error and neither is to be papered over.

    – `mutate-set-screen-size` shrinks the Logical Screen to 801x799 while frame 0 stays 800x800 at
      (0,0), so the frame no longer fits inside the screen. `encode_gif` refuses ("frame 0 region
      exceeds the logical screen"); the reference writer emits the file, leaving a frame that
      overhangs the canvas.
    – `mutate-set-frame-geometry` re-declares frame 0 as 100x100 at (5,5) while its index buffer
      stays 640 000 bytes. `encode_gif` refuses ("frame 0 indices length mismatch"); the reference
      writer accepts any buffer at least as large as the rectangle and LZW-encodes the whole thing,
      so the image data block carries 64x more pixels than the descriptor declares.

  In both cases GIF89a permits the file the reference produced (§18/§20 constrain nothing a writer
  must enforce), and this repository's codec is the stricter of the two. Resolving them means
  specifying what `set-screen-size` and `set-frame-geometry` do to the raster they invalidate —
  clip, resize, or refuse at mutation time — and making both sides implement that one answer. Do not
  widen the profile, change these rows' parameters, or swap the fixture to close them.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real animation
    Given the real input document asset://💃️dancing/🧪️dancing/🖼️.gif
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id | params |
      | set-snapshot | {"snapshot":{"schema":"stdio.gif.89a","width":2,"height":2,"gct":{"sorted":false,"colors":[{"r":4,"g":5,"b":6},{"r":4,"g":5,"b":6}]},"backgroundColorIndex":0,"pixelAspectRatio":0,"loopCount":0,"frames":[{"left":0,"top":0,"width":2,"height":2,"interlace":false,"lct":{"sorted":false,"colors":[{"r":9,"g":9,"b":9},{"r":9,"g":9,"b":9}]},"indices":[0,1,1,0],"delayCs":10,"disposal":"doNotDispose","transparentIndex":null,"userInput":false,"plainText":null}],"comments":["c0"],"appExtensions":[]}} |
      | set-screen-size | {"width":801,"height":799} |
      | set-global-color-table | {"gct":{"sorted":false,"colors":[{"r":10,"g":20,"b":30},{"r":40,"g":50,"b":60}]}} |
      | set-background-color-index | {"index":3} |
      | set-pixel-aspect-ratio | {"ratio":12} |
      | insert-frame | {"index":10,"frame":{"left":398,"top":398,"width":4,"height":4,"interlace":false,"lct":{"sorted":false,"colors":[{"r":85,"g":95,"b":10},{"r":79,"g":94,"b":10},{"r":72,"g":93,"b":14},{"r":114,"g":99,"b":15},{"r":115,"g":108,"b":16},{"r":123,"g":98,"b":110},{"r":124,"g":124,"b":22},{"r":131,"g":214,"b":145}]},"indices":[6,4,4,0,4,3,0,1,0,0,1,2,5,5,7,7],"delayCs":33,"disposal":"unspecified","transparentIndex":null,"userInput":false,"plainText":null}} |
      | remove-frame | {"index":10} |
      | move-frame | {"from":5,"to":20} |
      | set-frame-geometry | {"index":0,"left":5,"top":5,"width":100,"height":100} |
      | set-frame-interlace | {"index":1,"interlace":true} |

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to a small animation
    Given the small input animation shared://🧱️set-frame-pixels-applied/⬅️before.gif
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id | params |
      | set-frame-pixels | {"index":0,"indices":[3,2,1,0,3,2,1,0,3,2,1,0]} |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real animation
    Given the real input document asset://💃️dancing/🧪️dancing/🖼️.gif
    When the <id> mutation is applied and its computed inverse is applied back
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the original semantic projection is recovered
    Examples:
      | id | params |
      | set-snapshot | {"snapshot":{"schema":"stdio.gif.89a","width":2,"height":2,"gct":{"sorted":false,"colors":[{"r":4,"g":5,"b":6},{"r":4,"g":5,"b":6}]},"backgroundColorIndex":0,"pixelAspectRatio":0,"loopCount":0,"frames":[{"left":0,"top":0,"width":2,"height":2,"interlace":false,"lct":{"sorted":false,"colors":[{"r":9,"g":9,"b":9},{"r":9,"g":9,"b":9}]},"indices":[0,1,1,0],"delayCs":10,"disposal":"doNotDispose","transparentIndex":null,"userInput":false,"plainText":null}],"comments":["c0"],"appExtensions":[]}} |
      | set-screen-size | {"width":801,"height":799} |
      | set-global-color-table | {"gct":{"sorted":false,"colors":[{"r":10,"g":20,"b":30},{"r":40,"g":50,"b":60}]}} |
      | set-background-color-index | {"index":3} |
      | set-pixel-aspect-ratio | {"ratio":12} |
      | insert-frame | {"index":10,"frame":{"left":398,"top":398,"width":4,"height":4,"interlace":false,"lct":{"sorted":false,"colors":[{"r":85,"g":95,"b":10},{"r":79,"g":94,"b":10},{"r":72,"g":93,"b":14},{"r":114,"g":99,"b":15},{"r":115,"g":108,"b":16},{"r":123,"g":98,"b":110},{"r":124,"g":124,"b":22},{"r":131,"g":214,"b":145}]},"indices":[6,4,4,0,4,3,0,1,0,0,1,2,5,5,7,7],"delayCs":33,"disposal":"unspecified","transparentIndex":null,"userInput":false,"plainText":null}} |
      | remove-frame | {"index":10} |
      | move-frame | {"from":5,"to":20} |
      | set-frame-geometry | {"index":0,"left":5,"top":5,"width":100,"height":100} |
      | set-frame-interlace | {"index":1,"interlace":true} |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores a small animation
    Given the small input animation shared://🧱️set-frame-pixels-applied/⬅️before.gif
    When the <id> mutation is applied and its computed inverse is applied back
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the original semantic projection is recovered
    Examples:
      | id | params |
      | set-frame-pixels | {"index":0,"indices":[3,2,1,0,3,2,1,0,3,2,1,0]} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real animation without passing bytes through
    Given the real input document asset://💃️dancing/🧪️dancing/🖼️.gif
    When the animation is decoded into a snapshot and re-encoded from that snapshot alone
    Then the output bytes differ from the input and the semantic projection is unchanged
