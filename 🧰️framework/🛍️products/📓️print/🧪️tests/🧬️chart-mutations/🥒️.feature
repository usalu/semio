Feature: Print Chart Mutation and Inference
  Scenario: Customization is replayed as guarded events
    Given the chart-mutations language-neutral fixture
    When its chart mutations are replayed
    Then its authored dimensions and explicit language match the expected values
    And the inferred numeric placement matches d3-scale
    And inverse diffs restore the original chart

  Scenario: Invalid edits are rejected atomically
    Given the chart-mutations language-neutral fixture
    When each malformed address is submitted
    Then the mutation outcome contains a rejection and an empty diff

  Scenario: Inference is deterministic and cancellation-aware
    Given the chart-mutations language-neutral fixture
    When a native inference service is dispatched twice
    Then its canonical TikZ payloads are equal
    And cancellation at a checkpoint prevents publishing a partial result

  Scenario: Authored CSS paint semantics agree across inference languages
    Given the neutral paint vectors and shared CSS named-color contract
    When native and numerical chart inference resolve each paint
    Then RGB aliases and fractional alpha match d3-color
    And ordinary table and annotation text remains unchanged

  Scenario: Valid and invalid inference outputs share a closed contract
    Given the authored chart and a chart without an explicit language
    When inference runs through the print plugin's registered service
    Then each result validates against the same inference output schema
    And invalid results contain typed diagnostics, empty TikZ and complete false

  Scenario: A user cancels expensive numerical inference from the event loop
    Given a valid point chart with 20000 rows
    When inference reports work has started and an independent event-loop timer aborts it
    Then the timer runs before inference completes
    And the cancelled result has empty TikZ, no plan and no scene
    And its diagnostics validate against the shared inference output schema

  Scenario: A user cancels a numerical layout after validation
    Given a valid point chart with 20000 rows
    When inference reports layout progress and an independent event-loop timer aborts it
    Then cancellation interrupts the owned worker before publication
    And the cancelled result contains no partial drawing outputs

  Scenario: Successful asynchronous inference publishes deterministic output
    Given the chart-mutations language-neutral fixture
    When chart inference is awaited twice
    Then both complete TikZ outputs match
    And progress starts at zero, strictly increases and reaches its unchanged total

  Scenario: Cancellation at completed progress prevents result publication
    Given the chart-mutations language-neutral fixture
    When the caller aborts after progress reaches its total
    Then inference returns a cancelled result with no partial drawing outputs

  Scenario: An already cancelled request performs no inference work
    Given the chart-mutations language-neutral fixture and an aborted signal
    When chart inference is dispatched
    Then no progress is published and no partial result is returned

  Scenario: Browser workers preserve the same arithmetic and cancellation behavior
    Given the first-party browser inference and worker bundles
    When Chromium infers the chart-mutations fixture
    Then its point coordinates match d3-scale
    And an independent browser timer cancels the 20000-row inference without partial output

  Scenario: A failing progress observer prevents partial publication
    Given the chart-mutations language-neutral fixture
    When the caller's progress observer throws
    Then inference returns an incomplete schema-valid diagnostic result without drawing outputs
