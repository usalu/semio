@capability-pets-spring-settling
@oracle-pets-numpy
@comparison-pets-float-v1
Feature: A damped spring advances tick by tick and the gaze spring settles calmly
  The pupils of a pet follow what it looks at through a spring per axis (design §4.4, §5.2).
  `springStep(position, velocity, target, stiffness, damping)` is one tick (`dt = 1/64` s) of
  semi-implicit Euler: the velocity first, `velocity + (stiffness × (target − position) − damping ×
  velocity) × dt`, then the position with the new velocity, `position + velocity′ × dt`. A spring at its
  target without velocity stays there exactly. Every motion dies out while `stiffness > 0`, `damping > 0`
  and `stiffness ÷ 4096 + damping ÷ 32 < 4`. The gaze spring is `GAZE_STIFFNESS = 512`, `GAZE_DAMPING = 32`
  (one tick is the matrix `[[0.875, 0.0078125], [−8, 0.5]]`, exact in binary): calm and slightly springy.

  THE REFERENCE is `🐍️.py` beside this file. One tick is a linear map of `(position − target, velocity)`;
  the oracle writes it as a 2×2 matrix from the design text and lets `numpy.linalg.matrix_power` take
  `n` ticks at once, which the subjects must reach by `n` single steps — and the oracle's own `n` single
  matrix products must land on the same state before it answers. `numpy.linalg.eigvals` judges the
  stated stability region: the spectral radius of the step matrix is below 1 for every committed spring
  inside it and at least 1 for every committed spring outside it (without damping, on the edge, too
  stiff, too heavy, pushing, repelling, without stiffness). Everything compares under `pets-float-v1`
  (1e-9). The subjects are `springStep`, `GAZE_STIFFNESS` and `GAZE_DAMPING` of `🔨️modules/🎞️animation`
  in `@semio-tech/pets` and `spring_step`, `GAZE_STIFFNESS`, `GAZE_DAMPING` of the `pets` crate.

  The settling properties are stated as shares of the way of a pupil that rests when its target jumps:
  it stays within 1 % of the way from the 16th tick (0.25 s) at the latest and never overshoots by more
  than 2 % of the way. The oracle refuses vectors that break either bound and projects the tick from
  which the pupil stays inside the band and its largest overshoot, over 128 ticks. The subjects project
  the same from their own constants together with the constants themselves, so a spring tuned in one
  language only, or without regenerating the vectors, fails the case.

  The vectors shared://🪀️spring-settling/🔣️.json are generated, never hand-edited, from the Python
  reference in this directory, from the repository root:
  `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/generate_animation_vectors.py`
  (`.venv/bin/python` outside Windows).

  @id-single-steps
  @level-fundamental
  @mode-differential
  Scenario: One tick moves the velocity first and the position with the new velocity
    Given the committed vectors shared://🪀️spring-settling/🔣️.json
    When every committed spring is advanced once with springStep
    Then every implementation projects the same position and velocity per spring

  @id-tick-runs
  @level-fundamental
  @mode-differential
  Scenario: Many single ticks equal the power of the one-tick matrix
    Given the committed vectors shared://🪀️spring-settling/🔣️.json
    When every committed spring is advanced by each committed number of single ticks
    Then every implementation projects the position and velocity the matrix power yields for that many ticks at once

  @id-resting
  @level-fundamental
  @mode-property
  Scenario: A spring at its target without velocity never leaves it
    Given the committed vectors shared://🪀️spring-settling/🔣️.json
    When every committed spring starts on its target at rest and is advanced by its committed ticks
    Then every implementation projects exactly the target and a velocity of 0

  @id-stable-springs
  @level-fundamental
  @mode-property
  Scenario: Every motion dies out inside the stated stability region
    Given the committed vectors shared://🪀️spring-settling/🔣️.json
    When every committed spring inside the region is released one unit from its target and advanced by its committed ticks
    Then every implementation projects the same faded position and velocity, and the oracle finds a spectral radius below 1 inside the region and none below 1 outside it

  @id-gaze-settling
  @level-fundamental
  @mode-property
  Scenario: The gaze spring settles within a quarter of a second and barely overshoots
    Given the committed vectors shared://🪀️spring-settling/🔣️.json
    When a resting pupil follows every committed jump of its target with the gaze spring for the committed horizon
    Then every implementation projects the gaze constants, the tick from which the pupil stays within 1 % of the way (the 16th at the latest) and its overshoot (2 % of the way at most)
