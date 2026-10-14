import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve, dirname } from "node:path";
import ts from "typescript";
import Ajv from "ajv";
import schema from "../../📇️descriptor/📦️package/🧬️schema/🔣️.json";
import { readPackageReceiptIdentity, type PackageReceiptSource } from "../../📇️descriptor/📦️package/🟦️.ts";
import fixture from "../🧫️fixtures/📦️receipt/🔣️.json";
import textSchema from "../../📇️descriptor/📥️text/🧬️schema/🔣️.json";
import textFixture from "../🧫️fixtures/📥️descriptor-text/🔣️.json";
import { readDescriptorText } from "../../📇️descriptor/📥️text/🟦️.ts";
import iconv from "iconv-lite";

test("the general kernel imports no specific receipt implementation", () => {
  const path = resolve(import.meta.dir, "../../🟦️.ts"), source = readFileSync(path, "utf8"), parsed = ts.createSourceFile(path, source, ts.ScriptTarget.Latest, true);
  const imports: string[] = [];
  const visit = (node: ts.Node): void => {
    if ((ts.isImportDeclaration(node) || ts.isExportDeclaration(node)) && node.moduleSpecifier && ts.isStringLiteral(node.moduleSpecifier)) imports.push(node.moduleSpecifier.text);
    if (ts.isCallExpression(node) && node.expression.kind === ts.SyntaxKind.ImportKeyword && node.arguments[0] && ts.isStringLiteral(node.arguments[0])) imports.push(node.arguments[0].text);
    ts.forEachChild(node, visit);
  };
  visit(parsed);
  const oracle = ts.preProcessFile(source, true, true).importedFiles.map(row => row.fileName);
  expect(imports).toEqual(oracle);
  for (const imported of imports.filter(value => value.startsWith("."))) expect(resolve(dirname(path), imported).includes("/🛍️products/")).toBe(false);
});

test("descriptor text bounds every arriving chunk and releases its stream", async () => {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(textSchema);
  for (const vector of textFixture.refusals) {
    expect(validate(vector.policy)).toBe(true);
    let index = 0, cancelled = false;
    const response = new Response(new ReadableStream<Uint8Array>({ pull(stream) { if (index < vector.chunks.length) stream.enqueue(Uint8Array.from(vector.chunks[index++]!)); else stream.close(); }, cancel() { cancelled = true; } }, { highWaterMark: 0 }));
    await expect(readDescriptorText(response, () => {}, { ...vector.policy, version: 1 }, undefined)).rejects.toThrow(/descriptor text exceeds/u);
    expect(index).toBe(vector.consumedChunks);
    expect(cancelled).toBe(true);
    expect(response.body!.locked).toBe(false);
  }
  for (const vector of textFixture.success) {
    expect(validate(vector.policy)).toBe(true);
    const bytes = vector.chunks.map(chunk => Uint8Array.from(chunk));
    let index = 0, progress = 0;
    const response = new Response(new ReadableStream<Uint8Array>({ pull(stream) { if (index < bytes.length) stream.enqueue(bytes[index++]!); else stream.close(); } }, { highWaterMark: 0 }));
    const oracle = iconv.decode(Buffer.concat(bytes.map(chunk => Buffer.from(chunk))), "utf8");
    expect(oracle).toBe(vector.text);
    expect(await readDescriptorText(response, () => { progress++; }, { ...vector.policy, version: 1 }, undefined)).toBe(oracle);
    expect(progress).toBe(vector.chunks.length);
    expect(response.body!.locked).toBe(false);
  }
  for (const policy of textFixture.invalidPolicies) {
    expect(validate(policy)).toBe(false);
    const response = new Response("a");
    await expect(Reflect.apply(readDescriptorText, undefined, [response, () => {}, policy, undefined])).rejects.toThrow(/descriptor text policy/u);
    expect(response.body!.locked).toBe(false);
  }
  console.log("[DEBUG] Whole descriptor stream byte/code-unit vectors agree with iconv-lite and Ajv; overflowing input stops at its first offending chunk");
});

test("descriptor stream cancellation and rejected asynchronous progress retain original reasons", async () => {
  const policy = { ...textFixture.success[0]!.policy, version: 1 as const };
  const refused = Error("original asynchronous progress refusal");
  let cancelled: unknown;
  let delivered = false;
  const response = new Response(new ReadableStream<Uint8Array>({ pull(stream) { if (delivered) stream.close(); else { delivered = true; stream.enqueue(Uint8Array.from([65])); } }, cancel(reason) { cancelled = reason; } }, { highWaterMark: 0 }));
  const rejectedProgress = Promise.reject(refused);
  void rejectedProgress.catch(() => {});
  await expect(readDescriptorText(response, () => rejectedProgress, policy, undefined)).rejects.toBe(refused);
  expect(cancelled).toBe(refused);
  expect(response.body!.locked).toBe(false);
  const controller = new AbortController(), reason = Error("original stream cancellation");
  const pending = new Response(new ReadableStream<Uint8Array>({ cancel(value) { cancelled = value; } }, { highWaterMark: 0 }));
  const reading = readDescriptorText(pending, () => {}, policy, controller.signal);
  controller.abort(reason);
  await expect(reading).rejects.toBe(reason);
  expect(cancelled).toBe(reason);
  expect(pending.body!.locked).toBe(false);
  console.log("[DEBUG] Original cancellation and asynchronous progress refusal cancel and unlock the actual response stream");
});

