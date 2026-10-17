/** 🧭️ Build, build-dev, publish and dev commands and the local bootstrap launch gate.
 * @see ../../📦️packages/🦀️rust/📜️script.ts
 */
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/process/compiler-capacity";
import { withRepositoryDevelopmentStateV1 } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/💾️development/🟦️.ts";
import { repositoryCargoNetworkV1 } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts";
import { verifyCapturedCargoArtifactsV1 } from "@semio-tech/process/native-build";
import { scriptCompilerAdmissionControlV1 } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { buildBudgetMs } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { terminateOwnedChildTree } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { parseLocalSessionBrokerRecordV1, parseLocalSessionRequestV1, parseLocalSessionV1 } from "@semio-tech/framework-os/modules/directory/local-session/broker/schema";
import { randomBytes } from "node:crypto";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { spawn, type ChildProcess } from "node:child_process";
import { buildRepositoryCargoArtifacts, workspaceCargoVersion } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts";
import { packageNativeRelease, signExecutableForDistribution } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🟦️.ts";
import { OS_HUB_PORT, OS_HUB_PORT_ENV, runCmd, orchestratorBudgetOpts } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { hubSchemaExport } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { deliverNativeCredentialEnvelope } from "../../🔐️auth/📤️credential-delivery/🟦️.ts";
import { proveMcpCredentialSourceOrder, proveNativeCredentialSourceOrder } from "../../🔐️auth/🧪️tests/🧭️credential-source-order/🟦️.ts";
import { issueLocalCredential, parseLocalAdminCapabilityV1, withLocalSessionBrokerV1 } from "../../🚀️local-bootstrap/🔐️credential-issuance/🟦️.ts";
import { type LocalProfile } from "../../🚀️local-bootstrap/🛂authentication/🟦️.ts";
import { ensureHubBackend, finishLocalHub, HUB_BINARY_SOURCES_FILE, HUB_DEV_BINARY_TARGET, hubBackendIdentity, hubBackendName, type HubBackendName, hubDevBinaryPath, LOCAL_HUB_ADMINISTRATOR_PROFILE, LOCAL_HUB_ADMINISTRATOR_SUBJECT, LOCAL_HUB_DEVELOPMENT_CATALOG_PACKAGES, LOCAL_HUB_DEVELOPMENT_PROFILES, stopLocalHubV1, waitForLocalHubExitV1, withPublishedDevelopmentLocalHubHostV1, startLocalHub, waitForReadiness } from "../../🚀️local-bootstrap/🏃️execution/🟦️.ts";
import { trustedBootstrapBuildControl, UNLIMITED_TRUSTED_BUILD_DEADLINE_MS } from "../../🏗️bootstrap/🧾️provenance/🗃️bootstrap-bundle/🟦️.ts";
import { validateAndPublishTrustedStdioGisCandidate } from "../../🏗️bootstrap/🧾️provenance/🚚️bootstrap-publication/🟦️.ts";
import { withMaterializedTrustedCatalogBundleV1 } from "../../🏗️bootstrap/🧾️provenance/🚚️bootstrap-publication/🧱️materialized-bundle/🟦️.ts";
import { trustedBootstrapCurrent } from "../../🏗️bootstrap/🧾️provenance/🚚️bootstrap-publication/🔄️rotation-and-current-pointer/🟦️.ts";
import { trustedBootstrapSelectPackages } from "../../🏗️bootstrap/🧾️provenance/🧭️bootstrap-selection/🟦️.ts";
import { buildAdminSpa, nativeWgpuExecutable, openExternalBrowser, startMcpWorkspaceChild, waitForUiReadiness } from "../../🚀️local-bootstrap/🧰️mcp-process-support/🟦️.ts";
import { type LocalAdminRelay, type LocalBrowserRelay, startLocalAdminRelay, startLocalBrowserRelay } from "../../🚀️local-relay/🛰️servers/🟦️.ts";

