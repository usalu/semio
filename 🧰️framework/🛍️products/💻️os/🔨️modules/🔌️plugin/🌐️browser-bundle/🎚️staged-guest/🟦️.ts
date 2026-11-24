import { ACTOR_COMPONENT_FUNCTIONS } from "../🛂️descriptor/🟦️.ts";

/** 🪜️ One budget stage of the actor world and the exports whose consuming calls it feeds. */
export interface StagedBudgetGroup {
  readonly stage: readonly [owner: string, name: string];
  readonly consumers: Readonly<Record<string, readonly string[]>>;
}

const STAGED_BUDGET_OWNERS = [
  { stage: ["reactor", "stageActorContextBudget"], owners: ["reactor", "checkpoint", "describe", "codec"] },
  { stage: ["jobs", "stageJobBudget"], owners: ["jobs"] },
] as const;

const UNBUDGETED_ASYNC_EXPORTS: Readonly<Record<string, readonly string[]>> = { reactor: ["stageCommandPage", "channelVersion"], jobs: ["takeSegmentedDownloadChunk"] };

/** 🧮️ Derives from the authored actor WIT which async exports consume a staged budget: every async function of a budgeted owner except the declared unbudgeted ones, behind the owner's sync stage export (`🔌️plugin/🧬️schema/📜️reactor.wit`, `📜️jobs.wit`). */
export const PLUGIN_STAGED_BUDGET_GROUPS: readonly StagedBudgetGroup[] = STAGED_BUDGET_OWNERS.map(({ stage: [owner, name], owners }) => {
  const stage = ACTOR_COMPONENT_FUNCTIONS[owner]?.find((method) => method.name === name);
  if (stage === undefined || stage.async) throw new Error(`Actor WIT must declare the sync budget stage ${owner}.${name}`);
  return { stage: [owner, name], consumers: Object.fromEntries(owners.map((consumer) => [consumer, ACTOR_COMPONENT_FUNCTIONS[consumer]!.filter((method) => method.async && !UNBUDGETED_ASYNC_EXPORTS[consumer]?.includes(method.name)).map((method) => method.name)])) };
});

/** 🎚️ Lowers the budget-taking domain guest surface (the budget is each consuming call's last argument) onto the actor world's exports: every async export takes at most 16 flat values, so the budget crosses through its sync stage export immediately before the consuming call. The caller serializes the calls of one instance, and the stage and the call are issued in one synchronous step, so they never interleave with another. */
export function stagedGuest(raw: any, groups: readonly StagedBudgetGroup[]): any {
  const guest: any = { ...raw };
  for (const { stage: [stageOwner, stageName], consumers } of groups) {
    const stage = raw[stageOwner]?.[stageName];
    for (const [owner, methods] of Object.entries(consumers)) {
      if (guest[owner] === undefined || guest[owner] === null) continue;
      guest[owner] = Object.fromEntries(Object.entries(guest[owner]).map(([name, call]: [string, any]) => [name, methods.includes(name) ? async (...args: any[]) => { stage(args.pop()); return call(...args); } : call]));
    }
  }
  return guest;
}

/** 🖨️ Emits the staged guest lowering into generated worker and bridge source as `pluginStagedGuest(raw)`. */
export function pluginStagedGuestSource(): string {
  return `const pluginStagedBudgetGroups = ${JSON.stringify(PLUGIN_STAGED_BUDGET_GROUPS)};\n${stagedGuest.toString()}\nconst pluginStagedGuest = (raw) => stagedGuest(raw, pluginStagedBudgetGroups);`;
}
