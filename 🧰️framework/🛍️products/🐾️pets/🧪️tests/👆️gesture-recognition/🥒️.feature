@capability-pets-gesture-recognition
@oracle-pets-scipy
@comparison-pets-float-v1
Feature: The learner's hand — a press, the heat of attention, and circling, stroking and shaking — is recognised without ever firing on ordinary pointer travel
  Every recogniser of `🔨️modules/👆️gesture` is a small state and a pure step per tick of 1/64 s on the
  sample-and-hold pointer, built from comparisons, `+ − × ÷`, `abs`, `min` and `max` (design §17, research
  §7, §8.4). A press only arms: a release before `HOLD_TICKS` without leaving the slop (`SLOP_FINE` for a
  mouse or a pen, `SLOP_COARSE` for a finger) is a click, a press that rests `HOLD_TICKS` is a hold until it
  is released, leaving the slop is a pick-up (`lift`) and the release then a `drop`; a cancellation, a
  still stage, a scroll under a press that lifted nothing, and a new press while one is open `abort`; a
  press over a control is not taken. Attention warms a pet: a click adds `HEAT_CLICK`, a tick of holding
  `HEAT_HOLD`, the heat leaks `HEAT_LEAK` per second and never falls below zero; a click is answered with
  `hello` up to `HEAT_HELLO`, a `trick` up to `HEAT_TRICK`, a `purr` below `HEAT_ENOUGH`, and the caress
  that reaches it with `enough`, after which every caress is ignored for `ENOUGH_TICKS` and the heat starts
  again at `HEAT_FORGIVEN`. A circle round a body counts quarter turns with a Schmitt trigger on both axes
  in a band round the body; it is a circle once the pointer has crossed again the axis where the lap began
  (`CIRCLE_QUARTERS`), not faster than `CIRCLE_FAST` ticks per quarter turn, round (`CIRCLE_ROUND`) and
  closed (`CIRCLE_CLOSE`) — `circle` clockwise as seen on screen with the y axis pointing down,
  `countercircle` the other way. A petting is three horizontal strokes in a row over the body, each
  beginning at a reversal, long, level and brisk enough, within `STROKE_WINDOW` ticks — `stroke`. A shake
  of a held pet is four reversals of the grip within `SHAKE_WINDOW` ticks, each after a swing long and fast
  enough — `shake`. Over a control, in a quiet or still stage and after a scroll no hover gesture is heard;
  only a still stage silences a shake.

  THE REFERENCE is `🐍️.py` beside this file; it never steps a state. A press is read in closed form —
  `numpy.hypot` for the distance of every drag, `numpy.flatnonzero` for the first drag at the slop and the
  first event that ends the press, set against the tick of the hold —, and so is the heat: a leaky bucket
  that never falls below zero is Lindley's recursion, solved by `numpy.cumsum` minus its
  `numpy.minimum.accumulate`, with the tiers from `numpy.searchsorted`. A circle is read from the winding
  `numpy.unwrap(numpy.arctan2(y, x))`, a stroke from the reversals `scipy.signal.find_peaks` finds in the
  horizontal offset, a shake from those it finds along twelve directions of the grip and along its
  principal axis (`numpy.linalg.eigh`). The cues of the traces were recorded from the TypeScript subject
  when the vectors were generated and are answered as committed only after these readings admitted every
  one of them: each cue ends a full turn, two strokes or three swings the libraries see, in its
  direction; every gesture the reading finds clean — in the band or the zone, steady, round, closed, long
  enough — is cued, and in time; a guard that silences a gesture leaves nothing; and no clean gesture
  hides in ordinary travel or in carrying a pet.

  The traces are synthesised, not recorded, at 64 Hz from a model of the hand: minimum-jerk reaches with
  overshoot and correction, tremor, rest, sweeps, reading zigzags, text selection, lingering on a pet,
  idle wandering, and deliberate circles, strokes, shakes, carries, clicks, double clicks, long presses and
  slow drags, reported by devices of 30 to 144 Hz and rounded to whole or quarter pixels. A path is a list
  of integers in quarter pixels — the first point, then per tick a step or `hold + n` for `n` ticks
  without one —, and a variant of a trace is the trace mirrored, transposed or played backwards, exactly,
  on those integers. The subjects are the constants, `pressStep`, `warmthAfter`, `hoverStep` and
  `shakeStep` of `🔨️modules/👆️gesture` in `@semio-tech/pets` and the same names in snake case of the
  `pets` crate; they project their own constants, so a threshold tuned in one language only, or without
  regenerating the vectors, fails the case.

  The vectors shared://👆️gesture-recognition/🔣️.json are generated, never hand-edited, from the hand
  model, the Python reference and the TypeScript subject, from the repository root:
  `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/generate_gesture_vectors.py`
  (`.venv/bin/python` outside Windows; `bun` on the path).

  @id-constants
  @level-fundamental
  @mode-conformance
  Scenario: The thresholds are the ones the vectors were generated with
    Given the committed vectors shared://👆️gesture-recognition/🔣️.json
    When every implementation states its slops, its hold, its heat and the thresholds of its three gestures
    Then every implementation projects the committed constants

  @id-presses
  @level-fundamental
  @mode-differential
  Scenario: A press only arms and turns into a click, a hold, a pick-up or nothing by what follows it
    Given the committed vectors shared://👆️gesture-recognition/🔣️.json
    When every committed press is replayed tick by tick, the passing of each tick before the events that arrive at it
    Then every implementation projects the signals numpy reads off the distances and the first ending of each press, at their ticks

  @id-warmth
  @level-fundamental
  @mode-differential
  Scenario: Clicks and holding warm a pet through hello, tricks and purring until it has had enough, and it forgives
    Given the committed vectors shared://👆️gesture-recognition/🔣️.json
    When every committed history of attention is answered caress by caress
    Then every implementation projects the heat of Lindley's recursion, the tier, the run, the tricks so far and the end of the refractory time

  @id-circles
  @level-fundamental
  @mode-differential
  Scenario: A deliberate circle round a pet is recognised in its direction, and only when the pointer really went round
    Given the committed vectors shared://👆️gesture-recognition/🔣️.json
    When every committed circle is replayed tick by tick round its body, plainly and under every guard
    Then every implementation projects the committed cues, each one the end of a full turn in its direction that numpy's winding sees, every clean circle cued in time, none under a guard

  @id-strokes
  @level-fundamental
  @mode-differential
  Scenario: Petting — strokes back and forth over a pet — is recognised
    Given the committed vectors shared://👆️gesture-recognition/🔣️.json
    When every committed petting is replayed tick by tick over its body, plainly and under every guard
    Then every implementation projects the committed cues, each one after two reversals scipy finds within its window, every clean petting cued in time, none under a guard

  @id-shakes
  @level-fundamental
  @mode-differential
  Scenario: Shaking a held pet is recognised and carrying it about is not
    Given the committed vectors shared://👆️gesture-recognition/🔣️.json
    When every committed held path is replayed tick by tick, plainly and under every guard
    Then every implementation projects the committed cues, each one after three swings scipy finds within its window, every clean shake cued in time, no cue for carrying, none on a still stage and the same under the other guards

  @id-detection
  @level-fundamental
  @mode-property
  Scenario: Deliberate gestures are recognised at the committed rates, in their mirrors as well
    Given the committed vectors shared://👆️gesture-recognition/🔣️.json
    When every deliberate circle, petting and shake is replayed in its four mirrored variants
    Then every implementation projects how many were recognised, at the same ticks in every mirror, a circle turned the other way round by one mirror, no gesture answered with another, and no rate below the committed floor

  @id-travel-sample
  @level-fundamental
  @mode-property
  Scenario: A sample of ordinary pointer travel past forty pets sets nothing off
    Given the committed vectors shared://👆️gesture-recognition/🔣️.json
    When the sample stretches of ordinary travel are replayed past every committed body
    Then every implementation projects how many pointer and pet ticks it watched and not a single cue, and the reading finds no clean gesture in them

  @id-travel-hours
  @level-exhaustive
  @mode-property
  Scenario: Hours of ordinary pointer travel in every variant set nothing off
    Given the committed vectors shared://👆️gesture-recognition/🔣️.json
    When every stretch of ordinary travel is replayed in each of its sixteen variants past every committed body
    Then every implementation projects how many pointer and pet ticks it watched and not a single cue, and the reading finds no clean gesture in them
