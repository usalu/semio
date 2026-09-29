@capability-os-config-local-catalog-1-mutate
@no-oracle-os-config-local-catalog-mutation-semantics
@comparison-ordered-json-v1
@mutations-os-config-local-catalog-1-any
Feature: Apply every typed local-catalog mutation to its committed specification vectors
  `os.config.local-catalog` is this operating system's own persisted-local-only record of the documents one device keeps
  on its own disk: entries keyed on `documentId`, each naming the document's artifact schema, its name and the folder
  event log or single file its events persist in. No third party implements it and none could adjudicate it, so there
  is no reference implementation to register (recorded as the `os-config-local-catalog-mutation-semantics` no-oracle
  decision in `../../../../../🎚️config/🔮️oracles/🔣️.json`).

  ⚠️ Like its opening, merge-policy and identity siblings, this case lives under `🔌️plugin/🖥️host` rather than beside the
  vocabulary it exercises, because `🎚️config` has no crate of its own and `🖥️host/📦️packages/🦀️rust` is the ONE place
  `LocalCatalogConfigMutation` is mounted.

  Two kinds close the vocabulary: an admission is an UPSERT keyed on `documentId` (a re-admission replaces the entry, a new
  id is inserted in id order) and a retirement unlists one id without touching the document's own events. Both committed
  vectors run against a catalog that also lists an imported sibling, which is what lets each kind be held to the claim that
  it touches its own id and leaves the sibling standing: the `sibling` column names the entry that must survive untouched.

  Because this case records a no-oracle decision the runner executes NO oracle role, so every assertion lives inside the
  subject handler, which compares the applied catalog against the committed after-snapshot, checks the listing and sibling
  claims, and checks the reported diagnostics against the committed outcome.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to its committed before-snapshot fixture
    Given the committed before-snapshot, mutation and outcome fixtures for the <id> kind
    When <id> is applied through apply_local_catalog_config_mutation_reporting
      """
      {"kind": "<id>", "document": "<document>", "listed": "<listed>", "sibling": "<sibling>"}
      """
    Then the resulting catalog matches the committed after-snapshot, lists <document> exactly when <listed> says so, keeps <sibling> untouched, and reports the committed outcome
    Examples:
      | id                    | document     | listed | sibling         |
      | admit-local-document  | studio-alpha | yes    | studio-imported |
      | retire-local-document | studio-alpha | no     | studio-imported |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot fixture
    Given the committed before-snapshot and mutation fixtures for the <id> kind
    When <id> is applied and then its own computed inverse steps are applied
      """
      {"kind": "<id>", "document": "<document>", "listed": "<listed>", "sibling": "<sibling>"}
      """
    Then the catalog equals the committed before-snapshot again, entry for entry, including every admission time
    Examples:
      | id                    | document     | listed | sibling         |
      | admit-local-document  | studio-alpha | yes    | studio-imported |
      | retire-local-document | studio-alpha | no     | studio-imported |

  @id-unlisted-retirement-has-no-undo
  @level-exhaustive
  @mode-property
  Scenario: Retiring an id the catalog does not list changes nothing and has no undo step to offer
    Given the committed after-snapshot of the retire vector, which no longer lists studio-alpha
    When retire-local-document is applied to it again and its own inverse steps are computed from that same catalog
    Then the catalog stays as it was, the only diagnostic is the no-op warning, and the inverse is empty rather than an admission of an invented entry

  @id-local-catalog-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the committed two-entry catalog without passing bytes through
    Given the committed before-snapshot of the retire vector
    When the catalog is decoded into LocalCatalog and re-encoded from the typed value alone
    Then the re-encoded projection is the committed one, and the decode is proven real by reading both ids and the file storage of the imported entry back off the typed value
