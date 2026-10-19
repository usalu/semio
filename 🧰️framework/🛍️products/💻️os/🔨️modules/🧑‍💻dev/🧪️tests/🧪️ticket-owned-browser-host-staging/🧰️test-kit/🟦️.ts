/** 🧰️ The shared fixtures of the ticket-owned browser host staging suite: the namespace bag of every owner module the cases drive, the source location, and the helpers several case groups use. */
import { atTestLevel } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { fixtureOriginalWorkStock } from "@semio-tech/framework-actor/retained-turn/bootstrap/testing/fixtures";
import { spawnSync } from "node:child_process";
import * as Vm from "node:vm";
import { APP_CHANNEL_VERSION } from "@semio-tech/framework-os";
import * as Fs from "node:fs";
import * as Crypto from "node:crypto";
import * as Events from "node:events";
import * as Os from "node:os";
import * as Path from "node:path";
import * as Url from "node:url";
import * as Owner01 from "../../../🎮️playground-session/🏃️execution/🟦️.ts";
import * as Owner02 from "@semio-tech/plugin-registry/refresh";
import * as Owner03 from "@semio-tech/framework-plugin/build/plan";
import * as Owner04 from "@semio-tech/framework-plugin/build/materialization";
import * as Owner05 from "@semio-tech/framework-plugin/build/descriptor";
import * as Owner06 from "@semio-tech/framework-plugin/build/installation";
import * as Owner07 from "@semio-tech/framework-plugin/build/execution";
import * as Owner08 from "@semio-tech/framework-plugin/build/watch";
import * as Owner09 from "@semio-tech/framework-plugin/size";
import * as Owner11 from "../../../⚙️engine/🧭️selection/🟦️.ts";
import * as Owner12 from "../../../⚙️engine/📤️publication/🟦️.ts";
import * as Owner13 from "../../../♻️activation/🔐️lease/🟦️.ts";
import * as Owner14 from "../../../♻️activation/🩺️readiness/🟦️.ts";
import * as Owner15 from "../../../♻️activation/📥️installation/🟦️.ts";
import * as Owner16 from "../../../♻️activation/🔍️freshness/🟦️.ts";
import * as Owner17 from "../../../♻️activation/🧰️preparation/🟦️.ts";
import * as Owner18 from "../../../♻️activation/🏃️execution/🟦️.ts";
import * as Owner19 from "../../../♻️activation/🌐️serve/🟦️.ts";
import * as Owner20 from "../../🧹️capability-policy/🟦️.ts";
import * as Owner22 from "../../🧹️export-path-policy/🟦️.ts";
import * as Owner23 from "../../🧹️host-handle-policy/🟦️.ts";
import * as Owner24 from "../../🏃️execution/🟦️.ts";
import * as Owner25 from "../../🎬️studio/🟦️.ts";
import * as Owner26 from "../../🔬️catalog-smoke/🟦️.ts";
import * as Owner28 from "../../✅️verification/🟦️.ts";
import * as Owner29 from "../../../⚖️parity/🏗️structure/🟦️.ts";
import * as Owner30 from "../../../⚖️parity/🖼️pixels/🟦️.ts";
import * as Owner31 from "../../../⚖️parity/🔬️probe/🟦️.ts";
import * as Owner32 from "../../../⚖️parity/🌐️server-pool/🟦️.ts";
import * as Owner33 from "../../../⚖️parity/📊️report/🟦️.ts";
import * as Owner34 from "../../../⚖️parity/🏃️execution/🟦️.ts";
import * as Owner35 from "../../../🚚️distribution/📥️source/🟦️.ts";
import * as Owner36 from "../../../🚚️distribution/📋️plan/🟦️.ts";
import * as Owner37 from "../../../🚚️distribution/🏗️compiler/🟦️.ts";
import * as Owner38 from "../../../🚚️distribution/📤️publication/🟦️.ts";
import * as Owner39 from "../../../🚚️distribution/🔍️freshness/🟦️.ts";
import * as Owner40 from "../../../🚚️distribution/🏃️execution/🟦️.ts";
import * as Owner41 from "@semio-tech/framework-os/testing/scale/projection";
import * as Owner42 from "@semio-tech/framework-os/testing/scale/publication";
import * as Owner43 from "../../../📊️benchmarks/🔌️plugins/📋️plan/🟦️.ts";
import * as Owner44 from "../../../📊️benchmarks/🔌️plugins/🖥️host/🟦️.ts";
import * as Owner45 from "../../../📊️benchmarks/🔌️plugins/🌐️browser/🟦️.ts";
import * as Owner46 from "../../../📊️benchmarks/🔌️plugins/📏️measured-row/🟦️.ts";
import * as Owner47 from "../../../📊️benchmarks/🔌️plugins/🏃️execution/🟦️.ts";
import * as Owner48 from "../../../🧬️schema/🛂️validation/🟦️.ts";
import * as Owner49 from "../../📇️canonical-bootstrap-folder-mirror/🟦️.ts";
import * as Owner50 from "@semio-tech/framework-plugin/describe/component-stage";
import * as Owner51 from "@semio-tech/framework-plugin/describe/component-receipt";
import * as Owner52 from "@semio-tech/framework-plugin/describe/component-build";
import { cargoProfileDir } from "@semio-tech/process/execution";
import { findWorkspaceRoot } from "@semio-tech/process/routing";
import * as External02 from "@semio-tech/framework-plugin/browser-bundle/descriptor";
import * as External03 from "@semio-tech/framework-plugin/browser-bundle/distribution/inventory";
import * as External04 from "../../../♻️activation/🟦️.ts";
import * as External06 from "../../../🔌️vite-plugins/🟦️.ts";
import * as External16 from "../../../🔌️vite-plugins/🗄️backbone-storage/🟦️.ts";
import * as External07 from "@semio-tech/plugin-registry/catalog-view";
import * as External08 from "@semio-tech/plugin-registry/discovery";
import * as External09 from "@semio-tech/plugin-registry/playground/session";
import * as External10 from "../../../../🔌️plugin/📇️registry/🟦️.ts";
import * as External11 from "@semio-tech/framework-plugin/browser-bundle/materialization";
import * as External17 from "@semio-tech/framework-plugin/browser-bundle/materialization/contract";
import * as External18 from "@semio-tech/framework-plugin/browser-bundle/materialization/guestslim-fonts";
import * as External19 from "@semio-tech/framework-plugin/browser-bundle/materialization/preview2-shim-patches";
import * as External20 from "@semio-tech/framework-plugin/browser-bundle/materialization/shard-worker-source";
import * as External21 from "@semio-tech/framework-plugin/browser-bundle/materialization/component-bridge-source";
import * as External22 from "@semio-tech/framework-plugin/browser-bundle/materialization/wasm-opt";
import * as External23 from "@semio-tech/framework-plugin/browser-bundle/materialization/jco-module-rewrites";
import * as External24 from "@semio-tech/framework-plugin/browser-bundle/materialization/host-shim-source";
import * as External25 from "@semio-tech/framework-plugin/browser-bundle/imports";
import * as External12 from "../../../../🔌️plugin/🏪️store/📥️installation/🟦️.ts";
import * as External13 from "@semio-tech/plugin-registry/deployment";
import * as External14 from "../../../🚚️distribution/🟦️.ts";
import * as External15 from "@semio-tech/framework-os";

