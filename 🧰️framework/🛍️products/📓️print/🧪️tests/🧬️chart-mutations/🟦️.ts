/** 🧪️ Schema-first chart replay checks with independent AJV and D3 output oracles. */
import { scaleLinear } from "d3-scale";
import { color as d3Color } from "d3-color";
import Ajv2020 from "ajv/dist/2020.js";
import fixture from "../../🧫️fixtures/🧬️chart-mutations/🔣️.json";
import nativeGrammar from "../../🧫️fixtures/🧬️chart-mutations/📊️native-grammar.json";
import schema from "../../🧬️schema/🔣️.json";
import resultSchema from "../../🚪️io/📝️text/💡️inferences/🔣️.json";
import paintFixture from "../../🧫️fixtures/🧬️chart-mutations/🎨️paint.json";
import { changeVizChartValue } from "../../🧬️schema/🧬️mutations/🟦️.ts";
import { applyVizChartDiff, absorbVizChartDiff, inverseVizChartDiff, equalVizChartValue, type VizChartDiff } from "../../🧬️schema/🔀️diff/🟦️.ts";
import {inferVizChart} from "../../🔨️modules/🏠️host/💡️inferences/🟦️.ts";
import { validateVizChartSpecification } from "../../🧬️schema/💡️inferences/🟦️.ts";
import type { VizChartSnapshot } from "../../🧬️schema/📸️snapshot/🟦️.ts";

type Check = { readonly module: string; readonly name: string; readonly subject: () => unknown; readonly oracle: () => unknown; readonly tolerance?: number };
const base = fixture.snapshot as unknown as VizChartSnapshot;
function replay(): { snapshot: VizChartSnapshot; diff: VizChartDiff } {
  let snapshot = base, diff: VizChartDiff = { edits: [] };
  for (const mutation of fixture.mutations) {
    const outcome = changeVizChartValue(snapshot, mutation);
    if (outcome.messages.length > 0) throw new Error(JSON.stringify(outcome.messages));
    const applied = applyVizChartDiff(snapshot, outcome.diff);
    if (applied.messages.length > 0) throw new Error(JSON.stringify(applied.messages));
    snapshot = applied.snapshot;
    diff = absorbVizChartDiff(diff, outcome.diff);
  }
  return { snapshot, diff };
}

/** 🌐️ Exercises the real browser worker with first-party inference bundles and an independent timer. */
async function browserInference(): Promise<readonly unknown[]> {
  const entries = [new URL("../../🔨️modules/🏠️host/💡️inferences/🟦️.ts", import.meta.url).pathname, new URL("../../🔨️modules/🏠️host/💡️inferences/🧵️worker/🟦️.ts", import.meta.url).pathname];
  const bundles = await Promise.all(entries.map(entry => Bun.build({ entrypoints: [decodeURIComponent(entry).replace(/^\/(?=[A-Za-z]:)/, "")], target: "browser" })));
  for (const bundle of bundles) if (!bundle.success) throw new Error(bundle.logs.map(String).join("\n"));
  const [main, worker] = await Promise.all(bundles.map(bundle => bundle.outputs[0]!.text()));
  const server = Bun.serve({ hostname: "127.0.0.1", port: 0, fetch: request => new Response(new URL(request.url).pathname === "/main.js" ? main : new URL(request.url).pathname.endsWith(".ts") ? worker : "", { headers: { "Content-Type": "text/javascript" } }) });
  const replay = async ({ url, snapshot }: { url: string; snapshot: VizChartSnapshot }) => {
      const { inferVizChart: infer } = await import(url);
      const valid = await infer(snapshot);
      const controller = new AbortController();
      const expensive = { chart: { ...snapshot.chart, tables: [{ ...snapshot.chart.tables![0]!, rows: Array.from({ length: 20000 }, (_, index) => ({ x: index % 3, y: index % 5 })) }] } };
      let started = false, fired = false, timer: ReturnType<typeof setTimeout> | undefined;
      try {
        const result = await infer(expensive, { signal: controller.signal, onProgress: () => { if (!started) { started = true; timer = setTimeout(() => { fired = true; controller.abort(); }, 0); } } });
        return [valid.complete, valid.plan.items.filter((item: { kind: string }) => item.kind === "circle").map((item: { cx: number; cy: number }) => [item.cx, item.cy]), started, fired, result.complete, result.tikz === "", result.plan === undefined, result.scene === undefined, result.diagnostics[0]?.code];
      } finally { if (timer !== undefined) clearTimeout(timer); }
  };
  const source = `(async()=>{const {chromium}=await import("playwright"),browser=await chromium.launch({headless:true,timeout:30000});try{const page=await browser.newPage();await page.goto(${JSON.stringify(server.url.href)});const result=await page.evaluate(async({source,input})=>await(0,eval)("("+source+")")(input),${JSON.stringify({ source: replay.toString(), input: { url: new URL("main.js", server.url).href, snapshot: base } })});console.log(JSON.stringify(result));}finally{await browser.close();}})().catch(error=>{console.error(error);process.exitCode=1;});`;
  try {
    const controller = Bun.spawn(["node", "-e", source], { cwd: import.meta.dir, stdout: "pipe", stderr: "pipe" });
    const [stdout, stderr, status] = await Promise.all([new Response(controller.stdout).text(), new Response(controller.stderr).text(), controller.exited]);
    if (status !== 0) throw new Error(`browser controller exited ${status}: ${stderr}`);
    return JSON.parse(stdout.trim());
  } finally { await server.stop(true); }
}

