@capability-staffage-model-exports
Feature: A pose is a GLB, a clip is an FBX, either is the neutral JSON model, and all of them are a ZIP
  One GLB is written per static pose with all seven views inside as material variants, the feature edges as lines and the glyphs of the analytic views; one FBX per clip in one view (default
  light-solid, any of the seven selectable); the neutral `semio.model/v1` JSON of a pose or a clip, procedural (the declarative solids) or resolved (tessellated meshes with edges); and one ZIP
  of every pose and clip. The command line takes `--format glb|fbx|model|model-zip` and refuses every nonsensical combination with a named, bilingual error before any work starts. File names
  follow D14: asset, pose or clip, view or `all`, `model` where a drawing names its viewpoint, scale. Bytes are deterministic, progress runs from 0 to the end and cancellation stops the work.

  THE REFERENCES are the Khronos gltf-validator and three's GLTFLoader for every GLB, three's FBXLoader for every FBX, ajv with the kernel's JSON Schema for the document and fflate for the ZIP.

  Scenario: Every pose is a valid GLB
    Given each named pose of the sample figure
    When its GLB is written
    Then the Khronos validator reports no error and no warning, three loads it, it holds seven variants, lines, one skin, the joints in the extras and the glyphs tagged

  Scenario: Every clip is a loadable FBX in every view
    Given each clip of the sample figure in the light, line, thermal, force and moment views
    When its FBX is written
    Then three's FBXLoader loads one animation named like the clip with the rig in it

  Scenario: The neutral JSON model is valid
    Given a pose and a clip of the sample figure, procedural and resolved
    When the model is written
    Then the kernel schema and the kernel rules accept it, the resolved one has no solids and has lines, and the same input gives the same text

  Scenario: The scale filters the solids
    Given the nose and the buttons that are shown only at 1:50
    When the model is written at 1:50 and at 1:100
    Then they exist at 1:50 and not at 1:100

  Scenario: Progress and cancellation
    Given a GLB, an FBX, a resolved model and a bundle
    When the export runs with a progress listener and then is aborted after three reports
    Then the progress is monotonic and ends complete, and each aborted export rejects with an AbortError

  Scenario: The bundle holds everything
    Given the sample figure
    When its bundle is written
    Then it holds a GLB and a JSON per pose and an FBX and a JSON per clip with the D14 names, each valid

  Scenario: Refusals are named in both languages
    Given an asset without a volume, an unknown pose or clip, and the command lines clip as GLB, pose as FBX, a viewpoint, a time, a view for a file of all views, an environment for a non-thermal FBX
    When they are asked
    Then each is refused with its own code, an English and a German reason, before any file is written

  Scenario: The command writes named files
    Given the four commands for the sample figure
    When they run
    Then the files are named asset-pose-all-model-1to50.glb, asset-clip-light-solid-model-1to50.fbx, asset-pose-all-model-1to50.json and asset-all-model-1to50.zip and each is valid

  Scenario: The catalog knows which formats an asset can produce
    Given assets with and without a volume, clips and poses
    When their exports list glb, fbx and model
    Then a format the asset cannot produce is an issue with a stable rule id
