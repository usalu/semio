@capability-pets-gaze-tracking
@oracle-pets-numpy
@comparison-pets-float-v1
Feature: A pet's pupils are drawn towards what it looks at
  A pet that stands still follows the pointer with its eyes (design §5.2): each tick the gaze is given
  a target, and `lookOffset(eye, target, reach)` says where the pupil belongs inside the white of the
  eye (design §4.3): the direction from the eye to the target, scaled to the length `d ÷ (d + reach)`
  for their distance `d`. The offset therefore lies inside the unit disc whatever the distance — half
  way out when the target is `reach` away, close to the rim when it is far — and it is `(0, 0)` when
  the two points coincide. A reach that is not positive counts as 0 and puts the pupil on the rim. The
  eye itself sits on a bone: `transform(bone matrix, eye.x, eye.y)` carries it into the pet's frame
  before it looks. The spring that moves the pupil towards the offset is specified by the
  🪀️spring-settling case.

  THE REFERENCE is numpy: the distance is `numpy.linalg.norm` of the difference of the two points, the
  offset the difference divided by that norm and scaled by `d ÷ (d + reach)`, and the carried eye the
  product of the bone's 3×3 homogeneous matrix with `[x, y, 1]`. The oracle refuses any offset longer
  than 1. The subjects are `@semio-tech/pets` (`lookOffset`, `transform`) and the `pets` crate
  (`look_offset`, `transform`).

  The twins must agree with each other bit for bit (design §2.4), which a tolerance cannot show.
  `bit-patterns` holds that, as a supplement and not as third-party evidence: the oracle adapter
  restates `lookOffset` and `transform` in Python's IEEE doubles, holds every result to numpy within
  1e-12, and projects the 64-bit pattern of every number as sixteen hexadecimal digits — text, which
  compares exactly under every profile.

  The vectors shared://👀️gaze-tracking/🔣️.json are generated, never hand-edited, from the Python
  reference in this directory, from the repository root:
  `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/generate_kinematics_vectors.py`
  (`.venv/bin/python` outside Windows).

  @id-offsets
  @level-fundamental
  @mode-differential
  Scenario: The look offset points at the target and stays inside the unit disc
    Given the committed vectors shared://👀️gaze-tracking/🔣️.json
    When lookOffset is taken from every committed eye to its target with its reach
    Then every implementation projects the same offset per vector, 0 0 for coinciding points and a length of one half for a target that is the reach away

  @id-eyes
  @level-fundamental
  @mode-differential
  Scenario: An eye on a posed bone looks at a target in the pet's frame
    Given the committed vectors shared://👀️gaze-tracking/🔣️.json
    When every committed eye is carried by its bone's matrix and lookOffset is taken from the carried eye to its target
    Then every implementation projects the same carried eye and the same offset per vector

  @id-bit-patterns
  @level-fundamental
  @mode-differential
  Scenario: Offsets and carried eyes have the same 64 bits in every language
    Given the committed vectors shared://👀️gaze-tracking/🔣️.json
    When every committed offset and every committed eye on a posed bone is computed and each number is written as the sixteen hexadecimal digits of its IEEE bit pattern
    Then every implementation projects the same digits per offset and per eye
