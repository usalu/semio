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
