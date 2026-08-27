# 🧪️ Repository test platform

A test is owned by the **nearest language-neutral domain owner** — the taxonomy entity that defines
the behaviour — never by a Rust crate, a TypeScript package, a Go module or a .NET project. Its
behaviour is specified once in a language-neutral feature file and exercised through one native
adapter per implementation.

```
<owner>/
├── 🧫️fixtures/                     immutable, shared by every case of this owner
├── 🧪️tests/<kebab-case>/
│   ├── 🧫️fixtures/                 immutable, private to this case
│   ├── component.feature           the normative, language-neutral contract
│   ├── 🦀️component.rs              one adapter per implementation that claims the capability
│   ├── 🟦️component.ts
│   ├── 🐹️component.go
│   ├── 🐍️component.py
│   └── 🔷️component.cs
└── 📦️packages/<language>/          the implementations under test
```

## Commands

```bash
bun ./📜️script.ts discover                 # every case Nx will generate a project for
bun ./📜️script.ts doctor                   # toolchains; a missing one fails setup, never skips
bun ./📜️script.ts contract                 # everything provable without executing a test
bun ./📜️script.ts oracle   <level>         # the reference implementation only
bun ./📜️script.ts subject  <level>         # this repository's implementations only
bun ./📜️script.ts parity   <level>         # oracle + subjects + semantic comparison
bun ./📜️script.ts run      <level>         # contract, then parity
bun ./📜️script.ts report                   # re-render JUnit from the last run
bun ./📜️script.ts clean [--dry] [--stale]  # marked generated test state, nothing else
bun ./📜️script.ts dependency               # oracle purity + production reachability
```

`--owner <path>`, `--case <slug>`, `--project <name>` and `--implementation <id>` narrow any phase.
From the repository root the same phases are reachable as `bun ./📜️script.ts test <phase>`, and every
case also has generated Nx targets (`test`, `test-quick`, `test-long`, `test-exhaustive`,
`test-contract`, `test-oracle`, `test-subject`, `test-parity`).

## Adding a feature — the lifecycle

1. **Declare the owner and capability.** Find the smallest language-neutral owner that defines the
   behaviour.
2. **Create the case.** `🧪️tests/<kebab-case>/component.feature`.
3. **Research a reference implementation.** Search `📇️registry/🔣️component.json` first, then the
   existing test dependencies. Only when no approved library can support the behaviour do you compare
   new candidates — on feature coverage, standard conformance, determinism, platform support,
   malformed-input behaviour, maintenance, license, transitive cost, offline operation, and whether
   every external type can be hidden behind the owned test protocol. The detailed comparison belongs
   in the ticket; the durable outcome is the registry entry and the lockfile.
4. **Write the scenarios.** Feature tags `@capability-…`, `@oracle-…` (or `@no-oracle-…`) and
   `@comparison-…`; per scenario `@id-…`, exactly one `@level-…` and exactly one `@mode-…`.
5. **Make `contract` green** before writing any code.
6. **Make `oracle` green.** This proves the library really supports the case, that the fixtures are
   valid, and that the expected result was not invented by hand.
7. **Implement the subject in one language** and make `subject` green.
8. **Implement every other claimed language** against the same feature and fixtures.
9. **Run `parity`.** Every subject must match the oracle, and the subjects must match each other.
10. **Run `dependency`.** Every external dependency of the feature must be test-only and absent from
    the public API and from production artifacts.
11. **Run `clean --dry`.** All output must be cache-local; no fixture may change.
12. **Delete the replaced legacy test** in the same owner change, and lower this owner's count in
    `📇️registry/🔒️migration.json`. Never leave two test hierarchies alive.

## Reaching a reference library — in any language

An oracle runs in whichever implementation its registry `ecosystem` names, and the library it needs
is declared by the OWNER, never by the framework. One field carries this, in the owner's
`🧪️oracle/🔣️.json`:

```json
"oracleHostPackages": [
  { "implementation": "rust",       "package": "semio-s-plugin-stdio-test-oracle", "path": "…/📦️packages/🦀️rust", "features": ["oracles"] },
  { "implementation": "python",     "package": "pypdf",  "version": "6.14.2" },
  { "implementation": "typescript", "package": "semver", "version": "7.8.5" }
]
```

`path` decides how the entry is provisioned, and it is the only thing that does:

* **With a path** the entry is LOCAL in-repo source. The Rust host writes a `Cargo.toml` that
  depends on it by path — a crates.io coordinate is refused, because it would be an unreviewed
  dependency of a generated host.
* **Without a path** the entry is an EXTERNAL distribution. The Python host builds a cache-local
  virtual environment under `.🧬semio/🦑️repo/⚡️cache/tests/hosts/`, keyed by the declared package
  set so it is created once and reused by every run; it is created with `--system-site-packages`, so
  a distribution the machine already provides at the declared version is reused rather than
  downloaded, and anything else is installed INTO the environment. The system interpreter is never
  written to. The TypeScript host resolves the package from the repository's own `node_modules`
  instead of installing a private copy — one lockfile, one version of every library — and reports
  the declaration as unmet if it does not resolve.

An external host package is a third-party test dependency, so it is classified in
`🔒️dependencies.json` as `test-oracle` and held to the same purity rule as a registered oracle
package: it must be registered, it must not be production-reachable, and the production scan looks
for each ecosystem's own import syntax (`import x` / `from x import` for Python, `use x` for Rust,
`from "x"` / `require("x")` for JavaScript) in that ecosystem's own files.

## Rules that are enforced, not advisory

* `compose/**` is excluded in the discovery library, not by a CI path filter. No other area is
  permanently exempt — not legacy, not mixed, not currently broken.
* A missing tool fails setup. A timeout fails the case. There is no quarantine list, no known-broken
  allowlist and no mechanism anywhere that turns a failure into a skip.
* Fixtures are immutable. A scenario that needs to mutate one copies it into the case work directory
  first. `local://` never shadows `shared://`.
* Comparison belongs to an owned profile, never to an adapter.
* Generated hosts and outputs live only under `.🧬semio/🦑️repo/⚡️cache/tests/`, carry an ownership
  marker, and are safe to delete. A committed generated wrapper is a taxonomy breach.
* The legacy backlog is shrink-only. `contract` fails when an area's unmanaged-test count grows.
