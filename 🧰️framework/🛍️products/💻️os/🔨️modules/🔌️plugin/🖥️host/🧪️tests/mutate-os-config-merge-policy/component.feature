@capability-os-config-merge-policy-1-mutate
@no-oracle-os-config-merge-policy-mutation-semantics
@comparison-ordered-json-v1
@mutations-os-config-merge-policy-1-any
Feature: Apply every typed merge-policy mutation to its committed specification vector
  `os.config.merge-policy` is this operating system's own authority configuration: a single
  `policy` choice that decides which severity a replicated outcome must reach before the authority
  quarantines it rather than merging it. No third party implements it and none could adjudicate it,
  so there is no reference implementation to register (recorded as the
  `os-config-merge-policy-mutation-semantics` no-oracle decision in
  `../../../../../🎚️config/🧪️oracle/🔣️.json`).

  ⚠️ Like its opening-preferences sibling, this case lives under `🔌️plugin/🖥️host` rather than beside
  the vocabulary it exercises, because `🎚️config` has no crate of its own: `📦️glue.rs` in
  `🖥️host/📦️packages/🦀️rust` is the ONE place `MergePolicyConfigMutation` is mounted, and a case
  placed at `🎚️config` would resolve its subject crate to the OS kernel, which does not expose the
  facet at all.

  The vocabulary closes at ONE kind, and that is a statement about the record rather than an
  omission. `MergePolicySetting` holds exactly one field, so there is nothing to insert, remove or
  reorder; a change IS the whole record. The consequence is that this facet's declared `Diff` type
  is the setting itself, and its `apply` ignores `base` outright — which is precisely why the one
  guard the kind carries matters so much. Setting the policy to the one already active returns
  `MutationOutcome::new(*base)` — the UNCHANGED record — with a `mutation.no-op` warning, rather
  than `MutationOutcome::empty()`; an empty diff would apply as `MergePolicySetting::default()` and
  silently loosen a `Vigilant` authority back to `Normal`. Both branches are exercised here: the
  committed vector takes the accepted one (`Normal` → `Vigilant`), and the guard scenario takes the
  refused one against the same committed record.

  Because this case records a no-oracle decision the runner executes NO oracle role, so every
  assertion lives inside the subject handler, which compares the applied setting against the
  committed after-snapshot and checks the reported diagnostics against the committed outcome. A
  handler that merely ran the mutation and returned would report a pass having checked nothing.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to its committed before-snapshot fixture
    Given the committed before-snapshot, mutation and outcome fixtures for the <id> kind
    When <id> is applied through apply_merge_policy_config_mutation_reporting
      """
      {"kind": "<id>", "policy": "<policy>", "wasPolicy": "<wasPolicy>"}
      """
    Then the resulting setting matches the committed after-snapshot, carries the <policy> authority, no longer carries <wasPolicy>, and reports the committed outcome
    Examples:
      | id                  | policy   | wasPolicy |
      | change-merge-policy | Vigilant | Normal    |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot fixture
    Given the committed before-snapshot and mutation fixtures for the <id> kind
    When <id> is applied and then its own computed inverse steps are applied
      """
      {"kind": "<id>", "policy": "<policy>", "wasPolicy": "<wasPolicy>"}
      """
    Then the setting equals the committed before-snapshot again and carries <wasPolicy> once more
    Examples:
      | id                  | policy   | wasPolicy |
      | change-merge-policy | Vigilant | Normal    |

  @id-no-op-guard
  @level-exhaustive
  @mode-error
  Scenario: Re-applying the policy a record already carries is a warned no-op that keeps the record
    Given the committed before-snapshot of the change-merge-policy vector
    When change-merge-policy is applied carrying the policy that record already holds
    Then a mutation.no-op warning is raised and the setting is returned unchanged rather than reset to the facet default

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the committed merge-policy setting without passing bytes through
    Given the committed before-snapshot of the change-merge-policy vector
    When the setting is decoded into MergePolicySetting and re-encoded from the typed value alone
    Then the re-encoded projection is the committed one, and the decode is proven real by reading the active policy back off the typed value rather than off the text
