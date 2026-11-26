Feature: Chart Primitives Of The Terminal Target
  Sparkline, line chart, gauge and stat cell turn multi-resolution series into a cell grid. A bucket carries min, max, mean and
  last; a gap is a bucket without data. The scaling math is pinned by 🧫️fixtures/📈️charts/🔣️.json, written by an independent
  Python implementation, so any language implementation can pass the same vectors.

  Scenario: A sparkline scales to the data range
    Given a series of one value per second from 0 to 7
    When it is drawn 8 cells wide
    Then it shows the eight block glyphs from the lowest to the highest

  Scenario: A gap has its own glyph
    Given a series with no data in two of its buckets
    When it is drawn
    Then those columns show the gap glyph and not the lowest block

  Scenario: A flat series sits in the middle and a single point is a flat series
    Given a series whose values are all equal
    When it is drawn
    Then every column shows the middle block

  Scenario: Coarse old buckets keep their share of time, or of the index
    Given a series whose old buckets last 60 seconds and whose recent buckets last 1 second
    When the horizontal axis is proportional to time
    Then the old tier fills most of the width
    And when the axis is proportional to the bucket index every bucket gets the same width and a separator marks each tier boundary

  Scenario: A gauge states its level in words
    Given a value against a capacity with warn and critical thresholds
    When the value crosses a threshold
    Then the gauge shows the localized word of the level besides its colour

  Scenario: Values carry their unit
    Given a byte count, a percentage, a rate and a duration
    When they are formatted for English or German
    Then the unit steps, the decimal separator and the unit words follow the language
