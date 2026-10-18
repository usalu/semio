/** 🏭️ Couples generator contracts, their output roots and verifying targets to Build producers. */
import { resolve } from "node:path";

/** 🏷️ Names this component as its roster entry does. */
export const component = "generator-outputs";
let POLICY;

/** 🔌️ Binds this component's dependencies from the revision registry once its needs are wired. */
export function wire(registry) {
  ({ POLICY } = registry.coordinates);
}

/** 🧬️ A generator contract's own declared source inputs, plus the input-discovery fingerprint dependency it authors, if any. */
export function generatorContractInputs(contract) {
  const fingerprint = contract.inputDiscovery ? POLICY.generatorInputs[contract.inputDiscovery.kind] : undefined;
  if (contract.inputDiscovery && !fingerprint) throw new Error(`Generator input discovery has no fingerprint owner: ${contract.inputDiscovery.kind}`);
  return {
    inputs: [...contract.inputPatterns.map((path) => `{workspaceRoot}/${path}`), ...(fingerprint ? [{ dependentTasksOutputFiles: fingerprint.output }] : [])],
    dependsOn: fingerprint ? [fingerprint.target] : [],
  };
}

/** 🧬️ Assigns each output to its physical Build producer while retaining the generator's complete entry point. */
export function generatorOutputOwners(contracts, root, project, targets) {
  const outputs = new Map();
  for (const contract of Object.values(contracts)) {
    if (contract.ownership !== "owned") continue;
    if (contract.ownerPath === root) outputs.set(contract.target, outputs.get(contract.target) ?? []);
    for (const output of contract.outputRoots) {
      const owner = output.producer ?? contract;
      if (owner.ownerPath !== root) continue;
      if (!owner.target?.startsWith(`${project}:`) || !targets[owner.target.slice(project.length + 1)]) throw new Error(`Generator output has no Build producer: ${output.path}`);
      if (!outputs.has(owner.target)) outputs.set(owner.target, []);
      outputs.get(owner.target).push(`{workspaceRoot}/${output.path}`);
    }
  }
  return outputs;
}

/** 🌲️ Reads one root's current bytes into a stable digest, independently of directory-vs-file shape or a missing path. */
const OUTPUT_ROOT_DIGEST_SCRIPT = 'const c=require("crypto"),f=require("fs"),p=require("path"),r=process.argv[1],h=c.createHash("sha256");(function walk(x){if(!f.existsSync(x))return;const s=f.lstatSync(x);if(s.isDirectory())for(const e of f.readdirSync(x).sort())walk(p.join(x,e));else if(s.isFile()){h.update(p.relative(r,x).split(p.sep).join("/")+"\\0");h.update(f.readFileSync(x));}})(r);process.stdout.write(h.digest("hex"));';

/**
 * 🧮️ A committed generated output root as a live positive input. Nx's own fileset hashing only ever
 * sees files its native git-based walker can see, which silently excludes anything `.gitignore` covers —
 * exactly every `outputRoots` entry with `inclusion: "ignored"`. A `runtime` input instead spawns a fresh
 * read of the current bytes at every hash computation, so it is immune to that exclusion in either direction.
 */
export function outputRootInputs(output, workspaceRoot) {
  const absolute = resolve(workspaceRoot, output.path);
  return { runtime: `node -e ${JSON.stringify(OUTPUT_ROOT_DIGEST_SCRIPT)} ${JSON.stringify(absolute)}` };
}

/** 📍️ Resolves a declared `{projectRoot}`/`{workspaceRoot}` output string to its absolute path, or `undefined` for an unrecognized shape. */
export function resolveOutputPath(output, root, workspaceRoot) {
  if (!output.startsWith("{projectRoot}/") && !output.startsWith("{workspaceRoot}/")) return undefined;
  return resolve(workspaceRoot, output.replace("{projectRoot}", root).replace("{workspaceRoot}", "."));
}

/**
 * 🪞 A `check-*` target verifying a same-project `generate-*` producer's bytes still needs those CURRENT
 * bytes to invalidate its own cache, whether or not it also `dependsOn` the producer — `dependsOn` alone only
 * orders execution here (every authored instance in this repo names its target fully qualified, even for a
 * same-project producer) and never by itself feeds the producer's bytes into the hash; only a paired
 * `dependentTasksOutputFiles` input would, and this repo does not author that pairing for hand-written
 * `check-*`/`generate-*` pairs. `projectInputs` excludes every declared output from every named bucket
 * project-wide (`default`, `nativeSources`, …) to stop a producer from hashing its own freshly-written
 * output — but a proven Nx contract (see the `⚡️caching/🔁️verification/📋️orchestration/🟦️.ts` `CacheVerifyScript` fixture family) is
 * that once a bucket carrying that exclusion is also referenced, a plain positive glob for the same exact
 * path added elsewhere in the same `inputs` array is suppressed too; only a `runtime` digest (immune to
 * fileset inclusion/exclusion entirely) reliably restores visibility. Reused here uniformly for tracked and
 * gitignored outputs alike, exactly like `outputRootInputs`; deduped against whatever digest a generator
 * contract's own `checkTarget` wiring already added for the same absolute path.
 */
export function generatorOutputCouplingInputs(name, target, targets, root, workspaceRoot) {
  if (!/^check(?:-|$)/.test(name)) return [];
  const existing = new Set((target.inputs ?? []).filter((entry) => typeof entry === "object" && typeof entry?.runtime === "string").map((entry) => entry.runtime));
  const inputs = [];
  for (const [sibling, siblingTarget] of Object.entries(targets)) {
    if (sibling === name || !/^generate(?:-|$)/.test(sibling)) continue;
    for (const output of siblingTarget.outputs ?? []) {
      const absolute = resolveOutputPath(output, root, workspaceRoot);
      if (!absolute) continue;
      const runtime = `node -e ${JSON.stringify(OUTPUT_ROOT_DIGEST_SCRIPT)} ${JSON.stringify(absolute)}`;
      if (!existing.has(runtime)) inputs.push({ runtime });
    }
  }
  return inputs;
}
