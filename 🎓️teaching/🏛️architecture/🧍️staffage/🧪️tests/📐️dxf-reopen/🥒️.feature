Feature: DXF export reopens in an independent reader
  Scenario: A model scene written as DXF R2000 is read and audited by ezdxf
    Given the model scene of a 1.80 m figure with four layers
    When it is written as DXF AC1015
    Then ezdxf reads version AC1015, $INSUNITS 6 and $MEASUREMENT 1
    And ezdxf.audit() reports zero errors and zero fixes
    And the header extents equal the model bounds to 1e-6 and equal ezdxf's bounding box of the polylines
    And every layer keeps its ACI colour and snapped lineweight, every polyline its closed flag and coordinates, and the text its escapes
