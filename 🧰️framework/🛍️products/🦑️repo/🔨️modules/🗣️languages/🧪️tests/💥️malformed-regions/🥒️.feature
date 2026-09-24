@capability-repo-malformed-regions
@no-oracle-repo-region-markers
@comparison-ordered-json-v1
Feature: Malformed region markers degrade the same way in every implementation
  Region markers are written by hand, so they are unbalanced in real files. The parser never throws
  and never guesses: an unclosed region runs to the end of the file, a closing marker with nothing
  open is ignored, a marker whose name is only an emoji keeps the raw text as the name, and a file
  whose extension no language claims yields no sections at all. Nothing third party implements this
  recovery, so the recorded no-oracle decision `repo-region-markers` applies here too.

  @id-unclosed-region-runs-to-the-end
  @level-fundamental
  @mode-error
  Scenario: An unclosed region ends at the last line of the file instead of failing
    Given the case source shared://💥️malformed-regions/❌️unclosed.ts
    When each implementation parses it
    Then every implementation reports one section ending at the last line, with no error

  @id-stray-endregion-is-ignored
  @level-fundamental
  @mode-error
  Scenario: A closing marker with nothing open is ignored instead of failing
    Given the case source shared://💥️malformed-regions/❌️stray-end.ts
    When each implementation parses it
    Then every implementation reports the sections that were actually opened, with no error

  @id-unclaimed-extension-has-no-sections
  @level-fundamental
  @mode-error
  Scenario: A file whose extension no language claims yields no sections
    Given the case source shared://💥️malformed-regions/❌️unclaimed.unknown
    When each implementation parses it by path
    Then every implementation reports no sections, with no error
