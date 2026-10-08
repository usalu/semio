Feature: The launcher finds, configures and starts any command of the registry
  The launcher is a pure state machine over the command registry: a tree filed by verb, an incremental
  search over all words, a parameter form and a confirmation for commands that change the repository.
  Keyboard and mouse use the same selection.

  Scenario: The fixture workspace offers exactly the pinned commands
    Given the shared launcher fixture with its workspace facts and its pinned list of commands
    When the registry is built from the facts
    Then the listed commands equal the pinned list
    And the registry reports no problem

  Scenario: Journeys match the shared vectors
    Given the shared launcher journeys with their steps and expectations
    When the native launcher and an independent JavaScript implementation play the steps
    Then the stage, the outcome, the caption, the input, the visible ids and the items equal the expectation

  Scenario: A registry update keeps the search, the selection and the open groups
    Given a launcher with a search typed, a command selected and groups opened
    When the registry publishes a new set of commands
    Then the search, the selection and the open groups stay

  Scenario: The verbs are filed in the language of the dashboard
    Given the preference language de
    When the launcher lists the verbs
    Then each verb is named in German

  Scenario: A collapsed tree lists only verbs
    Given a launcher without a search
    When the tree is collapsed and then expanded
    Then collapsed it lists only the verbs and expanded it reveals their children
