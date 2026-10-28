Feature: Reusable assignment document
  Assignment sheets explicitly select a supported language and theme.
  Course authors provide domain content independently of the document class.

  Scenario Outline: Assignment metadata follows the language-neutral schema
    Given the assignment declaration "<name>" from the shared fixtures
    When the owned validator and JSON Schema oracle evaluate the declaration
    Then both produce the declared validity

    Examples:
      | name                 |
      | English assignment   |
      | German assignment    |
      | Language is required |
      | Unsupported language |
      | Empty title          |
      | No implicit theme    |
      | Unknown metadata     |
      | Typed metadata       |
      | Inherited property toString |
      | Inherited property constructor |

  Scenario: Plain metadata is safe in LaTeX
    Given the shared plain-text escaping fixture
    When an assignment preamble is inferred
    Then all special LaTeX characters are represented literally

  Scenario: Parameter worksheets remain writable
    Given the assignment class parameter table
    Then five writable parameter columns follow the shape column
    And column names and values are blank
    And the number of authored shape rows is unrestricted

  Scenario: Canonical black icons remain legible in both document themes
    Given the shared image contrast fixture
    Then light documents preserve the original transparent icon appearance
    And dark documents use the existing light canvas token as an image backing
    And the padded image fits inside the worksheet row height

  Scenario Outline: PDF metadata follows the authored assignment
    Given an assignment with explicit language <language>, title and author
    When the document begins after its metadata is declared
    Then the PDF title and author match the authored values
    And its catalog language is <pdfLanguage>

    Examples:
      | language | pdfLanguage |
      | en       | en-US       |
      | de       | de-DE       |
  Scenario: Matrices center bounded artwork above one-line writing rows
    Given the shared matrix fixture with eight type columns and eight parameter rows
    Then the table has nine columns and nine rows
    And every header cell centers its content inside a fixed 16 millimetre frame
    And the image preserves its proportions inside the frame with 2 millimetre padding
    And every writing row is exactly 6 millimetres tall independently of the header
    And two matrix bodies occupy 128 millimetres before borders and surrounding prose

  Scenario: Worked example uses the native table silhouette
    Given the worked example parameter matrix in either supported language
    Then its left name chip is absent while its automatic right number uses the connected outline
    And blank titles do not disable table chrome or its counter
    And its type icons and parameter values remain in the matrix cells
