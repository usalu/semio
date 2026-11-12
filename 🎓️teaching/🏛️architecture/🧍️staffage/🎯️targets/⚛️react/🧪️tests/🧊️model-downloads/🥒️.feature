Feature: 3D downloads
  Scenario: GLB per pose
    Given the sample figure with the exports glb, fbx and model
    When the GLB of the pose stand is exported
    Then it validates with the Khronos glTF validator with no errors and no warnings, three's GLTFLoader loads it, and it holds the seven views as material variants
    And it is named asset_pose_model_all_1-100.glb and is the same bytes for the same request
  Scenario: FBX per clip
    When the FBX of the clip walk is exported in the thermal representation
    Then three's FBXLoader loads it with the clip's animation and it is named asset_clip_model_thermal_1-100.fbx
  Scenario: Model JSON and ZIP
    When the model JSON of a pose and of a clip and the bundle are exported
    Then the documents validate against the kernel, and the ZIP holds a GLB and a JSON per pose and an FBX and a JSON per clip, read back by an independent unzip
  Scenario: Progress and cancellation
    Then progress runs from 0 to 100 percent, a signal aborted before or during the work returns cancelled, and an asset without volume is refused in English and German
  Scenario: The export panel
    Then it offers the 3D formats the asset's exports name (the bundle with the model), marks a missing volume, lets the pose, the clip or either be chosen, runs with progress and cancel and blocks drawings under the 3D model viewpoint