export type TestSource = { readonly directory: string; readonly url: string };
/** 🧰️ One namespace bag, typed by an object literal rather than `Object.assign`: past three sources
 * `Object.assign` falls back to its `...sources: any[]` overload and the whole bag degrades to `any`,
 * which silently untyped every case below. `node:url` is spread BEFORE `node:path` because both export
 * `resolve`, and every call site here means the variadic path one, not `url.resolve(from, to)`.
 * `buildPluginCatalog` is pinned for the same reason: `📇️registry/🟦️.ts` exports a zero-argument
 * projection builder under that same name and spreads later, so only the pin reaches the
 * `📇️registry/🔄️refresh` catalog BUILD these cases drive. */
export const testDependencies = { ...Fs, ...Crypto, ...Events, ...Os, ...Url, ...Path, ...Owner01, ...Owner02, ...Owner03, ...Owner04, ...Owner05, ...Owner06, ...Owner07, ...Owner08, ...Owner09, ...Owner11, ...Owner12, ...Owner13, ...Owner14, ...Owner15, ...Owner16, ...Owner17, ...Owner18, ...Owner19, ...Owner20, ...Owner22, ...Owner23, ...Owner24, ...Owner25, ...Owner26, ...Owner28, ...Owner29, ...Owner30, ...Owner31, ...Owner32, ...Owner33, ...Owner34, ...Owner35, ...Owner36, ...Owner37, ...Owner38, ...Owner39, ...Owner40, ...Owner41, ...Owner42, ...Owner43, ...Owner44, ...Owner45, ...Owner46, ...Owner47, ...Owner48, ...Owner49, ...Owner50, ...Owner51, ...Owner52, ...{ cargoProfileDir, getWorkspaceRoot: findWorkspaceRoot }, ...External02, ...External03, ...External04, ...External06, ...External16, ...External07, ...External08, ...External09, ...External10, ...External11, ...External17, ...External18, ...External19, ...External20, ...External21, ...External22, ...External23, ...External24, ...External25, ...External12, ...External13, ...External14, ...External15, buildPluginCatalog: Owner02.buildPluginCatalog, repoRoot: findWorkspaceRoot() };

export type TestDependencies = typeof testDependencies;

export type ContractValidator = { (value: unknown): boolean; errors?: unknown };

