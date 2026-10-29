import { createHash, randomBytes } from "node:crypto";
import { lstatSync, mkdirSync, readFileSync, realpathSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { nativeCompilerEnvironmentV1, nativeCompilerSourceClosureV1, nativeCompilerSourcePlanV1, nativeCompilerShimSourceV1, selectNativeCompilerV1, systemNativeCompilerHostV1, type NativeCompilerContextV1, type NativeCompilerSelectionV1 } from "../../../../../../../../../🔨️modules/🏃️process/🧰️tools/🛠️native-compiler/🟦️.ts";
import { requireNativeStorageReservationV1, type NativeStorageReservationV1 } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import { requireNativeStoragePathV1 } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import { checkResourceLeaseControl, type ResourceLeaseControl } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";

type ProfileV1 = { live: boolean; alias: NativeCompilerSelectionV1; source: string; inputs: Awaited<ReturnType<typeof nativeCompilerSourceClosureV1>>; digest: string; control: ResourceLeaseControl; recheck(): Promise<void> };
const profiles = new WeakMap<NativeStorageReservationV1, ProfileV1>();
const hash = (value: Uint8Array | string): string => createHash("sha256").update(value).digest("hex");


/** 🛠️ Issues a content-addressed native driver only inside a live enforcing compiler store. */
export async function withRepositoryNativeCompilerV1<T>(storage: NativeStorageReservationV1, control: ResourceLeaseControl, environment: Readonly<Record<string, string | undefined>>, body: (environment: Readonly<Record<string, string | undefined>>) => Promise<T>): Promise<T> {
  requireNativeStorageReservationV1(storage, control);
  const selected = selectNativeCompilerV1(systemNativeCompilerHostV1(environment));
  if (!selected) return body(environment);
  await storage.recheck();
  const text = nativeCompilerShimSourceV1(selected), { roots, port } = nativeCompilerSourcePlanV1(selected, environment, [fileURLToPath(import.meta.url)]);
  const context = { check: () => checkResourceLeaseControl(control), yield: () => new Promise<void>(done => setImmediate(done)) };
  const inputs = await nativeCompilerSourceClosureV1(roots, context, port);
  const digest = hash(JSON.stringify({ version: 1, selected, text, inputs })), command = requireNativeStoragePathV1(storage.backing.root, join(storage.backing.root, "n", digest, "cc"));
  await storage.recheck();
  mkdirSync(dirname(command), { recursive: true });
  let current: string | undefined;
  try { current = readFileSync(command, "utf8"); } catch {}
  if (current !== undefined && current !== text) throw Error("Original native compiler driver differs from its source identity");
  if (current === undefined) {
    const temporary = command + "." + randomBytes(6).toString("hex");
    try { writeFileSync(temporary, text, { flag: "wx", mode: 0o555 }); await storage.recheck(); renameSync(temporary, command); } finally { rmSync(temporary, { force: true }); }
  }
  const profile: ProfileV1 = { live: true, alias: Object.freeze({ ...selected, command }), source: text, inputs, digest, control, recheck: async () => {
    if (!profile.live || profiles.get(storage) !== profile) throw Error("Original native compiler scope required");
    requireNativeStorageReservationV1(storage, control);
    await storage.recheck();
    const status = lstatSync(command);
    if (!status.isFile() || status.isSymbolicLink() || status.nlink !== 1 || (status.mode & 0o111) === 0 || (status.mode & 0o222) !== 0 || readFileSync(command, "utf8") !== text) throw Error("Original native compiler driver changed");
    if (JSON.stringify(await nativeCompilerSourceClosureV1(roots, context, port)) !== JSON.stringify(inputs)) throw Error("Original native compiler source changed");
  } };
  if (profiles.has(storage)) throw Error("One original native compiler scope required");
  profiles.set(storage, profile);
  try { await profile.recheck(); return await body(environment); }
  finally { profile.live = false; profiles.delete(storage); }
}

/** 🔐️ Receives the live store's driver before the immutable configuration and source plan are captured. */
export async function repositoryNativeCompilerConfigurationV1(storage: NativeStorageReservationV1, environment: Readonly<Record<string, string | undefined>>, context: NativeCompilerContextV1): Promise<Readonly<{ environment: Readonly<Record<string, string | undefined>>; identity: unknown; recheck(): Promise<void> }>> {
  const profile = profiles.get(storage);
  if (!profile) return { environment, identity: null, recheck: async () => {} };
  await profile.recheck();
  const projected = nativeCompilerEnvironmentV1(environment, profile.alias, context);
  if (projected === environment) return { environment, identity: null, recheck: async () => {} };
  return Object.freeze({ environment: projected, identity: Object.freeze({ version: 1, digest: profile.digest, command: profile.alias.command, inputs: profile.inputs, environment: { CC: projected.CC, HOST_CC: projected.HOST_CC, PATH: projected.PATH } }), recheck: () => profile.recheck() });
}
