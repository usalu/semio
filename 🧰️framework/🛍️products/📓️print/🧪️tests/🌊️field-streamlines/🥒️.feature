@capability-field-integration
@no-oracle-rk-field-integration
@comparison-viz-probe-v1
Feature: The streamline integrator of semio-viz-geo-routes reproduces classical RK4
  A streamline is the trajectory a massless particle takes through a plane vector field, and
  `spatial-vector-field` draws one by integrating the field with the classical fourth-order
  Runge–Kutta step: four slope samples `k1 = f(y)`, `k2 = f(y + h k1 / 2)`, `k3 = f(y + h k2 / 2)`,
  `k4 = f(y + h k3)`, advanced by `h (k1 + 2k2 + 2k3 + k4) / 6`. The quiver renderer samples the
  same `f`, so an arrow and a streamline through the same point always agree.

  There is no d3 module for this. d3 has no ODE integrator at all — `d3-force` steps a velocity
  Verlet simulation with damping, which is a different scheme solving a different problem, and using
  it here would compare two things that are not the same function. The recorded decision
  `rk-field-integration` therefore names `independent-implementations` as its substitute: the
  oracle is a second implementation of RK4 written in TypeScript in the adapter, over the same five
  analytic fields, with the same seeds and the same step size. Two independent programs, one
  specification, four hundred sampled coordinates.

  That is a real test and not a tautology because the two implementations differ in everything
  except the formula: expl3 fixed point against IEEE double, macro expansion against expressions,
  and a field dispatch written once as a string switch and once as an object lookup. An error in the
  coefficients, in the order of the four slopes, or in the half-step of `k2` and `k3` shows up in the
  third sample; an error in a field's own algebra shows up in the first.

  The five fields are the ones the family offers: `shear` (sin y, sin x), `source` (x, y),
  `vortex` (−y, x), `saddle` (x, −y) and `dipole`, whose denominator is clamped away from the
  singularity at the origin by `max(0.05, (x² + y²)²)` — the clamp is part of the specification and
  the oracle applies it identically.

  The integrator clamps its state to `[-3.4, 3.4] × [-2.4, 2.4]` after every step, because `source`,
  `saddle` and `dipole` are divergent and would otherwise leave TeX's dimension range within a dozen
  steps. That clamp is part of the specification, not a rendering detail, so the oracle applies it
  identically — and the seeds below are chosen so that three of the five trajectories actually reach
  the boundary, which makes the clamp itself part of what is measured.

  Probe document: shared://🌊️field-streamlines/streamlines.tex is committed.

  @id-rk-integration
  @level-quick
  @mode-conformance
  Scenario: Sixteen RK4 samples of five analytic fields agree with the published fourth-order scheme
    Given the committed probe document shared://🌊️field-streamlines/streamlines.tex and the seeds
      | field  | x    | y     | dt   |
      | shear  | -1.5 | 0.75  | 0.25 |
      | vortex | 1.25 | -0.5  | 0.2  |
      | source | 0.4  | 0.3   | 0.1  |
      | saddle | 0.6  | 1.1   | 0.15 |
      | dipole | 1.6  | 0.9   | 0.05 |
    Then the compiled probe and the independent implementation agree on every sampled position
