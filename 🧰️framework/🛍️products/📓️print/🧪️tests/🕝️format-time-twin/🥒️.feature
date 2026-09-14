@capability-viz-format-time-twin
@oracle-d3-time-format
@comparison-viz-probe-v1
Feature: The TypeScript twin renders every time directive as d3-time-format does
  `🕰️format-time` measures the LaTeX kernel. This case measures the second subject of the same
  specification, the TypeScript twin `@semio-tech/print-viz-kernel`, on the same directive list and
  against the same oracle, because a kernel that exists twice is only a kernel if both copies answer
  alike.

  The platform gives a case one adapter per language and one subject per scenario, so the twin cannot
  share `🕰️format-time`'s adapter with the LaTeX probe subject. It gets its own case instead; the
  directives below are the directives of `🕰️format-time` verbatim.

  One thing is deliberately not shared: the clock. The LaTeX formatter is given calendar fields and
  no zone, so `🕰️format-time` reads them through d3's `utcFormat`; the twin's `formatVizTime` reads a
  JavaScript `Date` through its local getters, exactly as d3's plain `format` does. This case
  therefore adjudicates with `d3-time-format`'s local `format` over the same locale definitions and
  the same instants, which is the same oracle asked the question the twin is actually answering.
  `%Z` is in the list precisely because it is the directive where the two clocks are visible.

  Both sides are held to the probe protocol's string transport — a leading pipe, spaces written as
  underscores — so neither is given a wider target than the other.

  @id-en-locale
  @level-quick
  @mode-differential
  Scenario: The English locale renders every directive as d3-time-format does
    Given the directives
      | specifier      | instant             |
      | %Y-%m-%d       | 2026-09-05          |
      | %A %B %e, %Y   | 2026-09-05          |
      | %a %b %d       | 2024-02-29T13:07:09 |
      | %H:%M:%S %p    | 2024-02-29T13:07:09 |
      | %j %U %W %Z    | 2024-02-29T13:07:09 |
      | %y/%I %p       | 1999-01-03T00:30    |
      | %A            | 2000-01-01          |
      | %B %Y          | 2026-03-01          |
      | %d.%m.%Y       | 2026-12-31          |
      | %U %W          | 2021-01-01          |
      | %j            | 2020-12-31          |
      | 100%% %b       | 2026-05-04          |
    Then the twin kernel and the reference implementation agree on every rendered string

  @id-de-locale
  @level-quick
  @mode-differential
  Scenario: The German locale renders the localized month and weekday names
    Given the same directives with the German calendar names
    Then the twin kernel and the reference implementation agree on every rendered string
