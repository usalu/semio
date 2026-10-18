/** 🧭️ Thin hub gate commands: source-gate runners, setup, the cargo test lanes and the hostile-input and CAS checks.
 * @see ../../📦️packages/🦀️rust/📜️script.ts
 */
import { runLiveSignInV1 } from "../../🔐️auth/🧪️tests/🤝️live-sign-in/🟦️.ts";
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/process/compiler-capacity";
import { scriptCompilerAdmissionControlV1 } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { configuredExactCargoLawPolicyV1 } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import { resolveTestLevel } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { buildBudgetMs } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { mkdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { spawnSync } from "node:child_process";
import Ajv from "ajv";
import { runCargo } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🏃️run/🟦️.ts";
import { runRepositoryCargoTests } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts";
import { runRepositoryExactCargoLaws, configuredRepositoryCargoPreparationPolicyV1 } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎯️exact/🟦️.ts";
import { BundleScript } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { exactCargoStageEnvironments } from "../../🏗️build/🛂staging-environment/🟦️.ts";
import { runRepositoryCommand } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";
import { HUB_TEST_PACKAGES } from "../../🧪️testing/📖️source-readers/🟦️.ts";

//#region 🧱️SourceGateRunners
/** 🕸️ Supplies Hub's actual actor publication capability to the neutral runtime graph verifier. */
export class RuntimeFixtureGraphScript extends BundleScript {
  async run(segments:string[]):Promise<void>{
    const {verifyRuntimeFixtureGraphV1}=await import("../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🔎️verification/🟦️.ts");
    const {hubRuntimeActorPublicationPortV1}=await import("../../🏗️bootstrap/🎭️actors/🟦️.ts");
    await verifyRuntimeFixtureGraphV1(this.repoRoot,segments,hubRuntimeActorPublicationPortV1(this.repoRoot),["🧰️framework","✏️s","🌎️hub"],this.invocation);
  }
}

/** 🧱️ Runs the bounded Hub foundation source contract through the shared owned-process boundary. */
export class HubDocumentSweepScript extends BundleScript {
 async run(segments:string[]):Promise<void>{const {runHubDocumentSweepCli}=await import("../../🧪️tests/🗂️document-sweep/🟦️.ts");const {withIssuedResourceControlV1}=await import("../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");await withIssuedResourceControlV1(this.invocation,"hub:document-sweep",0,phase=>runHubDocumentSweepCli(this.repoRoot,join(this.repoRoot,"🌎️hub/🤖️generated/🗂️document-sweep"),segments,this.invocation,phase));}
}
export class HubIoMatrixScript extends BundleScript {
 async run(segments:string[]):Promise<void>{const {runIoMatrixCli}=await import("../../🧪️tests/🚪️io-matrix/🟦️.ts");await runIoMatrixCli(this.invocation,scriptCompilerAdmissionControlV1(this.invocation,"io-matrix",0),this.repoRoot,join(this.repoRoot,"🌎️hub/🤖️generated/🚪️io-matrix"),segments);}
}

export class HubFoundationSourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("foundation-source-check accepts no arguments");
    await runRepositoryCommand(this.invocation,"bun",["test", join(this.repoRoot, "🌎️hub/🧪️tests/🧱️foundation-source/🟦️.ts")],this.repoRoot,"hub-foundation-source",{version:1,maximumElapsedMilliseconds:120_000,output:{maximumBytes:134217728,maximumLines:65536}});
  }
}

/** 🌳️ Runs the hub prebuild unit census: every crate of the planned component closure compiles once per profile
 * (https://doc.rust-lang.org/cargo/reference/unstable.html#unit-graph). */
export class HubUnitCensusScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("unit-census-check accepts no arguments");
    await runRepositoryCommand(this.invocation,"bun",["test", join(this.repoRoot, "🌎️hub/🧪️tests/🌳️unit-census/🟦️.ts")],this.repoRoot,"hub-unit-census",{version:1,maximumElapsedMilliseconds:1_800_000,output:{maximumBytes:134217728,maximumLines:65536}});
  }
}

