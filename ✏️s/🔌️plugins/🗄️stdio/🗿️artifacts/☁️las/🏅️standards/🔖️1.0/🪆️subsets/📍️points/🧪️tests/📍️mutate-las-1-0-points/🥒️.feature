@capability-las-1-0-points-mutate
@oracle-las-1-0-any-mutate
@comparison-semantic-las-v1
@mutations-las-1.0-points
Feature: Apply every typed LAS 1.0 mutation to a real-world point cloud
  See ../🎩️mutate-las-1-0/🥒️.feature for the full fixture/provenance narrative -- this subset's own scenarios exercise only the mutation kinds `../../🏅️standards` places under this subset.

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
      | insert-point           | {"index": 4000, "point": {"x": 583005.0, "y": 5804005.0, "z": 5.0, "intensity": 4242, "returnNumber": 1, "numberOfReturns": 1, "scanDirectionFlag": true, "edgeOfFlightLine": false, "classification": 6, "scanAngleRank": 12, "userData": 1, "pointSourceId": 1, "gpsTime": null, "rgb": null}} |
      | remove-point           | {"index": 4000} |
      | set-point              | {"index": 4000, "point": {"x": 583006.0, "y": 5804006.0, "z": -8.0, "intensity": 999, "returnNumber": 1, "numberOfReturns": 1, "scanDirectionFlag": false, "edgeOfFlightLine": true, "classification": 9, "scanAngleRank": -30, "userData": 2, "pointSourceId": 1, "gpsTime": null, "rgb": null}} |

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
      | insert-point           | {"index": 4000, "point": {"x": 583005.0, "y": 5804005.0, "z": 5.0, "intensity": 4242, "returnNumber": 1, "numberOfReturns": 1, "scanDirectionFlag": true, "edgeOfFlightLine": false, "classification": 6, "scanAngleRank": 12, "userData": 1, "pointSourceId": 1, "gpsTime": null, "rgb": null}} |
      | remove-point           | {"index": 4000} |
      | set-point              | {"index": 4000, "point": {"x": 583006.0, "y": 5804006.0, "z": -8.0, "intensity": 999, "returnNumber": 1, "numberOfReturns": 1, "scanDirectionFlag": false, "edgeOfFlightLine": true, "classification": 9, "scanAngleRank": -30, "userData": 2, "pointSourceId": 1, "gpsTime": null, "rgb": null}} |
