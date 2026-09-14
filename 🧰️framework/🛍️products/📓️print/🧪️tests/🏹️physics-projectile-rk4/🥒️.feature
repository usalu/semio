@capability-viz-scientific-physics
@no-oracle-projectile-integration
@comparison-viz-probe-v1
Feature: Trajectories are integrated and land where the closed form says they land
  `sci-kinematics` integrates a projectile with a velocity-Verlet step and `semio-viz-scientific-field`
  integrates arbitrary planar systems with fourth-order Runge–Kutta. Both are drawn through the same
  millimetre projection, so this case measures the projected geometry: the numbers are the ones the
  page actually receives, not an intermediate the renderer might ignore.

  The projectile reference is the closed-form parabola, which the velocity-Verlet step reproduces
  exactly for a constant acceleration. The Runge–Kutta reference is the analytic solution of the
  harmonic oscillator, so the comparison also measures the integrator's own truncation error; the
  emission grid of five decimals is what a step of 0.1 over two radians can hold. The damped scenario
  is a property test: a system with negative divergence may never gain energy.

  `mathjs` would provide an independent ODE integrator; it is not in the print oracle registry yet and
  the request is recorded in the ticket's SCIENTIFIC status note.

  @id-projectile-path
  @level-long
  @mode-conformance
  Scenario: The projected projectile path is the closed-form parabola in millimetres
    Given the committed probe document local://physics-projectile-rk4.tex and the launch
      | speed | angle | gravity | step | steps |
      | 22    | 52    | 9.81    | 0.05 | 40    |
    Then the compiled probe and the reference implementation agree on every value

  @id-harmonic-orbit
  @level-quick
  @mode-conformance
  Scenario: Runge–Kutta follows the analytic circle of the harmonic oscillator
    Given the committed probe document local://physics-projectile-rk4.tex and the systems
      | key             | derivative-x | derivative-y | x0 | y0 | step | steps | analytic      |
      | rk4/orbit       | y            | -x           | 1  | 0  | 0.1  | 20    | cos t, -sin t |
      | rk4/orbit-fine  | y            | -x           | 1  | 0  | 0.05 | 40    | cos t, -sin t |
    Then the compiled probe and the reference implementation agree on every value

  @id-damped-decay
  @level-quick
  @mode-property
  Scenario: A damped oscillator never gains energy along its own trajectory
    Given the committed probe document local://physics-projectile-rk4.tex and the damped system
      | key         | derivative-x | derivative-y  | x0 | y0 | step | steps |
      | rk4/damped  | y            | -x - 0.4 y    | 1  | 0  | 0.1  | 30    |
    Then the compiled probe and the reference implementation agree on every value
