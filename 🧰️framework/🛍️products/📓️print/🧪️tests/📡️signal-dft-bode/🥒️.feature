@capability-viz-scientific-signal
@oracle-fft-js
@comparison-viz-probe-v1
Feature: The discrete Fourier transform, the transfer function and the root finder are real computations
  Every §31 and §32 kind of the catalogue reduces to three routines in `semio-viz-scientific-field`:
  an O(n²) discrete Fourier transform, a complex evaluation of a rational transfer function on the
  imaginary axis, and Durand–Kerner root finding. A spectrogram, a Bode plot and a root locus are only
  as correct as those three, so they are measured directly rather than through a rendered picture.

  The transform is measured differentially against `fft.js`, the registered oracle: the adapter samples
  the same three signals, runs `fft.js`'s radix-4 real transform over them, and reads the one-sided
  amplitude spectrum `2/N · |X_k|` and — through the Wiener–Khinchin identity on a zero-padded signal —
  the same biased autocorrelation the probe accumulates directly. Two genuinely different algorithms
  therefore produce the numbers: an O(n²) sum in expl3 fixed point against a radix-4 butterfly in IEEE
  double.

  The other two scenarios are conformance, not differential, and say so in their mode tag: their
  reference is a closed form written down in advance — |H(jω)| of a rational function evaluated on the
  imaginary axis, and polynomials chosen so that their roots can be stated exactly — which is a
  specification vector rather than a second implementation, and no FFT library can serve as one.

  @id-dft-magnitudes
  @level-quick
  @mode-differential
  Scenario: A tone at a bin centre transforms to exactly its own amplitude in that bin
    Given the committed probe document local://signal-dft-bode.tex and the sampled signals
      | key               | signal                          | samples | rate | expectation                    |
      | dft/two-tones     | sin(2 pi 3 t) + 0.5 sin(2 pi 7t)| 32      | 32   | 1 in bin 3, 0.5 in bin 7, else 0 |
      | dft/constant      | 1                               | 16      | 16   | 2 in bin 0, else 0             |
      | correlation/cosine| cos(2 pi 2 t)                   | 16      | 16   | a cosine of the same period    |
    Then the compiled probe and `fft.js` agree on every value

  @id-transfer-function
  @level-quick
  @mode-conformance
  Scenario: The transfer function on the imaginary axis matches its closed-form magnitude and phase
    Given the committed probe document local://signal-dft-bode.tex and the transfer functions
      | key                 | numerator | denominator | frequencies              |
      | bode/second-order   | 1         | 1, 0.4, 1   | 0.1, 0.5, 1, 2, 10       |
      | bode/bandpass       | 1, 0      | 1, 3, 2     | 1, 4                     |
    Then the compiled probe and the reference implementation agree on every value

  @id-polynomial-roots
  @level-quick
  @mode-conformance
  Scenario: Durand–Kerner finds the roots of polynomials whose roots are known
    Given the committed probe document local://signal-dft-bode.tex and the polynomials
      | key               | coefficients | roots                       |
      | roots/cubic       | 1,-6,11,-6   | 1, 2, 3                     |
      | roots/quadratic   | 1,0.4,1      | -0.2 ± 0.9797958971132712 i |
      | roots/real-pair   | 1,3,2        | -2, -1                      |
    Then the compiled probe and the reference implementation agree on every value
