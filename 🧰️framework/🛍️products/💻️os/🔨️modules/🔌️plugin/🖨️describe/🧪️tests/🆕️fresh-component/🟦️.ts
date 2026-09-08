export function createFreshComponentTests(dependencies: Record<string, any>, source: { directory: string; url: string }) {
  const { captureFreshComponentInputs, captureFreshSourceEpochV1, closeSync, createHash, existsSync, FRESH_COMPONENT_MAX_BYTES, FRESH_IO_CHUNK_BYTES, FRESH_SOURCE_EPOCH_LIMITS, freshRun, freshSourceEpochBytesV1, freshSourceOrderedJson, freshStage, freshWasmArtifactSize, isAbsolute, join, mkdirSync, mkdtempSync, openSync, parseFreshRustDepInfoV1, readdirSync, readFileSync, readStableBuildFile, renameSync, resolve, rmSync, semanticOwnedInputFileSnapshot, stageFreshComponentInputs, writeFileSync } = dependencies;
  type FreshBuildControlV1 = any;
  type FreshComponentLeaseV1 = any;
  type FreshSourceEpochLegV1 = any;
  type FreshSourceEpochPlanV1 = any;
  /** 🧪️ Exercises the physical epoch owner against neutral races and independent canonical/hash oracles. */
  async function testFreshComponentSourceEpochV1(repoRoot: string): Promise<void> {
    const { default: assert } = await import("node:assert/strict");
    const { default: stableStringify } = await import("fast-json-stable-stringify");
    const { dirname } = await import("node:path");
    const { symlinkSync, ftruncateSync } = await import("node:fs");
    const fixtureRoot = resolve(source.directory, "../../🧫️fixtures/🧾️fresh-source-epoch");
    const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
    const { default: Ajv } = await import("ajv");
    const ajv = new Ajv({ strict: true, allErrors: true });
    const describeSchema = JSON.parse(readFileSync(resolve(source.directory, "../../🧬️schema/🔣️.json"), "utf8"));
    ajv.addSchema(describeSchema);
    const validate = ajv.getSchema(`${describeSchema.$id}#/$defs/FreshSourceEpochLawsV1`)!;
    const validateEpoch = ajv.getSchema(`${describeSchema.$id}#/$defs/FreshSourceEpochV1`)!;
    assert(validate(fixture), JSON.stringify(validate.errors));
    assert.deepEqual(fixture.limits, FRESH_SOURCE_EPOCH_LIMITS);
    const depInfo = JSON.parse(readFileSync(join(fixtureRoot, "📃️dep-info.json"), "utf8"));
    const validateDepInfo = ajv.getSchema(`${describeSchema.$id}#/$defs/RustDepInfoLawsV1`)!;
    assert(validateDepInfo(depInfo), JSON.stringify(validateDepInfo.errors));
    for (const row of depInfo.cases) {
      const bytes = Buffer.from(row.text);
      if (row.expected === null) assert.throws(() => parseFreshRustDepInfoV1(bytes, () => {}), undefined, row.id);
      else assert.equal(stableStringify(parseFreshRustDepInfoV1(bytes, () => {})), stableStringify(row.expected), row.id);
    }
    assert.throws(() => parseFreshRustDepInfoV1(new Uint8Array([0xff, 0x0a]), () => {}), undefined, "lossy UTF-8");
    assert.throws(() => parseFreshRustDepInfoV1(new Uint8Array(depInfo.maximumBytes + 1), () => {}), undefined, "dep-info byte bound");
    assert.throws(() => parseFreshRustDepInfoV1(Buffer.from(depInfo.cases[0].text), () => { throw new Error("dep-info cancelled"); }), /dep-info cancelled/);
    const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
    assert(artifactRoot && isAbsolute(artifactRoot) && artifactRoot.split(/[\\/]/u).includes("🗑️generated"));
    const evidence = mkdtempSync(join(artifactRoot, "fresh-source-epoch-"));
    const legs: FreshSourceEpochLegV1[] = fixture.legs.map(({ id, package: cargoPackage, args }) => ({ id, package: cargoPackage, args }));
    const environment = [["CARGO_INCREMENTAL", "0"], ["RUSTC_WRAPPER", ""], ["RUSTFLAGS", null]] as const;
    const plan: FreshSourceEpochPlanV1 = { toolchain: { cargo: "cargo fixture", rustc: "rustc fixture" }, environment, legs, files: fixture.files.map((file) => file.path) };
    for (const name of fixture.cases as string[]) {
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
        }, undefined, name);
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
    console.log(`[DEBUG] fresh-source-epoch: AJV=3 stable-stringify=1 WebCrypto=1 physical-laws=${fixture.cases.length} bounded-capture=2 unsafe-paths=7 dep-info=${depInfo.cases.length}+3 evidence=${evidence}; Cargo resolver/dep-info integration remains unqualified`);
  }
  
  /** 🧪️ Qualifies retained verified inputs and staging independently of Cargo or descriptor execution. */
  async function testFreshComponentStagingV1(repoRoot: string): Promise<void> {
    const { default: assert } = await import("node:assert/strict");
    const { default: Ajv } = await import("ajv");
    const { encodePackValue } = await import("../../../../../🟦️.ts");
    const fixtureRoot = resolve(source.directory, "../../🧫️fixtures/🧊️fresh-staging");
    const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
    const describeSchema = JSON.parse(readFileSync(resolve(source.directory, "../../🧬️schema/🔣️.json"), "utf8"));
    const stagingAjv = new Ajv({ strict: true, allErrors: true });
    stagingAjv.addSchema(describeSchema);
    const validate = stagingAjv.getSchema(`${describeSchema.$id}#/$defs/FreshStagingV1`)!;
    assert(validate(fixture), JSON.stringify(validate.errors));
    const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
    assert(artifactRoot?.includes("🗑️generated"));
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
      executionProtocol: { appChannelVersion: 14 },
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
    let retainedLease: FreshComponentLeaseV1 | undefined, retainedLoan: Uint8Array | undefined;
    const source = cloneSnapshot();
    const produced = await handoff("loan-success", source, control, async (lease) => {
      retainedLease = lease;
      assert.deepEqual(Object.keys(lease), ["consume"]);
      assert(Object.isFrozen(lease));
      const digest = await lease.consume(async (bytes) => {
        retainedLoan = bytes;
        assert.notEqual(bytes.buffer, source.componentBytes.buffer);
        const sha256 = Buffer.from(await crypto.subtle.digest("SHA-256", bytes)).toString("hex");
        bytes.fill(9);
        assert.deepEqual(Buffer.from(source.componentBytes), component);
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
    assert(source.componentBytes.every((byte) => byte === 0));
    assert(source.descriptorBytes.every((byte) => byte === 0));
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
    console.log(`fresh-component-staging: AJV=1 WebCrypto=1 Pack=1 BLAKE3=1 laws=${fixture.laws.length} evidence=${evidence}`);
  }
  
  /** 🧪️ Qualifies fresh producer diagnostics and bounded real process retirement without Cargo. */
  async function testFreshComponentProcessV1(repoRoot: string): Promise<void> {
    const { default: assert } = await import("node:assert/strict");
    const { default: Ajv } = await import("ajv");
    const { default: deepEqual } = await import("fast-deep-equal");
    const fixtureRoot = resolve(source.directory, "../../🧫️fixtures/🧵️fresh-process");
    const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
    const describeSchema = JSON.parse(readFileSync(resolve(source.directory, "../../🧬️schema/🔣️.json"), "utf8"));
    const processAjv = new Ajv({ strict: true, allErrors: true });
    processAjv.addSchema(describeSchema);
    const validate = processAjv.getSchema(`${describeSchema.$id}#/$defs/FreshProcessV1`)!;
    assert(validate(fixture), JSON.stringify(validate.errors));
    const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
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
        assert(failure?.message.includes(row.diagnostic), row.name + " surfaced cause: " + failure?.message);
        assert(failure.message.includes(trace), row.name + " exact trace location");
        assert(failure.message.length <= fixture.diagnosticChars + trace.length + 500);
        assert.deepEqual(checkpoints, [0]);
      }
    }
    console.log("fresh-component-process: AJV=1 fast-deep-equal=3 runtime-laws=" + fixture.cases.length + " evidence=" + evidence);
  }
  return { testFreshComponentSourceEpochV1, testFreshComponentStagingV1, testFreshComponentProcessV1 };
}
