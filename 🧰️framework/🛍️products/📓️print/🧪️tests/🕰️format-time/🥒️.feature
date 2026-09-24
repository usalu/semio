@capability-viz-format-time
@oracle-d3-time-format
@comparison-viz-probe-v1
Feature: ISO timestamps are formatted by the d3-time-format grammar in both locales
  `\SemioVizTimeFormat{spec}{iso}` implements d3-time-format's directive grammar over an ISO
  timestamp `YYYY-MM-DD[THH:MM[:SS]]` read as UTC, so the reference implementation is
  `d3-time-format` driven through `utcFormat`. The weekday and the day of the year are derived from
  the Julian day number of the date, and `%U`/`%W` follow the C rule `(yday + 7 - wday) / 7` that
  d3-time-format's Sunday- and Monday-based week counters implement.

  The localized month and weekday names are the two lists the library ships: English and German,
  selected by `\SemioVizLocale` and, in a real document, by the document language. There is no
  default locale in the code.

  The transport encoding is the one of case format-number: a leading `|` keeps every result a JSON
  string, the result is brace wrapped so a comma inside a formatted date survives the comma-list,
  and the padding space of `%e` (and every other space) becomes an underscore.

  @id-en-locale
  @level-quick
  @mode-differential
  Scenario: The English locale renders every directive as d3-time-format does
    Given the committed probe document shared://🕰️format-time/format-time.tex and the directives
      | specifier      | timestamp           |
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
    Then the compiled probe and the reference implementation agree on every rendered string

  @id-de-locale
  @level-quick
  @mode-differential
  Scenario: The German locale renders the localized month and weekday names
    Given the committed probe document shared://🕰️format-time/format-time.tex and the same directives
      | locale | january | saturday | shortMarch |
      | de     | Januar  | Samstag  | Mrz        |
    Then the compiled probe and the reference implementation agree on every rendered string