/** ⏳️ Proves the two launch-route boundaries a fresh `OS_HUB_DATA` boot depends on: an unset
 * `SEMIO_BUILD_BUDGET_MS` (the repository's unlimited budget, `0`) never presents as an already expired
 * deadline, and a missing staged executable is staged through its own Nx target before it is reported. */
export class LocalBootstrapLaunchCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("local-bootstrap-launch-check accepts no arguments");
    const assert: (typeof import("node:assert"))["strict"] = (await import("node:assert/strict")).default;
    const { tmpdir } = await import("node:os");
    const configured = process.env.SEMIO_BUILD_BUDGET_MS;
    delete process.env.SEMIO_BUILD_BUDGET_MS;
    try {
      assert.equal(buildBudgetMs(), 0);
      const unlimited = trustedBootstrapBuildControl(buildBudgetMs());
      try {
        assert.equal(unlimited.control.cancelled(), false);
        const remaining = unlimited.control.remainingMs();
        assert(remaining > 0 && Number.isSafeInteger(remaining) && remaining <= UNLIMITED_TRUSTED_BUILD_DEADLINE_MS);
        assert(remaining > UNLIMITED_TRUSTED_BUILD_DEADLINE_MS - 60_000);
      } finally {
        unlimited.close();
      }
      const opted = trustedBootstrapBuildControl(5_000);
      try {
        assert(opted.control.remainingMs() > 0 && opted.control.remainingMs() <= 5_000);
      } finally {
        opted.close();
      }
      const expired = trustedBootstrapBuildControl(-1);
      try {
        assert(expired.control.remainingMs() > 0);
      } finally {
        expired.close();
      }
    } finally {
      if (configured === undefined) delete process.env.SEMIO_BUILD_BUDGET_MS;
      else process.env.SEMIO_BUILD_BUDGET_MS = configured;
    }
    const stagingRoot = mkdtempSync(join(tmpdir(), "hub-dev-binary-"));
    try {
      const staged = join(stagingRoot, "dist", "build-dev", process.platform === "win32" ? "os-hub.exe" : "os-hub");
      let attempts = 0;
      assert.throws(
        () => hubDevBinaryPath(stagingRoot, { stage: () => (attempts += 1) && 1 }),
        (error: Error) => error.message.includes(HUB_DEV_BINARY_TARGET) && error.message.includes(staged) && error.message.includes("status 1"),
      );
      assert.equal(attempts, 1);
      assert.throws(
        () => hubDevBinaryPath(stagingRoot, { stage: () => (attempts += 1) && 0 }),
        (error: Error) => error.message.includes(HUB_DEV_BINARY_TARGET),
      );
      assert.equal(attempts, 2);
      assert.equal(
        hubDevBinaryPath(stagingRoot, {
          stage: () => {
            attempts += 1;
            mkdirSync(dirname(staged), { recursive: true });
            writeFileSync(staged, "staged");
            return 0;
          },
        }),
        staged,
      );
      assert.equal(attempts, 3);
      assert.equal(hubDevBinaryPath(stagingRoot, { stage: () => (attempts += 1) && 0 }), staged);
      assert.equal(attempts, 4);
      assert.throws(
        () => hubDevBinaryPath(stagingRoot, { stage: () => (attempts += 1) && 1 }),
        (error: Error) => error.message.includes(HUB_DEV_BINARY_TARGET) && error.message.includes("status 1"),
      );
      assert.equal(attempts, 5);
    } finally {
      rmSync(stagingRoot, { recursive: true, force: true });
    }
    const brokerCases = JSON.parse(readFileSync(join(this.repoRoot, "🌎️hub/🚀️local-bootstrap/🧪️tests/🧫️fixtures/🎫️session-broker-v1/🔣️.json"), "utf8")) as { readonly cases: readonly { readonly id: string; readonly def: "LocalSessionBrokerRecordV1" | "LocalSessionRequestV1" | "LocalSessionV1" | "LocalAdminCapabilityV1"; readonly value: unknown; readonly valid: boolean }[] };
    const brokerParsers = { LocalSessionBrokerRecordV1: parseLocalSessionBrokerRecordV1, LocalSessionRequestV1: parseLocalSessionRequestV1, LocalSessionV1: parseLocalSessionV1, LocalAdminCapabilityV1: parseLocalAdminCapabilityV1 } as const;
    for (const row of brokerCases.cases) {
      const ajv = hubSchemaExport(this.repoRoot, `schema://hub.local-bootstrap/${row.def}`)(row.value) === true;
      let parsed = true;
      try {
        brokerParsers[row.def](row.value);
      } catch {
        parsed = false;
      }
      assert.equal(ajv, row.valid, `session broker case ${row.id}: Ajv disagrees with the fixture`);
      assert.equal(parsed, row.valid, `session broker case ${row.id}: TypeScript parser disagrees with the fixture`);
    }
    assert(brokerCases.cases.some((row) => row.valid) && brokerCases.cases.some((row) => !row.valid));
    console.log(`local-bootstrap-launch: unlimited-budget=1 opted-budget=1 negative-budget=1 staging-cases=5 session-broker-cases=${brokerCases.cases.length} (ajv+parser agree) passed`);
  }
}

