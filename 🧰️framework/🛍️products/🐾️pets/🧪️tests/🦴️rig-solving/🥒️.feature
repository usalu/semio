@capability-pets-rig-solving
@oracle-pets-numpy
@comparison-pets-float-v1
Feature: A posed rig solves to one matrix per bone
  A species is a rig: bones listed parents first, each with a rest offset and a rest rotation relative
  to its parent (design §3). A pose adds to every bone an offset in pixels, an offset in degrees and
  scale factors (rest = 0, 0, 0, 1, 1). `solveRig(species, pose)` yields six numbers `a b c d e f` per
  bone in rig order — the bone's world matrix relative to the feet origin in the SVG
  `matrix(a b c d e f)` convention (`x′ = a·x + c·y + e`, `y′ = b·x + d·y + f`, y pointing down, positive
  rotation clockwise on screen) — with (design §4.3)

    local = translate(bone.x + pose.x, bone.y + pose.y) × rotate((bone.rotation + pose.rotation) ÷ 360 turns) × scale(pose.scaleX, pose.scaleY)
    world = parent world × local, and a root's world matrix is its local one,

  so scaling or turning a bone carries its children along. Bones a pose does not cover stay at rest.
  `compose(parent, local)` is that product, `invert(matrix)` its inverse (a matrix without an inverse,
  determinant 0, yields the identity, so the result is always finite), `transform(matrix, x, y)` the
  point the matrix carries, and `restPose(species)` the pose in which nothing moves.

  THE REFERENCE is numpy on 3×3 homogeneous matrices `[[a, c, e], [b, d, f], [0, 0, 1]]`: products
  with `@`, inverses with `numpy.linalg.inv` (the identity where numpy raises `LinAlgError` for a
  singular matrix; the oracle also requires `matrix @ inverse` to be the identity elsewhere), and
  skeletons as the chain of translate, rotate and scale matrices with the rotation built from
  `numpy.cos` and `numpy.sin` of `numpy.deg2rad` of the degrees. The committed species are complete
  documents: the reference species of the ticket and a relative with rest rotations and a four-bone
  arm chain; the poses cover rest, breathing, strides, waves, squashes, whole and quarter turns, a
  mirrored and a collapsed bone, a pose shorter than the rig and poses with every bone moved. The
  subjects are `@semio-tech/pets` (`compose`, `invert`, `transform`, `restPose`, `solveRig`) and the
  `pets` crate (`compose`, `invert`, `transform`, `rest_pose`, `solve_rig`).

  The twins must agree with each other bit for bit (design §2.4), which a tolerance cannot show.
  `bit-patterns` holds that, as a supplement and not as third-party evidence: the oracle adapter
  restates the products, the adjugate inverse and the bone chain in Python's IEEE doubles, term for
  term, holds every result to numpy within 1e-12, and projects the 64-bit pattern of every number as
  sixteen hexadecimal digits — text, which compares exactly under every profile.

  The vectors shared://🦴️rig-solving/🔣️.json are generated, never hand-edited, from the Python
  reference in this directory, from the repository root:
  `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/generate_kinematics_vectors.py`
  (`.venv/bin/python` outside Windows).

  @id-products
  @level-fundamental
  @mode-differential
  Scenario: Two matrices compose to parent × local
    Given the committed vectors shared://🦴️rig-solving/🔣️.json
    When every committed parent matrix is composed with its local matrix
    Then every implementation projects the same six numbers per pair, the local matrix applied first

  @id-inverses
  @level-fundamental
  @mode-differential
  Scenario: A matrix is inverted, and a singular one yields the identity
    Given the committed vectors shared://🦴️rig-solving/🔣️.json
    When every committed matrix is inverted
    Then every implementation projects the same six numbers per matrix, and 1 0 0 1 0 0 for the collapsed, the rank-one and the flat-column matrix

  @id-points
  @level-fundamental
  @mode-differential
  Scenario: A matrix carries a point
    Given the committed vectors shared://🦴️rig-solving/🔣️.json
    When every committed point is transformed by its matrix
    Then every implementation projects the same x and y per point

  @id-rest-poses
  @level-fundamental
  @mode-differential
  Scenario: The rest pose leaves every bone at its rest transform
    Given the committed vectors shared://🦴️rig-solving/🔣️.json
    When the rest pose of every committed species is taken and solved
    Then every implementation projects one entry 0, 0, 0, 1, 1 per bone and the same six numbers per bone in rig order

  @id-skeletons
  @level-fundamental
  @mode-differential
  Scenario: A pose solves to the world matrix of every bone
    Given the committed vectors shared://🦴️rig-solving/🔣️.json
    When every committed pose is solved on its species
    Then every implementation projects the same six numbers per bone in rig order, children following their parents' offsets, rotations and scales

  @id-bit-patterns
  @level-fundamental
  @mode-differential
  Scenario: Products, inverses, carried points and skeletons have the same 64 bits in every language
    Given the committed vectors shared://🦴️rig-solving/🔣️.json
    When every committed pair is composed, every matrix inverted, every point transformed and every pose solved, and each number is written as the sixteen hexadecimal digits of its IEEE bit pattern
    Then every implementation projects the same digits per product, inverse, point and skeleton
