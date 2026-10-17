/** 🧰️ The shared fixtures of the ticket-owned browser host staging suite: the namespace bag of every owner module the cases drive, the source location, and the helpers several case groups use. */
import { atTestLevel } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import * as Fs from "node:fs";
import * as Crypto from "node:crypto";
import * as Events from "node:events";
import * as Os from "node:os";
import * as Path from "node:path";
import * as Url from "node:url";
import * as Owner01 from "../../../🎮️playground-session/🏃️execution/🟦️.ts";
import * as Owner02 from "../../../../🔌️plugin/📇️registry/🔄️refresh/🟦️.ts";
import * as Owner03 from "../../../../🔌️plugin/🏗️build/📋️plan/🟦️.ts";
import * as Owner04 from "../../../../🔌️plugin/🏗️build/📦️materialization/🟦️.ts";
import * as Owner05 from "../../../../🔌️plugin/🏗️build/🛂️descriptor/🟦️.ts";
import * as Owner06 from "../../../../🔌️plugin/🏗️build/📥️installation/🟦️.ts";
import * as Owner07 from "../../../../🔌️plugin/🏗️build/🏃️execution/🟦️.ts";
import * as Owner08 from "../../../../🔌️plugin/🏗️build/👁️watch/🟦️.ts";
import * as Owner09 from "../../../../🔌️plugin/📊️size/🟦️.ts";
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
import * as Owner50 from "../../../../🔌️plugin/🖨️describe/🧩️component-stage/🟦️.ts";
import * as Owner51 from "../../../../🔌️plugin/🖨️describe/🏷️component-receipt/🟦️.ts";
import * as Owner52 from "../../../../🔌️plugin/🖨️describe/🏗️component-build/🟦️.ts";
import * as External01 from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import * as External02 from "../../../../🔌️plugin/🌐️browser-bundle/🛂️descriptor/🟦️.ts";
import * as External03 from "../../../../🔌️plugin/🌐️browser-bundle/📦️distribution/📋️inventory/🟦️.ts";
import * as External04 from "../../../♻️activation/🟦️.ts";
import * as External06 from "../../../🔌️vite-plugins/🟦️.ts";
import * as External16 from "../../../🔌️vite-plugins/🗄️backbone-storage/🟦️.ts";
import * as External07 from "../../../../🔌️plugin/📇️registry/📖️catalog-view/🟦️.ts";
import * as External08 from "../../../../🔌️plugin/📇️registry/🔎️discovery/🟦️.ts";
import * as External09 from "../../../../🔌️plugin/📇️registry/🎮️playground/🧭️session/🟦️.ts";
import * as External10 from "../../../../🔌️plugin/📇️registry/🟦️.ts";
import * as External11 from "../../../../🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts";
import * as External12 from "../../../../🔌️plugin/🏪️store/📥️installation/🟦️.ts";
import * as External13 from "../../../../🔌️plugin/📇️registry/📦️deployment/🟦️.ts";
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
export const testDependencies = { ...Fs, ...Crypto, ...Events, ...Os, ...Url, ...Path, ...Owner01, ...Owner02, ...Owner03, ...Owner04, ...Owner05, ...Owner06, ...Owner07, ...Owner08, ...Owner09, ...Owner11, ...Owner12, ...Owner13, ...Owner14, ...Owner15, ...Owner16, ...Owner17, ...Owner18, ...Owner19, ...Owner20, ...Owner22, ...Owner23, ...Owner24, ...Owner25, ...Owner26, ...Owner28, ...Owner29, ...Owner30, ...Owner31, ...Owner32, ...Owner33, ...Owner34, ...Owner35, ...Owner36, ...Owner37, ...Owner38, ...Owner39, ...Owner40, ...Owner41, ...Owner42, ...Owner43, ...Owner44, ...Owner45, ...Owner46, ...Owner47, ...Owner48, ...Owner49, ...Owner50, ...Owner51, ...Owner52, ...External01, ...External02, ...External03, ...External04, ...External06, ...External16, ...External07, ...External08, ...External09, ...External10, ...External11, ...External12, ...External13, ...External14, ...External15, buildPluginCatalog: Owner02.buildPluginCatalog, repoRoot: External01.getWorkspaceRoot() };

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

  return { contractModuleExport, devContractModule, itLong };
}
