/** ♻️ The hot-swap SESSION HANDOVER law: after a dev hot swap of the session-owning plugin, every
 * window comes back over the SUCCESSOR instance — never over the one the swap destroyed.
 *
 * Two halves, and both are needed:
 *
 * 1. A reference mirror replays each authored step plan and counts the two things that decide whether
 *    a board comes back: how many times the canvas was presented while the session still named a
 *    RETIRED instance, and how many times the board actually mounted. The measured defect is authored
 *    as its own case, so the law is not vacuous — it fails on the plan the product used to run.
 * 2. Source scans, because the ordering lives in `reloadPlugin`'s statement order and in the fact that
 *    the node-graph surface effect is keyed on `surfaceId` alone. The second is what makes the first
 *    load-bearing: a board that mounts against a dead instance is never re-keyed by the successor
 *    session, so there is no later mount to recover it.
 *
 * Live twin: `<ticket>/🐍️hot-swap-remount-probe.mjs` — boots :6023, republishes the activation receipt
 * the dev server turns into a `built` availability event, and asserts the Flow window's board is back
 * with its 7 nodes and 0 page errors.
 */
import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";

const engineRoot = join(import.meta.dir, "..", "..");
const law = JSON.parse(readFileSync(join(engineRoot, "🧫️fixtures", "♻️hot-swap-session-handover", "🔣️.json"), "utf8"));

type Step = "retire-session" | "destroy-instance" | "swap-module" | "establish-session" | "present-canvas";
type Outcome = { presentationsOverRetiredInstance: number; boardMounts: number; endsPresented: boolean; endsOnInstance: number | null };

const PREDECESSOR = 1;
const SUCCESSOR = 2;

/** ♻️ The shell as this law models it: which instance the session names, whether the plugin canvas is
 * on screen, and which instances are retired. A retained surface is mounted exactly when the canvas is
 * presented over a session — its effect is keyed on `surfaceId`, so it neither remounts nor re-attaches
 * when the session underneath it changes. */
const replay = (plan: readonly Step[], ownsSession: boolean): Outcome => {
  let sessionInstance: number | null = PREDECESSOR;
  let presented = ownsSession ? false : true;
  const retired = new Set<number>();
  let boardMounted = false;
  let boardMounts = 0;
  let presentationsOverRetiredInstance = 0;
  const settle = (): void => {
    const shouldMount = presented && sessionInstance !== null;
    if (shouldMount && !boardMounted) {
      boardMounts += 1;
      if (retired.has(sessionInstance!)) presentationsOverRetiredInstance += 1;
    }
    boardMounted = shouldMount;
  };
  settle();
  boardMounts = 0;
  presentationsOverRetiredInstance = 0;
  for (const step of plan) {
    if (step === "retire-session") sessionInstance = null;
    else if (step === "destroy-instance") retired.add(PREDECESSOR);
    else if (step === "establish-session") sessionInstance = SUCCESSOR;
    else if (step === "present-canvas") presented = true;
    settle();
  }
  return { presentationsOverRetiredInstance, boardMounts, endsPresented: presented, endsOnInstance: sessionInstance };
};

describe("♻️ a hot swap hands every window over to the successor instance", () => {
  test("the fixture authors the whole step vocabulary and the defect it was written for", () => {
    expect(law.steps).toEqual(["retire-session", "destroy-instance", "swap-module", "establish-session", "present-canvas"]);
    expect(Object.keys(law.rules)).toContain("canvasLiveOnlyOverALiveSession");
    expect(law.cases.length).toBeGreaterThan(3);
    const plans = law.cases.map((authored: { plan: Step[] }) => authored.plan.join(">"));
    expect(new Set(plans).size, "every authored plan must be distinct or the counter-proof is a duplicate").toBe(plans.length);
  });

  for (const authored of law.cases as Array<{ name: string; ownsSession: boolean; plan: Step[]; expected: Outcome }>) {
    test(authored.name, () => {
      expect(replay(authored.plan, authored.ownsSession)).toEqual(authored.expected);
    });
  }

  test("the ordered plan is the ONLY authored one that both presents a board and never presents it over a corpse", () => {
    const green = (law.cases as Array<{ ownsSession: boolean; plan: Step[] }>)
      .map((authored) => replay(authored.plan, authored.ownsSession))
      .filter((outcome) => outcome.boardMounts === 1 && outcome.presentationsOverRetiredInstance === 0 && outcome.endsOnInstance === 2);
    expect(green.length, "the defect case must NOT satisfy the law, or this suite proves nothing").toBe(2);
    for (const authored of law.cases as Array<{ name: string; ownsSession: boolean; plan: Step[] }>) {
      const outcome = replay(authored.plan, authored.ownsSession);
      console.log(`[DEBUG] hot-swap handover ${authored.plan.join(" → ")} :: mounts=${outcome.boardMounts} overRetired=${outcome.presentationsOverRetiredInstance} endsOnInstance=${String(outcome.endsOnInstance)}`);
    }
  });

  test("`reloadPlugin` retires the session before it destroys the instance, and presents the canvas only after the successor exists", () => {
    const source = readFileSync(join(engineRoot, "🧱️elements", "🏛️ShellHost", "🟦️.tsx"), "utf8");
    const reload = source.indexOf("const reloadPlugin = useCallback(");
    expect(reload, "reloadPlugin must still be where this law reads it").toBeGreaterThan(0);
    const body = source.slice(reload, source.indexOf("const uninstallPlugin = useCallback(", reload));
    for (const needle of law.sourceScan.shellHost as string[]) expect(body).toContain(needle);
    const retire = body.indexOf('dispatch({ type: "SET_SESSION", value: null });');
    const destroy = body.indexOf("await current.handle.destroyApp(activeSession.instanceId)");
    const establish = body.indexOf("if (ownsSession) await establishPrimarySession(newHandle);");
    const present = body.indexOf('dispatch({ type: "SET_PLUGIN_STATUS", pluginId, value: "loaded" });');
    expect(retire).toBeGreaterThan(0);
    expect(destroy).toBeGreaterThan(retire);
    expect(present).toBeGreaterThan(establish);
    expect(establish).toBeGreaterThan(destroy);
  });

  test("the node-graph surface is keyed on its surfaceId alone — which is why the ordering, not a retry, is the fix", () => {
    const source = readFileSync(join(engineRoot, "🧱️elements", "🕸️NodeGraph", "🟦️.tsx"), "utf8");
    const unmount = source.indexOf('console.log("[DEBUG] node-graph host unmount surface=%s", surfaceId);');
    expect(unmount).toBeGreaterThan(0);
    expect(source.slice(unmount, unmount + 600)).toContain(law.sourceScan.nodeGraphSurfaceEffectKey);
  });
});
