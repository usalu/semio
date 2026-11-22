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
    Then every manifest entry is one ready row and the manifest check is empty

  Scenario: Cancelling leaves the category retryable
    Given a category load in flight
    When the reader cancels it
    Then the category is idle and loads fully on the next request

  Scenario: A category file without its revisions fails in both languages
    Given a category document that lists assets but no revisions record
    When the category is loaded
    Then it is failed with a message in English and German and never stays in loading

  Scenario: A manifest that cannot be read stops the loads that need it
    Given a catalog whose manifest cannot be read
    When an asset and a category are asked for
    Then both requests settle, the manifest is failed and no category file is read

  Scenario: Viewing the archive keeps digital geometry on demand
    Given an authored model with four scale volumes
    When the archive loads the model and a reader opens its 1:50 volume
    Then that volume retains its identity on subsequent reads
    And each unopened scale remains encoded

  Scenario: Opening the archive defers paper drawings
    Given an asset with a paper drawing at 1:100
    When its archive row is opened
    Then its print status is available before its geometry is decoded
    And repeated access to the drawing returns the same geometry

  Scenario: An asset is offered while its findings are computed
    Given an asset whose findings arrive later
    When the asset is loaded
    Then its row is ready at once and says how far the check is
    When the findings arrive
    Then the row stays ready and carries the findings beside its asset, or carries none when there are none

  Scenario: A withdrawn check is asked again
    Given an asset whose reader left before its findings arrived
    Then its row stays ready and no longer says a check is running
    When a reader asks for the asset again
    Then the findings are asked for again and settle the row

  Scenario: Findings of a category arrive after the category is ready
    Given a category whose assets get their findings later
    When the category is loaded
    Then the category is ready with every asset offered
    And an asset whose findings arrive carries them and stays offered like the others

  Scenario: The findings stream is read line by line
    Given a findings stream of two progress lines and one findings line
    When it is read
    Then both progress lines are heard in order and the findings are returned
    And a stream that says the revision is stale has no findings
    And at most two streams are read at once

