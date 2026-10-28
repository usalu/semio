Feature: Artifact-owned composition route capabilities
  Scenario: Preserve native output ownership
    Given a text artifact's native text composer
    Then its capability claims the existing dialect namespace

  Scenario: Independent exporters share a foreign target
    Given alpha and beta artifacts exporting their own dialect to text
    When both definitions register beside the native text artifact
    Then all three definitions register without claim collisions
    And alpha and beta each claim their complete directed composition route

  Scenario: Refuse a duplicate directed route atomically
    Given an already registered alpha to text route
    When another definition claims that exact route
    Then registration fails without changing the registry

  Scenario: Reject malformed or unowned route claims
    Given composition-route.json acceptance vectors
    When route claims are parsed and validated
    Then canonical UTF-8 length framing and exact dialect coordinates are required
    And a foreign exporter requires at least one owned source dialect
