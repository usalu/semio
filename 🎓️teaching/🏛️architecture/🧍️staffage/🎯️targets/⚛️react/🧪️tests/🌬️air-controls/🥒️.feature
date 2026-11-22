Feature: Full control over the air of the flow view
  The reader sets the air the thermal view and its flow are solved in and how the flow is drawn, on every surface of the web target, in English and German, by pointer, keyboard and screen reader.

  Scenario: The wind dial sets speed and direction together
    Given the wind dial of a planar viewpoint and the winds of the fixture "wind-dial"
    When a wind is placed on the dial and the dial point is read back
    Then the point and the wind equal the numpy oracle, and the bearing is worded by its 45° sector

  Scenario: The dial is operated by keyboard
    Given the wind section with a wind of 0.2 m/s towards 180°
    When the reader presses right, up, Shift+right, Home and End on the dial
    Then the bearing rises by 5° and 45°, the speed by 0.1 m/s, Home points the wind ahead and End stills the air
    And the text alternative says where the air goes in words

  Scenario: Every value has a unit, a default and the way back
    Given the air controls in English and in German
    Then every quantity is labelled with its unit in both languages and shown in the reader's number format
    And a changed value is marked, counted by its section and goes back to its default with one button

  Scenario: A reference environment is applied whole
    Given the asset's own environment
    When the reader chooses a preset and then moves one value
    Then the preset is named as active, replaces every field, and afterwards reads "changed by hand" with the preset it started from

  Scenario: Conditions solve again, mark detail only redraws
    Given a developed flow layer
    When the mark detail changes
    Then the layer stays ready and no solve starts
    When the wind or a condition of the flow changes
    Then the session reopens and the new field carries the new wind speed and direction

  Scenario: The viewer's controls reach the solver
    Given the viewer on the thermal view of the reference walker
    When the reader turns the wind on the dial
    Then the flow is solved again and its ambient wind follows

  Scenario: The custom element takes the air as attributes
    Given a staffage-figure with flow, wind speed, wind direction and conditions as attributes
    Then its drawing, its three-view sheet and its 3D model mount the flow layer and its field carries that wind
    When the wind direction attribute changes
    Then the field follows

  Scenario: A download is asked for with the air of the screen
    Given a depiction request with flow conditions and mark detail
    Then the depiction options carry them and develop the flow for the chosen parcel life
