Feature: The axonometric view is exported as SVG, PDF, DXF and DWG like the planar views

  @capability-axonometric-svg
  Scenario Outline: The SVG is well-formed and an independent rasteriser agrees with the vector fills
    Given the axonometric light-solid view of <subject> at 1:50
    When the SVG is parsed by fast-xml-parser and rasterised by sharp
    Then it states the viewpoint, the isometric camera and its axis scales
    And at least 96 % of the sampled pixels are painted where the vector fills are

    Examples:
      | subject                  |
      | the sample figure        |
      | elderly-with-cane        |
      | delivery-van             |
      | duct-fan                 |

  @capability-axonometric-pdf
  Scenario: The PDF is one page of the paper size
    Given the dark-line axonometric view of each subject
    When pdfjs reads the document
    Then it has one page of the scene's width and height and draws operators

  @capability-axonometric-dxf
  Scenario: The DXF holds every visible polyline on its layer and passes the ezdxf audit
    Given the CAD scene of each subject
    When the group codes are read and ezdxf audits the sample figure and the vehicle
    Then every polyline is there, with no error and no fix, and the header note is kept

  @capability-axonometric-cli
  Scenario: The command writes the view under a name that carries the viewpoint
    Given --viewpoint axonometric with and without --camera
    When svg, pdf and dxf are exported
    Then the files exist, their names carry axonometric and the camera, and progress is reported

  @capability-axonometric-dwg
  Scenario: The DWG holds every visible polyline on its layer and ezdwg reads it back
    Given the CAD scene of each subject and the export command with --format dwg
    When the wasm writer encodes it and ezdwg reads the file
    Then AC1024 in metres, the layers with their colours and lineweights and every polyline and the note are the scene's, the file name carries the viewpoint, progress is reported and a cancelled export is rejected
