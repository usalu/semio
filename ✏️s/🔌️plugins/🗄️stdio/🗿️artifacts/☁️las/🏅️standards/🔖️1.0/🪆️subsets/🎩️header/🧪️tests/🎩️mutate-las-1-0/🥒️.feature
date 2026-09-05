@capability-las-1-0-any-mutate
@oracle-las-1-0-any-mutate
@comparison-semantic-las-v1
@mutations-las-1-0-any
Feature: Apply every typed LAS 1.0 mutation to a real-world point cloud
  The input is shared://🧪️pattern-sphere/🧊️.las, a real 8,448-point LAS 1.0 point cloud derived ONCE
  from the real committed 🧊️pattern-sphere.glb (679 KB, real modelled geometry) by hand-parsing its
  GLB container (12-byte header, JSON chunk, BIN chunk — no gltf crate is linked) to read the real
  POSITION accessor, scaling that unit sphere ×10 and translating it onto a plausible Hannover-area
  UTM-ish easting/northing/elevation, then writing it with the real `las` 0.11 reference crate (the
  derivation scripts are committed alongside this feature). Every point's `intensity` and
  `classification` are genuinely derived from that real geometry's height, not constants — the
  committed fixture's classification histogram is {2: 1157, 3: 1600, 4: 2934, 5: 1600, 6: 1157} —
  and it carries two real VLRs (`LASF_Projection`/34735 GeoKeyDirectoryTag, `semio`/1 fixture
  provenance). Every scenario copies it into the case work directory before touching it; the
  committed file is never written to.

  The oracle works against `las::raw::{Header, Vlr, Point}` — the crate's byte-exact typed mirror of
  the LAS public header block / VLR / point record — rather than its friendlier `Reader`/`Writer`/
  `Builder` façade, which auto-recomputes bounds and the points-by-return histogram from whatever is
  actually written and offers no way to override either; this subset's own `LasHeader` retains both
  directly instead, so `set-bounds`/`set-points-by-return` need that independent control to be
  checked mutations rather than no-ops. `set-version` stays within the LAS 1.0-1.2 family: the
  crate's header writer appends 1.3+ extension fields once the declared version supports them,
  growing the header past the fixed 227 bytes this subset's own `encode_las` always emits (its own
  `EncodeScopeNote`: "no LAS 1.3/1.4 extensions") — exercising 1.3/1.4 would be testing a real
  subject capability gap under a version-label mutation, not the mutation itself.

  🔴 The PARITY phase ran for this case for the first time and reproduced the one place the two
  sides disagreed about what a kind MEANS, rather than about how to write it: `set-scale-and-offset`
  diverged on all 8,448 points, 24,320 differences, `$.points[1].x` oracle 583000.246 against our
  583000.491. The committed fixture stores that point as the integer record 491 under scale 0.001
  and offset 583000. Setting the scale to 0.0005, the reference left the record at 491 and read it
  as 583000.246; we held the coordinate at 583000.491 and re-quantized the record to 982. Both were
  self-consistent, so this was a vocabulary decision, not a writer bug — and the reference is not an
  independent authority on it either, since `las` 0.11 is a reader/writer and the mutation semantics
  in the oracle module are this repository's own choice too. It was settled on the LAS specification
  and on invertibility, not on convenience. §"Public Header Block" defines `coordinate = record *
  scale + offset`, so the RECORD is what the file carries; `set-scale-and-offset` sits in a group of
  kinds (`set-version`/`set-system-identifier`/`set-software-info`/`set-creation-date`/`set-bounds`/
  `set-points-by-return`) every one of which writes header bytes and nothing else; and, decisively,
  the coordinate-preserving reading is LOSSY in one direction — moving to a coarser scale rounds
  every coordinate away and `LasMutation::inverse`'s "put the old scale and offset back" cannot
  restore it, so the inverse law was holding only because this row's 0.0005 happens to REFINE
  0.001. The record-preserving reading is lossless and exactly invertible for any scale either way.
  Our side was changed to match: `🏅️standards/🔖️1.0/🪆️subsets/✳️base/🧬️schema/🔺️diff/🦀️component.rs`'s
  `diff_set_scale_and_offset` now carries, alongside the six header fields, the coordinate every
  point is re-read to under the new parameters, so the records stay put — pinned by
  `set_scale_and_offset_keeps_every_point_record_where_it_is`, which asserts the records are
  unchanged and the undo is exact in the coarsening direction too. Nothing was relaxed to reach it:
  `semantic-las-v1` and its 0.001 tolerance, the fixture and every assertion are untouched.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real point cloud
    Given the real input point cloud shared://🧪️pattern-sphere/🧊️.las
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                    | params |
      | set-snapshot           | {"header": {"versionMajor": 1, "versionMinor": 0, "systemIdentifier": "SEMIO-SNAP", "generatingSoftware": "semio-test", "dayOfYear": 1, "year": 2026, "scale": [0.01, 0.01, 0.01], "offset": [0.0, 0.0, 0.0], "max": [10.0, 10.0, 10.0], "min": [0.0, 0.0, 0.0], "counts": [3, 0, 0, 0, 0]}, "vlrs": [{"userId": "semio", "recordId": 1, "description": "snap-vlr", "data": "snap-data"}], "points": [{"x": 0.0, "y": 0.0, "z": 0.0, "intensity": 10, "returnNumber": 1, "numberOfReturns": 1, "scanDirectionFlag": false, "edgeOfFlightLine": false, "classification": 2, "scanAngleRank": 0, "userData": 0, "pointSourceId": 1, "gpsTime": null, "rgb": null}, {"x": 1.0, "y": 1.0, "z": 1.0, "intensity": 20, "returnNumber": 1, "numberOfReturns": 1, "scanDirectionFlag": false, "edgeOfFlightLine": false, "classification": 4, "scanAngleRank": 5, "userData": 0, "pointSourceId": 1, "gpsTime": null, "rgb": null}, {"x": 2.0, "y": 2.0, "z": 2.5, "intensity": 30, "returnNumber": 1, "numberOfReturns": 1, "scanDirectionFlag": true, "edgeOfFlightLine": false, "classification": 6, "scanAngleRank": -5, "userData": 0, "pointSourceId": 1, "gpsTime": null, "rgb": null}]} |
      | set-version            | {"major": 1, "minor": 1} |
      | set-system-identifier  | {"systemIdentifier": "RENAMED-SYSTEM"} |
      | set-software-info      | {"generatingSoftware": "renamed-software"} |
      | set-creation-date      | {"dayOfYear": 42, "year": 2027} |
      | set-scale-and-offset   | {"scale": [0.0005, 0.0005, 0.0005], "offset": [583000.0, 5804000.0, 0.0]} |
      | set-bounds             | {"max": [583020.0, 5804020.0, 20.0], "min": [582980.0, 5803980.0, -20.0]} |
      | set-points-by-return   | {"counts": [8000, 300, 100, 40, 8]} |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the real point cloud
    Given the real input point cloud shared://🧪️pattern-sphere/🧊️.las
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the inverse mutation is applied to that result
    Then the oracle and the subject agree on the semantic projection of the original point cloud
    Examples:
      | id                    | params |
      | set-snapshot           | {"header": {"versionMajor": 1, "versionMinor": 0, "systemIdentifier": "SEMIO-SNAP", "generatingSoftware": "semio-test", "dayOfYear": 1, "year": 2026, "scale": [0.01, 0.01, 0.01], "offset": [0.0, 0.0, 0.0], "max": [10.0, 10.0, 10.0], "min": [0.0, 0.0, 0.0], "counts": [3, 0, 0, 0, 0]}, "vlrs": [{"userId": "semio", "recordId": 1, "description": "snap-vlr", "data": "snap-data"}], "points": [{"x": 0.0, "y": 0.0, "z": 0.0, "intensity": 10, "returnNumber": 1, "numberOfReturns": 1, "scanDirectionFlag": false, "edgeOfFlightLine": false, "classification": 2, "scanAngleRank": 0, "userData": 0, "pointSourceId": 1, "gpsTime": null, "rgb": null}, {"x": 1.0, "y": 1.0, "z": 1.0, "intensity": 20, "returnNumber": 1, "numberOfReturns": 1, "scanDirectionFlag": false, "edgeOfFlightLine": false, "classification": 4, "scanAngleRank": 5, "userData": 0, "pointSourceId": 1, "gpsTime": null, "rgb": null}, {"x": 2.0, "y": 2.0, "z": 2.5, "intensity": 30, "returnNumber": 1, "numberOfReturns": 1, "scanDirectionFlag": true, "edgeOfFlightLine": false, "classification": 6, "scanAngleRank": -5, "userData": 0, "pointSourceId": 1, "gpsTime": null, "rgb": null}]} |
      | set-version            | {"major": 1, "minor": 1} |
      | set-system-identifier  | {"systemIdentifier": "RENAMED-SYSTEM"} |
      | set-software-info      | {"generatingSoftware": "renamed-software"} |
      | set-creation-date      | {"dayOfYear": 42, "year": 2027} |
      | set-scale-and-offset   | {"scale": [0.0005, 0.0005, 0.0005], "offset": [583000.0, 5804000.0, 0.0]} |
      | set-bounds             | {"max": [583020.0, 5804020.0, 20.0], "min": [582980.0, 5803980.0, -20.0]} |
      | set-points-by-return   | {"counts": [8000, 300, 100, 40, 8]} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real point cloud without passing bytes through
    Given the real input point cloud shared://🧪️pattern-sphere/🧊️.las
    When the point cloud is decoded to the typed snapshot and re-encoded from it alone
    Then the oracle and the subject agree on the semantic projection
    And the re-encoded bytes are not bit-identical to the input
