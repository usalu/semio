import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { describe, expect, test } from "vitest";

const engineRoot = join(dirname(fileURLToPath(import.meta.url)), "..", "..");
const fixture = JSON.parse(readFileSync(join(engineRoot, "🧫️fixtures", "⏱️frame-latency", "🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(join(engineRoot, "🧬️schema", "⏱️frame-latency", "🔣️.json"), "utf8"));

type Authority = { domain: "rendererFrame" | "browserInputBatch"; generation: number };
type Observation = { authority: Authority; stage: string; startedUs: number; durationUs: number; workItems: number };
type PhaseSummary = {
  authority: Authority;
  stage: string;
  firstSequence: number;
  lastSequence: number;
  observationCount: number;
  firstStartedUs: number;
  lastFinishedUs: number;
  totalDurationUs: number;
  maxDurationUs: number;
  totalWorkItems: number;
};
type StageTotal = Omit<PhaseSummary, "authority" | "firstSequence" | "lastSequence" | "firstStartedUs" | "lastFinishedUs"> & { authorityDomain: Authority["domain"] };

const observations: Observation[] = fixture.observations.flatMap((row: (typeof fixture.observations)[number]) =>
  Array.from({ length: row.repeat }, (_, index) => ({
    authority: row.authority,
    stage: row.stage,
    startedUs: row.startedUs + index * row.spacingUs,
    durationUs: row.durationUs,
    workItems: row.workItems,
  })),
);

const independentOracle = (rows: Observation[], capacity: number) => {
  const phases: PhaseSummary[] = [];
  const totals = new Map<string, StageTotal>();
  let summaryEvictions = 0;
  rows.forEach((row, index) => {
    const sequence = index + 1;
    const key = `${row.authority.domain}:${row.authority.generation}:${row.stage}`;
    const phase = phases.find((candidate) => `${candidate.authority.domain}:${candidate.authority.generation}:${candidate.stage}` === key);
    if (phase) {
      phase.lastSequence = sequence;
      phase.observationCount += 1;
      phase.firstStartedUs = Math.min(phase.firstStartedUs, row.startedUs);
      phase.lastFinishedUs = Math.max(phase.lastFinishedUs, row.startedUs + row.durationUs);
      phase.totalDurationUs += row.durationUs;
      phase.maxDurationUs = Math.max(phase.maxDurationUs, row.durationUs);
      phase.totalWorkItems += row.workItems;
    } else {
      if (phases.length === capacity) {
        phases.shift();
        summaryEvictions += 1;
      }
      phases.push({
        authority: row.authority,
        stage: row.stage,
        firstSequence: sequence,
        lastSequence: sequence,
        observationCount: 1,
        firstStartedUs: row.startedUs,
        lastFinishedUs: row.startedUs + row.durationUs,
        totalDurationUs: row.durationUs,
        maxDurationUs: row.durationUs,
        totalWorkItems: row.workItems,
      });
    }
    const totalKey = `${row.authority.domain}:${row.stage}`;
    const total = totals.get(totalKey) ?? { authorityDomain: row.authority.domain, stage: row.stage, observationCount: 0, totalDurationUs: 0, maxDurationUs: 0, totalWorkItems: 0 };
    total.observationCount += 1;
    total.totalDurationUs += row.durationUs;
    total.maxDurationUs = Math.max(total.maxDurationUs, row.durationUs);
    total.totalWorkItems += row.workItems;
    totals.set(totalKey, total);
  });
  const authorityOrder: Authority["domain"][] = ["rendererFrame", "browserInputBatch"];
  const stageTotals = authorityOrder.flatMap((domain) => fixture.requiredStages.map((stage: string) => totals.get(`${domain}:${stage}`)).filter(Boolean));
  return { totalObservations: rows.length, summaryEvictions, recentPhases: phases, stageTotals };
};

describe("⏱️ authority-correlated frame latency diagnostics", () => {
  test("the language-neutral aggregation vectors satisfy their schema", () => {
    const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  });

  test("an independent fixed-capacity oracle coalesces scalar floods without losing cumulative stages", () => {
    expect(independentOracle(observations, fixture.oracleCapacity)).toEqual(fixture.expected);
    expect(fixture.expected.recentPhases).toHaveLength(fixture.oracleCapacity);
    expect(fixture.expected.recentPhases.some((phase: PhaseSummary) => phase.observationCount === 301)).toBe(true);
    expect(fixture.expected.stageTotals.some((total: StageTotal) => total.stage === "transactionRouteIntents" && total.observationCount === 300)).toBe(true);
  });

  test("equal generation numbers remain separated by renderer-frame and browser-input authority", () => {
    const generation41 = independentOracle(observations, fixture.oracleCapacity).stageTotals.filter((total) =>
      fixture.expected.recentPhases.some((phase: PhaseSummary) => phase.authority.generation === 41 && phase.stage === total.stage),
    );
    expect(generation41.map(({ authorityDomain, stage }) => [authorityDomain, stage])).toContainEqual(["rendererFrame", "queueSubmit"]);
    expect(generation41.map(({ authorityDomain, stage }) => [authorityDomain, stage])).toContainEqual(["browserInputBatch", "wireApply"]);
  });

  test("the WGPU boundaries publish explicit authority domains through dumpFrameStats", () => {
    const latency = readFileSync(join(engineRoot, "🎯️targets", "🧊️wgpu", "⏱️frame-latency", "🦀️.rs"), "utf8");
    const renderer = readFileSync(join(engineRoot, "🎯️targets", "🧊️wgpu", "🧊️renderer", "🦀️.rs"), "utf8");
    const worker = readFileSync(join(engineRoot, "🎯️targets", "🧊️wgpu", "🌐️browser-worker", "🦀️.rs"), "utf8");
    const host = readFileSync(join(engineRoot, "🎯️targets", "🧊️wgpu", "🪟️winit-app", "🦀️.rs"), "utf8");
    const shell = readFileSync(join(engineRoot, "🧱️elements", "🐚️Shell", "🎯️targets", "🧊️wgpu", "🦀️.rs"), "utf8");
    const bridge = readFileSync(join(engineRoot, "🧱️elements", "🌉️ProgramBridge", "🎯️targets", "🧊️wgpu", "🦀️.rs"), "utf8");
    const interpreter = readFileSync(join(engineRoot, "🧱️elements", "🗣️Interpreter", "🎯️targets", "🧊️wgpu", "🦀️.rs"), "utf8");
    expect(latency).toContain("FrameLatencyAuthorityDomain::RendererFrame");
    expect(latency).toContain("FrameLatencyAuthorityDomain::BrowserInputBatch");
    expect(worker).toContain("FrameLatencyAuthority::browser_input_batch");
    expect(renderer).toContain("FrameLatencyAuthority::renderer_frame");
    expect(host).toContain("FrameLatencyStage::SnapshotPublish");
    expect(shell).toContain("FrameLatencyStage::ShellRefresh");
    expect(bridge).toContain("FrameLatencyStage::RetainedExchange");
    expect(interpreter).toContain("frame_latency: crate::frame_latency::snapshot()");
  });
});
