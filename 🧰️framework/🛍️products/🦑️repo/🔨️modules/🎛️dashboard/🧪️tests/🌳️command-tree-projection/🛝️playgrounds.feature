Feature: Playground Command Projection
  Scenario Outline: Select a renderer and example
    Given the canonical catalog registers a playground variant and two examples
    When a developer selects <renderer> and an example
    Then the dashboard runs <target> through Nx
    And the selected example is passed through the shell preference contract
    And browser renderers use their distinct registered ports

    Examples:
      | renderer    | target                           |
      | react       | dev-variant-react-dev             |
      | wgpu-wasm   | dev-variant-wgpu-dev              |
      | wgpu-native | run-variant-native-dev            |
