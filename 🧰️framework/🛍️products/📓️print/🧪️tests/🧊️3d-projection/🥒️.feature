@capability-viz-scientific-3d
@oracle-gl-matrix
@comparison-viz-probe-v1
Feature: One projection helper carries every three-dimensional kind onto the page
  `semio-viz-scientific-3d` projects a world point with a single routine: an azimuth rotation about z,
  an elevation rotation about x, an optional perspective divide, and a millimetre zoom — with oblique
  as a separate shear. Every §29 kind, from a scatter to a Platonic solid, reaches the page through it,
  and the painter's algorithm sorts on the camera depth the same routine returns, so this case measures
  the screen coordinates and the depth together.

  The reference is `gl-matrix`, the registered oracle: the adapter composes the very same camera as a
  `mat4` — a screen-basis permutation times a rotation about the view axis by the elevation times a
  rotation about z by the negated azimuth — and transforms every point with `vec3.transformMat4`, so
  the reference is a third-party linear-algebra library rather than the subject's own formula written
  twice. `gl-matrix` defaults to `Float32Array`, which carries only about seven digits and would decide
  the last place of the comparison, so the adapter switches its array type to `Array` (IEEE double)
  before building anything. The oblique camera is the same construction with a shear `mat4` instead of
  the two rotations. The isometric camera is additionally pinned to its exact constants — arctan of one
  over the square root of two as elevation, at 45 degrees azimuth — where the three unit axes project
  to equal lengths.

  @id-isometric
  @level-quick
  @mode-differential
  Scenario: The isometric camera projects the three unit axes to equal lengths
    Given the committed probe document local://3d-projection.tex and the isometric camera
      | azimuth | elevation         | zoom | points                                   |
      | 45      | 35.26438968275465 | 1    | (1,0,0) (0,1,0) (0,0,1) (1,1,1) (-1,2,3) |
    Then the compiled probe and the reference implementation agree on every value

  @id-axonometric
  @level-quick
  @mode-differential
  Scenario: A general axonometric camera and the orthographic preset project as their matrices do
    Given the committed probe document local://3d-projection.tex and the cameras
      | projection   | azimuth | elevation | zoom |
      | axonometric  | 30      | 20        | 2    |
      | orthographic | 0       | 0         | 1    |
    Then the compiled probe and the reference implementation agree on every value

  @id-perspective-and-oblique
  @level-quick
  @mode-differential
  Scenario: The perspective divide and the cabinet shear follow their own definitions
    Given the committed probe document local://3d-projection.tex and the cameras
      | projection  | azimuth | elevation | distance | obliqueAngle | obliqueScale |
      | perspective | 45      | 30        | 10       | 45           | 0.5          |
      | oblique     | 0       | 0         | 60       | 45           | 0.5          |
    Then the compiled probe and the reference implementation agree on every value
