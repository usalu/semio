Feature: Contextual Architecture Companions
  Every teaching topic has an authored cast of pets depicting its concepts.

  Scenario: Every topic has its own complete cast
    Given the architecture course and the context-casts fixture
    When the architecture ensemble is assembled
    Then all 39 species are accepted by the owned validator and AJV
    And every topic has exactly the species named by its fixture
    And every requested concept has an English and German accessible name
    And every companion has an animated concept detail and distinct silhouette
    And every topic cast can perform a domain reaction
    And the home cast includes the complete menagerie
