Feature: Three-dimensional statics and the projection of forces and moments
  Forces, supports, reactions, joint loads and member cuts are solved in physical metres and newtons (x ahead, y up, z the asset's left, a left-handed triple). Side elevation, front elevation and plan only project the result.
  The numpy oracle in 🐍️.py converts explicitly to a right-handed frame (points and forces (x, y, -z), moments (-mx, -my, mz)) and derives the three cameras from the physical position of the viewer.

  Scenario: A balcony with a lateral load
    Given a 2 m cantilever fixed at the wall with a 1000 N downward and a 300 N lateral tip load
    When the structure is solved
    Then the wall reacts +1000 N up, -300 N laterally, 2000 N·m about z and 600 N·m about y
    And the member carries a lateral shear of -300 N, a lateral bending moment of -300·(2 - s) N·m and no torsion
    And all six residual components vanish

  Scenario: A crane slewed by 90 degrees
    Given a mast with a twist of 90 degrees carrying a jib and a hook load
    Then the jib points along the lateral axis and the base moment is about the forward axis
    And the jib bends by the full lever arm although its side projection is a point

  Scenario: A person leaning sideways
    Given a body whose torso is spread 20 degrees to the left on two feet
    Then the feet share the weight by the lever rule in the lateral direction and the shoulder joint carries a moment about the forward axis

  Scenario: Magnitudes are identical in every view
    Given any force or moment vector
    Then its in-plane part, its component along the line of sight and its total satisfy |v|² = in-plane² + out-of-plane² in side elevation, front elevation and plan
    And a component along the line of sight is a dot (towards the viewer) or a cross (away) with its value in the label
    And the legends and the load case do not depend on the view