/** 🧱️ Runs the bounded socket-grant command ownership contract. */
export class HubSocketGrantCommandSourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("socket-grant-command-source-check accepts no arguments");
    await runRepositoryCommand(this.invocation,"bun",["test", join(this.repoRoot, "🌎️hub/🧪️tests/🧱️socket-grant-command-source/🟦️.ts")],this.repoRoot,"hub-socket-grant-command-source",{version:1,maximumElapsedMilliseconds:120_000,output:{maximumBytes:134217728,maximumLines:65536}});
  }
}

/** 🤝️ Runs the live sign-in gate: a real `os-hub` child against a fresh data root, the operator bootstrap verb, and the whole
 * two-principal session lifecycle over HTTP. Credential sign-in is enabled for the child here because the gate exists to
 * exercise exactly that policy — it is not a default any other route inherits. */
export class HubLiveSignInScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("live-sign-in-check accepts no arguments");
    await runLiveSignInV1(this.invocation,exactCargoCallerControlV1(this.invocation,"hub:live-sign-in",300000),false);
  }
}
//#endregion 🧱️SourceGateRunners

export class SetupScript extends BundleScript {
  run(): void {
    runCargo(["fetch", "--manifest-path", "Cargo.toml"], this.root);
  }
}

/** 🗂️ The package-owned artifact root the descriptor-rooted hub laws demand when no ticket supplies one:
 * `fixture_root()` in the trusted-catalog, publication and browser-actor suites reads
 * `SEMIO_TEST_ARTIFACT_DIR` and panics without it, so a plain `bun repo run os-hub:test` would report panics
 * instead of assertions. */
function hubTestArtifactRoot(root: string): string {
  const path = join(root, "../../🧪️tests/🗂️artifact-root/📦️packages/🦀️rust/🗑️generated/test-artifacts");
  mkdirSync(path, { recursive: true, mode: 0o700 });
  return path;
}

/** 🧱️ Checks Hub's projections and GIS witnesses against their general client contracts. */
export class InferenceClientContractTestScript extends BundleScript {
  async run(): Promise<void> {
    
    
    
    
    
      const receipts = await runRepositoryExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),preparation: configuredRepositoryCargoPreparationPolicyV1(),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: buildBudgetMs(), listMilliseconds: 60_000, lawMilliseconds: 120_000 },cwd: this.repoRoot,...exactCargoStageEnvironments(),progress: event => console.log(`inference-client-contract ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`),groups: [{ package: "semio-hub", target: { kind: "test", name: "inference_client_contract" }, laws: [
          "the_published_error_vocabulary_is_exactly_the_neutral_fixtures_and_status_alone_is_ambiguous",
          "the_client_mirrors_the_neutral_fixtures_exact_fixed_limits",
          "an_offered_page_carries_the_corpus_preview_and_a_forged_or_open_ring_is_refused",
          "the_neutral_lifecycles_decode_into_the_closed_event_page_in_order",
          "a_durable_approval_undo_posts_only_the_hub_target_frontier_and_retry_identity",
          "the_hub_approval_request_consumes_the_framework_contract",
        ] }]});
      console.log(`inference-client-contract: ${receipts.reduce((count, receipt) => count + receipt.assertions, 0)} exact native laws passed`);
    
  }
}

/** 🧩️ Checks actual composed component codecs against the caller's schema and budgets. */
export class ComponentCodecCheckScript extends BundleScript {
  async run(segments:string[]):Promise<void> {
    const fixture=JSON.parse(readFileSync(join(this.repoRoot,"🌎️hub/🧪️tests/🧩️component-codecs/🧫️fixtures/🔣️.json"),"utf8"));
    const schema=JSON.parse(readFileSync(join(this.repoRoot,"🌎️hub/🧬️schema/🧩️component-codecs/🔣️.json"),"utf8"));
    const oracle=new Ajv({strict:true,allErrors:true}).compile(schema);
    if(!oracle(fixture)) throw new Error(JSON.stringify(oracle.errors));
    resolveTestLevel([],segments[0]==="budget"?"exhaustive":"quick");
    const filter=segments[0]==="budget"?"actual_gis_owned_codec_honors":"actual_component_codec_exports";
    await runRepositoryCargoTests(["semio-hub-component-codec-laws"], this.repoRoot, this.invocation, ["--lib","--features","integration-fixtures",filter,"--","--nocapture"]);
  }
}

