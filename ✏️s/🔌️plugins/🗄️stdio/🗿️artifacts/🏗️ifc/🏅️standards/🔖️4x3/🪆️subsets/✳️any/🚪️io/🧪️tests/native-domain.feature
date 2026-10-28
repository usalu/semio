Feature: Native ADD2 domain artifact interchange
  Scenario: Native BIM spatial artifact packs retain authored spatial parameters
    Given a source-owned BIM project with a site, rotated building and two authored storeys
    When the registered ADD2 target composer exports it and the BIM target composer imports it
    Then project metadata, geographic fields, building frame and authored storey parameters agree
    And unsupported element-family omissions are explicit lossy diagnostics

  Scenario Outline: Native BIM import refuses unsupported necessary semantics atomically
    Given an IFC spatial graph with <defect>
    When its native BIM artifact projection is requested
    Then no destination artifact is published
    Examples:
      | defect                                      |
      | absent authored storey height               |
      | explicit property unit of wrong dimensions  |
      | translated project context or site          |
      | contradictory storey elevation and level    |
      | cancellation during properties or hierarchy |

  Scenario: Native FEM analytical artifacts preserve explicitly admitted engineering data
    Given a source-owned FEM3d artifact within the admitted analytical subset
    When registered ADD2 and FEM native artifact routes exchange it
    Then actual native node, member and canonical engineering references remain valid
    And every unsupported necessary analysis semantic refuses atomically
    And cancellation retains original caller authority before publication

  Scenario Outline: Expensive native domain scans observe their original cancellation
    Given an admitted IFC graph with a large <scan>
    When the original caller cancels inside that scan
    Then no destination artifact is published
    And the original terminal refusal is retained
    Examples:
      | scan                         |
      | entity identity index        |
      | aggregate spatial children   |
      | canonical material properties |

  Scenario: Native FEM rows receive through the source owner's persistent list receiver
    Given original authoring nodes and materials in an admitted retained source
    When the FEM owner receives them using the original physical grant
    Then the output is the actual FEM snapshot with persistent sealed rows
    And denied grants perform no ownership expansion
    And cancellation at every receiving prefix retains all owners for funded closure
    And terminal receiver destruction releases no remaining ownership

  Scenario: Registered native FEM exchange uses the original paired host controls
    Given an ADD2 native graph pack with two members sharing one material and one section
    And a normal IO job retains its original decoder, encoder, wallet and retirement recipients
    When the registered IFC to FEM native entry produces a target artifact pack
    And the registered FEM to IFC native entry receives that target artifact pack
    Then the actual persistent FEM owner receives nodes and materials
    And both members still reference one canonical material and one canonical section
    And the independent IFC oracle observes the authored engineering dimensions and topology
    And no intermediate default pack or plain record bridge is constructed

  Scenario: Original cancellation during FEM target packing is terminal
    Given a valid analytical IFC graph and its original paired host controls
    When the original encoder cancels before target pack publication
    Then the route returns the original canceled refusal
    And no target artifact is returned
    And every retained receiving frontier remains with its original funded retirement recipient

  Scenario: Successful FEM pack publication drains every scratch owner
    Given a canonical FEM pack writer has completed emission with several retained scratch owners
    When original grants close the first scratch frontier
    Then every later writer turn remains in successful draining until terminal closure
    And zero release grants preserve any frontier that requires release
    And the paid final wire remains retained until the separately funded take

  Scenario Outline: Analytical identity admission grows with sorted lookup work
    Given an unloaded analytical source with <nodes> distinct isolated nodes
    When original snapshot authority admits its identities and references
    Then namespace identities are sorted in original funded backing
    And work for twice the rows is less than three times the smaller fixture work
    And duplicate or absent identities still refuse before output publication
    And cancellation during sorting keeps original custody for funded closure
    Examples:
      | nodes |
      | 512   |
      | 1024  |
      | 10000 |
