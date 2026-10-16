import{validateJsonSchemaSubset}from"../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};

/** 📦️ One planned component package and its manifest. */
export type NativeBuildPackageV1 = Readonly<{ name: string; manifest: string }>;

/** 🛣️ One Cargo pass declares packages, profile and target; actual generation custody owns its staging. */
export type NativeBuildPassV1 = Readonly<{ id: string; manifest: string; packages: readonly NativeBuildPackageV1[]; target: string; profile: string }>;

/** 🏃️ A command the driver runs under its jobserver once every completion it names happened (`JobV1`); a job with a `lane` runs inside that profile's Cargo lane before its passes. */
export type NativeBuildJobV1 = Readonly<{ id: string; cwd: string; command: readonly string[]; after: readonly string[]; profile: string; target: string; workerProtocol: "jobserver" | "reserved" | "exclusive"; lane?: string; environment?: Readonly<Record<string, string>> }>;

/** 🧾️ One job of a product's native build pipeline declaration (`PipelineJobV1`). */
export type NativeBuildPipelineJobV1 = Readonly<{ id: string; root: string; command: readonly string[]; after: readonly string[]; lane?: string; profile: string; target: string; workerProtocol: "jobserver" | "reserved" | "exclusive"; policies: readonly ("process" | "cargo" | "wasm")[] }>;

/** 🗂️ A product's native build pipeline declaration (`PipelineV1`): Nx prerequisites and the jobs the driver runs beside the component passes. */
export type NativeBuildPipelineV1 = Readonly<{ prerequisites: readonly string[]; jobs: readonly NativeBuildPipelineJobV1[]; stageAfter?: Readonly<Record<string, readonly string[]>> }>;

/** 🗺️ Every Cargo pass and job one native build driver owns (`NativeBuildPlanV1`). */
export type NativeBuildPlanV1 = Readonly<{ version: 1; passes: readonly NativeBuildPassV1[]; jobs?: readonly NativeBuildJobV1[]; stageAfter?: Readonly<Record<string, readonly string[]>> }>;

/** 🌡️ A warm generation reuses the shared compiler store and stage cache; a fresh one compiles cold into a new generation directory with its own stage cache. */
export type NativeBuildGenerationKindV1 = "warm" | "fresh";

/** 🗂️ Where one driver run compiles, logs, caches stages and reports. */
export type NativeBuildGenerationV1 = Readonly<{ kind: NativeBuildGenerationKindV1; root: string; target: string; build: string; report: string; logs: string; jobserver: string; stages: string }>;

/** 🛂️ Admits a native build pipeline declaration against the first-party schema, with unique job ids and lane jobs named only once. */
export function admitNativeBuildPipelineV1(value: unknown): NativeBuildPipelineV1 {
  const errors = validateJsonSchemaSubset(schema.definitions.PipelineV1, value, schema);
  if (errors.length) throw Error(`Invalid native build pipeline: ${errors.join("; ")}`);
  const pipeline = value as NativeBuildPipelineV1;
  if (new Set(pipeline.jobs.map((job) => job.id)).size !== pipeline.jobs.length) throw Error("Invalid native build pipeline: duplicate job id");
  return pipeline;
}

/** 🛂️ Admits only a plan that satisfies the first-party schema and names every pass, package and job once. */
export function admitNativeBuildPlanV1(value: unknown): NativeBuildPlanV1 {
  const errors = validateJsonSchemaSubset(schema, value);
  if (errors.length) throw Error(`Invalid native build plan: ${errors.join("; ")}`);
  const plan = value as NativeBuildPlanV1;
  if (new Set(plan.passes.map((pass) => pass.id)).size !== plan.passes.length) throw Error("Invalid native build plan: duplicate pass id");
  for (const pass of plan.passes) if (new Set(pass.packages.map((row) => row.name)).size !== pass.packages.length) throw Error(`Invalid native build plan: duplicate package in ${pass.id}`);
  if (new Set((plan.jobs ?? []).map((job) => job.id)).size !== (plan.jobs ?? []).length) throw Error("Invalid native build plan: duplicate job id");
  return plan;
}