export class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    const [head, ...tail] = rest;
    const allFeatures = head === "all-features";
    process.env.SEMIO_TEST_ARTIFACT_DIR ??= hubTestArtifactRoot(this.root);
    await runRepositoryCargoTests([...HUB_TEST_PACKAGES], this.repoRoot, this.invocation, allFeatures ? ["--all-features", ...tail] : rest);
  }
}

/** 🐘️ The directory backends' LIVE lanes: the `postgres` and `neo4j` laws of `directory::postgres` / `directory::neo4j`
 * (incl. the backend-neutral share-scope and space-administration corpora and the DB3 document-storage law) against the ONE
 * shared development server claimed by `os-hub-ts backend run <postgres|neo4j> -- …` — each postgres law in a database
 * of its own on the claim, the neo4j laws one at a time over the claim's emptied graph (`db_storage::claimed_backend`).
 * They are `#[ignore]`d outside a claim and never start a server of their own (preamble rule 22).
 * `directory-live-lanes [postgres|neo4j]`: without a lane, every lane whose claim environment is present. */
const DIRECTORY_LIVE_LANES: Readonly<Record<string, Readonly<{ filters: ReadonlyArray<readonly [string, string]>; claimed: string; threads: number }>>> = {
  postgres: { filters: [["semio-hub-directory-backend-laws", "postgres_tests"], ["semio-hub-directory-laws", "postgres_creation_tests"]], claimed: "OS_HUB_DATABASE_URL", threads: 2 },
  neo4j: { filters: [["semio-hub-directory-backend-laws", "neo4j_tests"], ["semio-hub-directory-laws", "neo4j_creation_tests"]], claimed: "OS_HUB_NEO4J_URI", threads: 1 },
};

export class DirectoryLiveLanesScript extends BundleScript {
  run(segments: string[]): void {
    const lanes = segments.length ? segments : Object.keys(DIRECTORY_LIVE_LANES).filter((name) => process.env[DIRECTORY_LIVE_LANES[name]!.claimed]);
    const unknown = lanes.filter((name) => !(name in DIRECTORY_LIVE_LANES));
    if (unknown.length || lanes.length === 0) throw new Error(`directory-live-lanes accepts ${Object.keys(DIRECTORY_LIVE_LANES).join(" | ")} and runs only under a claim: \`os-hub-ts backend run <postgres|neo4j> -- …\`${unknown.length ? `, got ${unknown.join(",")}` : ""}`);
    const unclaimed = lanes.filter((name) => !process.env[DIRECTORY_LIVE_LANES[name]!.claimed]);
    if (unclaimed.length) throw new Error(`directory-live-lanes ${unclaimed.join(",")} needs the claimed shared server: run it under \`os-hub-ts backend run ${unclaimed[0]} -- …\``);
    for (const lane of lanes) {
      const { filters, threads } = DIRECTORY_LIVE_LANES[lane]!;
      for (const [owner, filter] of filters) runCargo(["test", "--manifest-path", "Cargo.toml", "-p", owner, "--lib", filter, "--", "--include-ignored", `--test-threads=${threads}`], this.root);
    }
  }
}

/** 🏳️ The value following `flag` in a verb's segments, or `undefined` when absent or followed by another flag. */
function flagValue(segments: readonly string[], flag: string): string | undefined {
  const index = segments.indexOf(flag);
  const value = index >= 0 ? segments[index + 1] : undefined;
  return value === undefined || value.startsWith("--") ? undefined : value;
}

