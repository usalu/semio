Feature: Background document source work
  Scenario: A real document ingests while another document is focused
    Given the canonical runtime and client identity of an open document
    When source work starts before its remote message or archive is handed to the plugin
    Then its exact real owner and plugin cache retain an original hold
    And focusing that document before ingestion completes cannot publish a ready revision
    When ingestion finishes successfully
    Then a fresh admitted read is requested
    And an old owner, coincident client or replacement plugin cannot close the hold
