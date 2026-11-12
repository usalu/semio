@capability-staffage-model-batch
Feature: The batch export of the 3D models
  One command writes the complete 3D model set of the catalog into a build directory that is not checked in: per asset a GLB per pose (seven variants inside), an FBX per clip, the
  neutral model JSON of every pose and clip and the ZIP of all of them, plus a manifest with asset, pose or clip, view, file, bytes, SHA-256 and triangle count. Assets are streamed one
  at a time, the bytes are deterministic, a run resumes where files still hash to the manifest, and the run is cancellable.

  THE REFERENCES are node:crypto SHA-256, fflate for the ZIP and the single exports of the library, which the batch must equal byte for byte.

  Scenario: The set and its manifest
    Given two assets
    When the batch runs
    Then every planned file exists once, the manifest lists exactly those files with their bytes and hashes, and every path is at most 256 characters

  Scenario: A pose and a clip of one id stay apart
    Given an asset whose pose and clip share an id
    When the batch runs
    Then both model documents exist under different names and every file equals the single export

  Scenario: The bytes are deterministic
    Given two runs into two directories
    Then the manifests and every file are identical

  Scenario: A run resumes
    Given a finished run
    When it runs again, with a damaged file, with force and with another scale
    Then nothing is rewritten, only the damaged file and the ZIP are rewritten, everything is rewritten, everything is rewritten

  Scenario: A run is cancellable
    Given a running batch
    When the signal aborts
    Then it stops, leaves no temporary file and a manifest of the finished assets, and resumes from them

  Scenario: The command line is checked in both languages
    Given a malformed command line
    Then the refusal names the flag in English and in German

  Scenario: The emit loops of the export command are cancellable
    Given an export of many frames
    When the signal aborts after the evaluation
    Then no file is written
