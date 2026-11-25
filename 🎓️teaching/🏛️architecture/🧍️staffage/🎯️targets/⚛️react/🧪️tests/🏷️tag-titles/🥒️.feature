Feature: Tags read in the language of the page
  A tag has a title in English and German. The facts of an asset and the header of its page show the title of the page's language, never the slug, and a search finds an asset by the title of its tags in either language.

  Scenario: The facts of an asset show its tags in the language of the page
    Given the reference walker whose tags are titled in both languages
    When its dimensions are shown in English and then in German
    Then the English page shows the English titles and the German page the German titles
    And no slug with a hyphen is shown

  Scenario: The header of the page shows the same titles
    Given the reference walker page in German
    Then the tags beside its title are the German titles

  Scenario: A search finds an asset by a tag title of either language
    Given a ready row whose tag is titled "commuter" and "Pendler"
    When the list is searched for the English title, the German title and the slug
    Then the row is found each time
