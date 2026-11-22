@capability-staffage-catalog
Feature: Assets and the manifest are validated semantically
  Beyond the JSON Schema an asset must mean something: ids are unique, the rig is one tree with a single root, every reference resolves (shape bone, region and
  material; anchor bone; pose bones, contacts, thermal keys and loads; clip key poses, contact anchors, track targets; emitter anchors; structure supports;
  presentation defaults), there are at least 3 poses and 2 clips, key times ascend within the clip, every pose and key respects the joint limits, masses are
  not negative, emissivity lies in (0, 1], every physical parameter has a source, the elevation geometry is not empty, the nominal height is within 5 % and the
  width within 10 % of the evaluated default pose, phases and contact spans lie within the clip, planted looping clips advance as far as they plant, and every
  text has a non-empty English and German version. Every finding carries a path, a stable code and an English and German message.

  Scenario: The reference walker is valid
    Given the reference walker asset
    When it is validated
    Then there are no issues

  Scenario Outline: A broken variant raises the matching code
    Given the reference walker with one defect applied
    When it is validated
    Then the issues contain the code of the defect

  Scenario: The manifest must agree with the assets
    Given a manifest entry whose height differs by more than 5 percent
    When the manifest is validated against the assets
    Then a manifest-mismatch issue is reported

  Scenario: A flat elevation shape on a bone that rests inclined is placed by the rig
    Given the reference walker whose arm rests 12 degrees out of the paper and swings 25 degrees about its own inclined axis
    When it is validated
    Then no lateral-ambiguous issue is reported, because the rest placement defines the plane of the shape and the solid of the bone bounds the part to 0.04365 m beside it (0.08 mm on paper at 1:50)

  Scenario: Without a solid nothing bounds the part beside its plate
    Given the same arm without a 3D model at 1:50
    When it is validated
    Then lateral-ambiguous reports 0.6776 m x 0.091919 = 1.246 mm on paper at 1:50

  Scenario: A plane that rests edge-on cannot hold a flat shape
    Given the reference walker whose arm rests 90 degrees out of the paper
    When it is validated
    Then lateral-ambiguous reports the arm at 90 degrees for every scale at which its solid is wider than the print minimum

  Scenario: A pose that turns a bone out of the paper is measured as before
    Given the reference walker whose arm is spread 30 degrees in one pose
    When it is validated
    Then lateral-ambiguous reports the arm at 30 degrees with sin 30 of the reach of its solid

  Scenario: A third-party library recomputes the lean
    Given the arms of the vectors
    When numpy solves the chain with its rest placement and maps the rest drawing to the pose
    Then the tilt and the lean equal the validator's
