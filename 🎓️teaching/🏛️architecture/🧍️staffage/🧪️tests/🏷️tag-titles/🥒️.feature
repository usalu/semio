@capability-staffage-catalog
Feature: Every tag of an asset is titled in every language
  A tag is an identifier (`id`, a slug that search and links use) with a title in English and German (`title`). The page of an asset shows the title of the
  reader's language; the slug of a tag is never shown to a reader. The same id carries the same title in every asset, so a tag reads the same wherever it stands.

  THE REFERENCE is ajv, which judges the schema; the document of every asset is read as it lies in the catalog.

  Scenario: The schema defines a tag as an identifier with a bilingual title
    Given the JSON Schema of the catalog
    When ajv compiles the definition of a tag
    Then a tag with an id and a title in both languages is accepted
    And a bare slug, a tag without a German title, a tag with an empty title and a tag with another member are refused

  Scenario: Every tag of every asset conforms to it
    Given the document of every asset of the catalog
    When ajv judges each of its tags
    Then all of them conform and no asset names a tag twice

  Scenario: No two tags of one asset read alike
    Given the tags of every asset
    When their titles are compared within the asset
    Then no two tags share a title in English and no two share one in German

  Scenario: A tag reads the same in every asset
    Given every tag of the catalog
    When the titles of equal ids are compared
    Then they are equal in English and in German
    And most titles differ between the two languages
