Feature: Lazy, validated catalog

  Scenario: Categories load on demand
    Given a manifest of three planned assets
    Then no category file is read until a page asks for it

  Scenario: One bad asset does not stop the others
    Given a category with a valid and an invalid asset and one asset nobody authored
    When the category is loaded
    Then the valid asset is ready, the invalid one is unavailable with its reasons and the third is not authored yet

  Scenario: An invalid document is still a document
    Given a category that contains one invalid asset document
    When the archive is checked
    Then that entry is not reported as missing a document

  Scenario: The authored archive is ready
    Given the manifest and every catalog asset file
    When the archive is loaded
    Then one hundred seven rows are ready and the manifest check is empty

  Scenario: Cancelling leaves the category retryable
    Given a category load in flight
    When the reader cancels it
    Then the category is idle and loads fully on the next request
