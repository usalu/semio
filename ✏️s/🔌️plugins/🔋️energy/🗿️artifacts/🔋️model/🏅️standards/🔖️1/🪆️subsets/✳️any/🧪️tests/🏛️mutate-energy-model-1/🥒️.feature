@capability-energy-model-1-mutate
@oracle-energy-model-1-python-independent
@comparison-ordered-json-v1
@mutations-energy-model-1-any
Feature: Apply every typed s.energy.model mutation against an independent Python implementation

  `s.energy.model` is a semio-NATIVE artifact and no third party reads or writes `.dsl.semio` — the
  recorded survey (kept verbatim in `🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracle/🔣️.json`'s history)
  named and DECLINED EnergyPlus and OpenStudio, and the `energyplus` weather reader already
  registered under `✏️s/🔌️plugins/🗄️stdio`'s `🌦️epw` subset is deliberately NOT reused here. The
  second producer a differential comparison needs is therefore a second IMPLEMENTATION, and
  `🐍️.py` beside this file is it: written in Python from this subset's own committed
  `🧬️schema/📸️snapshot/🔣️.json`, its per-kind payload schemas, and
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/`'s
  `📓️taxonomy.md` verb table and `📓️derivation-rules.md` shape rules. It imports nothing from the
  Rust it judges and transliterates none of it.

  The honest boundary this feature used to carry is GONE. `replace-model` — a whole-document swap
  the taxonomy bans outright — has been removed from the vocabulary; whole-model load now goes
  through `store::ArtifactStore::reset`, outside history. In its place every kind below carries two
  committed specification vectors, one that really moves the document and one that is really
  refused, so `UNOBSERVABLE` is empty and no kind's coverage is manufactured.

  Both implementations read the SAME committed bytes: the `(before, mutation, after, diff, outcome)`
  quintet under `🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/`, declared as `asset://` fixtures so
  the plan pins their digests and a Python reference can resolve them at all.

  Where the assertions live. `mutate-<id>` and `inverse-<id>` dispatch BOTH an oracle role (the
  Python implementation, reached through this plugin's `oracleHostPackages` entry) and a subject
  role (this repository's own `energy_model_mutation_report_json`), each independently asserting the
  forward/inverse laws in role before the two are compared byte for byte.
  `identity-round-trip` keeps asserting through the shared law module
  `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law/🦀️.rs` that the stdio subsets use.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to its committed specification vector
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🎯️outcome/🔣️.json
    When both implementations apply the committed mutation to the committed before-snapshot
    Then each reaches the committed after-snapshot under the committed outcome status and the two agree
    Examples:
      | id | dir | fixture |
      | rename-model-renames-the-model | 🏷️rename-model | ✅️renames-the-model |
      | rename-model-refuses-a-blank-name | 🏷️rename-model | ⛔️refuses-a-blank-name |
      | change-model-version-bumps-the-version | 🔢️change-model-version | ✅️bumps-the-version |
      | change-model-version-refuses-a-blank-version | 🔢️change-model-version | ⛔️refuses-a-blank-version |
      | update-site-relocates-to-denver | 🌍️update-site | ✅️relocates-to-denver |
      | update-site-refuses-a-bad-latitude | 🌍️update-site | ⛔️refuses-a-bad-latitude |
      | update-ground-temperature-sets-denver-ground | 🌡️update-ground-temperature | ✅️sets-denver-ground |
      | update-ground-temperature-refuses-a-short-year | 🌡️update-ground-temperature | ⛔️refuses-a-short-year |
      | update-run-period-shortens-to-january | 📅️update-run-period | ✅️shortens-to-january |
      | update-run-period-refuses-month-13 | 📅️update-run-period | ⛔️refuses-month-13 |
      | replace-airflow-network-attaches-a-network | 🫧️replace-airflow-network | ✅️attaches-a-network |
      | replace-airflow-network-refuses-unpaired-nodes | 🫧️replace-airflow-network | ⛔️refuses-unpaired-nodes |
      | add-output-variable-adds-zone-air-temp | 📊️add-output-variable | ✅️adds-zone-air-temp |
      | add-output-variable-refuses-a-duplicate | 📊️add-output-variable | ⛔️refuses-a-duplicate |
      | remove-output-variable-drops-zone-air-temp | 📉️remove-output-variable | ✅️drops-zone-air-temp |
      | remove-output-variable-refuses-an-absent-one | 📉️remove-output-variable | ⛔️refuses-an-absent-one |
      | bind-weather-file-binds-hannover-epw | 🌦️bind-weather-file | ✅️binds-hannover-epw |
      | bind-weather-file-refuses-a-bad-uri | 🌦️bind-weather-file | ⛔️refuses-a-bad-uri |
      | unbind-weather-file-unbinds-the-weather | 🌤️unbind-weather-file | ✅️unbinds-the-weather |
      | unbind-weather-file-refuses-when-unbound | 🌤️unbind-weather-file | ⛔️refuses-when-unbound |
      | connect-referenced-model-connects-the-geometry | 🪢️connect-referenced-model | ✅️connects-the-geometry |
      | connect-referenced-model-refuses-a-bad-uri | 🪢️connect-referenced-model | ⛔️refuses-a-bad-uri |
      | disconnect-referenced-model-disconnects-the-geometry | ✂️disconnect-referenced-model | ✅️disconnects-the-geometry |
      | disconnect-referenced-model-refuses-when-absent | ✂️disconnect-referenced-model | ⛔️refuses-when-absent |
      | rename-zone-renames-zone-one | 🏠️rename-zone | ✅️renames-zone-one |
      | rename-zone-refuses-a-missing-zone | 🏠️rename-zone | ⛔️refuses-a-missing-zone |
      | change-zone-volume-resizes-zone-one | 📦️change-zone-volume | ✅️resizes-zone-one |
      | change-zone-volume-refuses-zero-volume | 📦️change-zone-volume | ⛔️refuses-zero-volume |
      | change-zone-multiplier-stacks-four-storeys | ✖️change-zone-multiplier | ✅️stacks-four-storeys |
      | change-zone-multiplier-refuses-zero-instances | ✖️change-zone-multiplier | ⛔️refuses-zero-instances |
      | change-zone-conditioned-frees-the-zone | 🌬️change-zone-conditioned | ✅️frees-the-zone |
      | change-zone-conditioned-refuses-a-missing-zone | 🌬️change-zone-conditioned | ⛔️refuses-a-missing-zone |
      | change-zone-floor-area-participation-excludes-the-zone | 📐️change-zone-floor-area-participation | ✅️excludes-the-zone |
      | change-zone-floor-area-participation-refuses-a-missing-zone | 📐️change-zone-floor-area-participation | ⛔️refuses-a-missing-zone |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores its committed before-snapshot
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🎯️outcome/🔣️.json
    When each implementation applies the committed mutation and then its OWN computed inverse
    Then both restore the before-snapshot and agree on the mutated and the restored document
    Examples:
      | id | dir | fixture |
      | rename-model-renames-the-model | 🏷️rename-model | ✅️renames-the-model |
      | rename-model-refuses-a-blank-name | 🏷️rename-model | ⛔️refuses-a-blank-name |
      | change-model-version-bumps-the-version | 🔢️change-model-version | ✅️bumps-the-version |
      | change-model-version-refuses-a-blank-version | 🔢️change-model-version | ⛔️refuses-a-blank-version |
      | update-site-relocates-to-denver | 🌍️update-site | ✅️relocates-to-denver |
      | update-site-refuses-a-bad-latitude | 🌍️update-site | ⛔️refuses-a-bad-latitude |
      | update-ground-temperature-sets-denver-ground | 🌡️update-ground-temperature | ✅️sets-denver-ground |
      | update-ground-temperature-refuses-a-short-year | 🌡️update-ground-temperature | ⛔️refuses-a-short-year |
      | update-run-period-shortens-to-january | 📅️update-run-period | ✅️shortens-to-january |
      | update-run-period-refuses-month-13 | 📅️update-run-period | ⛔️refuses-month-13 |
      | replace-airflow-network-attaches-a-network | 🫧️replace-airflow-network | ✅️attaches-a-network |
      | replace-airflow-network-refuses-unpaired-nodes | 🫧️replace-airflow-network | ⛔️refuses-unpaired-nodes |
      | add-output-variable-adds-zone-air-temp | 📊️add-output-variable | ✅️adds-zone-air-temp |
      | add-output-variable-refuses-a-duplicate | 📊️add-output-variable | ⛔️refuses-a-duplicate |
      | remove-output-variable-drops-zone-air-temp | 📉️remove-output-variable | ✅️drops-zone-air-temp |
      | remove-output-variable-refuses-an-absent-one | 📉️remove-output-variable | ⛔️refuses-an-absent-one |
      | bind-weather-file-binds-hannover-epw | 🌦️bind-weather-file | ✅️binds-hannover-epw |
      | bind-weather-file-refuses-a-bad-uri | 🌦️bind-weather-file | ⛔️refuses-a-bad-uri |
      | unbind-weather-file-unbinds-the-weather | 🌤️unbind-weather-file | ✅️unbinds-the-weather |
      | unbind-weather-file-refuses-when-unbound | 🌤️unbind-weather-file | ⛔️refuses-when-unbound |
      | connect-referenced-model-connects-the-geometry | 🪢️connect-referenced-model | ✅️connects-the-geometry |
      | connect-referenced-model-refuses-a-bad-uri | 🪢️connect-referenced-model | ⛔️refuses-a-bad-uri |
      | disconnect-referenced-model-disconnects-the-geometry | ✂️disconnect-referenced-model | ✅️disconnects-the-geometry |
      | disconnect-referenced-model-refuses-when-absent | ✂️disconnect-referenced-model | ⛔️refuses-when-absent |
      | rename-zone-renames-zone-one | 🏠️rename-zone | ✅️renames-zone-one |
      | rename-zone-refuses-a-missing-zone | 🏠️rename-zone | ⛔️refuses-a-missing-zone |
      | change-zone-volume-resizes-zone-one | 📦️change-zone-volume | ✅️resizes-zone-one |
      | change-zone-volume-refuses-zero-volume | 📦️change-zone-volume | ⛔️refuses-zero-volume |
      | change-zone-multiplier-stacks-four-storeys | ✖️change-zone-multiplier | ✅️stacks-four-storeys |
      | change-zone-multiplier-refuses-zero-instances | ✖️change-zone-multiplier | ⛔️refuses-zero-instances |
      | change-zone-conditioned-frees-the-zone | 🌬️change-zone-conditioned | ✅️frees-the-zone |
      | change-zone-conditioned-refuses-a-missing-zone | 🌬️change-zone-conditioned | ⛔️refuses-a-missing-zone |
      | change-zone-floor-area-participation-excludes-the-zone | 📐️change-zone-floor-area-participation | ✅️excludes-the-zone |
      | change-zone-floor-area-participation-refuses-a-missing-zone | 📐️change-zone-floor-area-participation | ⛔️refuses-a-missing-zone |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse the real committed energy model document, print it back and cross it against its binary encoding
    Given the real committed document asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    When the document is parsed, printed back to canonical DSL, parsed again, and separately encoded to a pack and decoded back
    Then every decoding agrees on one snapshot, and printing the canonical text a second time reproduces it byte for byte as ArtifactDsl's own fixpoint law requires
