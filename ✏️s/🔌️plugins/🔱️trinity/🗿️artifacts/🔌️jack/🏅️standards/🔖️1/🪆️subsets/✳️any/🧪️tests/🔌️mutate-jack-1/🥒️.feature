@capability-jack-1-mutate
@oracle-jack-python-independent
@comparison-ordered-json-v1
@mutations-jack-1-any
Feature: Compare the complete Jack parent query event and its literal child address in Rust and Python

  Both implementations read the authored closed parent carrier. The content reference remains opaque;
  its child id, artifact id, kind, standard and subset are compared unchanged through each parent query event.
  Registered rich graph storage, edits, UI and archive laws belong to the specific composition.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the authored parent
    Given the authored parent asset://🎬️demo/🔣️.json
    When the <id> mutation is applied
      """
      <mutation>
      """
    Then both implementations compare every parent field and its complete unchanged child address
    Examples:
      | id        | mutation                                                        |
      | set-query | {"mutation":"setQuery","value":"MATCH (a:Piece) RETURN a.name"} |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Restore the authored parent with the inverse of <id>
    Given the authored parent asset://🎬️demo/🔣️.json
    When the <id> mutation and its computed inverse are applied
      """
      <mutation>
      """
    Then both implementations compare the changed parent and exact restored parent
    Examples:
      | id        | mutation                                                        |
      | set-query | {"mutation":"setQuery","value":"MATCH (a:Piece) RETURN a.name"} |

  @id-spec-vector
  @level-exhaustive
  @mode-differential
  Scenario Outline: Replay the committed <id> parent specification
    Given the committed parent shared://🧬️mutations/<dir>/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed event shared://🧬️mutations/<dir>/<fixture>/🦠️mutation/🔣️.json
    And the committed after-parent shared://🧬️mutations/<dir>/<fixture>/📸️snapshot/➡️after/🔣️.json
    When the event is applied
      """
      {"verdict":"<verdict>"}
      """
    Then both implementations produce the complete committed parent
    Examples:
      | id        | verdict | dir         | fixture              |
      | set-query | applied | 🔎️set-query | 🔎️replaces-the-query |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Read and re-encode the complete authored parent
    Given the authored parent asset://🎬️demo/🔣️.json
    When each implementation parses and prints its declared parent carrier
    Then both implementations retain every parent field and literal child address
