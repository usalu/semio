Feature: Native Surface Ownership
  Scenario: Dashboard native startup creates its GPU surface on the window event thread
    Given an explicitly selected locale, terminology and example
    When the window event loop creates the native window
    Then its surface is captured before delegating GPU device preparation
    And the asynchronous device preparation is sendable to the worker
    And the installed winit and wgpu runtime admit the window surface
    And runtime readiness is reported after the shell boots
