Feature: Original queued remote ingestion
  Scenario: A queued message is transferred into original receiving holds
    Given two accepted remote messages waiting for the actual document port
    When the port is prepared and both ingestions begin
    Then the queue hold retires only after both original receiving holds exist
    And the first completed ingestion cannot restore readiness
    And readiness can request a new admitted capture after both ingestions finish

  Scenario: A failed ingestion has no terminal proof
    Given an original ingestion hold
    When its promise rejects
    Then the failure is preserved
    And source readiness stays unresolved until actual document retirement

  Scenario: A cold archive is still loading until its original lane closes
    Given a held archive load with a separate asynchronous lane retirement
    When loading finishes before lane retirement
    Then source readiness remains unavailable
    When original lane retirement finishes
    Then the complete attempt may request a fresh admitted capture
