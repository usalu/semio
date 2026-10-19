/** 🧰️ The shared frame Worker step budget fixtures: the scheduler harness helpers the budget tests use. */
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { TurnClock, TurnLedger } from "@semio-tech/framework-renderer-wgpu/turn-budget";
import { FrameTurnScheduler } from "@semio-tech/framework-renderer-wgpu/frame-turn-scheduler";
import { admitPluginCatalogV1, pluginCatalogV1 } from "../../../../../🔌️plugin/📇️registry/🟦️.ts";
import catalogFixture from "@semio-tech/plugin-registry/testing/fixtures/composition.json";

export const catalog = pluginCatalogV1(admitPluginCatalogV1(catalogFixture.cases[1].input, { maxBytes: 2097152, maxRows: 128, maxEdges: 4096, maxWork: 65536, deadlineMs: performance.now() + 30000, now: () => performance.now(), cancelled: () => false, progress: () => {} }));

export const ENGINE_ROOT = join(dirname(fileURLToPath(import.meta.url)), "..", "..", "..");

export const FRAME_WORKER_TS = join(ENGINE_ROOT, "🎯️targets", "🧊️wgpu", "🎞️frame-worker", "🟦️.ts");

export const FRAME_JOB_RS = join(ENGINE_ROOT, "🎯️targets", "🧊️wgpu", "🧵️frame-job", "🦀️.rs");

export const BROWSER_WORKER_RS = join(ENGINE_ROOT, "🎯️targets", "🧊️wgpu", "🌐️browser-worker", "🦀️.rs");

export const RENDERER_RS = join(ENGINE_ROOT, "🎯️targets", "🧊️wgpu", "🧊️renderer", "🦀️.rs");

export const FRAME_TURN_FIXTURE = join(ENGINE_ROOT, "🧪️tests/🧫️fixtures", "🧵️frame-turn-scheduling", "🔣️.json");

export const COMPONENT_CLOSE_TURN_FIXTURE = join(ENGINE_ROOT, "🧪️tests/🧫️fixtures", "🧵️component-close-frame-turn", "🔣️.json");

export const FRAME_TURN_SCHEDULER_TS = join(ENGINE_ROOT, "🎯️targets", "🧊️wgpu", "🧵️frame-turn-scheduler", "🟦️.ts");

export type WorkerTurnOwner = "frame" | "assetDecode";

export type TwoKindFrameTurnScheduler = {
  request(owner?: WorkerTurnOwner): void;
  requestRuntimeWake(): void;
  beginClose(): void;
  terminalIsEmpty(): boolean;
};

export const TwoKindFrameTurnScheduler = FrameTurnScheduler as unknown as new (
  schedule: (callback: () => void) => void,
  frameStep: () => boolean,
  closeStep: () => boolean,
  assetDecodeStep: () => boolean,
) => TwoKindFrameTurnScheduler;

/** 🔥️ A genuinely EXECUTING span — the only thing the ceiling is allowed to charge for. */
export function spinMs(milliseconds: number): void {
  const until = performance.now() + milliseconds;
  while (performance.now() < until) {}
}

/** 🧵️ One frame-Worker boot step, driven the way `🎞️frame-worker/🟦️.ts` drives one: priced on the
 * executing clock, admitted to the ledger, yielded after when the ledger says the Worker is running long,
 * and NEVER able to end the boot. */
export async function workerBootStep<T>(ledger: TurnLedger, clock: TurnClock, stage: string, run: () => T, yields: string[]): Promise<T> {
  clock.enter();
  let value: T;
  try {
    value = run();
  } finally {
    ledger.admit(stage, clock.leave());
  }
  if (ledger.degraded()) {
    yields.push(stage);
    await new Promise<void>((resolve) => setTimeout(resolve, 0));
  }
  return value;
}
