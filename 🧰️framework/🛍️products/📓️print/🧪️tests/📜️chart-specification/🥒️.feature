@domain-print @case-chart-specification @oracle-ajv
Feature: Authored Chart Specification
  The mutation and inference surface shares one schema-first chart contract.

  @id-chart-specification-contract @mode-differential @level-quick
  Scenario: Schema Admission Matches the Independent Validator
    Given the neutral chart specification vectors
    When the shared inference validator admits each authored chart
    Then its admission matches JSON Schema and the expected validity