/** 📦️ Stages the release-profile `os-hub` binary as the project's `dist/build` deliverable. The hub owns
 * this the same way it owns `build-dev`, so `📋️project.json` names only this package's `📜️script.ts`. */
export class BuildScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Select build through Nx without additional arguments");
    await buildRepositoryCargoArtifacts(join(this.root, "Cargo.toml"), ["--release", "--bin", "os-hub"], this.repoRoot, {producer:"artifact",invocation:this.invocation,consume:verifyCapturedCargoArtifactsV1,network:repositoryCargoNetworkV1(this.invocation,this.repoRoot), control: scriptCompilerAdmissionControlV1(this.invocation, "cargo:build", 0),  output: "dist/build", sourcesRecord: HUB_BINARY_SOURCES_FILE });
    signExecutableForDistribution(join(this.root, "dist", "build", process.platform === "win32" ? "os-hub.exe" : "os-hub"));
  }
}

/** 🚚️ Packages `build`'s staged release binary as a versioned, checksummed tarball under
 * `dist/publish` — the missing link between "a release binary compiles" and "an operator can be
 * handed one". Local artifact only: nothing is uploaded or pushed anywhere, and where the tarball
 * goes next is a deployment decision this repository does not take. Reached through the root
 * `bun ./📜️script.ts publish os-hub`. */
export class PublishScript extends BundleScript {
  run(args: string[]): void {
    if (args.length) throw new Error("Select publish through Nx without additional arguments");
    packageNativeRelease({
      binary: join(this.root, "dist", "build", process.platform === "win32" ? "os-hub.exe" : "os-hub"),
      name: "os-hub",
      version: workspaceCargoVersion(this.repoRoot),
      output: join(this.root, "dist", "publish"),
    });
  }
}

/** 🏗️ Stages the dev-profile `os-hub` binary as a cached Nx package deliverable, mirroring
 * `framework-renderer-wgpu:native-build`'s `dist/native-<profile>` pattern, so `DevScript` execs a
 * built artifact instead of running `cargo build` inline on every launch. */
export class BuildDevScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Select build-dev through Nx without additional arguments");
    await buildRepositoryCargoArtifacts(join(this.root, "Cargo.toml"), ["--bin", "os-hub"], this.repoRoot, {producer:"artifact",invocation:this.invocation,consume:verifyCapturedCargoArtifactsV1,network:repositoryCargoNetworkV1(this.invocation,this.repoRoot), control: scriptCompilerAdmissionControlV1(this.invocation, "cargo:build", 0),  output: "dist/build-dev", sourcesRecord: HUB_BINARY_SOURCES_FILE });
  }
}

