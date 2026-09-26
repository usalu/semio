#!/usr/bin/env bun
/** 🌎️ `os-hub-ts` (nx `os-hub-ts`) router: `bun ./📜️script.ts <test [quick|long|exhaustive] [args…]|two-client-e2e <sqlite|postgres|neo4j>|document-growth-e2e <sqlite|postgres|neo4j>|typecheck>`.
 * Bun integration-test harness that boots the REAL `os-hub` binary and drives it with two
 * independent clients to prove the hub's collaboration contract end-to-end (ticket
 * 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS, lane 3-E). Gated behind
 * `HUB_E2E=1` (see `🤝️index.test.ts`'s own doc) — the default `test` run never touches cargo and
 * reports the whole e2e suite as skipped in well under a second. */
import { join } from "node:path";
import { setTimeout as sleep } from "node:timers/promises";
import { BundleScript, ScriptRouter, resolveTestLevel, runBunxStatus, runBundleScriptMain, runCargo, runProbe, runVitest, type TestLevel } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { freeLoopbackPort, hubDevPostgresBinaryPath } from "../../🚀️local-bootstrap/🏃️execution/🟦️.ts";

const HUB_RUST_DIR = "🌎️hub/📦️packages/🦀️rust";

/** 🏗️ Builds the real `os-hub` debug binary the test spawns directly (never `cargo run` — no
 * wrapper-process tree to chase on teardown). Default cargo features only (sqlite): contract-freeze
 * Amendment 2 says `--all-features` is red repo-wide and pre-existing (`🛢️db`'s postgres/neo4j
 * features have no wired driver deps), so this never passes it. Only runs when `HUB_E2E=1`, so the
 * default `test` target never pays a cargo build. */
function buildHubBinary(repoRoot: string): void {
  if (process.env.HUB_E2E !== "1") return;
  const preexisting = process.env.OS_HUB_BINARY;
  if (preexisting) {
    console.log(`[os-hub-ts] HUB_E2E=1 — using OS_HUB_BINARY=${preexisting} (skip cargo build)`);
    return;
  }
  console.log("[os-hub-ts] HUB_E2E=1 — building os-hub (default features, no --all-features)…");
  runCargo(["build", "--manifest-path", "Cargo.toml"], join(repoRoot, HUB_RUST_DIR));
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    buildHubBinary(this.repoRoot);
    await runVitest(this.root, rest, "../../🧪️tests/🎚️config/🟦️.ts");
  }
}

/** 🗄️ One storage backend the two-client e2e runs on: the `🌎️hub/compose.yaml` service that serves
 * it, the container port that service listens on, its readiness probe, the server's own client the
 * e2e counts live WAL writers with (the query itself is the fixture's `shutdown.liveWriterProbe`),
 * and the hub environment that points `os-hub` at it. `sqlite` needs no service. */
type TwoClientBackend = { readonly service?: string; readonly containerPort?: number; readonly ready?: readonly string[]; readonly client?: readonly string[]; readonly env: (port: number) => Record<string, string> };

const TWO_CLIENT_BACKENDS: Readonly<Record<string, TwoClientBackend>> = {
  sqlite: { env: () => ({ OS_HUB_STORAGE_BACKEND: "sqlite" }) },
  postgres: {
    service: "postgres",
    containerPort: 5432,
    ready: ["pg_isready", "--username=semio", "--dbname=semio"],
    client: ["psql", "--username=semio", "--dbname=semio", "--tuples-only", "--no-align", "--command"],
    env: (port) => ({ OS_HUB_STORAGE_BACKEND: "postgres", OS_HUB_DATABASE_URL: `postgres://semio:semio@127.0.0.1:${port}/semio` }),
  },
  neo4j: {
    service: "neo4j",
    containerPort: 7687,
    ready: ["cypher-shell", "--username", "neo4j", "--password", "semio-hub", "RETURN 1"],
    client: ["cypher-shell", "--username", "neo4j", "--password", "semio-hub", "--format", "plain"],
    env: (port) => ({ OS_HUB_STORAGE_BACKEND: "neo4j", OS_HUB_NEO4J_URI: `bolt://127.0.0.1:${port}`, OS_HUB_NEO4J_USER: "neo4j", OS_HUB_NEO4J_PASSWORD: "semio-hub" }),
  },
};

