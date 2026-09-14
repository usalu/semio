@capability-viz-format-number-twin
@oracle-d3-format
@comparison-viz-probe-v1
Feature: The TypeScript twin formats numbers by the d3-format grammar in both locales
  `format-number` measures the LaTeX kernel. This case measures the second subject of the same
  specification, the TypeScript twin `@semio-tech/print-viz-kernel`, on the same vectors and against
  the same oracle, because a kernel that exists twice is only a kernel if both copies answer alike.

  The platform gives a case one adapter per language and one subject per scenario, so the twin cannot
  share `format-number`'s adapter with the LaTeX probe. It gets its own case instead; the vectors
  below are the vectors of `format-number` verbatim, and the adapter reuses that case's oracle
  handlers unchanged so the two subjects are measured against literally the same reference strings.

  The library ships no default locale: the twin takes the document language as an argument, and in a
  real document the document language supplies it. The two locale definitions are d3-format's own —
  decimal point and thousands separator swapped for German, `−` (U+2212) as the minus sign in both,
  which is the typographically correct minus for print and d3-format's own default.

  The transport encoding of the probe protocol is kept on both sides so the twin is compared through
  the same channel the LaTeX subject is: a leading `|` forces every result to travel as a JSON
  string, so `4.200e+1` is compared as those eight characters instead of collapsing to the number 42,
  and the fill spaces of a padded result become underscores.

  @id-en-locale
  @level-quick
  @mode-differential
  Scenario: The English locale renders every specifier as d3-format does
    Given the specifiers
      | specifier | value       |
      | .2f       | 3.14159     |
      | .0f       | 2.5         |
      | d         | 42.6        |
      | ,         | 1234567.891 |
      | ,.2f      | 1234567.891 |
      | .3e       | 42          |
      | .3s       | 1234567     |
      | .2s       | 0.00042     |
      | .1%       | 0.234       |
      | .2p       | 0.1234      |
      | .3r       | 123456      |
      | .4g       | 0.00012367  |
      | .3~f      | 1.5         |
      |           | 1234.5678   |
      | +.2f      | 3.14159     |
      | 08.2f     | -3.14159    |
      | >10.2f    | 3.1         |
      | <10.2f    | 3.1         |
      | ^10.2f    | 3.1         |
      | =10.2f    | -3.1        |
      | 012,.2f   | 1234.5      |
      | (.2f      | -3.5        |
      | .2f       | -0.001      |
      | x         | 255         |
      | X         | 255         |
      | b         | 10          |
      | o         | 64          |
      | c         | unit        |
    Then the twin kernel and the reference implementation agree on every rendered string

  @id-de-locale
  @level-quick
  @mode-differential
  Scenario: The German locale swaps the decimal and grouping separators
    Given the same specifiers
      | locale | decimal | thousands |
      | de     | ,       | .         |
    Then the twin kernel and the reference implementation agree on every rendered string
