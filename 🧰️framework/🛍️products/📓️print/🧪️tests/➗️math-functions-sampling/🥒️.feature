@capability-viz-scientific-function-sampling
@oracle-mathjs
@comparison-viz-probe-v1
Feature: Function sampling, adaptive refinement and Riemann sums are computed, not sketched
  `semio-viz-scientific-field` samples an l3fp expression over a domain, refines the sample where the
  curve leaves its own chord, and integrates it by the four Riemann rules. Every §25 function kind of
  the catalogue is drawn from those three routines, so if they are right the whole family is right and
  if they are wrong every curve in the section is wrong in the same way.

  The reference is `mathjs`, the registered oracle. It parses and compiles the very expression strings
  the fixture hands the LaTeX subject — `sin(x)`, `exp(-x)*cos(4x)`, `x^3-3x`, `x^2`, `exp(x)` and
  `sin(4x)` — and every function value the oracle needs comes out of that compiled program, so the
  sampled curve is a differential comparison between an l3fp expression parser in fixed point and a
  third-party expression parser in IEEE double, over the same abscissae. The abscissae matter: both
  sides evaluate the partition the feature's own `samples` column defines, so no adaptive grid of the
  oracle's own can straddle the probe's emission grid and force a widened tolerance.

  The Riemann scenario and the refinement scenario are conformance and say so in their mode tags: the
  four rules and the sagitta criterion are specifications written down in the taxonomy, not something a
  second library recomputes, and the exact integrals in the table are the closed-form antiderivatives
  the sums must bracket. `mathjs` supplies their integrand values; the rule arithmetic is the
  specification itself.

  Numbers are emitted rounded to six decimals, the probe protocol's emission grid.

  @id-uniform-sampling
  @level-quick
  @mode-differential
  Scenario: Uniform sampling puts n+1 points on the domain and evaluates the expression at each
    Given the committed probe document shared://➗️math-functions-sampling/math-functions-sampling.tex and the sampled functions
      | key            | expression                 | from | to          | samples |
      | sample/sine    | sin(x)                     | 0    | pi          | 8       |
      | sample/damped  | exp(-x)*cos(4x)            | 0    | 2           | 10      |
      | sample/cubic   | x^3-3x                     | -2   | 2           | 8       |
    Then the compiled probe and the reference implementation agree on every value

  @id-riemann-rules
  @level-quick
  @mode-conformance
  Scenario: The four Riemann rules bracket the exact integral the way the rules say they must
    Given the committed probe document shared://➗️math-functions-sampling/math-functions-sampling.tex and the integrals
      | key                  | integrand | from | to | bins | exact                |
      | riemann/square       | x^2       | 0    | 1  | 8    | 0.3333333333333333   |
      | riemann/sine         | sin(x)    | 0    | pi | 12   | 2                    |
      | riemann/exponential  | exp(x)    | 0    | 2  | 16   | 6.38905609893065     |
    Then the compiled probe and the reference implementation agree on every value

  @id-adaptive-refinement
  @level-quick
  @mode-conformance
  Scenario: Refinement inserts a midpoint exactly where the chord sagitta exceeds the tolerance
    Given the committed probe document shared://➗️math-functions-sampling/math-functions-sampling.tex and the refinement budget
      | key                     | expression | samples | passes | tolerance |
      | refine/pass-one         | sin(4x)    | 4       | 1      | 0.05      |
      | refine/pass-two-count   | sin(4x)    | 4       | 2      | 0.05      |
    Then the compiled probe and the reference implementation agree on every value
