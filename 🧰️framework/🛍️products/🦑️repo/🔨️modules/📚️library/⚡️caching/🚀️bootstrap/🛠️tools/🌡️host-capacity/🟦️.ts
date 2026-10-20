import type {HostPlanV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import type {WorkerMappingV1} from "../../../../🏗️build/👷️workers/🟦️.ts";

/** 🧰️ What the host capacity plan becomes in Nx: environment variables and a report of how exactly each contract field is honoured. */
export type NxHostResourceSettingsV1=Readonly<{environment:Readonly<Record<"NX_PARALLEL",string>>;mapping:readonly WorkerMappingV1[]}>;

/** 🗺️ Renders the plan into Nx: its only resource knob is the number of concurrent tasks, so the granted jobs become `NX_PARALLEL`; the memory and core budgets hold because the grant sized the jobs by the per-job estimates of the requested targets. */
export function nxHostResourceSettingsV1(plan:HostPlanV1):NxHostResourceSettingsV1{
 const mapping:WorkerMappingV1[]=[
  {field:"jobs",support:"exact",setting:`NX_PARALLEL=${plan.jobs}`},
  {field:"cpu",support:"approximate",setting:`bounded by NX_PARALLEL=${plan.jobs}; Nx has no core budget`},
  {field:"memoryMegabytes",support:"approximate",setting:`bounded by NX_PARALLEL=${plan.jobs}; Nx has no memory budget`},
  {field:"fixedMegabytes",support:"unsupported",setting:"held by the invocation outside Nx"},
  {field:"floorMegabytes",support:"unsupported",setting:"enforced by admission before Nx starts"},
 ];
 return Object.freeze({environment:Object.freeze({NX_PARALLEL:String(plan.jobs)}),mapping:Object.freeze(mapping)});
}
