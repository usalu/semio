Feature: The 3D code is split from the 2D path
  Scenario: Classification
    Then the volume module, the GLB, FBX, model and bundle exporters and the model kernel with its viewer are declared free of import-time side effects and nothing else is
  Scenario: Release build
    When the site is built in release mode
    Then the WebGL2 shaders, the GLB variants extension, the FBX header and the stage style are in lazy chunks and not in the entry chunk