export function createTestKit(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: TestDependencies, source: TestSource) {
  const { dirname, fileURLToPath, join, readFileSync } = dependencies;
  const { it } = vitest;

  const contractModuleExport = async (modulePath: string, exportId: string): Promise<ContractValidator> => {
    const { default: AjvDraft7 } = await import("ajv");
    const document = JSON.parse(readFileSync(join(dirname(fileURLToPath(source.url)), modulePath), "utf8"));
    const ajv = new AjvDraft7({ strict: true, allErrors: true });
    ajv.addSchema(document);
    return ajv.getSchema(`${document.$id}#/$defs/${exportId}`) as ContractValidator;
  };
  const devContractModule = (exportId: string): Promise<ContractValidator> => contractModuleExport("../../🧬️schema/🔣️.json", exportId);
  /** ⏱️Cases that drive a real Rollup/Vite build, a strict-TypeScript program, a detached `node`/`bun`
   * child, or a Wasm instantiation — each measured well past a second on 2026-09-05, together far past the
   * 30 s `quick` wall-clock budget. They stay in the suite and run from `test long` upwards; `test quick`
   * keeps every pure-helper case. */
  const itLong = atTestLevel(it, "long");

  /** 🥟️ Evaluates one module script on Bun and answers its single JSON result: the code under test is Bun-hosted (`bun:sqlite`, `Bun.Transpiler`, `Bun.resolveSync`), so a Node-hosted suite drives it in a Bun child, the way the bridge cases drive a Node one. */
  const underBun = <T>(script: string, ...args: readonly string[]): T => {
    const result = spawnSync(process.versions.bun ? process.execPath : "bun", ["-e", script, ...args], { encoding: "utf8", timeout: 60_000, maxBuffer: 16 * 1024 * 1024 });
    if (result.status !== 0) throw new Error(`bun child failed (${result.status}): ${result.stderr || result.stdout}`);
    return JSON.parse(result.stdout) as T;
  };
  /** 🧭️ A dynamic `import()` that the test runner's module graph never sees: the generated modules identify an activation by the query of their own `import.meta.url`, which the runner's loader strips, so only the platform loader keeps their identity. */
  const nativeImport = Vm.runInThisContext("(url) => import(url)", { importModuleDynamically: Vm.constants.USE_MAIN_CONTEXT_DEFAULT_LOADER }) as (url: string) => Promise<unknown>;
  /** 🌉️ Writes the actual generated bridge, host shim and a guest module into one directory and loads the bridge there, so a case drives the real `createActorApi` against a guest that answers the retained original-turn protocol (`channelVersion`, `admitActorContext`, `closeActorContext`, `poll` with a retained receipt). Every activation takes its own actor id because the host shim binds one clock per actor/generation module. */
  const generatedBridge = async (guestSource: string, hostShim: string = dependencies.hostShimSource()) => {
    const directory = dependencies.mkdtempSync(join(dependencies.tmpdir(), "semio-generated-bridge-"));
    dependencies.writeFileSync(join(directory, "component.js"), guestSource);
    dependencies.writeFileSync(join(directory, dependencies.PLUGIN_HOST_SHIM_FILE), hostShim);
    dependencies.writeFileSync(join(directory, "bridge.mjs"), dependencies.pluginComponentBridgeSource("component", "component.core.wasm"));
    const { createActorApi } = (await nativeImport(dependencies.pathToFileURL(join(directory, "bridge.mjs")).href)) as { createActorApi: (actorId: string, generation: bigint, admission: unknown, clock: () => bigint, original: { receipt(value: unknown): void; hold(value: unknown): void }) => Promise<any> };
    const held: unknown[] = [], receipts: unknown[] = [];
    let sequence = 0, now = 1_000n;
    const activate = async (generation: bigint, actorId = `actor-${++sequence}`): Promise<any> => createActorApi(actorId, generation, await fixtureOriginalWorkStock().admit(), () => now++, { receipt: (value) => receipts.push(value), hold: (value) => held.push(value) });
    return { directory, activate, held, receipts, dispose: () => dependencies.rmSync(directory, { recursive: true, force: true }) };
  };
  /** 🛡️ A guest module whose `reactor` answers the retained original-turn protocol with zero spend; its default `poll` answers what the case supplies through `globalThis.__bridgeNext`, and a case may replace the poll body (it sees `events`, `grant` and `receipt`), add imports before the module (`prelude`) and members after the protocol (`extra`). */
  const retainedGuestSource = (options: { readonly prelude?: string; readonly poll?: string; readonly extra?: string } = {}): string => `${options.prelude ?? ""}
const zero = { copiedItems: 0n, copiedBytes: 0n, retainedCapacityBytes: 0n, releasedBytes: 0n };
const receipt = (retained) => ({ input: retained, spent: zero });
export const reactor = {
  channelVersion: async () => ${APP_CHANNEL_VERSION},
  admitActorContext: async (retained) => ({ ...receipt(retained), ready: true, blocked: false, complete: false, refusal: undefined }),
  closeActorContext: async (retained) => ({ ...receipt(retained), ready: true, blocked: false, complete: true, refusal: undefined }),
  poll: async (events, grant) => { ${options.poll ?? "globalThis.__bridgePolled?.(events, grant); return { retainedReceipt: receipt(grant.retained), fuelUsed: 0n, uiPatches: [], status: { tag: \"idle\" }, commandIngress: { kind: 0 }, ...globalThis.__bridgeNext };"} },
  ${options.extra ?? ""}
};
export const jobs = {};
export const checkpoint = {};
export const describe = {};
`;
  return { contractModuleExport, devContractModule, itLong, underBun, generatedBridge, retainedGuestSource };
}
