@capability-os-config-local-folders-1-mutate
@no-oracle-os-config-local-folders-mutation-semantics
@comparison-ordered-json-v1
@mutations-os-config-local-folders-1-any
Feature: Apply every typed local-folder binding mutation to its committed specification vectors
  `os.config.local-folders` is this operating system's own persisted-local-only record of the folder on one device each
  attached document's archive persists in: bindings keyed on the document's own identity, each naming the program that
  holds the document and the folder path. It never enters a shared lane or a URL. No third party implements it and none
  could adjudicate it, so there is no reference implementation to register (recorded as the
  `os-config-local-folders-mutation-semantics` no-oracle decision in `../../../../../🎚️config/🔮️oracles/🔣️.json`).

  ⚠️ Like its local-catalog sibling, this case lives under `🔌️plugin/🖥️host` rather than beside the vocabulary it
  exercises, because `🖥️host/📦️packages/🦀️rust` is the ONE place `LocalFoldersConfigMutation` is mounted for these cases.

  Two kinds close the vocabulary: an attachment is an UPSERT keyed on `documentId` (a re-attachment replaces the binding,
  a new document is inserted in id order) and a detachment forgets one document's folder without touching the folder or
  the archive in it. Both committed vectors run against bindings that also hold a puzzle's folder, which is what lets each
  kind be held to the claim that it touches its own document and leaves the sibling standing: the `sibling` column names
  the binding that must survive untouched.

  Because this case records a no-oracle decision the runner executes NO oracle role, so every assertion lives inside the
  subject handler, which compares the applied bindings against the committed after-snapshot, checks the binding and
  sibling claims, and checks the reported diagnostics against the committed outcome.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to its committed before-snapshot fixture
    Given the committed before-snapshot, mutation and outcome fixtures for the <id> kind
    When <id> is applied through apply_local_folders_config_mutation_reporting
      """
      {"kind": "<id>", "document": "<document>", "bound": "<bound>", "sibling": "<sibling>"}
      """
    Then the resulting bindings match the committed after-snapshot, bind <document> exactly when <bound> says so, keep <sibling> untouched, and report the committed outcome
    Examples:
      | id                  | document            | bound | sibling           |
      | attach-local-folder | cad.drawing.fixture | yes   | board.ports.directed.v1 |
      | detach-local-folder | cad.drawing.fixture | no    | board.ports.directed.v1 |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot fixture
    Given the committed before-snapshot and mutation fixtures for the <id> kind
    When <id> is applied and then its own computed inverse steps are applied
      """
      {"kind": "<id>", "document": "<document>", "bound": "<bound>", "sibling": "<sibling>"}
      """
    Then the bindings equal the committed before-snapshot again, binding for binding, including every folder
    Examples:
      | id                  | document            | bound | sibling           |
      | attach-local-folder | cad.drawing.fixture | yes   | board.ports.directed.v1 |
      | detach-local-folder | cad.drawing.fixture | no    | board.ports.directed.v1 |

  @id-unbound-detachment-has-no-undo
  @level-exhaustive
  @mode-property
  Scenario: Detaching a document the bindings do not hold changes nothing and has no undo step to offer
    Given the committed after-snapshot of the detach vector, which no longer binds cad.drawing.fixture
    When detach-local-folder is applied to it again and its own inverse steps are computed from those same bindings
    Then the bindings stay as they were, the only diagnostic is the no-op warning, and the inverse is empty rather than an attachment to an invented folder

  @id-local-folders-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the committed two-binding record without passing bytes through
    Given the committed before-snapshot of the detach vector
    When the bindings are decoded into LocalFolderBindings and re-encoded from the typed value alone
    Then the re-encoded projection is the committed one, and the decode is proven real by reading both ids and the puzzle's folder path back off the typed value
