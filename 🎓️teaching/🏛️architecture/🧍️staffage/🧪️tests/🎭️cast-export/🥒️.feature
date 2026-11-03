@capability-staffage-cast
Feature: A cast manifest becomes figure-only frames
  A cast manifest names renderings of the staffage archive. Each cast is written to one folder with an index and one SVG per distinct frame. All frames of a cast share one
  viewBox and one world origin, so playing them back never jumps. The vectors live in `🧫️fixtures/🎭️cast-export/🔣️.json`.

  THE REFERENCES are ajv (draft-07 validation of the manifest schema) and jsdom (the exported SVG read back with a W3C XML parser).

  Scenario: The manifest schema is enforced by two implementations
    Given the fixture manifest, the course manifest and the invalid casts of the fixture
    When each is validated by ajv and by the dependency-free twin
    Then both accept the valid manifests and both reject every invalid cast, including unknown viewpoints, scales, frame rates, transport layers and timelines

  Scenario: A clip yields durationS times fps frames and a pose one frame
    Given a looping walk cycle at 15 fps and a plain radiator pose
    When the casts are exported
    Then the clip has 18 frames and the pose one frame at fps 1

  Scenario: A long schedule is thinned to 600 samples
    Given a toaster clip and a 100 s radiator timeline at 15 fps
    When the casts are exported
    Then each has 600 frames with a fractional fps, the timeline ends exactly on its last key time and identical frames are written once

  Scenario: The cast is mirrored about the world origin
    Given the same cast with and without mirror in elevation, plan and front
    When the casts are exported
    Then the bounds are reflected, the mirrored frames carry a matrix with a negative x scale and the plain frames do not

  Scenario: The viewpoints draw different pictures
    Given a tilt window in elevation and in front and a walking commuter in plan
    When the casts are exported
    Then the pictures differ, the plan is shallow and the plan limbs stretch from frame to frame

  Scenario: A timeline is sorted by time
    Given a timeline whose keys are listed out of order
    When the cast is exported
    Then the frames equal those of the sorted timeline

  Scenario: The theme chooses the ink of the analytic views
    Given a thermal radiator cast in the dark and in the light theme
    When the casts are exported
    Then the dark cast draws the dark-theme ink only and the light cast the light-theme ink only

  Scenario: A cancelled export stops between frames and leaves the output untouched
    Given the cancellation fixture with one quick cast and three slow thermal casts of 600 frames each, the quick cast already exported
    When the abort signal fires after 30 frames of the first slow cast, or the command line receives SIGTERM during the first slow cast
    Then exactly 30 frames were rendered, no temporary folder and no partial cast remain, the finished cast is byte for byte unchanged and the command exits with 130 after printing `[cancelled]`

  Scenario: Progress is reported per process, cast and frame
    Given the quick cast of the cancellation fixture
    When the command line exports it
    Then stderr names the process (`[pid n] ppid m`), the cast (`[cast 1/1] quick`) and the frames (`[1/1] Rendering cast frames / Besetzungsbilder werden gerendert`) and the exit code is 0

  Scenario: The Python render pipeline kills the whole exporter process tree when it is cancelled
    Given the same slow casts started through `ensure_casts` from Python and a KeyboardInterrupt after the first cast starts
    When the interrupt arrives while the exporter runs
    Then the repo shim, the bun process and its parent are gone, the cache folder is byte for byte as before, no temporary folder remains and the progress lines reached the caller

  Scenario: The command runs when it is started through a symbolic link or junction
    Given the exporter file addressed through a linked folder, as a symlinked checkout or `/var` on macOS does
    When bun runs it with the quick cast
    Then it reports `[cast 1/1] quick`, writes the cast and exits 0 instead of doing nothing silently

  Scenario: Cancellation and bad input leave nothing behind
    Given an aborted signal, an unknown cast id and a `--lang` flag
    When the exporter runs
    Then it throws before writing a cast folder