/** 🎲️ The hostile-input laws the check runs: the language-neutral draw vectors, the fixture's coverage of every registered
 * route, the enumerated typed refusals, and the generative law (drawn mutations of every route's own request and drawn
 * frame sequences on both sockets, shrunk findings) — the last once per seed. */
const HOSTILE_INPUT_LAWS = {
  draws: ["semio-hub-hostile-input-laws", "hostile_generative::long::hostile_draws_reproduce_the_language_neutral_vectors"],
  coverage: ["semio-hub-boot-laws", "the_hostile_input_fixture_covers_every_registered_route"],
  refusals: ["semio-hub-boot-laws", "every_route_answers_hostile_input_with_a_typed_signed_refusal"],
  generative: ["semio-hub-hostile-input-laws", "hostile_generative::long::every_route_and_socket_survives_generated_hostile_input"],
} as const;

/** 🎲️ `hostile-input-check [--seeds <n,…> | --seed-range <from>..<to>] [--locale en|de]` — the hub's hostile-input
 * gate as one record: the fixture's language-neutral oracle (`os-hub-ts` `🚧️hostile-input`, Ajv), the enumerated laws, and
 * the generative law once per seed (`SEMIO_HUB_HOSTILE_SEED`; default the fixture's own seeds, a range turns it into a
 * longer fuzz run). Each seed's requests, socket sequences and findings are measured; any finding fails the record with
 * its seed. Ctrl-C stops the running law. */
