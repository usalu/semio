@capability-energy-model-1-io
@oracle-energyplus-25-2-0-direct-epjson
@oracle-input-subject-raw
@comparison-ordered-json-v1
Feature: The epJSON this subset writes is a real EnergyPlus document, and EnergyPlus agrees about the building

  This case judges the `⚡️epjson` io leaves, not the simulation kernel. `🏛️simulate-bestest-energyplus`
  asks whether semio's own solver gets the physics right; this one asks whether semio's own DOCUMENT
  says what it claims to say — and it asks EnergyPlus, in EnergyPlus's own terms, with no translator
  anywhere in the path.

  THE SECOND ORACLE ROUTE. `🏛️simulate-bestest-energyplus` reaches EnergyPlus through
  honeybee-energy → honeybee-openstudio → OpenStudio, and `📓️w2-oracle-toolchain.md` measures that
  translation as worth +5.7…+8.1 % of annual cooling all by itself. This case reaches the SAME
  EnergyPlus 25.2.0 binary out of the SAME sha256-verified archive with nothing in between:
  `🚪️io/📤️export/🧵️serializers/…/⚡️epjson/🔖️25.2/✳️any` writes the document, and
  `energyplus -a -w <epw> -d <dir> <file.epJSON>` runs it. Where the two routes agree, the residual
  disagreement between semio and EnergyPlus is physics; where they disagree, it is the honeybee
  translation. That is the only way to tell those two apart, and it is why this case exists.

  WHAT THE SUBJECT ACTUALLY PRODUCES. The subject's real output is the RAW epJSON, handed to the
  oracle verbatim under `@oracle-input-subject-raw` — the oracle never re-authors a document and
  never reads the subject's Rust. The subject's PROJECTION is what it claims about those bytes; the
  oracle recomputes every claimed field from the bytes themselves (and, for the run scenarios, from
  what EnergyPlus reports back about them), so a wrong document cannot agree.

  ONE INPUT. Every case is the committed `🧫️fixtures/🏛️bestest-<case>/🔋️model.json` — the same one
  input `🏛️simulate-bestest-energyplus` uses. ONE WEATHER FILE: the committed
  `🧫️fixtures/🌦️denver-tmy/🌦️.epw`, whose sha256 travels in the result document.

  THE THIRD-PARTY VALIDATOR. Schema conformance is not asserted by reading the schema by hand: the
  oracle runs `jsonschema` 4.26.0 (a third-party draft-07 validator, pinned in the oracle project's
  `test` dependency group) against EnergyPlus's OWN `Energy+.schema.epJSON`, shipped inside the
  verified OpenStudio 3.11.0 archive. Every violation is reported with its JSON path.

  ⚠️ HONEST BOUNDARIES, stated rather than tuned away.
  1. There is NO skip channel in this host — "a missing registration, a panic and an error are all
     results, never a silent skip". A host without the provisioned toolchain therefore FAILS with a
     message naming the absent binary and the `oracle-setup` target that produces it, rather than
     reporting a green that means nothing. `oracle-status` prints the same paths.
  2. Cases 630/930 (fins) and 650/950 (night ventilation) are absent, exactly as in the sibling
     case: no committed `🔮️energyplus.json` exists for them, so a run scenario could only ever be
     red for want of a reference.
  3. The run scenarios are restricted to 600/600FF/900/900FF. Every case is ~25 s of EnergyPlus and
     every one of them is the same codec on the same object subset; the schema-validity scenarios
     cover all ten committed cases at `@level-quick` instead.
  4. The exported window is a `WindowMaterial:SimpleGlazingSystem`, because `Fenestration` carries
     U/SHGC/VLT and no layer stack. Both routes therefore consume the SAME simple glazing, which is
     precisely why the tolerance below is 3 % and not the sibling case's 10 %: the glazing offset
     `📓️bestest-contract.md` documents is common to both sides here and cannot appear as a
     difference.

  Tolerance: annual heating and cooling within 3 % relative, free-float minimum/maximum/mean within
  ±0.5 K, of the committed `🔮️energyplus.json`. Both numbers come out of EnergyPlus 25.2.0; the only
  thing that can move them is the document. The oracle reports every metric separately with its
  measured deviation and writes its full `semio.energy.bestest-results/1` document as an artifact, so
  a failure names the metric and the number rather than only saying no.

  @id-schema-validity
  @level-quick
  @mode-differential
  Scenario Outline: Case <case>'s exported epJSON validates against EnergyPlus's own schema
    Given the committed case model asset://🧫️fixtures/🏛️bestest-<case>/🔋️model.json
    When the subject exports it as epJSON and a third-party JSON Schema validator checks those exact bytes against Energy+.schema.epJSON
    Then there are no schema violations and both implementations read the same object types, zones, surfaces, apertures and envelope numbers out of the document
    Examples:
      | case   |
      | 600    |
      | 600FF  |
      | 610    |
      | 620    |
      | 640    |
      | 900    |
      | 900FF  |
      | 910    |
      | 920    |
      | 940    |

  @id-nothing-dropped
  @level-quick
  @mode-differential
  Scenario Outline: Case <case>'s export reports every thing it could not carry
    Given the committed case model asset://🧫️fixtures/🏛️bestest-<case>/🔋️model.json
    When the subject exports it as epJSON and lists what the covered object subset could not represent
    Then nothing the model states is missing from the document without a structured diagnostic naming it
    Examples:
      | case   |
      | 600    |
      | 900    |

  @id-energyplus-run
  @level-long
  @mode-differential
  Scenario Outline: EnergyPlus run on case <case>'s exported epJSON reproduces the committed reference
    Given the committed case model asset://🧫️fixtures/🏛️bestest-<case>/🔋️model.json
    And the committed annual weather asset://🧫️fixtures/🌦️denver-tmy/🌦️.epw
    And the committed EnergyPlus reference asset://🧫️fixtures/🏛️bestest-<case>/🔮️energyplus.json
    When the subject exports it as epJSON and EnergyPlus 25.2.0 simulates those exact bytes directly, with no translator
    Then the annual energies, free-float temperatures and the building EnergyPlus reports back agree with the committed reference within the declared tolerance
    Examples:
      | case   |
      | 600    |
      | 600FF  |
      | 900    |
      | 900FF  |
