import { createHash } from "node:crypto";
import { closeSync, existsSync, mkdirSync, mkdtempSync, openSync, readFileSync, readdirSync, renameSync, rmSync, writeFileSync, cpSync } from "node:fs";
import { isAbsolute, join, resolve } from "node:path";
import { acquireCargoBuildLeaseV1 } from "../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts";
import { repoCacheDirectory } from "../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";
import { readStableBuildFile } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { semanticOwnedInputFileSnapshot } from "../../../../../../🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
import { FRESH_COMPONENT_MAX_BYTES, FRESH_IO_CHUNK_BYTES, freshWasmArtifactSize } from "../../🏗️component-build/🟦️.ts";
import { FRESH_SOURCE_EPOCH_LIMITS, captureFreshSourceEpochV1, freshSourceEpochBytesV1, freshSourceOrderedJson, parseFreshRustDepInfoV1 } from "../../🧾️source-epoch/🟦️.ts";
import { captureFreshComponentInputs, freshRun, freshStage, stageFreshComponentInputs } from "../../🏭️fresh-component/🟦️.ts";

export function createFreshComponentTests() {
  const source = { directory: resolve(import.meta.dir, "../../📦️packages/🦀️rust") };
  function testArtifactRoot(repoRoot: string): string {
    const root = process.env.SEMIO_TEST_ARTIFACT_DIR ?? repoCacheDirectory(repoRoot, "tests", "fresh-component", "🗑️generated");
    mkdirSync(root, { recursive: true });
    return root;
  }
  type FreshSourceEpochLawsFixtureV1 = Readonly<{
    limits: Readonly<{ fileBytes: number; totalBytes: number; files: number; legs: number }>;
    files: readonly Readonly<{ path: string; text: string }>[];
    legs: readonly Readonly<{ id: string; package: string; args: readonly string[]; inputs: readonly string[] }>[];
    cases: readonly string[];
  }>;
  type FreshStagingFixtureV1 = Readonly<{
    appChannelVersion: number;
    componentHex: string;
    componentSha256: string;
    componentBlake3: string;
    coreHex: string;
    laws: readonly string[];
  }>;
  type FreshProcessFixtureV1 = Readonly<{
    maxOutputBytes: number;
    diagnosticChars: number;
    cargoProvenance: Readonly<{name:string;stdout:string;source:string;buildSource:string;compilerEnv:string}>;
    cargoOwnership: Readonly<{rootName:string;nestedName:string;missingName:string;stdout:string;source:string}>;
    queuedCargoCases: readonly Readonly<{ name: string; mode: "cancel" | "timeout" }>[];
    cases: readonly Readonly<{ name: string; mode: "exit" | "missing" | "cancel" | "timeout" | "flood" | "pre-cancel"; stdout: string; stderr: string; exitCode: number; reason: "exit" | "spawn-error" | "cancelled" | "timeout" | "output-limit"; diagnostic: string }>[];
  }>;
  type RustDepInfoLawsFixtureV1 = Readonly<{ maximumBytes: number; cases: readonly Readonly<{ id: string; text: string; expected: unknown }>[] }>;
  type FreshBuildControlV1 = import("../../🧾️source-epoch/🟦️.ts").FreshBuildControlV1;
  type FreshComponentLeaseV1 = import("../../🧾️source-epoch/🟦️.ts").FreshComponentLeaseV1;
  type FreshSourceEpochLegV1 = import("../../🧾️source-epoch/🟦️.ts").FreshSourceEpochLegV1;
  type FreshSourceEpochPlanV1 = import("../../🧾️source-epoch/🟦️.ts").FreshSourceEpochPlanV1;
  /** 🧪️ Exercises the physical epoch owner against neutral races and independent canonical/hash oracles. */
  async function testFreshComponentSourceEpochV1(repoRoot: string): Promise<void> {
    const assert: typeof import("node:assert/strict") = (await import("node:assert/strict")).default;
    const { default: stableStringify } = await import("fast-json-stable-stringify");
    const { dirname } = await import("node:path");
    const { symlinkSync, ftruncateSync } = await import("node:fs");
    const fixtureRoot = resolve(source.directory, "../../🧫️fixtures/🧾️fresh-source-epoch");
    const fixture: FreshSourceEpochLawsFixtureV1 = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
    const { default: Ajv } = await import("ajv");
    const ajv = new Ajv({ strict: true, allErrors: true });
    const describeSchema = JSON.parse(readFileSync(resolve(source.directory, "../../🧬️schema/🔣️.json"), "utf8"));
    ajv.addSchema(describeSchema);
    const validateEpoch = ajv.compile({ $ref: `${describeSchema.$id}#/$defs/FreshSourceEpochV1` });
    assert.deepEqual(fixture.limits, FRESH_SOURCE_EPOCH_LIMITS);
    const depInfo: RustDepInfoLawsFixtureV1 = JSON.parse(readFileSync(join(fixtureRoot, "📃️dep-info.json"), "utf8"));
    for (const row of depInfo.cases) {
      const bytes = Buffer.from(row.text);
      if (row.expected === null) assert.throws(() => parseFreshRustDepInfoV1(bytes, () => {}), row.id);
      else assert.equal(stableStringify(parseFreshRustDepInfoV1(bytes, () => {})), stableStringify(row.expected), row.id);
    }
    assert.throws(() => parseFreshRustDepInfoV1(new Uint8Array([0xff, 0x0a]), () => {}), "lossy UTF-8");
    assert.throws(() => parseFreshRustDepInfoV1(new Uint8Array(depInfo.maximumBytes + 1), () => {}), "dep-info byte bound");
    assert.throws(() => parseFreshRustDepInfoV1(Buffer.from(depInfo.cases[0].text), () => { throw new Error("dep-info cancelled"); }), /dep-info cancelled/);
    const artifactRoot = testArtifactRoot(repoRoot);
    assert(artifactRoot && isAbsolute(artifactRoot) && artifactRoot.split(/[\\/]/u).includes("🗑️generated"));
    const evidence = mkdtempSync(join(artifactRoot, "fresh-source-epoch-"));
    const legs: FreshSourceEpochLegV1[] = fixture.legs.map(({ id, package: cargoPackage, args }) => ({ id, package: cargoPackage, args }));
    const environment = [["CARGO_INCREMENTAL", "0"], ["RUSTC_WRAPPER", ""], ["RUSTFLAGS", null]] as const;
    const plan: FreshSourceEpochPlanV1 = { toolchain: { cargo: "cargo fixture", rustc: "rustc fixture" }, environment, legs, files: fixture.files.map((file) => file.path) };
    for (const name of fixture.cases) {
      const root = join(evidence, name);
      mkdirSync(root);
      for (const file of fixture.files) {
        const path = join(root, file.path);
        mkdirSync(dirname(path), { recursive: true });
        writeFileSync(path, file.text, { flag: "wx", mode: 0o600 });
      }
      const pointer = join(root, "current.json"), prior = Buffer.from("unpublished-epoch-fixture\n");
      writeFileSync(pointer, prior, { flag: "wx", mode: 0o600 });
      let cancelled = false, expired = false;
      const control: FreshBuildControlV1 = { cancelled: () => cancelled, remainingMs: () => expired ? 0 : 10000, checkpoint() {} };
      const owner = captureFreshSourceEpochV1(root, plan, control);
      assert(validateEpoch(owner.record), JSON.stringify(validateEpoch.errors));
      assert(Object.isFrozen(owner) && Object.isFrozen(owner.record) && owner.record.files.every(Object.isFrozen));
      const mutate = (): void => writeFileSync(join(root, "artifacts/🦀️.rs"), "pub const CHANNEL_VERSION: u32 = 15;\n");
      const start = (index: number): void => owner.start(legs[index]!, environment);
      const complete = (index: number): void => owner.complete(legs[index]!.id, fixture.legs[index].inputs);
      const run = (index: number): void => { start(index); complete(index); };
      if (name === "shared-epoch") {
        assert.equal(freshSourceOrderedJson(owner.record), stableStringify(owner.record));
        const fields = [owner.record.schema, stableStringify(owner.record.toolchain), stableStringify(owner.record.environment), stableStringify(owner.record.legs), stableStringify(owner.record.files)];
        const parts = fields.map((field) => new TextEncoder().encode(field));
        const oracle = new Uint8Array(parts.reduce((sum, bytes) => sum + bytes.length + 4, 0));
        let offset = 0;
        for (const bytes of parts) { new DataView(oracle.buffer).setUint32(offset, bytes.length, false); oracle.set(bytes, offset + 4); offset += bytes.length + 4; }
        assert.deepEqual(Buffer.from(freshSourceEpochBytesV1(owner.record)), Buffer.from(oracle));
        assert.notDeepEqual(Buffer.from(freshSourceEpochBytesV1({ ...owner.record, legs: [...owner.record.legs].reverse() })), Buffer.from(oracle));
        assert.equal(owner.digest, Buffer.from(await crypto.subtle.digest("SHA-256", oracle)).toString("hex"));
        run(0); run(1);
        assert.equal(owner.finish(), owner.digest);
        assert.throws(() => owner.finish(), /closed/);
      } else {
        assert.throws(() => {
          if (name === "changed-before-first") { mutate(); start(0); }
          else if (name === "changed-between-legs") { run(0); mutate(); start(1); }
          else if (name === "changed-after-last") { run(0); run(1); mutate(); owner.finish(); }
          else if (name === "unknown-compiler-input") { start(0); owner.complete(legs[0]!.id, [...fixture.legs[0].inputs, "unknown/🦀️.rs"]); }
          else if (name === "missing-input") { rmSync(join(root, "Cargo.lock")); start(0); }
          else if (name === "same-bytes-name-replacement") { const path = join(root, "Cargo.lock"); renameSync(path, path + ".retained"); writeFileSync(path, readFileSync(path + ".retained")); start(0); }
          else if (name === "linked-parent") { const path = join(root, "assets"); renameSync(path, path + "-retained"); symlinkSync(path + "-retained", path, process.platform === "win32" ? "junction" : "dir"); start(0); }
          else if (name === "oversized-input") { const file = openSync(join(root, "Cargo.lock"), "r+"); try { ftruncateSync(file, fixture.limits.fileBytes + 1); } finally { closeSync(file); } start(0); }
          else if (name === "changed-arguments") owner.start({ ...legs[0]!, args: [...legs[0]!.args, "--features", "foreign"] }, environment);
          else if (name === "changed-environment") owner.start(legs[0]!, [["RUSTFLAGS", "--cfg foreign"]]);
          else if (name === "out-of-order-leg") start(1);
          else if (name === "duplicate-leg-completion") { run(0); complete(0); }
          else if (name === "incomplete-finish") { run(0); owner.finish(); }
          else if (name === "cancelled") { cancelled = true; start(0); }
          else if (name === "deadline") { expired = true; start(0); }
          else if (name === "closed-owner") { owner.abort(); start(0); }
          else assert.fail("unknown epoch law " + name);
        }, name);
        assert.throws(() => owner.finish(), /closed/, name + " terminal refusal");
      }
      assert.deepEqual(readFileSync(pointer), prior, name + " pointer is untouched");
    }
    const bounded = join(evidence, "bounded");
    mkdirSync(bounded);
    writeFileSync(join(bounded, "input.rs"), Buffer.alloc(2 * FRESH_IO_CHUNK_BYTES + 1, 65));
    assert.throws(() => semanticOwnedInputFileSnapshot(bounded, "input.rs", { maximumBytes: FRESH_IO_CHUNK_BYTES }), /byte boundary/);
    let checks = 0;
    assert.throws(() => semanticOwnedInputFileSnapshot(bounded, "input.rs", { maximumBytes: 3 * FRESH_IO_CHUNK_BYTES, checkpoint() { if (++checks === 4) throw new Error("bounded snapshot cancellation"); } }), /bounded snapshot cancellation/);
    assert.equal(checks, 4);
    for (const path of ["../foreign.rs", "/foreign.rs", "C:/foreign.rs", "a\\foreign.rs", "node_modules/source.rs", "target/source.rs", "🗑️generated/source.rs"]) assert.throws(() => captureFreshSourceEpochV1(bounded, { ...plan, files: [path] }, { cancelled: () => false, remainingMs: () => 10000, checkpoint() {} }), /coordinate|refuses/);
    console.log(`fresh-source-epoch: AJV=1 stable-stringify=1 WebCrypto=1 physical-laws=${fixture.cases.length} bounded-capture=2 unsafe-paths=7 dep-info=${depInfo.cases.length}+3 evidence=${evidence}; Cargo resolver/dep-info integration remains unqualified`);
  }
  
  /** 🧪️ Qualifies retained verified inputs and staging independently of Cargo or descriptor execution. */
  async function testFreshComponentStagingV1(repoRoot: string): Promise<void> {
    const assert: typeof import("node:assert/strict") = (await import("node:assert/strict")).default;
    const { encodePackValue } = await import("../../../../../🟦️.ts");
    const fixtureRoot = resolve(source.directory, "../../🧫️fixtures/🧊️fresh-staging");
    const fixture: FreshStagingFixtureV1 = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
    const artifactRoot = testArtifactRoot(repoRoot);
    assert(artifactRoot !== undefined && artifactRoot.includes("🗑️generated"));
    mkdirSync(artifactRoot, { recursive: true });
    const evidence = mkdtempSync(join(artifactRoot, "fresh-component-staging-"));
    const component = Buffer.from(fixture.componentHex, "hex"),
      core = Buffer.from(fixture.coreHex, "hex");
    const descriptor = {
      descriptorVersion: 1,
      packageId: "semio:gis",
      role: "plugin",
      manifest: { pluginId: "gis", label: "GIS", version: "0.1.0", apps: [], examples: [], capabilities: [], topicContributions: [], commands: [], artifactKinds: [], dependencies: [], contributions: [] },
      activationEvents: [],
      capabilityRequests: [],
      extensionPoints: [],
      execution: "isolated",
      executionProtocol: { appChannelVersion: fixture.appChannelVersion },
      quotas: {},
      contributions: {},
      assets: [],
      hashes: { wasmSha256: fixture.componentSha256, coreWasmSha256: createHash("sha256").update(core).digest("hex"), descriptorSha256: "" },
    };
    descriptor.hashes.descriptorSha256 = createHash("sha256").update(encodePackValue(descriptor)).digest("hex");
    const descriptorBytes = encodePackValue(descriptor);
    const paths = { component: join(evidence, "component.wasm"), core: join(evidence, "core.wasm"), descriptorPack: join(evidence, "descriptor.semio"), descriptorJson: join(evidence, "descriptor.json") };
    for (const [key, bytes] of [
      ["component", component],
      ["core", core],
      ["descriptorPack", descriptorBytes],
      ["descriptorJson", Buffer.from(JSON.stringify(descriptor))],
    ] as const)
      writeFileSync(paths[key], bytes);
    assert.equal(freshWasmArtifactSize(paths.component, component.byteLength, "fixture WASIp2 component"), component.byteLength);
    assert.throws(() => freshWasmArtifactSize(paths.component, component.byteLength - 1, "fixture WASIp2 component"), new RegExp(`byteLength=${component.byteLength} maximum=${component.byteLength - 1}`));
    const control: FreshBuildControlV1 = { cancelled: () => false, remainingMs: () => 60_000, checkpoint() {} };
    const capturedComponent = readStableBuildFile(paths.component, FRESH_COMPONENT_MAX_BYTES, { remaining: FRESH_COMPONENT_MAX_BYTES }, () => {});
    const capturedCore = readStableBuildFile(paths.core, FRESH_COMPONENT_MAX_BYTES, { remaining: FRESH_COMPONENT_MAX_BYTES }, () => {});
    writeFileSync(paths.core, "replaced-core");
    const snapshot = await captureFreshComponentInputs(repoRoot, { pluginId: "gis", componentPackageId: "semio:gis" }, capturedComponent, capturedCore, paths, control);
    assert.equal(snapshot.coreSha256, descriptor.hashes.coreWasmSha256);
    assert.equal(snapshot.componentSha256, fixture.componentSha256);
    assert.equal(snapshot.componentBlake3, fixture.componentBlake3);
    assert.equal(snapshot.componentSha256, Buffer.from(await crypto.subtle.digest("SHA-256", component)).toString("hex"));
    assert.deepEqual(Buffer.from(snapshot.descriptorBytes), Buffer.from(descriptorBytes));
    for (const path of Object.values(paths)) {
      renameSync(path, path + ".retained");
      writeFileSync(path, "replaced-source");
    }
    const destination = join(evidence, "staged.semio");
    const staged = freshStage(snapshot.descriptorBytes, destination, control, "stage-descriptor", 6, 8);
    assert.equal(staged.sha256, Buffer.from(await crypto.subtle.digest("SHA-256", descriptorBytes)).toString("hex"));
    assert.deepEqual(readFileSync(destination), Buffer.from(descriptorBytes));
    renameSync(destination, destination + ".retained");
    writeFileSync(destination, "replaced-stage");
    assert.deepEqual(Buffer.from(snapshot.descriptorBytes), Buffer.from(descriptorBytes));
    assert.equal(createHash("sha256").update(snapshot.componentBytes).digest("hex"), fixture.componentSha256);
    const cancelled = join(evidence, "cancelled.semio");
    let checkpoints = 0;
    assert.throws(
      () =>
        freshStage(
          snapshot.descriptorBytes,
          cancelled,
          {
            ...control,
            cancelled: () => checkpoints >= 2,
            checkpoint() {
              checkpoints++;
            },
          },
          "stage-descriptor",
          6,
          8,
        ),
      /cancelled/,
    );
    assert.equal(checkpoints, 2);
    assert.equal(existsSync(cancelled), false);
    writeFileSync(paths.component, component);
    writeFileSync(paths.core, core);
    writeFileSync(paths.descriptorPack, descriptorBytes);
    writeFileSync(paths.descriptorJson, JSON.stringify({ ...descriptor, packageId: "semio:foreign" }));
    await assert.rejects(captureFreshComponentInputs(repoRoot, { pluginId: "gis", componentPackageId: "semio:gis" }, capturedComponent, capturedCore, paths, control), /identity/);
    const cloneSnapshot = () => ({ ...snapshot, componentBytes: Uint8Array.from(component), descriptorBytes: Uint8Array.from(descriptorBytes) });
    const handoff = async <T>(name: string, input: ReturnType<typeof cloneSnapshot>, buildControl: FreshBuildControlV1, derive: (lease: FreshComponentLeaseV1) => Promise<T>) => {
      const stage = join(evidence, name);
      mkdirSync(stage);
      return await stageFreshComponentInputs({ pluginId: "gis", componentPackageId: "semio:gis" }, input, stage, ["checkpoint", "describe", "jobs", "reactor"], buildControl, derive);
    };
    let retainedLease: FreshComponentLeaseV1 | undefined, retainedLoan: Uint8Array<ArrayBuffer> | undefined;
    const retainedInput = cloneSnapshot();
    const produced = await handoff("loan-success", retainedInput, control, async (lease) => {
      retainedLease = lease;
      assert.deepEqual(Object.keys(lease), ["consume"]);
      assert(Object.isFrozen(lease));
      const digest = await lease.consume(async (bytes) => {
        retainedLoan = bytes;
        assert.notEqual(bytes.buffer, retainedInput.componentBytes.buffer);
        const sha256 = Buffer.from(await crypto.subtle.digest("SHA-256", bytes)).toString("hex");
        bytes.fill(9);
        assert.deepEqual(Buffer.from(retainedInput.componentBytes), component);
        return sha256;
      });
      await assert.rejects(
        lease.consume(async () => "second"),
        /already consumed/,
      );
      return digest;
    });
    assert.equal(produced.derived, produced.receipt.component.sha256);
    assert.equal(produced.receipt.component.sha256, fixture.componentSha256);
    assert.deepEqual(readFileSync(join(evidence, "loan-success/component.wasm")), component);
    assert.deepEqual(Object.keys(produced.receipt).sort(), ["component", "coreSha256", "descriptor", "packageId", "pluginId", "version", "witExports"]);
    assert(retainedLoan!.every((byte) => byte === 0));
    assert(retainedInput.componentBytes.every((byte) => byte === 0));
    assert(retainedInput.descriptorBytes.every((byte) => byte === 0));
    await assert.rejects(
      retainedLease!.consume(async () => "late"),
      /expired/,
    );
    const rejectedSource = cloneSnapshot();
    let rejectedLoan: Uint8Array | undefined;
    await assert.rejects(
      handoff("loan-rejected", rejectedSource, control, (lease) =>
        lease.consume(async (bytes) => {
          rejectedLoan = bytes;
          throw new Error("derive sentinel");
        }),
      ),
      /^Error: derive sentinel$/,
    );
    assert(rejectedLoan!.every((byte) => byte === 0));
    assert(rejectedSource.componentBytes.every((byte) => byte === 0));
    assert(rejectedSource.descriptorBytes.every((byte) => byte === 0));
    assert.deepEqual(readdirSync(join(evidence, "loan-rejected")), []);
    for (const when of ["before-consume", "after-consume"] as const) {
      let stop = false,
        invoked = false;
      const cancelledSource = cloneSnapshot();
      await assert.rejects(
        handoff(`loan-cancel-${when}`, cancelledSource, { ...control, cancelled: () => stop }, async (lease) => {
          if (when === "before-consume") stop = true;
          return await lease.consume(async () => {
            invoked = true;
            stop = true;
            return "must not publish";
          });
        }),
        /cancelled/,
      );
      assert.equal(invoked, when === "after-consume");
      assert(cancelledSource.componentBytes.every((byte) => byte === 0));
      assert(cancelledSource.descriptorBytes.every((byte) => byte === 0));
      assert.deepEqual(readdirSync(join(evidence, `loan-cancel-${when}`)), []);
    }
    for (const failure of [false, true]) {
      const unawaitedSource = cloneSnapshot();
      let release!: () => void,
        entered!: () => void,
        settled = false,
        loan: Uint8Array | undefined;
      const gate = new Promise<void>((resolve) => {
        release = resolve;
      });
      const running = new Promise<void>((resolve) => {
        entered = resolve;
      });
      const operation = handoff(`loan-unawaited-${failure}`, unawaitedSource, control, async (lease) => {
        void lease.consume(async (bytes) => {
          loan = bytes;
          entered();
          await gate;
          if (failure) throw new Error("unawaited sentinel");
          return "done";
        });
        return "callback finished";
      });
      void operation.then(
        () => {
          settled = true;
        },
        () => {
          settled = true;
        },
      );
      await running;
      await Promise.resolve();
      assert.equal(settled, false);
      assert.deepEqual(Buffer.from(loan!), component);
      release();
      if (failure) await assert.rejects(operation, /^Error: unawaited sentinel$/);
      else assert.equal((await operation).derived, "callback finished");
      assert(loan!.every((byte) => byte === 0));
      assert(unawaitedSource.componentBytes.every((byte) => byte === 0));
      if (failure) assert.deepEqual(readdirSync(join(evidence, `loan-unawaited-${failure}`)), []);
    }
    snapshot.componentBytes.fill(0);
    capturedCore.fill(0);
    snapshot.descriptorBytes.fill(0);
    console.log(`fresh-component-staging: WebCrypto=1 Pack=1 BLAKE3=1 laws=${fixture.laws.length} evidence=${evidence}`);
  }
  
  /** 🧪️ Qualifies bounded process retirement and refuses Cargo startup after queued cancellation or deadline. */
  async function testFreshComponentProcessV1(repoRoot: string): Promise<void> {
    const assert: typeof import("node:assert/strict") = (await import("node:assert/strict")).default;
    const { default: deepEqual } = await import("fast-deep-equal");
    const fixtureRoot = resolve(source.directory, "../../🧫️fixtures/🧵️fresh-process");
    const fixture: FreshProcessFixtureV1 = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
    const artifactRoot = testArtifactRoot(repoRoot);
    assert(artifactRoot && isAbsolute(artifactRoot) && artifactRoot.split(/[\\/]/u).includes("🗑️generated"));
    const evidence = mkdtempSync(join(artifactRoot, "fresh-process-laws-"));
    for (const row of fixture.cases) {
      const root = join(evidence, row.name);
      mkdirSync(root);
      const started = Date.now(),
        checkpoints: number[] = [];
      const deadline = started + (row.mode === "timeout" ? 150 : 10_000);
      const control: FreshBuildControlV1 = {
        diagnosticsRoot: root,
        cancelled: () => row.mode === "pre-cancel" || (row.mode === "cancel" && Date.now() - started >= 150),
        remainingMs: () => deadline - Date.now(),
        checkpoint: (_stage, completed) => {
          checkpoints.push(completed);
        },
      };
      const script =
        row.mode === "exit"
          ? "process.stdout.write(" + JSON.stringify(row.stdout) + "); process.stderr.write(" + JSON.stringify(row.stderr) + "); process.exitCode=" + row.exitCode
          : row.mode === "flood"
            ? "const {writeSync}=require('node:fs'); const b=Buffer.alloc(65536,120); for(let n=0;n<=1024;n++) writeSync(1,b); setInterval(()=>{},1000)"
            : "setInterval(()=>{},1000)";
      let failure: Error | undefined;
      try {
        await freshRun(
          row.mode === "missing" ? join(root, "absent-executable") : process.execPath,
          ["-e", script],
          repoRoot,
          { ...process.env, SEMIO_TEST_ARTIFACT_DIR: join(root, "must-not-use-ambient-evidence"), CARGO_TARGET_DIR: root },
          control,
          row.name,
          0,
          1,
        );
      } catch (error) {
        failure = error as Error;
      }
      assert(Date.now() - started < 6000, row.name + " bounded retirement");
      if (row.mode === "pre-cancel") {
        assert(failure?.message.includes(row.diagnostic));
        assert.deepEqual(readdirSync(root), []);
        continue;
      }
      const traces = readdirSync(root);
      assert.equal(traces.length, 1, row.name + " retained process trace");
      const trace = join(root, traces[0]!);
      const outcome = JSON.parse(readFileSync(join(trace, "outcome.json"), "utf8"));
      assert.equal(outcome.reason, row.reason);
      assert.equal(outcome.stage, row.name);
      assert.equal(outcome.cargoTargetDir, root);
      const stdout = readFileSync(join(trace, "stdout.jsonl"), "utf8"),
        stderr = readFileSync(join(trace, "stderr.txt"), "utf8");
      assert(Buffer.byteLength(stdout) + Buffer.byteLength(stderr) <= fixture.maxOutputBytes);
      if (row.mode === "exit") {
        assert(deepEqual({ stdout, stderr, status: outcome.status }, { stdout: row.stdout, stderr: row.stderr, status: row.exitCode }), row.name + " independent transcript oracle");
        assert.equal(outcome.signal, null);
      }
      if (row.name === "success") {
        assert.equal(failure, undefined);
        assert.deepEqual(checkpoints, [0, 1]);
      } else {
        assert(failure !== undefined && failure.message.includes(row.diagnostic), row.name + " surfaced cause: " + failure?.message);
        assert(failure.message.includes(trace), row.name + " exact trace location");
        assert(failure.message.length <= fixture.diagnosticChars + trace.length + 500);
        assert.deepEqual(checkpoints, [0]);
      }
    }
    for (const row of fixture.queuedCargoCases) {
      const root = join(evidence, row.name); mkdirSync(root);
      const buildDirectory = join(root, "compiler");
      const options = { directory: repoCacheDirectory(repoRoot, "agents", "resource-leases"), buildDirectory, args: ["build"], signal: new AbortController().signal };
      const holder = await acquireCargoBuildLeaseV1(options);
      const started = Date.now(), deadline = started + (row.mode === "timeout" ? 150 : 10_000);
      const stages: string[] = [];
      try {
        await assert.rejects(freshRun("cargo", ["build", "--manifest-path", join(root, "absent-Cargo.toml")], repoRoot,
          { ...process.env, CARGO_BUILD_BUILD_DIR: buildDirectory },
          { diagnosticsRoot: root, cancelled: () => row.mode === "cancel" && Date.now() - started >= 150, remainingMs: () => deadline - Date.now(), checkpoint: stage => { stages.push(stage); } }, row.name, 0, 1));
        assert(Date.now() - started < 2000, row.name + " bounded queue retirement");
        assert(stages.includes("wait-build-lease"));
        const traces = readdirSync(root); assert.equal(traces.length, 1);
        assert.deepEqual(readdirSync(join(root, traces[0]!)), [], "cancelled queue must not spawn Cargo");
      } finally { holder.release(); }
      const successor = await acquireCargoBuildLeaseV1(options); successor.release();
    }
    const ownership=fixture.cargoOwnership, ownerRoot=join(evidence,"actual-selected-package-owner");mkdirSync(ownerRoot);
    const ownerRows=[{directory:"root-package",name:ownership.rootName,workspace:"."},{directory:"nested/package",name:ownership.nestedName,workspace:"nested"}];
    for(const row of ownerRows){const path=join(ownerRoot,row.directory);mkdirSync(path,{recursive:true});writeFileSync(join(path,"Cargo.toml"),`[package]\nname=${JSON.stringify(row.name)}\nversion="0.0.0"\nedition="2021"\n[[bin]]\nname=${JSON.stringify(row.name)}\npath="main.rs"\n`);writeFileSync(join(path,"main.rs"),ownership.source);}
    writeFileSync(join(ownerRoot,"Cargo.toml"),'[workspace]\nresolver="2"\nmembers=["root-package"]\nexclude=["nested"]\n[workspace.metadata.semio.repository]\nschema-version=1\nowner-manifests=["nested/Cargo.toml"]\nmember-manifests=["root-package/Cargo.toml"]\nexclude-patterns=["nested"]\n');
    writeFileSync(join(ownerRoot,"nested/Cargo.toml"),'[workspace]\nresolver="2"\nmembers=["package"]\n[workspace.metadata.semio.repository]\nschema-version=1\nmember-manifests=["package/Cargo.toml"]\nexclude-patterns=[]\n');
    const ownerCompiler=join(ownerRoot,"compiler"),ownerTarget=join(ownerRoot,"target"),ownerEnv={...process.env,CARGO_TARGET_DIR:ownerTarget,CARGO_BUILD_BUILD_DIR:ownerCompiler},ownerControl={cancelled:()=>false,remainingMs:()=>60_000,checkpoint:()=>{}};
    const ownerInvocation=await freshRun("cargo",["build","-p",ownership.nestedName],ownerRoot,ownerEnv,ownerControl,"selected-nested-owner",0,1);
    assert(ownerInvocation);const ownerObserved=JSON.parse(readFileSync(ownerInvocation!,"utf8")),ownerManifest=join(ownerRoot,"nested/package/Cargo.toml");
    assert.equal(ownerObserved.cwd,ownerRoot);assert.equal(ownerObserved.manifest,ownerManifest);assert.deepEqual(ownerObserved.args,["build","--manifest-path",ownerManifest,"-p",ownership.nestedName,"--message-format=json"]);
    assert(ownerObserved.invocationInputs.some((input:any)=>input.path===join(ownerRoot,"nested/Cargo.toml")&&typeof input.sha256==="string"));assert.equal(await Bun.$`${join(ownerTarget,"debug",ownership.nestedName+(process.platform==="win32"?".exe":""))}`.text(),ownership.stdout);
    const cargoMetadata=await Bun.$`cargo metadata --manifest-path ${ownerManifest} --no-deps --format-version 1`.json();assert.equal(cargoMetadata.workspace_root,join(ownerRoot,"nested"));assert.equal(cargoMetadata.packages.find((row:any)=>row.name===ownership.nestedName).manifest_path,ownerManifest);
    await assert.rejects(freshRun("cargo",["build","-p",ownership.missingName],ownerRoot,ownerEnv,ownerControl,"missing-owner",0,1),/Unknown current repository Cargo package/);
    await assert.rejects(freshRun("cargo",["build","-p",ownership.rootName,"-p",ownership.nestedName],ownerRoot,ownerEnv,ownerControl,"cross-workspace-owner",0,1),/different workspaces/);
    console.log("fresh-component-package-ownership: actual-Cargo=1 metadata-oracle=1 unknown-refusal=1 cross-workspace-refusal=1 evidence="+ownerRoot);
    const neutral=fixture.cargoProvenance;
    const cargoRoot = join(evidence, "actual-cargo-provenance"); mkdirSync(cargoRoot);
    const manifest = join(cargoRoot, "Cargo.toml");
    writeFileSync(manifest, `[workspace]\n[package]\nname=${JSON.stringify(neutral.name)}\nversion="0.0.0"\nedition="2021"\n[[bin]]\nname=${JSON.stringify(neutral.name)}\npath="main.rs"\n`);
    writeFileSync(join(cargoRoot,"main.rs"), neutral.source);writeFileSync(join(cargoRoot,"build.rs"),neutral.buildSource);
    const compiler = join(cargoRoot,"compiler"), target = join(cargoRoot,"target");
    const invocation = await freshRun("cargo",["build","--manifest-path",manifest],cargoRoot,{...process.env,CARGO_TARGET_DIR:target,CARGO_BUILD_BUILD_DIR:compiler},{cancelled:()=>false,remainingMs:()=>60_000,checkpoint:()=>{}},"neutral-build",0,1);
    assert(invocation, "actual fresh Cargo must retain completed producer observation without diagnostic opt-in");
    const observed = JSON.parse(readFileSync(invocation!,"utf8"));
    assert.equal(observed.status,0); assert.equal(observed.command,"cargo");
    assert.deepEqual(observed.args,["build","--manifest-path",manifest,"--message-format=json"]);
    assert.equal(observed.manifest,manifest); assert.equal(observed.compilerResourceRoot,join(compiler,"semio-compiler-resources"));
    assert(observed.units.some((unit:any)=>resolve(unit.message.target.src_path)===join(cargoRoot,"main.rs")));
    assert(observed.units.every((unit:any)=>unit.inputs.every((input:any)=>typeof input.sha256==="string")));
    assert(observed.buildScripts.some((row:any)=>row.env.some(([key,value]:[string,string])=>key===neutral.compilerEnv&&value===join(compiler,"semio-compiler-resources"))));
    const binary=join(target,"debug",neutral.name+(process.platform==="win32"?".exe":""));
    const actual=await Bun.$`${binary}`.text(); assert.equal(actual,neutral.stdout);
    assert.equal(await crypto.subtle.digest("SHA-256",readFileSync(binary)).then(bytes=>Buffer.from(bytes).toString("hex")),observed.units.find((unit:any)=>unit.message.target.kind.includes("bin")).artifacts.find((row:any)=>row.path===binary).sha256);
    const {retainTrustedCargoInvocationV1}=await import(join(repoRoot,"🌎️hub/🏗️bootstrap/🧾️provenance/🟦️.ts"));
    const staged=join(cargoRoot,"staged"+ (process.platform==="win32"?".exe":""));cpSync(binary,staged,{errorOnExist:true,force:false});
    const finalReceipt=join(cargoRoot,"final-invocation.json");retainTrustedCargoInvocationV1(invocation!,finalReceipt,new Map([[binary,staged]]));
    const transferred=JSON.parse(readFileSync(finalReceipt,"utf8"));
    assert.deepEqual(transferred.units.map((unit:any)=>unit.message),observed.units.map((unit:any)=>unit.message));
    for(const key of ["args","invocationInputs","buildScripts","buildResources","compilerResources"])assert.deepEqual(transferred[key],observed[key]);
    assert.deepEqual(transferred.units.map((unit:any)=>[unit.depInfo,unit.inputs]),observed.units.map((unit:any)=>[unit.depInfo,unit.inputs]));
    const custody=transferred.units.flatMap((unit:any)=>unit.artifacts).find((row:any)=>row.path===binary);assert.equal(custody.stagedPath,staged);assert.equal(custody.stagedSha256,custody.sha256);
    rmSync(target,{recursive:true,force:true});assert.equal(await Bun.$`${staged}`.text(),neutral.stdout);assert(!existsSync(binary));
    const retainedStage=readFileSync(staged);writeFileSync(staged,"changed");assert.throws(()=>retainTrustedCargoInvocationV1(invocation!,join(cargoRoot,"must-not-exist.json"),new Map([[binary,staged]])));assert(!existsSync(join(cargoRoot,"must-not-exist.json")));writeFileSync(staged,retainedStage);
    console.log("fresh-component-process: fast-deep-equal=3 runtime-laws=" + (fixture.cases.length + fixture.queuedCargoCases.length) + " evidence=" + evidence);
  }
  return { testFreshComponentSourceEpochV1, testFreshComponentStagingV1, testFreshComponentProcessV1 };
}
