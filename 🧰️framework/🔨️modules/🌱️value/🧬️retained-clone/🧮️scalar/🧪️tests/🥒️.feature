Feature: Scalar and compound output custody
  Scenario: Close selects one correlated owner event
    Given a completed scalar or compound output and its original source
    When the caller begins closure and funds the exact selected quote
    Then output transfer is unavailable
    And each physical allocation or release equals its receipt
    And the cursor and original source become terminal within finite work

  Scenario: Parent admits child closure before observing its close protocol
    Given a child that refuses close queries before closure starts
    When its vector or optional parent begins closure
    Then the parent first quotes and funds its own child-start transition
    And no child close query occurs before that transition
