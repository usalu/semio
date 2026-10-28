Feature: Dense glTF accessor admission for IFC triangle import
  Scenario Outline: Refuse an invalid binary layout atomically
    Given the original Khronos Box GLB with one neutral invalid layout edit
    When the actual glTF artifact is imported into IFC
    Then no IFC graph is published

  Scenario: Cancel inside one large dense accessor
    Given a valid dense position buffer with the neutral row count
    When the original signal is aborted during interior accessor progress
    Then no IFC graph is published
    And the importer reports original cancellation

  Scenario Outline: Validate and copy original binary bytes cooperatively
    Given a GLB read or write with the neutral binary row count
    When the original caller aborts during interior byte progress
    Then no result is published
    And nonbyte source values are refused before coercion
