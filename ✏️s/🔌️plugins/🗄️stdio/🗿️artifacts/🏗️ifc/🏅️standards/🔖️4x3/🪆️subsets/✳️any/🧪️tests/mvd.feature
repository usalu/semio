Feature: IFC4x3 view scope accompanies complete graph editing
  Scenario Outline: Published scope badges retain their declared meaning
    Given the immutable IFC4X3_ADD2 entity and concept listings
    When the <view> scope for <entry> is inspected
    Then the scope is <state>
    And the result does not assert model conformance
    Examples:
      | view                         | entry             | state     |
      | ReferenceView                | IFCWALL           | universal |
      | Alignment-basedReferenceView | IFCWALL           | universal |
      | ReferenceView                | Object_Assignment | included  |

  Scenario: Unlisted and unknown scope never disable repair
    Given a complete editable IFC4x3 artifact graph
    When a declaration has no published badge or is not known
    Then the scope reports unlisted or unknown explicitly
    And it does not infer that editing is forbidden

  Scenario: View labels require an explicit supported language
    Given the two published ADD2 view identifiers
    When a label is requested in English or German
    Then that language is used without a default language
