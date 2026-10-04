@capability-jack-1-mutate
@oracle-jack-python-independent
@comparison-ordered-json-v1
@mutations-jack-1-any
Feature: Apply jack's parent-lane mutation twice — once in Rust, once in Python — and require the same answer

  This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️.py` in this directory: a second implementation of the
  `s.trinity.jack` parent document, of its `.dsl.semio` carrier and of its one parent-lane mutation, `set-query`, written
  in Python from `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`, from
  `…/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio` and from the committed specification vector. It imports nothing from
  this repository's Rust.

  The scene — nodes with ports, edges over `node@port` endpoints — lives in the composed `content` child
  (`s.stdio.semio@v1/graph`). Its edits are that shared graph vocabulary's leaves (design §20.15 of ticket
  26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING) and are adjudicated by the graph subset's own oracles, never by a jack parent
  leaf. Both implementations still read the real committed Nakagin Capsule Tower scene through the carrier and project it,
  so a parent edit that disturbed the scene would be caught here.

  `set-query` edits the document's query, never the scene: applying it must move `query` and leave every scene member
  alone, its own computed inverse must restore the document member for member, and its committed vector is applied
  against the composed scene committed beside this case, `🧩️capsule-stack.scene.json`.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real Nakagin Capsule Tower scene
    Given the real committed scene asset://🎬️demo/🗣️.dsl.semio
    When the <id> mutation is applied with the parameters the feature states
      """
      <mutation>
      """
    Then both implementations produce the same scene
    Examples:
      | id        | mutation                                                        |
      | set-query | {"mutation":"setQuery","value":"MATCH (a:Piece) RETURN a.name"} |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undo <id> on the real tower scene and land back on it
    Given the real committed scene asset://🎬️demo/🗣️.dsl.semio
    When the <id> mutation is applied and then its own computed inverse is applied
      """
      <mutation>
      """
    Then both implementations agree on the mutated scene AND on the restored one, member for member and index for index
    Examples:
      | id        | mutation                                                        |
      | set-query | {"mutation":"setQuery","value":"MATCH (a:Piece) RETURN a.name"} |

  @id-spec-vector
  @level-exhaustive
  @mode-differential
  Scenario Outline: Replay the committed <id> specification vector through both implementations
    Given the committed before-scene shared://🧬️mutations/<dir>/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation shared://🧬️mutations/<dir>/<fixture>/🦠️mutation/🔣️.json
    And the committed after-scene shared://🧬️mutations/<dir>/<fixture>/📸️snapshot/➡️after/🔣️.json
    And the composed scene both hold shared://🔌️mutate-jack-1/<scene>
    When the committed mutation is applied to the committed before-scene
      """
      {"verdict": "<verdict>"}
      """
    Then each implementation gives the committed <verdict> answer in role, and the two agree
    Examples:
      | id        | verdict | dir         | fixture              | scene                      |
      | set-query | applied | 🔎️set-query | 🔎️replaces-the-query | 🧩️capsule-stack.scene.json |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Read the real committed Nakagin tower scene in both languages and agree on it
    Given the real committed scene asset://🎬️demo/🗣️.dsl.semio
    When each implementation parses it, prints it back through its own carrier and parses it again
    Then both languages read the same nine nodes and six edges out of the same real bytes, the Python reproduces the file byte for byte, and the Rust holds its own canonical printing to ArtifactDsl's fixpoint law
