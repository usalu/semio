import { createHash } from "node:crypto";
import { mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { libraryBootstrap, readBuildOwnerRegistrySourceV1 } from "../🟨️.mjs";
import { exportProjectRecordV1, observeInferenceSnapshotV1 } from "../../../👁️observation/🟨️.mjs";
import { withBuildGraphStagingV1 } from "../../../📥️inputs/💾️storage/🟨️.mjs";
import { createProjectInferenceEnvironmentV1, ownerJobV1, projectGeneratorContractsV1 } from "../📦️project-nodes/🟨️.mjs";
import { nativeCommandInputs } from "../🧭️script-inputs/🟨️.mjs";
import * as service from "../📦️project-nodes/🏊️project-farm/👷️service/🟨️.mjs";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

const POLICY = schema.definitions.Policy.const, LATE_COMMAND_RECIPE_V1 = Object.freeze({ fn: "native-command" });

/** 🧭️ The project roots a farm child maps to owner names: the root of every owner and the native root a project declares in its metadata (a project whose Cargo crate lives below it), as the full read builds them; a crate dependency that only the native root names is otherwise refused as "no build project owner". */
export function projectRootsOfV1(workspaceRoot, source) {
  const rows = [];
  for (const owner of source.owners) {
    rows.push([owner.root, owner.name]);
    let text; try { text = readFileSync(join(workspaceRoot, owner.root, "📋️project.json"), "utf8"); } catch { continue; }
    if (!text.includes("\"nativeRoot\"")) continue;
    try { const nativeRoot = JSON.parse(text).metadata?.nativeRoot; if (typeof nativeRoot === "string" && nativeRoot !== owner.root) rows.push([nativeRoot, owner.name]); } catch {}
  }
  return rows;
}

/** 🔬️ Opens the session of a farm child for one workspace (the registry, the generator contracts, the command inputs and the project roots of the registry; staging in a private directory, so no custody of the graph store is needed) and answers an `infer` (`late` opens the session of a generation build: symbolic command closures in a private closure store) that runs ONE owner in it and reports what it cost: wall and processor milliseconds, the managed memory the process holds afterwards, the rows and bytes of the record. `selected` infers it the way a selected read does (the targets and inputs of the admission only), otherwise in full; the process keeps its memos between owners exactly as a child does, so the first owner pays the cold walk and the next ones show the steady state. `trim(budgetBytes)` drops the retained memos when they exceed the budget, as a child does after every job.
 * @see ./🧬️schema/🔣️.json
 * @see ./🧪️tests/🟦️.ts */
export async function openOwnerHarnessV1({ workspaceRoot, registry, late = false, analysisStore, stagingParent = realpathSync(tmpdir()) }) {
  await libraryBootstrap;
  const source = registry ?? JSON.parse(readBuildOwnerRegistrySourceV1(workspaceRoot)), base = mkdtempSync(join(realpathSync(stagingParent), POLICY.stagingPrefix));
  const closureStore = late ? join(base, "closures") : undefined;
  if (late) mkdirSync(closureStore, { recursive: true });
  await service.open({ version: 3, workspaceRoot, closureStore, registry: source, contracts: projectGeneratorContractsV1(workspaceRoot), commandInputs: late ? { closureRecipe: LATE_COMMAND_RECIPE_V1 } : null, lateClosures: late, analysisStore, projectsByRoot: projectRootsOfV1(workspaceRoot, source), componentRoots: [], stagingBase: base, authority: undefined }, { slot: 0, generation: 1 });
  const environment = service.sessionEnvironmentV1();
  if (!late) environment.commandInputs = await observeInferenceSnapshotV1(() => nativeCommandInputs(workspaceRoot, environment.scripts));
  const globals = { registryDigest: POLICY.registryDigest, commandRevision: POLICY.commandRevision, late };
  return Object.freeze({
    environment,
    async infer(root, { selected } = {}) {
      const json = { ...JSON.parse(readFileSync(join(workspaceRoot, root, "📋️project.json"), "utf8")), root }, job = ownerJobV1(environment, globals, { name: json.name, json, ...selected === undefined ? {} : { admission: { targets: selected.targets, inputs: selected.inputs ?? [] } } });
      const started = performance.now(), cpu = process.cpuUsage();
      await service.run({ ...job, ...selected === undefined ? {} : { admission: true } });
      const used = process.cpuUsage(cpu), wall = performance.now() - started, cachePath = join(service.stagingDirectoryV1(), "inference", "np", job.cacheKey + ".json");
      const record = withBuildGraphStagingV1({ root: service.stagingDirectoryV1() }, () => exportProjectRecordV1(cachePath));
      globalThis.Bun?.gc?.(true);
      const held = process.memoryUsage(), project = record === undefined ? {} : JSON.parse(record.image);
      return Object.freeze({ name: json.name, root, native: job.native, mode: selected === undefined ? "full" : "selected", wallMilliseconds: Math.round(wall), cpuMilliseconds: Math.round((used.user + used.system) / 1000), targets: Object.keys(project.targets ?? {}), rows: record?.observations.length ?? 0, recordBytes: record === undefined ? 0 : record.image.length, managedMegabytes: Math.round((held.heapUsed + held.external) / 1048576), digest: createHash("sha256").update(record?.image ?? "").digest("hex") });
    },
    trim: budgetBytes => service.trimRetainedV1(budgetBytes),
    close() { rmSync(base, { recursive: true, force: true }); },
  });
}
