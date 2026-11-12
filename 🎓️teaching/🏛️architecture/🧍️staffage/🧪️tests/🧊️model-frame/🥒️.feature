@capability-staffage-model-frame
Feature: The staffage world is carried into the neutral model frame by one reflection
  The staffage world is left-handed (x ahead, y up, z left); the neutral model is right-handed (+Y up, +Z ahead, +X the asset's left). A point (x, y, z) becomes (z, y, x), a moment or
  rotation axis becomes its negated, swapped image, and the node frame of a bone is (u, v, -w), the staffage bone frame with its lateral axis negated, which is right-handed. A volume
  solid is written in the bone-local (u, v, w) frame and reaches that node frame by mirroring w in its own transform and repeat, so the kernel tessellator winds every triangle
  counter-clockwise outward.

  THE REFERENCE is three.js (Matrix4, Quaternion, Box3) for the expected values; the hand-computed vectors are language-agnostic in `🧫️fixtures/🧊️model-frame/🔣️.json`.

  Scenario: Points and pseudo-vectors map by the reflection
    Given a point, a moment about the side axis and a rotation axis of the staffage world
    When they are carried into the neutral frame
    Then the point is (z, y, x) and the pseudo-vectors are negated and swapped

  Scenario: A bone frame becomes a right-handed node
    Given the frame of a bone pointing up, ahead, down and twisted by 90 degrees
    When its neutral basis and quaternion are built
    Then the basis columns are the hand-computed u, v and -w mapped, the quaternion equals the one three.js builds from that basis, and the basis is right-handed

  Scenario: A child is placed in its parent's node frame
    Given a parent and a child frame of the rig
    When the child's local translation and rotation are derived
    Then they equal the inverse of the parent basis applied to the world difference, and composing the nodes reproduces the solved world frames of the whole sample rig

  Scenario: A solid transform and a repeat are mirrored exactly
    Given solid transforms with translation, rotation and scale and repeats with a rotation about an axis through an origin
    When their mirrored forms move probe points
    Then each copy lands where mirroring the point transformed by the unmirrored form would put it

  Scenario: Solids reach the world exactly as the authoring tool computes them
    Given the sample figure, a cow, a bookshelf and a toaster
    When the pipeline tessellates every solid in its node frame, at rest and in the default pose
    Then every vertex equals the raw kernel vertex carried through the staffage bone frame and the reflection, and every solid is closed and wound outward

  Scenario: A loft keeps its sections
    Given the torso loft of the sample figure whose sections carry the rotation that turns their plane across the bone
    When it is tessellated through the pipeline with the sections passed unchanged
    Then the extent in the bone-local frame equals the hand-computed sections
