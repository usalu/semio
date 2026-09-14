@capability-viz-charts-financial
@oracle-d3-scale
@oracle-d3-array
@comparison-viz-probe-v1
Feature: A candlestick body and a moving average are computed, not drawn by eye
  The `financial` family renders every price chart of taxonomy §17 from one OHLC row store. Two of
  its pieces are pure arithmetic and therefore measurable without rasterising anything: where the
  body of a candle sits, and what the moving-average overlay averages.

  A candle body runs from the open to the close, both mapped through the same linear price scale
  that spans the low/high extent of the whole series; its width is a fixed fraction of the period
  step, and a doji — a period whose open equals its close — still draws a visible 0.4 mm body so
  that it is not silently missing. The oracle builds that rectangle on `d3-scale` alone.

  The moving-average overlay is a TRAILING window: the value at period i is the mean of the closes
  of periods max(1, i-window+1) through i, so the first periods average fewer than `window` values
  instead of disappearing. `d3-array`'s `mean` over the same slices is the oracle.

  @id-candlestick
  @level-long
  @mode-differential
  Scenario: Candle bodies run from open to close on the price scale
    Given the committed probe document local://candlestick.tex and the periods
      | t | open | high | low | close |
      | 1 | 10   | 12   | 9   | 11    |
      | 2 | 11   | 13   | 10  | 10    |
      | 3 | 10   | 11   | 8   | 9     |
      | 4 | 9    | 12   | 9   | 12    |
      | 5 | 12   | 14   | 11  | 13    |
      | 6 | 13   | 13   | 10  | 11    |
      | 7 | 11   | 15   | 11  | 14    |
      | 8 | 14   | 16   | 13  | 15    |
    Then the compiled probe and the reference implementation agree on every candle body
    # The frame is the 80x40 canvas with the family's own padding (10, 4, 4, 8) and the width of a
    # body is 62 % of the period step, which the oracle restates as `(x1 - x0) / periods * 0.62`.

  @id-moving-average
  @level-long
  @mode-differential
  Scenario: The moving-average overlay is a trailing mean
    Given the committed probe document local://moving-average.tex and the closes
      | closes                  | window |
      | 11,10,9,12,13,11,14,15  | 3      |
    Then the compiled probe and the reference implementation agree on every window mean
