Feature: The interactive 3D model of a staffage figure
  Scenario: Mode switching
    Given the sample figure with an authored volume
    When the viewpoint changes from the side elevation to the axonometric view, to the 3D model and back to the plan
    Then drawings are inline SVG, the model is the stage with a canvas, and the axonometric view is a drawing made by the library
    And an asset without volume gets a named refusal in English and German instead of a blank figure
  Scenario: The seven representations are variants of one model
    When the model is built
    Then it holds exactly the seven views as variants and the view and the camera never build it again, but scale, pose or clip and analytic settings do
    And the seven pictures differ and a coarser scale tessellates coarser
  Scenario: The shared transport drives the 3D time
    Given a clip on one clock
    When the clock is scrubbed, stepped or toggled with Space on the canvas
    Then the viewer shows the clock's frame time, never plays by itself, and a pose has no clip time
    And under prefers-reduced-motion the clock starts paused
  Scenario: Camera, keyboard and accessibility
    When keys 1 to 5, the arrow keys, the preset buttons, the zoom buttons and reset are used
    Then the preset follows, free orbiting leaves every preset unpressed, every control is a labelled native button and the canvas has role, name and a translated text alternative of asset, pose or clip, representation, scale and camera
    And forced colours, contrast, reduced motion and the customisation properties are in the style
  Scenario: Fallbacks, progress and cancellation
    When WebGL2 is missing, or its context is lost and does not return
    Then the model is drawn in software with a translated note, never a blank box
    And preparation reports progress through four phases, can be cancelled and retried, and an unbuildable model is an alert
  Scenario: The custom element
    When the viewpoint attribute is model, axonometric or plan
    Then the element mounts the stage in its shadow root, keeps the camera attribute in step and falls back to a drawing
  Scenario: Both languages are complete
    Then every 3D text exists in English and German with the same placeholders
