/** 🔌️ Adapter context
 *
 * Committed artifacts, adapter registration and the context handed to adapters.
 */

//#region 🔌️Adapters
import { type AdapterContext, type TestCasePlan, type FeatureScenario, type TestRole, type AdapterOutcome, type TestAdapter } from "@semio-tech/framework/test/adapter";
import { constants, cpSync, mkdirSync, readFileSync } from "node:fs";
import { basename, dirname, isAbsolute, join } from "node:path";
import { requireIssuedResourceControlInvocationV1, type ScriptInvocation } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type { ResourceLeaseControl } from "@semio-tech/process/leases";
//#endregion 🔌️Adapters

//#region 🧭️Adapters
/** 🧭️ Context one scenario handler receives — its plan slice, resolved fixtures and its work directory. */


/** 🧭️ What a scenario handler returns: the raw artifact plus the projection the profile compares. */


/** 🧭️ One implementation's registration for a case: which scenarios it serves, in which roles. */


/** 🧭️ Declares a TypeScript adapter. The coordinator validates the registration against the feature. */


/** 🧭️ Checks a registration against the plan: no unknown scenario, no unregistered scenario. */
export function validateRegistration(plan: TestCasePlan, adapter: TestAdapter, role: TestRole): string[] {
  const problems: string[] = [];
  const planned = new Set(plan.scenarios.flatMap((scenario) => (scenario.outlineOf === undefined ? [scenario.id] : [scenario.id, scenario.outlineOf])));
  for (const id of Object.keys(adapter.scenarios)) if (!planned.has(id)) problems.push(`adapter registers unknown scenario ${JSON.stringify(id)}`);
  for (const scenario of plan.scenarios) {
    if (registeredHandler(adapter, scenario, role) === undefined) problems.push(`adapter has no ${role} registration for scenario ${JSON.stringify(scenario.id)}`);
  }
  return problems;
}

/**
 * 🪆️ The handler for one scenario and role: the one registered under the scenario's own id, else the
 * one registered under its Scenario Outline's base id (`@id-<base>`), which serves every row the
 * feature expands — so an adapter never mirrors a catalog's kind list by hand.
 */
export function registeredHandler(adapter: TestAdapter, scenario: Pick<FeatureScenario, "id" | "outlineOf">, role: TestRole): ((ctx: AdapterContext) => AdapterOutcome | Promise<AdapterOutcome>) | undefined {
  return adapter.scenarios[scenario.id]?.[role] ?? (scenario.outlineOf === undefined ? undefined : adapter.scenarios[scenario.outlineOf]?.[role]);
}

/** 🧭️ Builds the context handed to one scenario handler, wiring fixture resolution and the work dir. */
export function makeAdapterContext(repoRoot: string, plan: TestCasePlan, scenario: FeatureScenario, role: TestRole, invocation: ScriptInvocation, phaseControl: ResourceLeaseControl): AdapterContext {
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);
  const check=()=>requireIssuedResourceControlInvocationV1(phaseControl,invocation);
  const workDir = isAbsolute(plan.workDir) ? plan.workDir : join(repoRoot, plan.workDir);
  mkdirSync(workDir, { recursive: true });
  const lookup = (uri: string): string => {
    check();
    const fixture = plan.inputs.find((entry) => entry.uri === uri);
    if (fixture === undefined) throw new Error(`fixture ${uri} is not part of this plan — declare it in the feature file`);
    return join(repoRoot, fixture.path);
  };
  const artifactDir = plan.artifactDir === undefined ? join(workDir, "📦️artifacts") : isAbsolute(plan.artifactDir) ? plan.artifactDir : join(repoRoot, plan.artifactDir);
  mkdirSync(artifactDir, { recursive: true });
  return Object.freeze<AdapterContext>({
    invocation,
    phaseControl,
    plan,
    scenario,
    role,
    repoRoot,
    workDir,
    artifactDir,
    artifact: (role_, filename) => {
      check();
      const target = join(artifactDir, scenario.id, role_, filename);
      mkdirSync(dirname(target), { recursive: true });
      return target;
    },
    seed: scenario.seed ?? "0",
    row: () => {
      check();
      if (scenario.outlineOf === undefined || !scenario.id.startsWith(`${scenario.outlineOf}-`)) throw new Error(`scenario ${scenario.id} expands no Scenario Outline row`);
      return scenario.id.slice(scenario.outlineOf.length + 1);
    },
    input: lookup,
    inputBytes: (uri) => readFileSync(lookup(uri)),
    subjectRawBytes: (implementation) => {
      check();
      const path = plan.subjectRawInputs?.[scenario.id]?.[implementation];
      if (path === undefined) throw new Error(`scenario ${scenario.id} has no raw subject output from ${implementation}; run its subject phase before this byte-decoding oracle`);
      return readFileSync(path);
    },
    copyInput: (uri, as) => {
      const source = lookup(uri);
      const target = join(workDir, as ?? basename(source));
      mkdirSync(dirname(target), { recursive: true });
      cpSync(source, target, { mode: constants.COPYFILE_FICLONE });
      return target;
    },
  });
}
//#endregion 🧭️Adapters
