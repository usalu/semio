Feature: Browser downloads carry the air flow the request asks for
  The request decides layers, conditions and mark detail, so the file shows what the screen shows.

  Background:
    Given the reference walker in the pose stand in the thermal view at 1:100

  Scenario: SVG
    When it is exported as SVG with all flow layers, with the streams only and without a flow
    Then the file holds exactly the marks of the layers asked for, and none in a view that is not thermal
    And with an environment, conditions and mark detail of its own the legend states that air and the marks are the ones the library draws for them

  Scenario: PDF of all views
    When the PDF is exported with the all-views sheet
    Then pdf.js reads the flow legends on the thermal page and on no other

  Scenario: DXF and DWG
    When it is exported as DXF and DWG
    Then the DXF names the flow layers and the DWG encoder receives them as polylines in metres
    And without a flow neither has a flow layer

  Scenario: A pose plays its flow as frames
    When the pose is exported as PNG sequence at one frame per second
    Then it has six frames on one canvas, the field one second older in each, the figure the same and the flow marks others
    And a pose without flow is still unavailable as a sequence
    And the recorder of a video receives the same frames

  Scenario: Progress and cancellation
    When the flow is developed for an export
    Then progress names the development in English and German
    And an abort during the development ends the export as cancelled
