Feature: Original canonical edit controls
  Scenario: Callbacks cannot redirect an admitted edit
    Given one captured signal, observer and set of nonnegative limits
    When the observer replaces caller controls and aborts the original signal
    Then cancellation prevents publication and the replacement observer is never called
  Scenario: Finite generator cleanup retains the first failure
    Given an entered generator whose finally block yields three times
    When cancellation or an observer failure ends ordinary progression
    Then cleanup drains to completion without observing cleanup progress
    And the original first thrown value is preserved exactly
