Feature: IFC4X3_ADD2 artifact interchange
  Scenario: Official triangulated item retains its complete STEP graph
    Given the pinned buildingSMART triangulated-item IFC4X3_ADD2 example
    When STEP text is parsed, serialized and parsed again
    Then every header value, instance id, entity type and argument is identical
    And IfcOpenShell reports the same entity ids, types, coordinate list and triangle indices

  Scenario: STEP values preserve exact native scalar identities
    Given strings with apostrophes, reverse solidus, X2 and X4 escapes and a real negative zero
    And integer ids beyond JavaScript's safe integer range and complex entity records
    When STEP text is parsed, serialized and parsed again
    Then typed values, complex members, integer ids and binary64 words are identical

  Scenario Outline: Invalid STEP never publishes an artifact
    Given STEP containing <defect>
    When text import is requested
    Then import fails with a code and source offset
    Examples:
      | defect                         |
      | a duplicate entity id          |
      | a missing reference target     |
      | a foreign FILE_SCHEMA          |
      | an unterminated string         |
      | an unterminated comment        |
      | an unsupported header record   |
      | a non-finite real              |

  Scenario: Exchange cancellation preserves the destination
    Given an abort signal and an unmodified source IFC graph
    When import or export is cancelled at a progress checkpoint
    Then no partial destination is returned
    And the source graph is identical

  Scenario: Geometry exchange states its fidelity
    Given the official triangulated item in millimetres at a nested local placement
    When exported to GLB and loaded independently by three.js
    Then the world bounds in metres are (0.5, -0.5, 0) through (1.5, 0.5, 2)
    And there are eight vertices and twelve triangles
    And semantic losses are reported explicitly

  Scenario: Unsupported semantic projection is refused
    Given an IFC graph whose geometry cannot supply authored BIM or analytical FEM parameters
    When converted to a BIM or FEM3d artifact
    Then the outcome lists unsupported entities and missing semantics
    And no invented authored or analytical elements are produced

  Scenario: Analytical members exchange through actual FEM3d and IFC entities
    Given an SI FEM3d frame model with engineering material and section properties
    And fixed supports, global nodal forces and uniform member loads
    When exported to IFC4x3 and imported to FEM3d
    Then real structural point connections and curve members retain topology and numerical properties
    And native IFC structural load cases and activities retain loads and restraints
    And unsupported continua, local-coordinate loads and nonuniform load configurations are refused

  Scenario: BIM spatial exchange never invents building elements
    Given an authored BIM project, sites, buildings and storeys
    When spatial structure is exported to IFC4x3
    Then real IFC spatial entities retain relative placements and aggregation
    And unsupported authored families produce explicit diagnostics

  Scenario Outline: Foreign analytical semantics are never silently changed
    Given a supported analytical IFC frame model
    And <foreign_input>
    When projected to FEM3d
    Then no artifact is published
    And an explicit diagnostic identifies the unsupported analytical semantics
    Examples:
      | foreign_input                                      |
      | moment of inertia unit declared as metre to power 1 |
      | density unit with incompatible physical dimensions  |
      | no membership assignment to the sole analysis        |
      | a load assignment by factor 2                       |
      | a member end release or connection condition         |
      | a missing member endpoint connection                 |
      | an action-local object placement                     |
      | an unassigned structural action                      |

  Scenario Outline: Foreign spatial placement contradictions are refused atomically
    Given a supported authored IFC spatial model
    And <foreign_input>
    When projected to BIM spatial parameters
    Then no artifact is published
    And an explicit diagnostic identifies the unsupported or contradictory placement
    Examples:
      | foreign_input                                           |
      | a site translated 100 metres horizontally                |
      | a nonidentity project geometry-context frame               |
      | an absolute building placement unrelated to its parent    |
      | a ground storey translated 99 metres vertically            |
      | a horizontally displaced or rotated storey                |
      | storey authored levels contradict cumulative heights      |

  Scenario: Explicit authored BIM length units override project units
    Given an authored CutHeight of 1200 millimetres and Height of 3000 millimetres
    And project length units in metres
    When projected to BIM spatial parameters
    Then cut height is 1.2 metres and storey height is 3 metres
    And explicit units with incompatible physical dimensions are refused atomically

  Scenario: Shared analytical definitions retain canonical identities
    Given two IFC curve members sharing one material and section property set
    And two structural point connections sharing one boundary condition
    When projected to FEM3d and exported and imported again
    Then one material and one section remain shared by both members
    And two distinct support instances refer to their respective nodes
    And conflicting material or authored property definitions are refused atomically
