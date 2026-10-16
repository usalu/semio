/** 🧩️ The contract a plugin scenario fulfils to run on the time-travel harness: its route, its steps and the hooks the harness calls around them. */
import { context } from "../🧠️state/🟦️.ts";
import type { Renderer, Locale } from "../🧠️state/🟦️.ts";

/** 🚧️ A prerequisite the scenario cannot meet: every step but the `exempt` ones fails with the verdict `name` naming the `prerequisite`. */
export type TimeTravelBlockV1 = Readonly<{ name: string; prerequisite: string; exempt: readonly number[] }>;

/** 👣️ One scenario step; the method form keeps a step over a narrower context assignable to the harness's. */
export type TimeTravelStepV1<Context extends object = object> = { run(context: Context): Promise<void> }["run"];

/** 🧩️ What a plugin supplies to the time-travel harness: the playground it drives, where its serve listens, the steps of its journey and the hooks the harness calls around them. The harness owns the CLI, the browser, the report and the plugin-neutral universal step. */
export type TimeTravelScenarioV1<Context extends object = object> = Readonly<{
  variant: string;
  defaultServes: Readonly<Record<Renderer, string>>;
  bootWindowPrefix: string;
  editorNumbers: readonly string[];
  paletteQuery: Readonly<Record<Locale, string>>;
  stepNumbers: readonly number[];
  order: readonly number[];
  steps: Readonly<Record<number, TimeTravelStepV1<Context>>>;
  context(): Context;
  prepare(context: Context): Promise<TimeTravelBlockV1 | null>;
  beforeStep(step: number, context: Context, blocked: TimeTravelBlockV1 | null): Promise<void>;
  afterSteps(context: Context, blocked: TimeTravelBlockV1 | null): Promise<void>;
  diagnostics(): Promise<Record<string, unknown>>;
  exploreInventory(): Promise<Record<string, unknown>>;
}>;
