@capability-staffage-model-looks
Feature: The seven views are looks of one geometry, painted by the rules of the 2D views
  Every solid has a base material (its bone, role, tone and thermal regions) in the light-solid look; each of the seven views is a variant that rebinds the base materials, as the 2D depiction
  paints the same shapes seven ways. The tone ramp, paper, glazing opacity, thermal ramp, range, centre and environment, and the member ramp, symmetric range, sign convention and member
  filter are the very functions of the 2D views (`thermalPaint`, `memberPaint`), not copies. Analytic colours animate over a clip as material colour channels. Force arrows, moment arcs
  and emitter cones are glyph solids with the paper sizes of the 2D views at the requested scale, hidden in every view but their own.

  THE REFERENCES are the 2D depiction (`depict`) for colours and sizes and three's colour management for sRGB to linear.

  Scenario: Seven variants over one geometry
    Given the sample figure's model
    When its variants and base materials are read
    Then dark-line, dark-solid, light-line, light-solid, thermal, forces and moments bind every base material and the base look equals light-solid

  Scenario: The solid and line views paint like the 2D views
    Given the dark and light solid and line views at 1:50
    When the colour of every solid is compared with the fill of the shape of the same id
    Then tones, paper and glazing opacity are equal, and the line views are unlit

  Scenario: The thermal view paints like the 2D thermal view
    Given the standing and the bag-carrying pose, shared range and a span of 12 K
    When the region colours are compared with the 2D thermal view
    Then they are equal in linear sRGB

  Scenario: The thermal view solves in the environment the caller names
    Given a warm, sunny, still environment
    When the thermal colours of the model and of the 2D thermal view are compared under that environment
    Then they are equal and differ from the colours of the asset's own environment

  Scenario: The force and moment views paint like the 2D structural views
    Given the bag-carrying pose
    When the member colours are compared with the 2D force and moment views
    Then they are equal, compression warm and tension cool, non-members neutral

  Scenario: Analytic colours animate with the clip
    Given the walk clip
    When the material colour channels are sampled at four instants
    Then every colour equals the 2D view at the same instant

  Scenario: Arrows and arcs have the paper sizes of the 2D views
    Given the force and moment glyphs of the bag-carrying pose at 1:50
    When their lengths and radii are measured in paper millimetres
    Then they equal the drawn arrows and curls, the arrow tip lies on its application point and points along the force

  Scenario: Glyphs show in their own view only
    Given the glyph solids of the model
    When the variants are read
    Then every glyph material is fully transparent in the base look and bound by exactly one analytic variant