/** 🗄️ Runs one real-binary hub e2e scenario (`🌎️hub/🧪️tests/<scenario>`) on one storage
 * backend: `sqlite`, or `postgres`/`neo4j` served by that backend's own `🌎️hub/compose.yaml` service
 * in a throwaway compose project (its own volume, a free loopback port), removed afterwards — also when
 * a failing run leaves through `process.exit`, which skips `finally`. The hub
 * is `OS_HUB_BINARY` when set, else the Nx-staged `build-dev-postgres` executable, which links all
 * three drivers. */
abstract class BackendE2eScript extends BundleScript {
  protected abstract readonly scenario: string;
  protected abstract readonly minimumLevel: TestLevel;

  async run(segments: string[]): Promise<void> {
    const [name, ...extra] = segments;
    const { rest } = resolveTestLevel(extra, this.minimumLevel);
    const backend = TWO_CLIENT_BACKENDS[name ?? ""];
    if (!backend) throw new Error(`${this.scenario} e2e needs one backend: ${Object.keys(TWO_CLIENT_BACKENDS).join(" | ")}`);
    const binary = process.env.OS_HUB_BINARY ?? hubDevPostgresBinaryPath(join(this.repoRoot, HUB_RUST_DIR));
    const compose = ["compose", "--file", join(this.repoRoot, "🌎️hub", "compose.yaml"), "--project-name", `semio-hub-e2e-${name}-${process.pid}`];
    const container = `semio-hub-e2e-${name}-${process.pid}`;
    const port = await freeLoopbackPort();
    let removed = !backend.service;
    const teardown = () => {
      if (removed) return;
      removed = true;
      runProbe("docker", ["rm", "--force", container], { cwd: this.repoRoot });
      runProbe("docker", [...compose, "--profile", backend.service!, "down", "--volumes", "--remove-orphans"], { cwd: this.repoRoot });
    };
    process.once("exit", teardown);
    try {
      if (backend.service) {
        const started = runProbe("docker", [...compose, "--profile", backend.service, "run", "--detach", "--rm", "--name", container, "--publish", `127.0.0.1:${port}:${backend.containerPort}`, backend.service], { cwd: this.repoRoot });
        if (started.status !== 0) throw new Error(`${this.scenario} e2e could not start the ${backend.service} compose service: ${started.stderr}`);
        const deadline = Date.now() + 180_000;
        while (runProbe("docker", ["exec", container, ...backend.ready!], { cwd: this.repoRoot }).status !== 0) {
          if (Date.now() > deadline) throw new Error(`${this.scenario} e2e: ${backend.service} not ready within 180 s`);
          await sleep(1000);
        }
      }
      Object.assign(process.env, backend.env(port), { HUB_E2E: "1", OS_HUB_BINARY: binary, HUB_TWO_CLIENT_PORT: process.env.HUB_TWO_CLIENT_PORT ?? String(await freeLoopbackPort()) });
      if (backend.client) process.env.HUB_E2E_WRITER_PROBE = JSON.stringify(["docker", "exec", container, ...backend.client]);
      console.log(`[os-hub-ts] ${this.scenario} e2e backend=${name} binary=${binary} hub-port=${process.env.HUB_TWO_CLIENT_PORT}${backend.service ? ` ${backend.service}=127.0.0.1:${port}` : ""}`);
      await runVitest(this.root, [this.scenario, ...rest], "../../🧪️tests/🎚️config/🟦️.ts");
    } finally {
      teardown();
    }
  }
}

/** 🤝️ The two-client document collaboration e2e (`🌎️hub/🧪️tests/🤝️two-client-document`). */
class TwoClientE2eScript extends BackendE2eScript {
  protected readonly scenario = "two-client-document";
  protected readonly minimumLevel = "long";
}

/** 📈️ The concurrent document growth e2e (`🌎️hub/🧪️tests/📈️document-growth`). */
class DocumentGrowthE2eScript extends BackendE2eScript {
  protected readonly scenario = "document-growth";
  protected readonly minimumLevel = "exhaustive";
}

/** 🪁️ Type-checks every `🌎️hub/**` TypeScript source against the hub-scoped `tsconfig.json`. */
class TypecheckScript extends BundleScript {
  run(segments: string[]): void {
    const status = runBunxStatus(["tsc", "--noEmit", "-p", "tsconfig.json", ...segments], this.root);
    if (status !== 0) process.exit(status);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("two-client-e2e", TwoClientE2eScript).register("document-growth-e2e", DocumentGrowthE2eScript).register("typecheck", TypecheckScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