export class HostileInputCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const locale = flagValue(segments, "--locale") === "de" ? "de" : "en";
    const fixture = JSON.parse(readFileSync(join(this.repoRoot, "🌎️hub/🧪️tests/🧫️fixtures/🚧️hostile-input-v1/🔣️.json"), "utf8"));
    const range = flagValue(segments, "--seed-range")?.match(/^(\d+)\.\.(\d+)$/u);
    const seeds: number[] = range ? Array.from({ length: Math.max(0, Number(range[2]) - Number(range[1]) + 1) }, (_, index) => Number(range[1]) + index) : (flagValue(segments, "--seeds")?.split(",").map(Number) ?? (fixture.generative.seeds as number[]));
    if (seeds.length === 0 || seeds.some((seed) => !Number.isSafeInteger(seed) || seed < 0)) throw new Error("hostile-input-check needs non-negative integer seeds: --seeds <n,…> | --seed-range <from>..<to>");
    const { acceptanceCheckResult, publishAcceptanceCheckResult, runLawProcess, withAcceptanceRecord } = await import("@semio-tech/repo-test/acceptance/orchestration");
    const startedAt = new Date();
    let cancelled = false;
    const interrupt = (): void => {
      cancelled = true;
    };
    process.once("SIGINT", interrupt);
    await withAcceptanceRecord(this.repoRoot, "hub-hostile-input", async () => {
      const typescriptRoot = join(this.repoRoot, "🌎️hub/📦️packages/🟦️typescript");
      const oracle = spawnSync("bunx", ["vitest", "run", "--config", join(this.repoRoot, "🌎️hub/🧪️tests/🎚️config/🟦️.ts"), join(this.repoRoot, "🌎️hub/🧪️tests/🚧️hostile-input/🟦️.ts")], { cwd: typescriptRoot, stdio: "inherit" });
      const law = ([owner, name]: readonly [string, string], seed?: number): Promise<{ status: number; lines: string[] }> =>
        runLawProcess("cargo", ["test", "--manifest-path", "Cargo.toml", "-p", owner, "--lib", "--no-fail-fast", "--", "--exact", `tests::${name}`, "--nocapture", "--test-threads=1"], { cwd: this.root, env: { ...process.env, RUST_MIN_STACK: "268435456", ...(seed === undefined ? {} : { SEMIO_HUB_HOSTILE_SEED: String(seed) }) } });
      const passes = (run: { status: number; lines: string[] }): boolean => run.status === 0 && run.lines.some((line) => line.includes("test result: ok. 1 passed"));
      const enumerated: Record<string, boolean> = {};
      for (const name of ["draws", "coverage", "refusals"] as const) {
        if (cancelled) break;
        enumerated[name] = passes(await law(HOSTILE_INPUT_LAWS[name]));
      }
      let requests = 0;
      let sequences = 0;
      const findings: string[] = [];
      const failedSeeds: number[] = [];
      for (const seed of seeds) {
        if (cancelled) break;
        const run = await law(HOSTILE_INPUT_LAWS.generative, seed);
        const counted = run.lines.map((line) => /hostile-generative: (\d+) requests, (\d+) socket sequences, (\d+) findings/u.exec(line)).find((match) => match !== null);
        requests += Number(counted?.[1] ?? 0);
        sequences += Number(counted?.[2] ?? 0);
        if (!passes(run)) {
          failedSeeds.push(seed);
          findings.push(...run.lines.filter((line) => line.startsWith("seed ")).slice(0, 8));
        }
      }
      const enumeratedPassed = Object.values(enumerated).filter(Boolean).length;
      const status = cancelled ? "skipped" : oracle.status === 0 && enumeratedPassed === 3 && failedSeeds.length === 0 ? "pass" : "fail";
      const summary = {
        en: `oracle ${oracle.status === 0 ? "passes" : "fails"}; enumerated laws ${enumeratedPassed}/3; generative ${seeds.length - failedSeeds.length}/${seeds.length} seeds clean over ${requests} requests and ${sequences} socket sequences${failedSeeds.length ? `; findings at seeds ${failedSeeds.join(", ")}` : ""}`,
        de: `Orakel ${oracle.status === 0 ? "bestanden" : "fehlgeschlagen"}; aufgezählte Gesetze ${enumeratedPassed}/3; generativ ${seeds.length - failedSeeds.length}/${seeds.length} Startwerte ohne Befund über ${requests} Anfragen und ${sequences} Socket-Folgen${failedSeeds.length ? `; Befunde bei Startwerten ${failedSeeds.join(", ")}` : ""}`,
      };
      console.log(`[hostile-input] ${summary[locale]}`);
      for (const finding of findings) console.log(`[hostile-input] ${finding}`);
      publishAcceptanceCheckResult(
        this.repoRoot,
        acceptanceCheckResult({
          check: "hub-hostile-input",
          status,
          startedAt,
          measured: { seeds: seeds.join(","), oracle: oracle.status === 0, draws: enumerated.draws ?? false, coverage: enumerated.coverage ?? false, refusals: enumerated.refusals ?? false, requests, socketSequences: sequences, failedSeeds: failedSeeds.join(",") },
          summary,
        }),
      );
      if (status !== "pass") process.exitCode = 1;
    });
    process.removeListener("SIGINT", interrupt);
  }
}

export class ArtifactCasCheckScript extends BundleScript {
  run(segments: string[]): void {
    runCargo(["test", "--manifest-path", "Cargo.toml", "--all-features", "-p", "semio-hub-artifact-authority", "-p", "semio-hub-directory-laws", "--lib", "artifact_chunk_cas", ...segments], this.root, { ...process.env, RUST_MIN_STACK: "16777216" });
    runCargo(["test", "--manifest-path", "Cargo.toml", "--all-features", "-p", "semio-hub-hostile-input-laws", "--lib", "artifact_cas_maintenance", ...segments], this.root, { ...process.env, RUST_MIN_STACK: "16777216" });
    runCargo(["check", "--manifest-path", "Cargo.toml", "--all-features", "--bin", "os-hub"], this.root);
  }
}

/** 🤝️ Executes the hub-owned two-browser collaboration acceptance suite. */
export class DevCollaborationScript extends BundleScript {
  async run(segments: string[]): Promise<void> { await (await import("../../🧪️tests/🤝️dev-collaboration/🟦️.ts")).runCollabE2eCli(this.invocation,scriptCompilerAdmissionControlV1(this.invocation,"collaboration",0),segments); }
}
