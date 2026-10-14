Feature: Configured Renderer Presentation
  Scenario Outline: Launch a renderer with a selected language and terminology
    Given a playground with React, WebGPU browser and native renderers
    When the dashboard preferences select <locale> and <terminology>
    And the developer launches the renderer and example directly
    Then the command passes SEMIO_LOCKED_LOCALE=<locale>
    And the command passes SEMIO_LOCKED_TERMINOLOGY=<terminology>
    And the command retains the selected renderer, example and Nx target

    Examples:
      | locale | terminology |
      | en     | native      |
      | en     | reuse       |
      | de     | native      |
      | de     | reuse       |

  Scenario: No customization has been made
    Given the developer has selected a renderer and example
    Then the command runs immediately with English and native terminology
    And it asks no presentation questions
