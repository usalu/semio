Feature: DWG export reopens in an independent reader
  Scenario: A model scene written as AC1024 is read back by acadrust
    Given the model scene of a 1.80 m figure with four layers, two polylines on the body layer and one text
    When it is written through the in-repo Rust AC1024 writer
    Then the file starts with the signature AC1024
    And acadrust reads insertion units 6 (metres)
    And every layer keeps its ACI colour and lineweight
    And the LWPOLYLINE count, closed flags and coordinates equal the scene to 1e-6
    And an empty scene still yields a genuine AC1024 and a scene in millimetres is refused
