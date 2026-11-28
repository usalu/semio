Feature: Compact GLB encoding shares and omits what no reader needs
  The compact encoding of a GLB (option `compact`) keeps every triangle and every primitive,
  and spends fewer bytes on vertex normals than the default encoding.

  Scenario: A normal is written as a normalized signed byte vector
    Given a document with a lit primitive whose normals are unit vectors
    When it is encoded with the compact option
    Then the NORMAL accessor is a normalized BYTE VEC3 whose 4-byte elements are laid out by `byteStride` 4
    And the extension KHR_mesh_quantization is listed as used and as required
    And the Khronos validator reports no error and no warning
    And glTF-Transform and three read every normal within 0.012 radians of the source

  Scenario: A primitive that no lit material can draw has no normals
    Given a primitive whose material and every material its variants bind are unlit
    When it is encoded with the compact option
    Then the primitive has no NORMAL attribute
    And a primitive whose variant binds a lit material keeps its normals

  Scenario: Identical normals of different primitives share one accessor
    Given two primitives with the same positions and the same normals
    When it is encoded with the compact option
    Then both primitives reference the same NORMAL accessor and the same POSITION accessor

  Scenario: The default encoding stays as it was
    Given the same document
    When it is encoded without the compact option
    Then the NORMAL accessor is a FLOAT VEC3 without `byteStride` and no extension is required

  Scenario: Colour factors and node transforms are the shortest decimals of their 32-bit floats
    Given a material and a node whose factors have 17 significant digits
    When it is encoded with the compact option
    Then every written factor reads back as the same 32-bit float and has at most 10 characters
    And the Khronos validator reports no error and no warning
    And the default encoding keeps the 17 digits

  Scenario: Vertices that differ only in an unread normal are welded inside a solid
    Given an unlit primitive of two solids whose vertices repeat positions, and a lines primitive over the same vertices
    When it is encoded with the compact option
    Then equal positions inside one solid share one vertex and none is shared between solids
    And every triangle and every line segment has the very corner positions of the source, in the source order, with the solid ranges unchanged
    And the same primitive drawn by a lit material, or encoded without the compact option, keeps every vertex
