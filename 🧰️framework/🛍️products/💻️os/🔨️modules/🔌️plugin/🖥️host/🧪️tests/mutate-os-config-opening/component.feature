@capability-os-config-opening-1-mutate
@no-oracle-os-config-opening-preferences-mutation-semantics
@comparison-ordered-json-v1
@mutations-os-config-opening-1-any
Feature: Apply every typed opening-preferences mutation to its committed specification vectors
  `os.config.opening` is this operating system's own preference record: a list of
  `(artifact dialect, role) -> app` pins, where the dialect is a semio `ArtifactDialect` triple and
  the app a semio `AppRef`. No third party implements it and none could adjudicate it, so there is
  no reference implementation to register (recorded as the
  `os-config-opening-preferences-mutation-semantics` no-oracle decision in
  `../../../../../🎚️config/🧪️oracle/🔣️component.json`).

  ⚠️ Two things about this case are unusual and both are deliberate. First, the case lives under
  `🔌️plugin/🖥️host` rather than beside the vocabulary it exercises, because `🎚️config` has no crate
  of its own: `📦️glue.rs` in `🖥️host/📦️packages/🦀️rust` is the ONE place `OpeningConfigMutation` is
  mounted, and a case placed at `🎚️config` would resolve its subject crate to the OS kernel, which
  does not expose the facet at all. Second, `🎚️config` carries a SECOND mutation vocabulary that
  this case does not cover: `MergePolicyConfigMutation`'s `change-merge-policy` sits in a file whose
  own module doc states it is mounted in no crate's `📦️glue.rs`, so nothing compiles it — not even
  its own committed fixture. That gap is recorded as the `os-config-merge-policy-unmounted-facet`
  decision rather than papered over with a catalog nothing could execute.

  What distinguishes the vocabulary that IS here is how completely two kinds cover it.
  `OpeningPreferences` holds one list of pins keyed on `(dialect, role)`, so a pin is an UPSERT — it
  replaces the app for a key that already has one and appends for a key that does not — and a clear
  is an idempotent removal. That closes the vocabulary at two: there is no rename (a pin has no name
  beyond its key), no reorder (the list has no user-meaningful order), and no `set-snapshot`
  (whole-document replace goes through the store's non-history path). Both committed vectors run
  against the SAME two-pin document — a cad VIEWER pin and a cad EDITOR pin — which is what lets
  each kind be held to the claim that it touches its own key and leaves the sibling standing: the
  `sibling` column names the pin that must survive untouched, and `set-default-app` must repin the
  editor to `drafting` without moving the viewer, while `clear-default-app` must unpin the editor
  and leave the viewer exactly where it was.

  Because this case records a no-oracle decision the runner executes NO oracle role, so every
  assertion lives inside the subject handler, which compares the applied document against the
  committed after-snapshot, checks the sibling claim, and checks the reported diagnostics against
  the committed outcome. A handler that merely ran the mutation and returned would report a pass
  having checked nothing.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to its committed before-snapshot fixture
    Given the committed before-snapshot, mutation and outcome fixtures for the <id> kind
    When <id> is applied through apply_opening_config_mutation_reporting
      """
      {"kind": "<id>", "sibling": "<sibling>", "pins": <pins>}
      """
    Then the resulting document matches the committed after-snapshot, holds <pins> pin(s), leaves the <sibling> pin untouched, and reports the committed outcome
    Examples:
      | id                | sibling | pins |
      | set-default-app   | viewer  | 2    |
      | clear-default-app | viewer  | 1    |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot fixture
    Given the committed before-snapshot and mutation fixtures for the <id> kind
    When <id> is applied and then its own computed inverse steps are applied
      """
      {"kind": "<id>", "sibling": "<sibling>", "pins": <pins>}
      """
    Then the document equals the committed before-snapshot again, pin for pin and in the same order
    Examples:
      | id                | sibling | pins |
      | set-default-app   | viewer  | 2    |
      | clear-default-app | viewer  | 1    |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the committed two-pin preference record without passing bytes through
    Given the committed before-snapshot of the set-default-app vector
    When the record is decoded into OpeningPreferences and re-encoded from the typed value alone
    Then the re-encoded projection is the committed one, member for member, and the decode is proven real by reading the two pins back off the typed value rather than off the text
