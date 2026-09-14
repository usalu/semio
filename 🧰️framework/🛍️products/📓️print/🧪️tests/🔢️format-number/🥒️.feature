@capability-viz-format-number
@oracle-d3-format
@comparison-viz-probe-v1
Feature: Numbers are formatted by the d3-format grammar in both locales
  `\SemioVizFormat{spec}{value}` is d3-format's specifier grammar
  `[[fill]align][sign][symbol][0][width][,][.precision][~][type]` reimplemented on l3fp, so the
  reference implementation is d3-format itself. The same specifier list runs through the same
  values twice, once with the English locale and once with the German one, and every rendered
  string is compared character by character.

  The library ships no default locale: `\SemioVizLocale` sets it explicitly, and in a real document
  the document language does. The two locale definitions are d3-format's own — decimal point and
  thousands separator swapped for German, `−` (U+2212) as the minus sign in both, which is the
  typographically correct minus for print and d3-format's own default.

  Three encodings keep the probe protocol lossless and are undone on the oracle side, so the
  comparison stays exact rather than being weakened to make the transport work:
  a leading `|` forces every result to travel as a JSON string, so `4.200e+1` is compared as those
  eight characters instead of collapsing to the number 42; the result is brace wrapped so that a
  grouped number keeps its separators inside one comma-list item; and the fill spaces of a padded
  result become underscores.

  Exact decimal ties round away from zero. l3fp is a decimal arithmetic, IEEE-754 doubles are not,
  so a vector whose decimal expansion sits exactly on a rounding boundary of a *non-tie* double
  would disagree in the last digit; the vectors below avoid that ambiguity, and `.0f` of `2.5`
  pins the tie rule itself.

  @id-en-locale
  @level-quick
  @mode-differential
  Scenario: The English locale renders every specifier as d3-format does
    Given the committed probe document local://format-number.tex and the specifiers
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
    Then the compiled probe and the reference implementation agree on every rendered string

  @id-de-locale
  @level-quick
  @mode-differential
  Scenario: The German locale swaps the decimal and grouping separators
    Given the committed probe document local://format-number.tex and the same specifiers
      | locale | decimal | thousands |
      | de     | ,       | .         |
    Then the compiled probe and the reference implementation agree on every rendered string
