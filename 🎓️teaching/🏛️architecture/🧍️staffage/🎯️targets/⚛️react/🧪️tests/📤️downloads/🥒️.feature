Feature: Browser exports are genuine, labelled honestly and cancellable
  Scenario: SVG, PDF and DXF carry signatures and metadata
    Given the reference walker in the pose stand at 1:100
    When it is exported as SVG, PDF and DXF
    Then the SVG has a metadata element naming the asset, the PDF starts with %PDF-1.7 and has one page, and the DXF starts with comment lines and an AC1015 header
    And the file names read asset_pose_view_1-100 with the format extension
  Scenario: The all-views sheet
    When the PDF is exported with the all-views sheet
    Then it has seven pages that pdf.js reopens
  Scenario: DWG is genuine or unavailable
    When DWG is exported without the Rust WebAssembly writer
    Then the result is unavailable with an English and a German reason and no file
  Scenario: PNG sequence
    Given the clip walk at 10 frames per second
    When it is exported as PNG sequence
    Then the ZIP holds 10 PNG frames with text chunks and a metadata.json, readable by an independent unzip
  Scenario: Cancellation and capability
    When the signal is aborted before the export
    Then the result is cancelled
    And capabilities report canvas and recorder support from the running browser
