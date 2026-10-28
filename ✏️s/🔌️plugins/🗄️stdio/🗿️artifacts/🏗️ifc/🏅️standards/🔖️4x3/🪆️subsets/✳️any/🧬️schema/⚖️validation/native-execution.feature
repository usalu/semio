Feature: Native EXPRESS semantic execution
  Scenario: Native expressions retain the shared scalar outcomes
    Given every parsed expression in cases.json
    When native evaluation completes within its controls
    Then its value equals the shared expected result
    And integer arithmetic remains exact within its declared checked domain

  Scenario: Evaluation limits cannot produce false validity
    Given integer overflow or exhausted work node depth or iteration budgets
    When native semantic evaluation runs
    Then it reports an explicit incomplete result
    And a population cannot be valid

  Scenario: The adversarial population agrees with the independent oracle
    Given the STEP population in models.json
    When native WHERE UNIQUE inverse bound derived and global rules execute
    Then the named failures equal the IfcOpenShell expectations

  Scenario: Cancellation is observed after reentrant progress callbacks
    Given a caller-controlled cancellation callback
    When construction or evaluation progress cancels the operation
    Then evaluation stops before publishing a valid result

  Scenario: Semantic graph validation leaves physical STEP conformance explicit
    Given a graph normalized from a complete complex subtype chain
    When the native retained EXPRESS program validates that graph
    Then the report marks physical conformance as not assessed
    And schema semantic validity does not certify its physical Part21 encoding

  Scenario: Revision-cached derived and inverse queries use one prepared context
    Given a structurally admitted graph owned by a prepared native context
    When computed attributes are requested by source entity identity
    Then every query observes the caller cancellation and work controls
    And immutable query values preserve source entity identities
    And the original graph remains unchanged
