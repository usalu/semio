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
