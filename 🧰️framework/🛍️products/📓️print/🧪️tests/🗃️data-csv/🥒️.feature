@capability-viz-data-csv
@oracle-d3-dsv
@comparison-viz-probe-v1
Feature: A delimiter-separated file loads into a table exactly as d3-dsv parses it
  `\SemioVizTableFromCSV{name}{file}[delimiter=,quote=,header=,columns=]` reads a file with l3file
  and splits every record with RFC 4180 quoting: a quoted field may contain the delimiter, and a
  doubled quote inside a quoted field is one literal quote. `d3-dsv` implements the same rule, so
  it is the reference — over the *same committed file*, not over a re-typed copy of it.

  Two options are exercised beyond the default: a different delimiter, and a headerless file, whose
  columns the loader names `c1`, `c2`, … positionally.

  Cell text travels brace wrapped and prefixed with a vertical bar, so a cell that contains the
  delimiter — or that looks like a number — is compared verbatim rather than parsed. Empty cells
  are not part of a column's value list: `\semio_viz_table_col_values:nnN` collects the cells that
  carry a value, which is what a renderer needs, so the reference filters the same way.

  @id-header-row
  @level-quick
  @mode-differential
  Scenario: The first record names the columns
    Given the committed probe document local://data-csv.tex and the committed file local://cities.csv
      | columns               |
      | city,population,note  |
    Then the compiled probe and the reference implementation agree on the columns and every cell

  @id-quoted-fields
  @level-quick
  @mode-differential
  Scenario: A quoted field keeps its delimiter and unescapes its doubled quotes
    Given the committed probe document local://data-csv.tex and the committed file local://cities.csv
      | row | note         |
      | 1   | Leine, Ihme  |
      | 2   | say "moin"   |
      | 3   | plain        |
      | 4   |              |
    Then the compiled probe and the reference implementation agree on every cell

  @id-custom-delimiter
  @level-quick
  @mode-differential
  Scenario: A headerless file with a semicolon delimiter gets positional column names
    Given the committed probe document local://data-csv.tex and the committed file local://places.tsv
      | delimiter | header | columns  |
      | ;         | false  | c1,c2,c3 |
    Then the compiled probe and the reference implementation agree on the columns and every cell
