Feature: Localized compiler source work
  Scenario: A source unit begins or reads its bounded files
    Given an authentic Script progress publication
    And a closed Cargo source-admission work record
    When its unit and cumulative work counters are formatted
    Then the developer reads the selected English or German phase and unit total
    And the message includes reads, files and work bytes
    And the raw structured event and original cancellation authority remain unchanged

  Scenario: A source record is malformed or has impossible totals
    Given an oversized, unknown, malformed or inconsistent source work payload
    When process progress is published
    Then no friendly source text is inferred
    And the raw structured event remains available