export function chartMutationInferenceChecks(): readonly Check[] {
  const ajv = new Ajv2020({ strict: false });
  ajv.addSchema(schema);
  const validate = ajv.compile({ $ref: `${schema.$id}#/$defs/ChartSpecification` });
  const validateResult=ajv.compile(resultSchema);
  const inputs = [base.chart,nativeGrammar.chart,fixture.collision.snapshot.chart, { ...base.chart, language: undefined }, { ...base.chart, width: -1 }, { ...base.chart, extra: true }, { ...base.chart, layers: [{ mark: "unknown" }] }, { ...base.chart, margin: { top: 0, left: 0, right: 0, bottom: -2 } }];
  return [
    {module:"mutation",name:"native-paint-vectors-d3",subject:()=>paintFixture.cases.map(value=>[value.hex,value.alpha]),oracle:()=>paintFixture.cases.map(value=>{const paint=d3Color(value.input)!.rgb();const hex=paint.opacity===0?"000000":paint.formatHex().slice(1).toUpperCase();return [hex,paint.opacity];}),tolerance:1e-12},
    { module:"mutation",name:"shared-inference-output-contract",subject:async()=>(await Promise.all([inferVizChart(base),inferVizChart({chart:{...base.chart,language:undefined}})])).map(result=>{if(!validateResult(result))throw new Error(JSON.stringify(validateResult.errors));return true;}),oracle:()=>[true,true] },
    { module: "mutation", name: "schema-validation-ajv", subject: () => inputs.map((chart) => validateVizChartSpecification(chart).length === 0), oracle: () => inputs.map((chart) => validate(chart)) },
    { module: "mutation", name: "replayed-customization", subject: () => { const { snapshot } = replay(); return [snapshot.chart.width, snapshot.chart.language]; }, oracle: () => [fixture.expected.width, fixture.expected.language] },
    { module: "mutation", name: "diff-inverse-restores", subject: () => { const { snapshot, diff } = replay(); return equalVizChartValue(applyVizChartDiff(snapshot, inverseVizChartDiff(base, diff)).snapshot, base); }, oracle: () => true },
    { module: "mutation", name: "invalid-addresses-rejected", subject: () => fixture.rejections.map((mutation) => { const result = changeVizChartValue(base, mutation); return result.messages.length > 0 && result.diff.edits.length === 0; }), oracle: () => fixture.rejections.map(() => true) },
    { module: "mutation", name: "inferred-d3-scale-placement", subject: async () => { const result = await inferVizChart(replay().snapshot); if (!result.complete) throw new Error(JSON.stringify(result.diagnostics)); return result.plan!.items.filter((item) => item.kind === "circle").map((item) => [item.cx, item.cy]); }, oracle: () => { const x = scaleLinear([0, 2], [0, 80]), y = scaleLinear([0, 4], [0, 40]); return fixture.snapshot.chart.tables[0]!.rows.map((row) => [x(row.x), y(row.y)]); }, tolerance: 1e-9 },
    { module: "mutation", name: "total-inference-invalid-input", subject: async () => (await inferVizChart({ chart: { ...base.chart, language: undefined } })).complete, oracle: () => false },
    { module: "mutation", name: "cancellation-no-partial-result", subject: async () => { const control = new AbortController(); const result = await inferVizChart(base, { signal: control.signal, onProgress: () => control.abort() }); return [result.complete, result.plan === undefined, result.tikz, result.scene === undefined, result.diagnostics[0]?.code]; }, oracle: () => [false, true, "", true, "print.chart.cancelled"] },
    { module: "mutation", name: "event-loop-cancellation-no-partial-result", subject: async () => {
      const control = new AbortController();
      const snapshot: VizChartSnapshot = { chart: { ...base.chart, tables: [{ ...base.chart.tables![0]!, rows: Array.from({ length: 20000 }, (_, index) => ({ x: index % 3, y: index % 5 })) }] } };
      let timer: ReturnType<typeof setTimeout> | undefined, fired = false, started = false;
      try {
        const result = await inferVizChart(snapshot, { signal: control.signal, onProgress: progress => { if (!started && progress.completed === 0) { started = true; timer = setTimeout(() => { fired = true; control.abort(); }, 0); } } });
        return [started, fired, result.complete, result.plan === undefined, result.tikz === "", result.scene === undefined, result.diagnostics[0]?.code, validateResult(result)];
      } finally { if (timer !== undefined) clearTimeout(timer); }
    }, oracle: () => [true, true, false, true, true, true, "print.chart.cancelled", true] },
    { module: "mutation", name: "early-cancellation-no-work", subject: async () => { const control = new AbortController(); control.abort(); let progress = 0; const result = await inferVizChart(base, { signal: control.signal, onProgress: () => { progress += 1; } }); return [progress, result.complete, result.tikz, result.plan === undefined, result.scene === undefined, validateResult(result)]; }, oracle: () => [0, false, "", true, true, true] },
    { module: "mutation", name: "event-loop-cancellation-during-layout", subject: async () => {
      const control = new AbortController();
      const snapshot: VizChartSnapshot = { chart: { ...base.chart, tables: [{ ...base.chart.tables![0]!, rows: Array.from({ length: 20000 }, (_, index) => ({ x: index % 3, y: index % 5 })) }] } };
      let timer: ReturnType<typeof setTimeout> | undefined, fired = false, started = false;
      try {
        const result = await inferVizChart(snapshot, { signal: control.signal, onProgress: progress => { if (!started && progress.completed > 1 && progress.completed < progress.total - 2) { started = true; timer = setTimeout(() => { fired = true; control.abort(); }, 0); } } });
        return [started, fired, result.complete, result.tikz === "", result.plan === undefined, result.scene === undefined, result.diagnostics[0]?.code, validateResult(result)];
      } finally { if (timer !== undefined) clearTimeout(timer); }
    }, oracle: () => [true, true, false, true, true, true, "print.chart.cancelled", true] },
    { module: "mutation", name: "asynchronous-progress-and-deterministic-publication", subject: async () => {
      const progress: { completed: number; total: number }[] = [];
      const first = await inferVizChart(base, { onProgress: value => progress.push(value) });
      const second = await inferVizChart(base);
      const last = progress.at(-1)!;
      return [first.complete, second.complete, first.tikz === second.tikz, progress.length > 2, progress[0]?.completed, last.completed === last.total, progress.every((value, index) => value.total === last.total && (index === 0 || value.completed > progress[index - 1]!.completed)), validateResult(first)];
    }, oracle: () => [true, true, true, true, 0, true, true, true] },
    { module: "mutation", name: "cancellation-before-final-publication", subject: async () => { const control = new AbortController(); const result = await inferVizChart(base, { signal: control.signal, onProgress: value => { if (value.completed === value.total) control.abort(); } }); return [result.complete, result.tikz === "", result.plan === undefined, result.scene === undefined, validateResult(result)]; }, oracle: () => [false, true, true, true, true] },
    { module: "mutation", name: "real-browser-worker-and-timer-cancellation", subject: browserInference, oracle: () => { const x = scaleLinear([0, 2], [0, 80]), y = scaleLinear([0, 4], [0, 40]); return [true, fixture.snapshot.chart.tables[0]!.rows.map(row => [x(row.x), y(row.y)]), true, true, false, true, true, true, "print.chart.cancelled"]; } },
    { module: "mutation", name: "progress-callback-failure-atomic", subject: async () => { const result = await inferVizChart(base, { onProgress: () => { throw new Error("progress observer failed"); } }); return [result.complete, result.tikz === "", result.plan === undefined, result.scene === undefined, result.diagnostics[0]?.code, validateResult(result)]; }, oracle: () => [false, true, true, true, "print.chart.inference", true] },
  ];
}