/** 🐘️ Points BOTH durable halves of a `dev postgres` / `dev neo4j` launch at that backend.
 *
 * Zero-touch: without a caller-named server (`OS_HUB_DATABASE_URL` for postgres, `OS_HUB_NEO4J_URI` for neo4j) the
 * launch reuses — or starts — the persistent development server `os-hub-ts:backend-up` manages
 * (`ensureHubBackend`, `🌎️hub/compose.yaml`), and takes every hub variable it declares. A caller-named server keeps
 * the caller's variables; only the directory half is filled from the storage half when unset.
 *
 * 🧵️ Both halves reach `sqlx`/`neo4rs` through `db_storage::DbIoAsyncDriverRuntime`, the bounded runtime the
 * storage layer owns: a driver future is never polled on a `WorkerPool` worker, which has no Tokio context and
 * aborted the process before that seam existed (ticket 26/09/18 D4). */
async function applyExternalBackendEnvironment(repoRoot: string, name: HubBackendName): Promise<void> {
  const named = name === "postgres" ? process.env.OS_HUB_DATABASE_URL : process.env.OS_HUB_NEO4J_URI;
  if (!named) {
    const server = await ensureHubBackend(repoRoot, hubBackendIdentity(name), { onProgress: (progress) => console.log(`[INFO] ${progress.name} backend ${progress.phase} ${progress.elapsedSeconds} s`) });
    for (const [key, value] of Object.entries(server.env)) process.env[key] ??= value;
    console.log(`[INFO] dev ${name}: backend server ${server.host}:${server.port} (container ${server.identity.container}; remove with os-hub-ts:backend-down)`);
    return;
  }
  process.env.OS_HUB_STORAGE_BACKEND = name;
  process.env.OS_HUB_DIRECTORY_BACKEND ??= name;
  if (process.env.OS_HUB_DIRECTORY_BACKEND === "postgres") process.env.OS_HUB_DIRECTORY_DATABASE_URL ??= process.env.OS_HUB_DATABASE_URL;
  if (process.env.OS_HUB_DIRECTORY_BACKEND === "neo4j") {
    process.env.OS_HUB_DIRECTORY_NEO4J_URI ??= process.env.OS_HUB_NEO4J_URI;
    process.env.OS_HUB_DIRECTORY_NEO4J_USER ??= process.env.OS_HUB_NEO4J_USER;
    process.env.OS_HUB_DIRECTORY_NEO4J_PASSWORD ??= process.env.OS_HUB_NEO4J_PASSWORD;
  }
  if (!process.env.OS_HUB_DIRECTORY_DATABASE_URL && process.env.OS_HUB_DIRECTORY_BACKEND === "postgres") throw new Error("OS_HUB_DIRECTORY_BACKEND=postgres needs OS_HUB_DIRECTORY_DATABASE_URL");
  if (!process.env.OS_HUB_DIRECTORY_NEO4J_URI && process.env.OS_HUB_DIRECTORY_BACKEND === "neo4j") throw new Error("OS_HUB_DIRECTORY_BACKEND=neo4j needs OS_HUB_DIRECTORY_NEO4J_URI (for example bolt://127.0.0.1:7687)");
}

/** 🔗️ `runCargo`'s `env` arg replaces `process.env` wholesale (see `runCmdInternal`'s
 * `opts.env ?? process.env`), so this inherits the full process env and only defaults the port —
 * otherwise the launcher's `OS_HUB_PORT`/`OS_HUB_DATA` (and `PATH`) would be silently dropped. */
