Feature: DWG export reopens in independent readers
  Scenario: The wasm module is built by its own target
    Given staffage_dwg.wasm is missing or older than a source of the Rust crate
    When a test asks for the DWG module
    Then repo ./📜️script.ts wasm builds it for wasm32-unknown-unknown and a failing build fails the test

  Scenario: A model scene written as AC1024 is read back by ezdwg
    Given the model scene of a 1.80 m figure with four layers, two polylines on the body layer and one text
    When it is written through the wasm module
    Then the file starts with the signature AC1024
    And ezdwg reads metres and insertion units 6
    And every layer keeps its ACI colour and lineweight
    And the LWPOLYLINE count, closed flags and coordinates equal the scene to 1e-9 and the TEXT keeps its string, position and height

  Scenario: The multiview sheet states its view and is byte deterministic
    Given the sheet model scene with PLAN- and FRONT- layers and a note layer
    When it is written twice
    Then both files are equal and ezdwg finds the three layers and the note

  Scenario: Empty, large and refused scenes
    Given an empty scene, 2000 polylines of 12 points, a scene in millimetres and a polyline of one point
    When they are written
    Then the first two are read back completely and the last two are refused with the reason

  Scenario: Progress and cancellation flow through the writer
    Given a job control with a progress callback and an abort signal
    When the signal is aborted before or during the encoding
    Then the write rejects, and an uncancelled write reports 0 of 1 and 1 of 1 in English and German

  Scenario: A missing module is named in both languages
    Given a wasm location that does not exist
    When the DWG is written
    Then a DwgUnavailableError names staffage_dwg.wasm and says how to build it in English and in German

  Scenario: The Rust crate reads its own files with acadrust and ezdwg-core
    Given the same scenes
    When cargo test runs the dwg_reopen and dwg_format tests
    Then acadrust reads layers, colours, lineweights, coordinates and texts back, and ezdwg-core reports the version, the header variables, the class map and one valid record per handle
