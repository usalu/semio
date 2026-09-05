@capability-os-config-identity-1-mutate
@no-oracle-os-config-identity-mutation-semantics
@comparison-ordered-json-v1
@mutations-os-config-identity-1-any
Feature: Apply every typed identity mutation to its committed specification vectors
  `os.config.identity` is this operating system's own session record: the account the operator is
  signed in as, or nothing at all. No third party implements it and none could adjudicate it, so
  there is no reference implementation to register (recorded as the
  `os-config-identity-mutation-semantics` no-oracle decision in
  `../../../../../🎚️config/🧪️oracle/🔣️.json`).

  ⚠️ Like its opening-preferences and merge-policy siblings, this case lives under `🔌️plugin/🖥️host`
  rather than beside the vocabulary it exercises, because `🎚️config` has no crate of its own:
  `📦️glue.rs` in `🖥️host/📦️packages/🦀️rust` is the ONE place `IdentityConfigMutation` is mounted, and
  a case placed at `🎚️config` would resolve its subject crate to the OS kernel, which does not
  expose the facet at all.

  Two kinds close this vocabulary because `IdentitySetting` is `Option<Identity>` and nothing else:
  a session is established whole or it is absent whole, so there is no rename, no per-field edit and
  no reorder to have. What that leaves is a record whose signed-out state is the bare JSON literal
  `null` — the transparent `Option` gives it no other spelling — and both committed vectors are
  written to pin that: `sign-in` runs against an ALREADY signed-in record so it exercises the
  REPLACEMENT branch rather than the establishment branch, and `sign-out` runs against the same
  active session so its undo has a prior account to restore.

  That choice is what makes the inverse law load-bearing here. `sign-in`'s inverse reads BASE, never
  its own payload: replacing Ada's session with Grace's must undo to `sign-in(Ada)` and not to
  `sign-out`, or the undo would sign the operator out of an account they were signed into before the
  mutation ran. `sign-out`'s inverse reads BASE the same way, which is why signing out of an ALREADY
  signed-out record yields no step at all — the one branch no committed vector can express, because
  it produces nothing to commit, and which the guard scenario states as a property instead.

  Because this case records a no-oracle decision the runner executes NO oracle role, so every
  assertion lives inside the subject handler, which compares the applied record against the
  committed after-snapshot, checks the declared session claim, and checks the reported diagnostics
  against the committed outcome. A handler that merely ran the mutation and returned would report a
  pass having checked nothing.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to its committed before-snapshot fixture
    Given the committed before-snapshot, mutation and outcome fixtures for the <id> kind
    When <id> is applied through apply_identity_config_mutation_reporting
      """
      {"kind": "<id>", "account": "<account>", "wasAccount": "<wasAccount>"}
      """
    Then the resulting record matches the committed after-snapshot, holds the <account> session, no longer holds <wasAccount>, and reports the committed outcome
    Examples:
      | id       | account | wasAccount |
      | sign-in  | grace   | ada        |
      | sign-out | none    | ada        |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot fixture
    Given the committed before-snapshot and mutation fixtures for the <id> kind
    When <id> is applied and then its own computed inverse steps are applied
      """
      {"kind": "<id>", "account": "<account>", "wasAccount": "<wasAccount>"}
      """
    Then the record equals the committed before-snapshot again and holds the <wasAccount> session with its original token and issue time
    Examples:
      | id       | account | wasAccount |
      | sign-in  | grace   | ada        |
      | sign-out | none    | ada        |

  @id-signed-out-inverse-is-empty
  @level-exhaustive
  @mode-property
  Scenario: Signing out of a record that already holds no session has no undo step to offer
    Given the committed after-snapshot of the sign-out vector, which holds no session
    When sign-out is applied to it and its own inverse steps are computed from that same record
    Then the record stays signed out and the inverse is empty rather than a sign-in carrying a fabricated session

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the committed signed-in record without passing bytes through
    Given the committed before-snapshot of the sign-in vector
    When the record is decoded into IdentitySetting and re-encoded from the typed value alone
    Then the re-encoded projection is the committed one, field for field, and the decode is proven real by reading the account and its token back off the typed value rather than off the text
