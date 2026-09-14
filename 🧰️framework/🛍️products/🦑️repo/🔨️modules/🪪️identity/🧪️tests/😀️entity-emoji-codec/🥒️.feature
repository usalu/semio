@capability-repo-identity-leading-grapheme
@oracle-intl-segmenter
@comparison-ordered-json-v1
Feature: The leading emoji grapheme ends where Unicode says it ends
  A semantic id is a run of emoji-tagged segments, so everything downstream depends on where the
  leading emoji grapheme of a string ends. That boundary is Unicode's, not this repository's: UAX #29
  extended grapheme cluster segmentation decides it, including zero-width joiner sequences, keycaps,
  regional-indicator pairs, skin-tone modifiers and variation selectors. A string that does not start
  with an emoji yields no emoji and the whole string as the remainder.

  @id-the-leading-emoji-grapheme-is-unicodes
  @level-fundamental
  @mode-differential
  @seed-1
  Scenario: Every vector splits at the same boundary
    Given the shared vector set shared://📡️emoji-vectors.json
    When the host splits the leading emoji grapheme off every vector
    Then every implementation projects the same emoji and remainder per vector