export class DevScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    buildAdminSpa(this.repoRoot);
    const external = segments[0] === "postgres" || segments[0] === "neo4j" ? hubBackendName(segments[0]) : undefined;
    const phaseControl=exactCargoCallerControlV1(this.invocation,"hub:development",0);
    if (external) await applyExternalBackendEnvironment(this.repoRoot, external);
    const secureSuite = segments[0] === "secure-suite";
    const secureNative = secureSuite || segments[0] === "secure-native";
    const secureMcp = secureSuite || segments[0] === "secure-mcp";
    const secureAdmin = secureSuite || segments[0] === "secure-admin";
    if (secureNative) {
      proveNativeCredentialSourceOrder(this.repoRoot);
      runCmd("bun", ["repo", "run", "@semio-tech/framework-renderer-wgpu:native-build", "--", process.env.SEMIO_PLUGIN ?? "s"], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
    }
    if (secureMcp) {
      proveMcpCredentialSourceOrder(this.repoRoot);
      runCmd("bun", ["repo", "run", "@semio-tech/framework-os-mcp-rs:build"], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
    }
    process.env.OS_HUB_CREDENTIAL_SIGN_IN = process.env.OS_HUB_CREDENTIAL_SIGN_IN ?? "1";
    process.env.OS_HUB_ADMIN_TOKEN = process.env.OS_HUB_ADMIN_TOKEN ?? "dev-local-hub-admin";
    const profiles: readonly LocalProfile[] = [
      ...LOCAL_HUB_DEVELOPMENT_PROFILES,
      ...(secureAdmin ? [LOCAL_HUB_ADMINISTRATOR_PROFILE] : []),
    ];
    await buildRepositoryCargoArtifacts(join(this.root,"Cargo.toml"),["--bin","os-hub"],this.repoRoot,{producer:"artifact",invocation:this.invocation,control:phaseControl,network:repositoryCargoNetworkV1(this.invocation,this.repoRoot),consume:verifyCapturedCargoArtifactsV1});
    return withRepositoryDevelopmentStateV1(this.invocation,phaseControl,(state,statePhase)=>withPublishedDevelopmentLocalHubHostV1(this.invocation,statePhase,state,async(host,context)=>{
    const {dataRoot}=context;
    let trustedCatalog = trustedBootstrapCurrent(dataRoot);
    if (!trustedCatalog) {
      await withMaterializedTrustedCatalogBundleV1(host,context.control,trustedBootstrapSelectPackages(LOCAL_HUB_DEVELOPMENT_CATALOG_PACKAGES),async(receipt,actualPhase)=>{
      await validateAndPublishTrustedStdioGisCandidate(host,actualPhase,receipt);
      trustedCatalog = trustedBootstrapCurrent(dataRoot);
      if (!trustedCatalog) throw new Error("trusted stdio+GIS candidate did not publish an exact current generation");
      });
    }
    const run = await startLocalHub(host,profiles,{
      port: Number(process.env[OS_HUB_PORT_ENV] ?? OS_HUB_PORT),
      ...(secureAdmin?{adminSubjects:[LOCAL_HUB_ADMINISTRATOR_SUBJECT]}:{}),
      capture: true,
      ...(process.env.OS_HUB_ADMIN_TOKEN?{adminToken:process.env.OS_HUB_ADMIN_TOKEN}:{}),
      credentialSignIn:true,
    });
    let relay: LocalBrowserRelay | undefined;
    let adminRelay: LocalAdminRelay | undefined;
    let stopping:ReturnType<typeof stopLocalHubV1>|undefined;
    let ui: ChildProcess | undefined;
    let native: ChildProcess | undefined;
    let mcp: ChildProcess | undefined;
    const stop = (): void => {
      void relay?.stop();
      void adminRelay?.stop();
      for (const child of [ui, native, mcp]) if (child) terminateOwnedChildTree(child);
      stopping??=stopLocalHubV1(run,"graceful");void stopping.catch(()=>{});
    };
    process.once("SIGINT", stop);
    process.once("SIGTERM", stop);
    try {
      await waitForReadiness(run);
      console.log(`[INFO] secure local hub ready at http://127.0.0.1:${run.port}`);
      if (secureNative) {
        const envelope = await issueLocalCredential(run, "developer", "native");
        native = await deliverNativeCredentialEnvelope(nativeWgpuExecutable(this.repoRoot), ["--plugin", process.env.SEMIO_PLUGIN ?? "s"], envelope, `http://127.0.0.1:${run.port}`);
        native.stdout?.pipe(process.stdout);
        native.stderr?.pipe(process.stderr);
        console.log("[INFO] secure native WGPU child started with an fd3 credential endpoint");
      }
      if (secureMcp) {
        const envelope = await issueLocalCredential(run, "developer", "mcp", secureNative ? 3 : 2);
        ({ child: mcp } = await startMcpWorkspaceChild(this.repoRoot, run, envelope));
        mcp.stdout?.pipe(process.stdout);
        mcp.stderr?.pipe(process.stderr);
        console.log("[INFO] secure MCP child started with an fd3 credential endpoint");
      }
      if (secureAdmin) {
        const envelope = await issueLocalCredential(run, "administrator", "admin-relay", secureSuite ? 5 : 2);
        const adminProof = randomBytes(32);
        const adminProofHex = adminProof.toString("hex");
        adminRelay = startLocalAdminRelay(`http://127.0.0.1:${run.port}`, envelope, adminProof);
        openExternalBrowser(`${adminRelay.url}/admin/#semio-admin=${adminProofHex}`);
        console.log(`[INFO] secure local admin starting at ${adminRelay.url}/admin/`);
      }
      if (secureSuite) {
        const uiPort = Number(process.env.S_OS_PORT ?? 6066);
        const uiOrigin = `http://127.0.0.1:${uiPort}`;
        const envelope = await issueLocalCredential(run, "developer", "react-relay", 4);
        relay = startLocalBrowserRelay(`http://127.0.0.1:${run.port}`, uiOrigin, envelope);
        const uiScript = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts");
        ui = spawn(process.execPath, [uiScript, "serve", "s", "react", "dev"], {
          cwd: join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"),
          env: { ...process.env, S_OS_PORT: String(uiPort), S_HUB_URL: `http://127.0.0.1:${run.port}`, S_LOCAL_RELAY_URL: relay.url, S_LOCAL_RELAY_SECRET: relay.secret.toString("hex"), SEMIO_PLUGIN: "s", SEMIO_RENDERER: "react" },
          shell: false,
          stdio: "inherit",
        });
        await waitForUiReadiness(uiOrigin, ui);
        const browserProof = relay.takeBrowserBootstrapProof();
        const browserProofHex = browserProof.toString("hex");
        browserProof.fill(0);
        openExternalBrowser(`${uiOrigin}/#semio-broker=${browserProofHex}`);
        console.log(`[INFO] secure local OS profile starting at ${uiOrigin}`);
      }
      await withLocalSessionBrokerV1(run,profiles,2+[secureNative,secureMcp,secureSuite,secureAdmin].filter(Boolean).length,undefined,async broker=>{
      console.log(`[INFO] local session broker for ${broker.record.profiles.join(",")} — every \`dev s\` row signs in through it`);
      await new Promise<void>((resolveExit, rejectExit) => {
        void waitForLocalHubExitV1(run).then(result=>result.status===0&&result.signal===null?resolveExit():rejectExit(Error(`hub child exited with status ${result.status} signal ${result.signal}`)),rejectExit);
        ui?.once("exit", (code) => (code === 0 ? resolveExit() : rejectExit(new Error(`secure local UI child exited with status ${code}`))));
        native?.once("exit", (code) => (code === 0 ? resolveExit() : rejectExit(new Error(`secure native WGPU child exited with status ${code}`))));
        mcp?.once("exit", (code) => (code === 0 ? resolveExit() : rejectExit(new Error(`secure MCP child exited with status ${code}`))));
      });
      });
    } finally {
      process.off("SIGINT", stop);
      process.off("SIGTERM", stop);
      await relay?.stop();
      await adminRelay?.stop();
      for (const child of [ui, native, mcp]) if (child) terminateOwnedChildTree(child);
      await stopping;
      await finishLocalHub(run);
    }
    }));
  }
}
