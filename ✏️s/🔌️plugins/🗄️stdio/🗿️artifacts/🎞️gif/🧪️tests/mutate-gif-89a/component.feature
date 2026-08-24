@capability-gif-89a-mutate
@oracle-gif-89a-any-mutate
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

  ⚠️ TWO KNOWN OPEN DIVERGENCES (parity 41/43, 2026-08-24), both the same disagreement: a mutation
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
    Given the real input document asset://🏅️standards/🔖️87a/🪆️subsets/✳️any/📚️examples/💃️dancing/🖼️assets/🖼️dancing.gif
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                          | params                                                                   |
      | no-mutation                 | {}                                                                       |
      | set-snapshot                | {"width":2,"height":2,"globalPalette":[[4,5,6],[4,5,6]],"backgroundColorIndex":0,"aspectRatio":0,"loopCount":0,"frames":[{"left":0,"top":0,"width":2,"height":2,"interlace":false,"palette":[[9,9,9],[9,9,9]],"indices":[0,1,1,0],"delayCs":10,"disposal":"doNotDispose","transparentIndex":null,"userInput":false}],"comments":["c0"],"appExtensions":[]} |
      | set-screen-size             | {"width":801,"height":799}                                              |
      | set-global-color-table      | {"colors":[[10,20,30],[40,50,60]]}                                      |
      | set-background-color-index  | {"index":3}                                                              |
      | set-pixel-aspect-ratio      | {"ratio":12}                                                             |
      | set-loop-count              | {"loopCount":5}                                                          |
      | insert-frame                | {"index":10,"sourceFrame":0,"delayCs":33}                                |
      | remove-frame                | {"index":10}                                                             |
      | move-frame                  | {"from":5,"to":20}                                                       |
      | set-frame-geometry          | {"index":0,"left":5,"top":5,"width":100,"height":100}                   |
      | set-frame-pixels            | {"index":0,"fillIndex":7}                                                |
      | set-frame-interlace         | {"index":1,"interlace":true}                                             |
      | set-frame-delay             | {"index":1,"delayCs":250}                                                |
      | set-frame-disposal          | {"index":1,"disposal":"restoreToBackground"}                            |
      | set-frame-transparency      | {"index":1,"transparentIndex":3}                                         |
      | set-frame-user-input        | {"index":1,"userInput":true}                                            |
      | insert-comment              | {"index":0,"text":"oracle mutation test"}                                |
      | remove-comment              | {"index":0}                                                              |
      | add-app-extension           | {"index":0,"identifier":"XMPDATA1","authCode":"XMP","data":[1,2,3]}      |
      | remove-app-extension        | {"index":0}                                                              |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real animation
    Given the real input document asset://🏅️standards/🔖️87a/🪆️subsets/✳️any/📚️examples/💃️dancing/🖼️assets/🖼️dancing.gif
    When the <id> mutation is applied and its computed inverse is applied back
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the original semantic projection is recovered
    Examples:
      | id                          | params                                                                   |
      | no-mutation                 | {}                                                                       |
      | set-snapshot                | {"width":2,"height":2,"globalPalette":[[4,5,6],[4,5,6]],"backgroundColorIndex":0,"aspectRatio":0,"loopCount":0,"frames":[{"left":0,"top":0,"width":2,"height":2,"interlace":false,"palette":[[9,9,9],[9,9,9]],"indices":[0,1,1,0],"delayCs":10,"disposal":"doNotDispose","transparentIndex":null,"userInput":false}],"comments":["c0"],"appExtensions":[]} |
      | set-screen-size             | {"width":801,"height":799}                                              |
      | set-global-color-table      | {"colors":[[10,20,30],[40,50,60]]}                                      |
      | set-background-color-index  | {"index":3}                                                              |
      | set-pixel-aspect-ratio      | {"ratio":12}                                                             |
      | set-loop-count              | {"loopCount":5}                                                          |
      | insert-frame                | {"index":10,"sourceFrame":0,"delayCs":33}                                |
      | remove-frame                | {"index":10}                                                             |
      | move-frame                  | {"from":5,"to":20}                                                       |
      | set-frame-geometry          | {"index":0,"left":5,"top":5,"width":100,"height":100}                   |
      | set-frame-pixels            | {"index":0,"fillIndex":7}                                                |
      | set-frame-interlace         | {"index":1,"interlace":true}                                             |
      | set-frame-delay             | {"index":1,"delayCs":250}                                                |
      | set-frame-disposal          | {"index":1,"disposal":"restoreToBackground"}                            |
      | set-frame-transparency      | {"index":1,"transparentIndex":3}                                         |
      | set-frame-user-input        | {"index":1,"userInput":true}                                            |
      | insert-comment              | {"index":0,"text":"oracle mutation test"}                                |
      | remove-comment              | {"index":0}                                                              |
      | add-app-extension           | {"index":0,"identifier":"XMPDATA1","authCode":"XMP","data":[1,2,3]}      |
      | remove-app-extension        | {"index":0}                                                              |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real animation without passing bytes through
    Given the real input document asset://🏅️standards/🔖️87a/🪆️subsets/✳️any/📚️examples/💃️dancing/🖼️assets/🖼️dancing.gif
    When the animation is decoded into a snapshot and re-encoded from that snapshot alone
    Then the output bytes differ from the input and the semantic projection is unchanged
