# 📎️ Retained Cargo Source

Lets a Cargo compiler owner keep compiling from the live worktree while others edit it, without weakening the artifact's source provenance.

| Member | Role |
| --- | --- |
| `captureAdmittedCargoSourcesV1` | Holds every admitted body in the private witness before a compiler reads it (descriptor and WASM owners). |
| `deferCargoSourceDriftV1` | Defers an edit to a file whose admitted body is held and publishes it once; `false` means stop the compiler. |
| `RetainedCargoSourcesV1.admit` | Captures a whole admitted roster into compiler-store-backed witness storage. |
| `RetainedCargoSourcesV1.defer` | The drift callback of a compiler command. |
| `RetainedCargoSourcesV1.prove` | After the compile, reads rustc dep-info consumed-byte checksums and requires them to equal the admitted bodies. Refuses with a concurrent edit on a contradicting, inconsistent or unproven deferred file. |
| `CargoSourceProvenanceV1` | Roster SHA-256, file and proof counts and every deferred edit; handed to the consumer of the artifact. |

Dep-info written before the attempt belongs to cached units: it may add proof but never contradicts the compile. A file edited while compiling is accepted only when a unit consumed its admitted version; otherwise the next attempt re-admits it.

Law: `./🧪️tests/🟦️.ts` with the neutral cases in `./🧪️tests/🔣️.json`, checked against independent BLAKE3 and SHA-256.