test("descriptor cancellation interrupts a pending progress publication", async () => {
  const policy = { ...textFixture.success[0]!.policy, version: 1 as const };
  const controller = new AbortController(), reason = Error("original cancellation during awaited progress");
  let enterProgress!: () => void, releaseProgress!: () => void, cancelled: unknown, progressReleased = false;
  const entered = new Promise<void>(resolve => { enterProgress = resolve; });
  const progress = new Promise<void>(resolve => { releaseProgress = resolve; });
  const response = new Response(new ReadableStream<Uint8Array>({ pull(stream) { stream.enqueue(Uint8Array.from([65])); }, cancel(value) { cancelled = value; } }, { highWaterMark: 0 }));
  const reading = readDescriptorText(response, () => { enterProgress(); return progress; }, policy, controller.signal);
  await entered;
  controller.abort(reason);
  const release = setTimeout(() => { progressReleased = true; releaseProgress(); }, textFixture.pendingProgressReleaseMilliseconds);
  try {
    await expect(reading).rejects.toBe(reason);
    expect(progressReleased).toBe(false);
    expect(cancelled).toBe(reason);
    expect(response.body!.locked).toBe(false);
    console.log("[DEBUG] Original cancellation interrupts pending progress publication and releases the response reader");
  } finally {
    clearTimeout(release);
    releaseProgress();
    await reading.catch(() => {});
  }
});

test("descriptor header refusal cancels its unread response body", async () => {
  const { fetchDescriptorManifest } = await import("../../🟦️.ts");
  const reason = Error(textFixture.headerRefusalMessage), originalFetch = globalThis.fetch;
  let cancelled: unknown;
  const response = new Response(new ReadableStream<Uint8Array>({ cancel(value) { cancelled = value; } }, { highWaterMark: 0 }));
  globalThis.fetch = Object.assign(async () => response, { preconnect: originalFetch.preconnect });
  try {
    await expect(fetchDescriptorManifest(fixture.pluginId, "/future/module.js", undefined, () => { throw reason; })).rejects.toBe(reason);
    expect(cancelled).toBe(reason);
    expect(response.body!.locked).toBe(false);
    console.log("[DEBUG] Original descriptor header publication refusal cancels the unread body");
  } finally { globalThis.fetch = originalFetch; }
});

test("an explicit future receipt source binds package identity with the original signal and progress", async () => {
  const { fetchPackageDescriptor, SemioFaultError } = await import("../../🟦️.ts");
  const fetchDescriptor = fetchPackageDescriptor;
  const controller = new AbortController(), originalFetch = globalThis.fetch, moduleUrl = "/future/module.js", requested: string[] = [];
  let reads = 0, progress = 0;
  const { refusals, invalidIdentities, expectedFault, ...rawIdentity } = fixture;
  const identity = readPackageReceiptIdentity(rawIdentity);
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  expect(validate(identity)).toBe(true);
  expect(JSON.stringify(identity)).toBe(JSON.stringify(rawIdentity));
  for (const mutation of invalidIdentities) {
    const invalid = { ...identity, ...mutation };
    expect(validate(invalid)).toBe(false);
    expect(() => readPackageReceiptIdentity(invalid)).toThrow();
  }
  globalThis.fetch = Object.assign(async (input: Parameters<typeof fetch>[0]) => {
    requested.push(String(input));
    return new Response(JSON.stringify({ packageId: fixture.packageId, manifest: { pluginId: fixture.pluginId, apps: [] } }));
  }, { preconnect: originalFetch.preconnect });
  const heartbeat = () => { progress++; };
  const source: PackageReceiptSource = { read: async request => {
    expect(request.pluginId).toBe(fixture.pluginId);
    expect(request.moduleUrl).toBe(moduleUrl);
    expect(request.signal).toBe(controller.signal);
    expect(request.onProgress).toBe(heartbeat);
    await request.onProgress();
    reads++;
    return identity;
  } };
  try {
    const descriptor = await fetchDescriptor(fixture.pluginId, moduleUrl, source, controller.signal, heartbeat);
    expect(descriptor.packageId).toBe(fixture.packageId);
    expect(descriptor.componentSha256).toBe(fixture.componentSha256);
    expect(descriptor.manifest.pluginId).toBe(fixture.pluginId);
    expect(requested).toEqual(["/future/🔣️.json"]);
    expect(reads).toBe(1);
    expect(progress).toBeGreaterThanOrEqual(2);
    for (const refusal of refusals) {
      const refused = fetchDescriptor(fixture.pluginId, moduleUrl, { read: async () => ({ ...identity, ...refusal }) }, controller.signal, heartbeat);
      await expect(refused).rejects.toThrow();
      await refused.catch((error: unknown) => {
        if (!(error instanceof SemioFaultError)) throw error;
        expect(String(error.fault.origin)).toBe(expectedFault.origin);
        expect(error.fault.code).toBe(expectedFault.code);
      });
    }
    const reason = Error("original cancellation");
    controller.abort(reason);
    await expect(fetchDescriptor(fixture.pluginId, moduleUrl, source, controller.signal, heartbeat)).rejects.toBe(reason);
    expect(reads).toBe(1);
    console.log("[DEBUG] Neutral receipt source identity, original signal/progress, foreign refusal and cancellation checked");
  } finally { globalThis.fetch = originalFetch; }
});
