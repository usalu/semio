@capability-pets-turn-trigonometry
@oracle-pets-numpy
@comparison-pets-float-v1
Feature: Sine and cosine in turns are deterministic and accurate
  Every pet frame is a pure function of the stage, bit for bit in every language (design §2.4), so the
  simulation may not call a platform sine or cosine: their last bits differ between engines. `sinTurns(t)`
  and `cosTurns(t)` take the angle in turns (1 = a full revolution) and use exact operations only
  (design §4.1): the magnitude of the angle loses its whole turns (`m − floor(m)`, exact), the rest is
  measured from the nearest quarter turn (a residue in [−⅛, ⅛), exact), one rounding multiplies it by
  2π, and a fixed polynomial (Horner, literal coefficients) yields the sine or cosine of that small
  angle; the quarter picks the polynomial and its sign. Sine is odd and cosine even by construction,
  quarter turns give exactly 0, 1 and −1, and the absolute error stays below 1e-12 for every angle.
  `clamp(value, low, high)` raises to `low`, then lowers to `high`; `lerp(from, to, amount)` is
  `from + (to − from) × amount` without clamping; `smoothstep(amount)` is `t²·(3 − 2t)` of the amount
  clamped to [0, 1].

  THE REFERENCE is numpy: `numpy.sin` and `numpy.cos` of `2π·t`, `numpy.clip`, and numpy's `polyval`
  for the line of `lerp` (corroborated by `numpy.interp` inside [0, 1]) and the cubic of `smoothstep`.
  Every angle is committed as the ratio of two integers, so each language forms the same double with
  one division; the committed ratios cover the quarter turns, every eighth of a turn and its nearest
  representable neighbours on both sides, whole degrees, sevenths and thousandths, negative angles and
  angles of more than a hundred thousand turns. The reference itself rounds `2π·t` before it reduces, which costs about
  `1e-15 × |t|`; the comparison tolerance of 1e-9 leaves that an order of magnitude of room. The
  subjects are `@semio-tech/pets` (`sinTurns`, `cosTurns`, `clamp`, `lerp`, `smoothstep`) and the `pets`
  crate (`sin_turns`, `cos_turns`, `clamp`, `lerp`, `smoothstep`), which must agree with each other
  bit for bit.

  That last demand is what the scenario `bit-patterns` holds, as a supplement and not as third-party
  evidence: the oracle adapter restates the reduction and the two polynomials in Python's IEEE doubles,
  operation by operation, holds its results to numpy within 1e-12, and projects the 64-bit pattern of
  every sine and cosine as sixteen hexadecimal digits — text, which compares exactly under every
  profile, where the tolerance of the other scenarios would hide a last-bit difference between
  languages.

  The vectors shared://📐️turn-trigonometry/🔣️.json are generated, never hand-edited, from the Python
  reference in this directory, from the repository root:
  `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/generate_kinematics_vectors.py`
  (`.venv/bin/python` outside Windows).

  @id-angles
  @level-fundamental
  @mode-differential
  Scenario: Sine and cosine of an angle in turns
    Given the committed vectors shared://📐️turn-trigonometry/🔣️.json
    When the sine and the cosine of every committed angle, numerator ÷ denominator turns, are taken with sinTurns and cosTurns
    Then every implementation projects the same sine and cosine per angle within the comparison tolerance

  @id-bit-patterns
  @level-fundamental
  @mode-differential
  Scenario: Sine and cosine have the same 64 bits in every language
    Given the committed vectors shared://📐️turn-trigonometry/🔣️.json
    When the sine and the cosine of every committed angle are taken and written as the sixteen hexadecimal digits of their IEEE bit patterns
    Then every implementation projects the same digits per angle, 0000000000000000 for the sine of a half turn and 3ff0000000000000 for the sine of a quarter turn

  @id-sweeps
  @level-quick
  @mode-differential
  Scenario: Sine and cosine along a sweep of evenly spaced angles
    Given the committed vectors shared://📐️turn-trigonometry/🔣️.json
    When sinTurns and cosTurns are taken of n ÷ denominator turns for every n from first to last of every committed sweep
    Then every implementation projects the same sines and cosines in order, across every eighth of a turn and every whole degree

  @id-clamps
  @level-fundamental
  @mode-differential
  Scenario: A value is held between two bounds
    Given the committed vectors shared://📐️turn-trigonometry/🔣️.json
    When every committed value is clamped between its low and its high bound
    Then every implementation projects the same value, the high bound winning when the bounds cross

  @id-lerps
  @level-fundamental
  @mode-differential
  Scenario: A value is interpolated between two ends
    Given the committed vectors shared://📐️turn-trigonometry/🔣️.json
    When lerp is taken from every committed start to its end at its amount
    Then every implementation projects the same value, extrapolating for amounts outside 0 … 1

  @id-smoothsteps
  @level-fundamental
  @mode-differential
  Scenario: An amount is eased with flat ends
    Given the committed vectors shared://📐️turn-trigonometry/🔣️.json
    When smoothstep is taken of every committed amount
    Then every implementation projects the same eased amount, 0 below 0 and 1 above 1
