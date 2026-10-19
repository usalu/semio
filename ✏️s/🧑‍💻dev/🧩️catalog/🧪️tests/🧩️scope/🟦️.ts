import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { readdir, readFile, stat } from "node:fs/promises";
import { join, resolve } from "node:path";
import Ajv from "ajv";
import TOML from "@iarna/toml";
import { parseGeneratedCatalogProjectionV1 } from "@semio-tech/plugin-registry/catalog-view";
export const workspace = resolve(import.meta.dirname, "../../../../..");
export const installedScope = JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
const schema = JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json", import.meta.url), "utf8"));
assert(Boolean(new Ajv({ strict: true }).compile(schema)(installedScope)));
export const hostProjection = parseGeneratedCatalogProjectionV1(JSON.parse(readFileSync(join(workspace, installedScope.hostFixturePath), "utf8")));
/** 🧮️ Independently reads current authored source identities through the locked TOML oracle, enumerating the directory tree with `sourceCensus.concurrency` overlapping reads because a walk is bound by filesystem latency, not by CPU. */
export async function sourceDeploymentOwnersV1(control: { signal: AbortSignal; progress(completed: number): void }): Promise<string[]> {
 const policy = installedScope.sourceCensus, pending: string[] = [...policy.roots], owners: string[] = [], started = Date.now(), excluded = new Set(["node_modules", "target", ".git", ".venv", "__pycache__", ".nx", "🗑️generated", "🤖️generated", "🔌️plugin-modules", "🧩️extension-modules", "⚡️cache", "dist"]); let files = 0, work = 0, active = 0, failed = false;
 const admit = (): void => { control.signal.throwIfAborted(); assert(Date.now() - started < policy.maxMs); };
 const read = async (path: string): Promise<void> => {
  assert(++files <= policy.maxFiles);
  assert((await stat(join(workspace, path))).size <= policy.maxBytes);
  const metadata = (TOML.parse(await readFile(join(workspace, path), "utf8")) as any).package?.metadata;
  if (policy.componentKinds.includes(metadata?.semio?.["component-kind"]) && metadata.semio[policy.deploymentField] !== undefined) {
   assert.equal(typeof metadata.semio[policy.deploymentField], "string"); assert.equal(typeof metadata.component?.package, "string"); assert(metadata.component.package.startsWith("semio:")); owners.push(metadata.component.package.slice(6));
  }
 };
 const walk = async (directory: string): Promise<void> => {
  const manifests: Promise<void>[] = [];
  for (const entry of await readdir(join(workspace, directory), { withFileTypes: true })) {
   if (excluded.has(entry.name)) continue;
   admit(); assert(++work <= policy.maxWork);
   const path = join(directory, entry.name);
   assert(!entry.isSymbolicLink());
   if (entry.isDirectory()) pending.push(path);
   else if (entry.isFile() && entry.name === "Cargo.toml") manifests.push(read(path));
   if (work % 4096 === 0) control.progress(work);
  }
  await Promise.all(manifests);
 };
 await new Promise<void>((resolve, reject) => {
  const pump = (): void => {
   if (failed) return;
   while (active < policy.concurrency && pending.length) { active += 1; walk(pending.pop()!).then(() => { active -= 1; pump(); }, error => { failed = true; reject(error); }); }
   if (!active && !pending.length) resolve();
  };
  pump();
 });
 assert.equal(new Set(owners).size, owners.length); control.progress(files); return owners.sort();
}
