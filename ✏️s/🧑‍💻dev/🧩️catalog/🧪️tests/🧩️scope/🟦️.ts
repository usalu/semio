import assert from "node:assert/strict";
import { readdirSync, readFileSync, statSync } from "node:fs";
import { join, resolve } from "node:path";
import Ajv from "ajv";
import TOML from "@iarna/toml";
export const workspace = resolve(import.meta.dirname, "../../../../..");
export const installedScope = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
const schema = JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json", import.meta.url), "utf8"));
assert(new Ajv({ strict: true }).compile(schema)(installedScope));
export const hostProjection = JSON.parse(readFileSync(join(workspace, installedScope.hostFixturePath), "utf8"));
/** 🧮️ Independently reads current authored source identities through the locked TOML oracle. */
export async function sourceDeploymentOwnersV1(control: { signal: AbortSignal; progress(completed: number): void }): Promise<string[]> {
 const policy = installedScope.sourceCensus, pending = [...policy.roots], owners: string[] = [], started = Date.now(), excluded = new Set(["node_modules", "target", ".git", ".venv", "__pycache__", ".nx", "🗑️generated", "🤖️generated", "🔌️plugin-modules", "🧩️extension-modules", "⚡️cache", "dist"]); let files = 0, work = 0;
 while (pending.length) {
  control.signal.throwIfAborted(); assert(Date.now() - started < policy.maxMs);
  const directory = pending.pop()!;
  for (const entry of readdirSync(join(workspace, directory), { withFileTypes: true })) {
   if (excluded.has(entry.name)) continue;
   control.signal.throwIfAborted(); assert(Date.now() - started < policy.maxMs); assert(++work <= policy.maxWork);
   const path = join(directory, entry.name);
   assert(!entry.isSymbolicLink());
   if (entry.isDirectory()) { pending.push(path); continue; }
   if (!entry.isFile()) continue;
   if (entry.name === "Cargo.toml") {
    assert(++files <= policy.maxFiles);
    assert(statSync(join(workspace, path)).size <= policy.maxBytes);
    const metadata = (TOML.parse(readFileSync(join(workspace, path), "utf8")) as any).package?.metadata;
    if (policy.componentKinds.includes(metadata?.semio?.["component-kind"]) && metadata.semio[policy.deploymentField] !== undefined) {
     assert.equal(typeof metadata.semio[policy.deploymentField], "string"); assert.equal(typeof metadata.component?.package, "string"); assert(metadata.component.package.startsWith("semio:")); owners.push(metadata.component.package.slice(6));
    }
   }
   if (work % 4096 === 0) { control.progress(work); await new Promise<void>(resolve => setTimeout(resolve, 0)); }
  }
 }
 assert.equal(new Set(owners).size, owners.length); control.progress(files); return owners.sort();
}
