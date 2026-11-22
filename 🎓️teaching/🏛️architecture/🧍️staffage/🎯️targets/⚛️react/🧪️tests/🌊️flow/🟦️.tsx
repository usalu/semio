/** 🌊️ The air flow on screen: the layer code of the address round-trips, a clock time maps to a transport time that never runs backwards, the solver protocol reports progress, answers with the field of a direct solve, cancels and resumes, the figure takes the paper its flow needs, mounts the flow layer for the thermal view only and says what is shown.
 * @see ./🥒️.feature — the scenarios */
import { cleanup, render, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { StaffageFigure } from "@teaching/staffage-react";
import { defaultDepictionOptions, depict, evaluate, flowDomainOf, solveTransport, type FlowField } from "@teaching/staffage";
import { ALL_FLOW_LAYERS, SHOWN_FLOW_LAYERS, FLOW_DEVELOP_S, flowFrameOf, flowLayersCode, flowLayersFrom, flowPeriodS, flowTimeS, flowsIn, unionFrame } from "../../🔨️modules/🌊️flow/🟦️.ts";
import { FlowSession } from "../../🔨️modules/🌊️flow/📡️session/🟦️.ts";
import { FlowServer, type FlowReply } from "../../🔨️modules/🌊️flow/🧵️server/🟦️.ts";
import { DEFAULT_VIEWER, viewerFromParams, viewerToParams } from "../../🔨️modules/🗺️routing/🟦️.ts";
import { sceneExtent } from "../../🔨️modules/🧮️pipeline/🟦️.ts";
import { walker } from "../🧰️support/🧰️.ts";

afterEach(cleanup);

const stand = { kind: "pose", pose: "stand" } as const;
const walk = { kind: "clip", clip: "walk", timeS: 0 } as const;
const environment = walker.thermal.environment;
const settle = (): Promise<void> => new Promise((resolve) => setTimeout(resolve, 0));
async function until(done: () => boolean): Promise<void> {
  for (let i = 0; i < 200000 && !done(); i++) await settle();
  expect(done()).toBe(true);
}

describe("🌊️ flow rules", () => {
  it("round-trips the layers through the address and writes nothing for the default", () => {
    expect(flowLayersCode(SHOWN_FLOW_LAYERS)).toBeNull();
    expect(flowLayersCode(ALL_FLOW_LAYERS)).toBe("s,t,m,f,p");
    expect(SHOWN_FLOW_LAYERS).not.toContain("pathlines");
    expect(viewerToParams(DEFAULT_VIEWER).has("fl")).toBe(false);
    for (const layers of [[], ["streams"], ["tracers", "fog"], ["streams", "moisture", "fog"]] as const) {
      const params = viewerToParams({ ...DEFAULT_VIEWER, flow: layers });
      expect(viewerFromParams(new URLSearchParams(params.toString())).flow).toEqual(layers);
    }
    expect(flowLayersFrom("0")).toEqual([]);
    expect(flowLayersFrom("x,t")).toEqual(["tracers"]);
    expect(flowLayersFrom(null)).toEqual(SHOWN_FLOW_LAYERS);
  });

  it("shows air flow only in the thermal view of an asset that moves air", () => {
    expect(flowsIn(walker, "thermal")).toBe(true);
    expect(flowsIn(walker, "forces")).toBe(false);
    expect(flowsIn({ ...walker, thermal: { ...walker.thermal, emitters: [] } }, "thermal")).toBe(false);
  });

  it("maps clock time to a transport time that never runs backwards and keeps the loop phase", () => {
    expect(flowTimeS(stand, 0, 3, 0, null)).toBe(FLOW_DEVELOP_S);
    expect(flowTimeS(stand, 0, 0.2, 7.5, 20)).toBe(FLOW_DEVELOP_S + 7.5);
    const period = flowPeriodS(walker, walk);
    expect(period).toBe(1);
    let last: number | null = null;
    for (const clock of [0, 0.25, 0.75, 0.125, 0.5, 0.5, 0.875, 0]) {
      const t = flowTimeS(walk, period, clock, 0, last);
      expect(t).toBeGreaterThanOrEqual(FLOW_DEVELOP_S);
      if (last !== null) expect(t).toBeGreaterThanOrEqual(last);
      expect(((t - clock) / period) % 1).toBeCloseTo(0, 9);
      last = t;
    }
    expect(flowTimeS({ kind: "clip", clip: "once", timeS: 0 }, 0, 2.5, 0, 9)).toBe(2.5);
  });

  it("gives the figure the paper its flow domain needs", () => {
    const options = defaultDepictionOptions("thermal", "en", 100);
    const state = evaluate(walker, stand, options.representation);
    const scene = depict(state, options);
    const domain = flowDomainOf(state.bounds, walker.dimensions.widthM / 2);
    const frame = flowFrameOf(state, "elevation", scene.metadata.mmPerMetre);
    expect(frame.minX).toBeCloseTo(domain.min.x * 10, 9);
    expect(frame.maxX).toBeCloseTo(domain.max.x * 10, 9);
    expect(frame.minY).toBeCloseTo(-domain.max.y * 10, 9);
    expect(frame.maxY).toBeCloseTo(0, 9);
    const plan = flowFrameOf(state, "plan", scene.metadata.mmPerMetre);
    expect(plan.maxY - plan.minY).toBeCloseTo((domain.max.z - domain.min.z) * 10, 9);
    const both = unionFrame(sceneExtent(scene), frame);
    expect(both.minX).toBeLessThanOrEqual(Math.min(frame.minX, sceneExtent(scene).minX));
    expect(both.maxY).toBeGreaterThanOrEqual(sceneExtent(scene).maxY);
  });
});

describe("🧵️ flow solver protocol", () => {
  const open = (replies: FlowReply[]): FlowServer => {
    const server = new FlowServer((reply) => replies.push(reply));
    server.post({ kind: "asset", id: 1, asset: walker });
    server.post({ kind: "open", session: 7, asset: 1, query: stand, environment, conditions: { maxAgeS: 2 }, periodS: 0, pathlines: false });
    return server;
  };

  it("reports progress and answers with the field of a direct solve, then resumes forward", async () => {
    const replies: FlowReply[] = [];
    const server = open(replies);
    server.post({ kind: "frame", session: 7, timeS: 2 });
    await until(() => replies.some((reply) => reply.kind === "flow"));
    const progress = replies.filter((reply) => reply.kind === "progress") as Extract<FlowReply, { kind: "progress" }>[];
    expect(progress.length).toBeGreaterThan(2);
    expect(progress.every((reply, i) => reply.total === progress[0]!.total && (i === 0 || reply.done > progress[i - 1]!.done))).toBe(true);
    const flow = (replies.find((reply) => reply.kind === "flow") as Extract<FlowReply, { kind: "flow" }>).flow;
    const direct = solveTransport(walker, { ...stand, timeS: 2 }, environment, { maxAgeS: 2, pathlines: false, flow: true }).flow!;
    expect(JSON.stringify(flow)).toBe(JSON.stringify(direct));
    expect(flow.ambient.airTempC).toBe(environment.airTempC);
    replies.length = 0;
    server.post({ kind: "frame", session: 7, timeS: 2.125 });
    server.post({ kind: "frame", session: 7, timeS: 2.25 });
    await until(() => replies.filter((reply) => reply.kind === "flow").length === 2 || (replies.some((reply) => reply.kind === "flow" && reply.timeS === 2.25)));
    expect(replies.some((reply) => reply.kind === "flow" && reply.timeS === 2.25)).toBe(true);
    expect(replies.every((reply) => reply.kind !== "error")).toBe(true);
  });

  it("stops when cancelled and solves again when asked", async () => {
    const replies: FlowReply[] = [];
    const server = open(replies);
    server.post({ kind: "frame", session: 7, timeS: 2 });
    await until(() => replies.some((reply) => reply.kind === "progress"));
    server.post({ kind: "cancel", session: 7 });
    await until(() => replies.some((reply) => reply.kind === "cancelled"));
    expect(replies.some((reply) => reply.kind === "flow")).toBe(false);
    server.post({ kind: "frame", session: 7, timeS: 2 });
    await until(() => replies.some((reply) => reply.kind === "flow"));
    server.post({ kind: "close", session: 7 });
    server.post({ kind: "frame", session: 7, timeS: 3 });
    await settle();
    expect(replies.filter((reply) => reply.kind === "flow")).toHaveLength(1);
  });

  it("refuses a session of an asset it does not hold", () => {
    const replies: FlowReply[] = [];
    new FlowServer((reply) => replies.push(reply)).post({ kind: "open", session: 1, asset: 99, query: stand, environment, conditions: { maxAgeS: 2 }, periodS: 0, pathlines: false });
    expect(replies).toEqual([{ session: 1, kind: "error", message: "The flow solver does not hold the requested asset" }]);
  });
});

describe("📡️ flow session and figure", () => {
  it("develops, becomes ready, cancels and retries, with tracer time continuous over a clock wrap", async () => {
    const phases: string[] = [];
    const session: FlowSession = new FlowSession(() => phases.push(session.status.phase));
    session.open(walker, walk, environment);
    session.frame(0.5, 1);
    expect(session.status.phase).toBe("developing");
    session.cancel();
    expect(session.status.phase).toBe("cancelled");
    await settle();
    expect(session.flow).toBeNull();
    session.retry();
    await until(() => session.status.phase === "ready");
    const first: FlowField = session.flow!;
    expect(first.streams.length + first.puffs.length).toBeGreaterThan(0);
    session.frame(0.9, 1);
    session.frame(0.1, 1);
    expect(session.elapsedS).toBeCloseTo(0.6, 9);
    session.frame(0.05, 1);
    expect(session.elapsedS).toBeCloseTo(0.6, 9);
    await until(() => session.flow !== first);
    expect(session.flow!.timeS).toBeGreaterThan(first.timeS);
    expect(phases).toContain("developing");
    session.close();
    expect(session.flow).toBeNull();
  }, 600_000);

  it("mounts the flow layer over the thermal view only, sizes the figure for it and states the field in words", async () => {
    const base = { asset: walker, scale: { denominator: 100 }, language: "de" as const, viewpoint: "elevation" as const, query: stand };
    const plain = render(<StaffageFigure {...base} view="thermal" />);
    const narrow = Number(plain.container.querySelector<HTMLElement>("[data-staffage-figure]")!.getAttribute("data-width-mm"));
    expect(plain.container.querySelector("[data-flow]")).toBeNull();
    plain.unmount();
    const other = render(<StaffageFigure {...base} view="forces" flow={ALL_FLOW_LAYERS} />);
    expect(other.container.querySelector("[data-flow]")).toBeNull();
    other.unmount();
    const flows: (FlowField | null)[] = [];
    const { container } = render(<StaffageFigure {...base} view="thermal" flow={ALL_FLOW_LAYERS} onFlow={(flow) => flows.push(flow)} />);
    const figure = container.querySelector<HTMLElement>("[data-staffage-figure]")!;
    expect(Number(figure.getAttribute("data-width-mm"))).toBeGreaterThan(narrow);
    const layer = container.querySelector<HTMLElement>("[data-flow]")!;
    expect(layer).not.toBeNull();
    expect(layer.lang).toBe("de");
    expect(layer.querySelector("canvas")!.getAttribute("aria-hidden")).toBe("true");
    expect(layer.querySelector("progress")!.getAttribute("aria-label")).toBe("Luftströmung wird entwickelt");
    expect(layer.querySelector<HTMLButtonElement>("button")!.textContent).toBe("Abbrechen");
    await waitFor(() => expect(layer.getAttribute("data-flow")).toBe("ready"), { timeout: 600_000, interval: 50 });
    expect(layer.querySelector("[role=status]")!.textContent).toMatch(/^Luftströmung gezeigt: \d+ Stromlinien in Luft von .* °C und \d+ % relativer Feuchte, Taupunkt .* °C\. Wind 0,2 m\/s, nach hinten, hinter das Asset \(180°\)\.$/u);
    expect(layer.querySelector<HTMLElement>(".semio-flow-bar")!.hidden).toBe(true);
    expect(flows.at(-1)).not.toBeNull();
  }, 600_000);
});
