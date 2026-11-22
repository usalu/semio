Feature: Every 2D export carries the thermodynamic air flow
  The thermal view shows the air flow by default everywhere, so a thermal export carries all its layers unless it is told not to.

  Background:
    Given a boiling kettle in cool, moist, slowly moving air set by --air-temp, --radiant-temp, --humidity, --wind and --wind-direction
    And every file is written by the export command line and reopened by a third-party reader

  Scenario: The command line draws the flow by default and refuses its flags by name where they make no sense
    When a thermal export names no --flow
    Then it draws all flow layers, and --flow none draws none
    And the environment flags lie over the resolved environment and the flow flags become its conditions and mark detail
    And --flow, the environment flags and the flow flags are refused by name outside the thermal view, without a flow or for a 3D model format

  Scenario: The SVG holds the flow in its analysis layer, inside the flow domain, in the colours of the flow paint
    When the thermal view is exported as SVG
    Then stream lines, arrow heads, tracers, vapour, fog and parcel paths are in the layer group of the analysis field
    And every mark lies inside the projected flow domain of the field, whose paper the drawing reserves
    And streams, tracers and fog have the colours the flow paint gives them
    And the export with --flow none has no flow mark and the same figure

  Scenario: The PDF shows warm air above the warm body
    When the thermal view is exported as PDF and rendered by PDFium
    Then the page shows warm-coloured pixels above the kettle that the page without flow does not show
    And the text of the page states the flow legends and the time of the field

  Scenario: The frames of a pose advance the flow and nothing else
    When the pose plays its flow for one second at two frames per second
    Then the legends state the field 14.00, 14.50 and 15.00 s after the start
    And everything but the flow marks and the flow legends is the same in every frame
    And every tracer has moved downstream by the frame time, measured as travel time along its stream

  Scenario: The frame after the last of a loop is its first
    When a looping clip is exported at its start, at its middle and one duration after its start
    Then the flow marks one duration later are the marks of the start
    And the marks of the middle are others

  Scenario: DXF and DWG carry the flow as model-space geometry on named layers
    When the thermal view is exported as DXF and reopened by ezdxf
    Then the flow layers have their colours and lineweights and the drawing audits clean
    And the stream polylines are the projected stream points in metres
    And vapour and fog are circles inside the flow domain
    And the DWG read by ezdwg has the same layers and polylines

  Scenario: The sheets draw one field at one instant in every cell
    When the multiview sheet is exported as SVG and as DXF
    Then every cell has the tracers of the same field, each at the same paper x in elevation and plan and the same paper y in elevation and front, unless the body hides it from one side
    And the flow legends stand once
    And the CAD sheet has the flow layers of every view

  Scenario: Frame sequences and videos play the flow of a pose
    When the pose is exported as PNG sequence and MP4
    Then the files hold the scheduled frames at the frame rate and the frames differ
