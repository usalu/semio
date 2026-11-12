@capability-staffage-volume
Feature: The declarative 3D volume of an asset is a schema contract with semantic rules
  `asset.volume` (`🧬️schema/🔣️.json`, `semio.staffage/v2`) lists solids bound to the bones of the rig, in the bone-local rest frame in metres: box, ellipsoid, capsule,
  revolve, extrude and loft, each with a material, a role, a tone, thermal regions, an optional coarsest scale, optional weights over bones, a transform and a repeat.
  The schema judges the shape; the owned validator judges the meaning with one stable rule id per rule: the bone, the material and the regions exist, weights name bones and
  sum to 1, sizes and radii are positive, an extrusion runs from a smaller to a larger coordinate, profiles are simple polygons with their holes strictly inside and apart,
  a loft has two sections, a repeat count is between 1 and 256, maxDenominator is a preset and solid ids are unique.

  THE REFERENCES are ajv for the schema, shapely for the validity of every profile, the exact predicates of robust-predicates for every segment intersection, and the
  hand-computed bounding box of the sample figure from the rest frames of its rig.

  Scenario: The sample figure's volume conforms
    Given the hand-authored volume of the sample figure
    When ajv validates the asset and the owned validator checks the volume
    Then both accept it and its bounding box equals the dimensions of the figure within the extent tolerance

  Scenario: Each rule has a broken variant that only it rejects
    Given the sample figure with one defect per rule, such as a missing bone, weights that sum to 0.8, a profile that crosses itself or a repeat of 257
    When the owned validator checks the volume
    Then it reports exactly the rule that the defect breaks, with a path and a message in English and German

  Scenario: The schema alone rejects what has no shape
    Given the sample figure with an unknown solid kind, a missing tone or a profile of two points
    When ajv validates it
    Then it is invalid

  Scenario: Third-party geometry judges the profiles
    Given polygons with holes and open profiles, valid and broken
    When shapely checks their validity and robust-predicates their segment intersections
    Then the owned verdicts are identical

  Scenario: The axonometric viewpoint is depicted, never refused
    Given the axonometric viewpoint that the schema declares
    When a depiction or the export command asks for it
    Then the 3D volume is drawn: see the scenarios of the axonometric depiction
