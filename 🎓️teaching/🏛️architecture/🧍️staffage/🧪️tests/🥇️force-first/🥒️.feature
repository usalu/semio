@capability-staffage-structure
Feature: Supports that cannot balance a load balance its forces first
  The structural solver projects the load wrench (three forces, three moments) onto what the supports can give. A plain least-squares projection trades newtons against newton-metres, so a body that only tips showed a force residual that depended on the unit of length. `solveForceFirst` gives, of all wrenches the supports can supply, the one with the smallest unbalanced force and among those the one with the smallest unbalanced moment.

  Scenario: A wall-hung boiler whose centre of mass lies beside its rail
    Given a pin on the wall rail and a roller against the wall below it, and a weight 0.07 mm beside the line through both
    When the reactions are solved
    Then no force is left unbalanced and the residual is the moment of the weight about the rail alone

  Scenario Outline: Seeded supports of rank <rank>
    Given a stiffness matrix of <rank> independent support rows and a seeded load wrench
    When the wrench the supports give is computed
    Then it equals the lexicographic least-squares solution numpy computes with its singular value decomposition

    Examples:
      | rank |
      | 1    |
      | 2    |
      | 3    |
      | 4    |
      | 5    |

  Scenario: Supports of full rank
    Given a stiffness matrix of six independent support rows
    Then the solution equals the minimum-norm solution and nothing is left unbalanced
