/** 🧭️ The retained GIS browser actor evidence and the child proof the trusted bootstrap publication runs before it publishes.
 * @see ../../../📦️packages/🦀️rust/📜️script.ts
 */
import { createHash, randomBytes } from "node:crypto";
import { existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, renameSync, rmSync } from "node:fs";
import { join } from "node:path";
import { decodePackValue, encodePackValue } from "../../../../🧰️framework/🛍️products/💻️os/🟦️.ts";
import { DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔏️document-open/🟦️.ts";
import { DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES } from "@semio-tech/framework/directory/schema";
import { DOCUMENT_BROWSER_ACTOR_MAX_BYTES } from "@semio-tech/framework/directory/schema/browser-actor";
import { type FreshBuildControlV1, type FreshComponentReceiptV1 } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧾️source-epoch/🟦️.ts";
import { type ClosedBrowserActorArtifactV1 } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts";
import { freeLoopbackPort } from "../../../🚀️local-bootstrap/🏃️execution/🟦️.ts";
import { trustedBootstrapFsyncDirectory, trustedBootstrapWriteNew } from "../🗃️bootstrap-bundle/🟦️.ts";
import { trustedBootstrapReadRegular } from "../🧭️bootstrap-selection/🟦️.ts";
import { webSha256Hex } from "../../../🚀️local-relay/🛰️servers/🟦️.ts";

/** 🧳️ Retains the exact qualified GIS component/descriptor/actor evidence after scratch retirement. */
export function retainBrowserActorGisEvidenceV1(
  artifactRoot: string,
  stageRoot: string,
  receipt: Pick<FreshComponentReceiptV1, "component" | "descriptor">,
  actor: Pick<ClosedBrowserActorArtifactV1, "bytes" | "byteLength" | "sha256">,
  descriptor: Uint8Array,
): string {
  const qualifiedRoot = join(artifactRoot, "qualified-gis-actors");
  mkdirSync(qualifiedRoot, { recursive: true, mode: 0o700 });
  const qualifiedInfo = lstatSync(qualifiedRoot);
  if (qualifiedInfo.isSymbolicLink() || !qualifiedInfo.isDirectory()) throw new Error("GIS qualified evidence root is not a regular directory");
  const destination = join(qualifiedRoot, actor.sha256);
  const files = [
    { name: "component.wasm", bytes: undefined as Uint8Array | undefined, byteLength: receipt.component.byteLength, sha256: receipt.component.sha256, maximum: DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES },
    { name: "descriptor.semio", bytes: descriptor, byteLength: receipt.descriptor.byteLength, sha256: receipt.descriptor.sha256, maximum: DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES },
    { name: "closed-actor.mjs", bytes: actor.bytes, byteLength: actor.byteLength, sha256: actor.sha256, maximum: DOCUMENT_BROWSER_ACTOR_MAX_BYTES },
  ];
  const verify = (root: string): void => {
    const info = lstatSync(root);
    if (info.isSymbolicLink() || !info.isDirectory() || JSON.stringify(readdirSync(root).sort()) !== JSON.stringify(files.map((file) => file.name).sort())) throw new Error("GIS qualified evidence closure differs");
    for (const file of files) {
      const bytes = trustedBootstrapReadRegular(join(root, file.name), file.maximum, `GIS qualified ${file.name}`, () => {});
      try {
        if (bytes.byteLength !== file.byteLength || createHash("sha256").update(bytes).digest("hex") !== file.sha256) throw new Error(`GIS qualified ${file.name} receipt mismatch`);
      } finally {
        bytes.fill(0);
      }
    }
  };
  if (existsSync(destination)) {
    verify(destination);
    return destination;
  }
  const temporary = join(qualifiedRoot, `.staging-${randomBytes(16).toString("hex")}`);
  mkdirSync(temporary, { mode: 0o700 });
  const component = trustedBootstrapReadRegular(join(stageRoot, receipt.component.relativePath), DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES, "GIS qualified component source", () => {});
  try {
    if (component.byteLength !== receipt.component.byteLength || createHash("sha256").update(component).digest("hex") !== receipt.component.sha256) throw new Error("GIS qualified component source receipt mismatch");
    files[0]!.bytes = component;
    for (const file of files) trustedBootstrapWriteNew(join(temporary, file.name), file.bytes!, () => {});
    trustedBootstrapFsyncDirectory(temporary);
    verify(temporary);
    renameSync(temporary, destination);
    trustedBootstrapFsyncDirectory(qualifiedRoot);
    verify(destination);
    return destination;
  } finally {
    component.fill(0);
    files[0]!.bytes = undefined;
    rmSync(temporary, { recursive: true, force: true });
  }
}

/** 🧪️ Proves exact retained-byte identity and replacement rejection with WebCrypto. */
export async function proveBrowserActorGisEvidenceV1(artifactRoot: string): Promise<void> {
  const assert: (typeof import("node:assert"))["strict"] = (await import("node:assert/strict")).default;
  const { writeFileSync } = await import("node:fs");
  const root = mkdtempSync(join(artifactRoot, "gis-qualified-evidence-law-"));
  const stage = join(root, "stage");
  mkdirSync(stage);
  const component = Uint8Array.from([0, 97, 115, 109, 1]),
    descriptor = Uint8Array.from([0x92, 0xa3, 0x67, 0x69, 0x73]),
    actorBytes = Uint8Array.from([0x65, 0x78, 0x70, 0x6f, 0x72, 0x74]);
  const identity = async (bytes: Uint8Array) => ({ byteLength: bytes.byteLength, sha256: await webSha256Hex(bytes) });
  const componentIdentity = await identity(component),
    descriptorIdentity = await identity(descriptor),
    actorIdentity = await identity(actorBytes);
  writeFileSync(join(stage, "component.wasm"), component);
  try {
    const retained = retainBrowserActorGisEvidenceV1(
      root,
      stage,
      { component: { ...componentIdentity, relativePath: "component.wasm", blake3: "00" }, descriptor: { ...descriptorIdentity, relativePath: "descriptor.semio" } },
      { bytes: actorBytes, ...actorIdentity },
      descriptor,
    );
    assert.equal(Buffer.from(await crypto.subtle.digest("SHA-256", readFileSync(join(retained, "closed-actor.mjs")))).toString("hex"), actorIdentity.sha256);
    writeFileSync(join(retained, "closed-actor.mjs"), "replaced");
    assert.throws(
      () =>
        retainBrowserActorGisEvidenceV1(
          root,
          stage,
          { component: { ...componentIdentity, relativePath: "component.wasm", blake3: "00" }, descriptor: { ...descriptorIdentity, relativePath: "descriptor.semio" } },
          { bytes: actorBytes, ...actorIdentity },
          descriptor,
        ),
      /receipt mismatch/,
    );
  } finally {
    component.fill(0);
    descriptor.fill(0);
    actorBytes.fill(0);
    rmSync(root, { recursive: true, force: true });
  }
}

type BrowserActorGisProofV1 = Readonly<{ stalledBodyRetired: boolean; sourceDetached: boolean; transferCount: number; capacity: { actors: number; bytes: number } }>;

/** 🧵️ Proves one already verified GIS actor in a dedicated Chromium child without producing assets. */
export async function proveBrowserActorGisChildV1(
  repoRoot: string,
  work: string,
  actor: Readonly<Pick<ClosedBrowserActorArtifactV1, "bytes" | "sha256" | "byteLength" | "componentSha256"> & { descriptorByteSha256: string }>,
  descriptor: Uint8Array,
  control: FreshBuildControlV1,
): Promise<BrowserActorGisProofV1> {
  const check = (): void => {
    if (control.cancelled() || control.remainingMs() <= 0) throw new Error("GIS child proof cancelled");
  };
  check();
  if (!/^[a-f0-9]{64}$/u.test(actor.componentSha256) || actor.bytes.byteLength !== actor.byteLength || createHash("sha256").update(actor.bytes).digest("hex") !== actor.sha256 || createHash("sha256").update(descriptor).digest("hex") !== actor.descriptorByteSha256)
    throw new Error("GIS child retained byte identity differs");
  const ownerPath = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle";
  const fixture = JSON.parse(readFileSync(join(repoRoot, ownerPath, "🧾️describe/🧫️fixtures/🔣️.json"), "utf8"));
  const api = await import(join(repoRoot, ownerPath, "🧾️describe/🟦️.ts"));
  const codec = { encode: encodePackValue, decode: decodePackValue };
  api.assertBrowserActorDescribeCapacityV1(descriptor.byteLength);
  control.checkpoint("gis-child-browser", 0, 1);
  const { createServer } = await import("vite");
  const actorBytes = actor.bytes;
  const vite = await createServer({
    configFile: false,
    root: repoRoot,
    cacheDir: join(work, "vite"),
    clearScreen: false,
    optimizeDeps: { noDiscovery: true },
    server: { host: "127.0.0.1", port: await freeLoopbackPort(), strictPort: true, watch: null },
    plugins: [
      {
        name: "semio-real-gis-child-law",
        configureServer(server) {
          server.middlewares.use((request, response, next) => {
            if (request.url === "/__semio-gis-child.html") {
              response.setHeader("content-type", "text/html");
              response.end("<!doctype html><meta charset=utf-8><title>Real GIS Child</title>");
              return;
            }
            if (request.url === "/__semio-gis-child-stall") {
              response.writeHead(200, { "content-type": "application/octet-stream", "content-length": String(actorBytes.byteLength) });
              response.flushHeaders();
              response.write(Buffer.from(actorBytes.subarray(0, 1)));
              return;
            }
            if (request.url === "/__semio-gis-child-body") {
              response.setHeader("content-type", "application/octet-stream");
              response.setHeader("content-length", String(actorBytes.byteLength));
              response.end(Buffer.from(actorBytes.buffer, actorBytes.byteOffset, actorBytes.byteLength));
              return;
            }
            next();
          });
        },
      },
    ],
  });
  let browser: Awaited<ReturnType<(typeof import("playwright"))["chromium"]["launch"]>> | undefined;
  const diagnostics: string[] = [];
  let aborted = false;
  const cancellation = setInterval(() => {
    if (!aborted && (control.cancelled() || control.remainingMs() <= 0)) {
      aborted = true;
      void browser?.close().catch(() => {});
    }
  }, 50);
  try {
    check();
    await vite.listen();
    check();
    browser = await (await import("playwright")).chromium.launch({ headless: true, timeout: Math.max(1, Math.min(30_000, control.remainingMs())) });
    check();
    const page = await browser.newPage();
    page.on("console", (message) => diagnostics.push(message.type() + ":" + message.text()));
    page.on("pageerror", (error) => diagnostics.push("pageerror:" + error.message));
    await page.goto(vite.resolvedUrls!.local[0]! + "__semio-gis-child.html", { timeout: 15000 });
    const result = await page.evaluate(
      async ({ url, sha256, byteLength, transport }) => {
        const childApi = await import(/* @vite-ignore */ url);
        const fetchBody = async (path: string, signal: AbortSignal): Promise<ArrayBuffer> => {
          const response = await fetch(path, { signal, redirect: "error" });
          if (!response.ok || response.redirected || response.headers.get("content-length") !== String(byteLength)) {
            void response.body?.cancel().catch(() => {});
            throw new Error("GIS local transport headers");
          }
          const bytes = await response.arrayBuffer();
          if (bytes.byteLength !== byteLength) {
            new Uint8Array(bytes).fill(0);
            throw new Error("GIS local transport length");
          }
          return bytes;
        };
        const stalledAbort = new AbortController();
        const stalled = await childApi.reserveBrowserActorChild({ actorId: "real-gis-describe-law", activationGeneration: 1n, bundleSha256: sha256, bundleByteLength: byteLength }, stalledAbort.signal);
        const stalledTimer = setTimeout(() => stalledAbort.abort(), transport.stallMs);
        let unexpected: ArrayBuffer | undefined;
        try {
          unexpected = await fetchBody("/__semio-gis-child-stall", stalledAbort.signal);
          throw new Error("GIS stall unexpectedly completed");
        } catch (error) {
          if (!stalledAbort.signal.aborted) throw error;
        } finally {
          clearTimeout(stalledTimer);
          stalledAbort.abort();
          if (unexpected) new Uint8Array(unexpected).fill(0);
          stalled.close();
        }
        if (childApi.browserActorChildCapacity().actors !== 0 || childApi.browserActorChildCapacity().bytes !== 0) throw new Error("GIS stalled body retained child");
        const abort = new AbortController();
        let owner: any, timer: ReturnType<typeof setTimeout> | undefined;
        let guest: Uint8Array | undefined, source: ArrayBuffer | undefined;
        try {
          owner = await childApi.reserveBrowserActorChild({ actorId: "real-gis-describe-law", activationGeneration: 2n, bundleSha256: sha256, bundleByteLength: byteLength }, abort.signal);
          timer = setTimeout(() => abort.abort(), transport.bodyMs);
          const bytes = (source = await fetchBody("/__semio-gis-child-body", abort.signal));
          clearTimeout(timer);
          const digest = Array.from(new Uint8Array(await crypto.subtle.digest("SHA-256", bytes)), (byte) => byte.toString(16).padStart(2, "0")).join("");
          if (bytes.byteLength !== byteLength || digest !== sha256) throw new Error("real GIS browser byte identity");
          await owner.load(bytes);
          if (bytes.byteLength !== 0 || !owner.progress().sourceDetached) throw new Error("real GIS source not detached");
          const value = await owner.invoke(["describe", "describe"], []);
          if (!(value instanceof Uint8Array)) throw new Error("real GIS describe is not bytes");
          guest = value;
          const transferCount = owner.progress().resultTransfersDetached;
          if (transferCount !== 1) throw new Error("real GIS result not detached");
          owner.close();
          const capacity = childApi.browserActorChildCapacity();
          if (capacity.actors !== 0 || capacity.bytes !== 0) throw new Error("real GIS child capacity retained");
          return { guest: Array.from(guest), stalledBodyRetired: true, sourceDetached: true, transferCount, capacity };
        } finally {
          clearTimeout(timer);
          abort.abort();
          guest?.fill(0);
          if (source?.byteLength) new Uint8Array(source).fill(0);
          owner?.close();
        }
      },
      { url: "/@fs" + join(repoRoot, ownerPath, "🧵️child/🟦️.ts"), sha256: actor.sha256, byteLength: actor.byteLength, transport: fixture.localTransport },
    );
    const guest = new Uint8Array(result.guest);
    try {
      api.verifyBrowserActorDescribeV1(guest, descriptor, codec);
    } finally {
      guest.fill(0);
      result.guest.fill(0);
    }
    if (diagnostics.some((value) => value.startsWith("pageerror:"))) throw new Error("real GIS page error");
    check();
    control.checkpoint("gis-child-browser", 1, 1);
    return { stalledBodyRetired: result.stalledBodyRetired, sourceDetached: result.sourceDetached, transferCount: result.transferCount, capacity: result.capacity };
  } catch (error) {
    console.error("real-gis-child-diagnostics: " + diagnostics.slice(-16).join("|"));
    throw error;
  } finally {
    clearInterval(cancellation);
    try { await browser?.close(); } finally { await vite.close(); }
  }
}
