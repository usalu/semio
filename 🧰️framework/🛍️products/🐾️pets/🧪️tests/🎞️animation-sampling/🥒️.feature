@capability-pets-animation-sampling
@oracle-pets-scipy
@comparison-pets-float-v1
Feature: Keyframed clips are sampled into the same poses in every language
  A pet moves by keyframed clips (design §3, §4.4). A key carries a value at a phase of its clip and,
  optionally, a CSS `cubic-bezier(x1, y1, x2, y2)` easing towards the next key. `easeBezier(ease, amount)`
  is 0 at and below 0, 1 at and above 1 and in between the curve's `y` at the parameter where its `x`
  equals `amount`, found by 48 bisection steps. `sampleTrack(track, phase)` holds the first key's value at
  and before its phase and the last key's at and after its phase, and in between blends the two
  neighbouring keys by their local phase, eased by the earlier key (linear without an easing); a phase
  on a key yields that key's value. `clipTicks(clip)` is `floor(seconds × 64 + 0.5)`, at least 1.
  `sampleClip(species, clip, ticks)` turns whole ticks into a phase — a looping clip wraps
  (`(ticks mod length) ÷ length`), any other clip holds its last key (`min(ticks, length) ÷ length`) —
  and writes every track into its channel of its bone; channels without a track stay at rest (offsets 0,
  scale 1). `blendPose(from, to, amount)` is `from` at and below 0, `to` at and above 1 and the linear
  blend of every channel in between. `lidAt(ticks)` is the closure of a blink of `BLINK_TICKS = 12` ticks:
  it closes over 4 ticks, stays shut for one and opens over 7, each way by the Hermite ease `t²·(3 − 2t)`.

  THE REFERENCE is `🐍️.py` beside this file. The easing is written there in Bernstein form and inverted
  by `scipy.optimize.brentq`, a root finder the subjects do not use (they bisect the Horner form). Tracks
  without easings are sampled by `numpy.interp` over the whole track; tracks with easings go through a
  `numpy.searchsorted` segment search whose arithmetic must reproduce `numpy.interp` on the same track
  with its easings removed. The lid is `scipy.interpolate.CubicHermiteSpline` through (0, 0), (4, 1),
  (5, 1), (12, 0) with flat tangents. Clip lengths are rounded by `numpy.floor` and confirmed as
  round-half-up in exact rational arithmetic; blends are numpy's convex combination `(1 − a)·from + a·to`.
  Everything compares under `pets-float-v1` (1e-9). The subjects are the functions of
  `🔨️modules/🎞️animation` in `@semio-tech/pets` and, in snake_case, in the `pets` crate.

  The easings cover the CSS keywords, the eases of the reference rig, curves that leave [0, 1]
  (anticipation and overshoot), the exact cubics `t³` and `1 − (1 − t)³`, the identity, and the two
  extremes whose `x` stands still: at both ends (`0, 1, 1, 0`) and in the middle (`1, 0, 0, 1`). Where
  `x` stands still the parameter is ill-conditioned for every solver, so the second is sampled on both
  sides of one half but not at it. The species is the reference rig of the ticket with one more clip
  (`lunge`, played once, with eased offsets and scales); every clip is sampled before, at and after its
  end, so wrapping and holding are both in the vectors.

  The vectors shared://🎞️animation-sampling/🔣️.json are generated, never hand-edited, from the Python
  reference in this directory, from the repository root:
  `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/generate_animation_vectors.py`
  (`.venv/bin/python` outside Windows).

  @id-bezier-easings
  @level-fundamental
  @mode-differential
  Scenario: A cubic Bézier easing yields the curve's height where its abscissa equals the amount
    Given the committed vectors shared://🎞️animation-sampling/🔣️.json
    When every committed easing is evaluated with easeBezier at every committed amount
    Then every implementation projects the same eased amounts per easing, 0 at and below 0 and 1 at and above 1

  @id-track-samples
  @level-fundamental
  @mode-differential
  Scenario: A track blends its neighbouring keys by their eased local phase
    Given the committed vectors shared://🎞️animation-sampling/🔣️.json
    When every committed track is sampled with sampleTrack at every committed phase
    Then every implementation projects the same values per track, each key's own value on its phase and the end values outside 0 … 1

  @id-clip-lengths
  @level-fundamental
  @mode-differential
  Scenario: A clip lasts its seconds rounded half up to whole ticks, at least one
    Given the committed vectors shared://🎞️animation-sampling/🔣️.json
    When every committed length in seconds is turned into ticks with clipTicks
    Then every implementation projects the same whole number of ticks per length

  @id-clip-poses
  @level-fundamental
  @mode-differential
  Scenario: A clip poses its bones, wraps when it loops and holds its last key when it does not
    Given the committed vectors shared://🎞️animation-sampling/🔣️.json
    When every clip of the committed species is sampled with sampleClip at every committed tick
    Then every implementation projects the same pose per tick, one bone pose per bone in rig order with untouched channels at rest

  @id-pose-blends
  @level-fundamental
  @mode-differential
  Scenario: Two poses blend channel by channel
    Given the committed vectors shared://🎞️animation-sampling/🔣️.json
    When every committed pair of poses is blended with blendPose at every committed amount
    Then every implementation projects the same poses, the first pose at and below 0 and the second at and above 1

  @id-blink-lids
  @level-fundamental
  @mode-differential
  Scenario: A blink shuts the lid fast and opens it slowly within twelve ticks
    Given the committed vectors shared://🎞️animation-sampling/🔣️.json
    When lidAt is evaluated at every committed tick after the blink began
    Then every implementation projects BLINK_TICKS and the same closures, 1 at the 4th and 5th tick and 0 from the 12th
