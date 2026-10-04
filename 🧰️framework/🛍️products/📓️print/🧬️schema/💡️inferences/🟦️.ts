/** 💡️ Asynchronous chart inference owns worker lifetime, progress and atomic publication. */
import type { VizChartSnapshot } from "../📸️snapshot/🟦️.ts";
import type { VizRenderPlan, renderVizScenePlan } from "./🖼️render/🟦️.ts";
import type { VizChartDiagnostic } from "./✅️validation/🟦️.ts";
export { validateVizChartSpecification } from "./✅️validation/🟦️.ts";
export type VizChartInference = { readonly plan?: VizRenderPlan; readonly tikz: string; readonly scene?: ReturnType<typeof renderVizScenePlan>; readonly diagnostics: readonly VizChartDiagnostic[]; readonly complete: true } | { readonly plan?: never; readonly tikz: ""; readonly scene?: never; readonly diagnostics: readonly VizChartDiagnostic[]; readonly complete: false };
export type VizChartInferenceProgress = { readonly completed: number; readonly total: number };
export type VizChartInferenceControl = { readonly signal?: AbortSignal; readonly onProgress?: (progress: VizChartInferenceProgress) => void };
export const VIZ_CHART_INFERENCE_FIELDS = [{ id: "framework.print.chart.inference.plan", reads: ["chart"] }, { id: "framework.print.chart.inference.tikz", reads: ["chart"] }, { id: "framework.print.chart.inference.scene", reads: ["chart"] }] as const;
type WorkerMessage = { readonly kind: "progress"; readonly progress: VizChartInferenceProgress } | { readonly kind: "result"; readonly result: VizChartInference };
type InferenceWorker = { postMessage(snapshot: VizChartSnapshot): void; terminate(): void; listen(message: (value: WorkerMessage) => void, error: (value: unknown) => void): () => void };

async function createInferenceWorker(): Promise<InferenceWorker> {
  if (typeof Worker !== "undefined") {
    const worker = new Worker(new URL("./🧵️worker/🟦️.ts", import.meta.url), { type: "module" });
    return {
      postMessage: snapshot => worker.postMessage(snapshot), terminate: () => { worker.terminate(); },
      listen: (message, error) => {
        worker.onmessage = event => message(event.data as WorkerMessage);
        worker.onerror = event => error(event.message);
        worker.onmessageerror = () => error("chart inference result could not be transferred");
        return () => { worker.onmessage = null; worker.onerror = null; worker.onmessageerror = null; };
      },
    };
  }
  const module = "node:worker_threads";
  const { Worker: Thread } = await import(module) as typeof import("node:worker_threads");
  const worker = new Thread(new URL("./🧵️worker/🟦️.ts", import.meta.url));
  return {
    postMessage: snapshot => worker.postMessage(snapshot), terminate: () => { void worker.terminate().catch(() => {}); },
    listen: (message, error) => {
      const exit = (code: number) => error(`chart inference worker exited before publication (${code})`);
      worker.on("message", message); worker.on("error", error); worker.on("messageerror", error); worker.on("exit", exit);
      return () => { worker.off("message", message); worker.off("error", error); worker.off("messageerror", error); worker.off("exit", exit); };
    },
  };
}

/** 🧵️ Derives a complete result off the caller thread; abort terminates every expensive stage. */
export async function inferVizChart(snapshot: VizChartSnapshot, control: VizChartInferenceControl = {}): Promise<VizChartInference> {
  const diagnostic = (code: string, message: string): VizChartInference => ({ tikz: "", diagnostics: [{ code, path: "chart", message }], complete: false });
  const cancelled = () => diagnostic("print.chart.cancelled", "chart inference cancelled");
  if (control.signal?.aborted) return cancelled();
  return new Promise(resolve => {
    let settled = false, worker: InferenceWorker | undefined, detach: (() => void) | undefined;
    const finish = (result: VizChartInference) => {
      if (settled) return;
      settled = true;
      control.signal?.removeEventListener("abort", abort);
      detach?.(); worker?.terminate();
      resolve(result);
    };
    const abort = () => finish(cancelled());
    const fail = (error: unknown) => finish(control.signal?.aborted ? cancelled() : diagnostic("print.chart.inference", error instanceof Error ? error.message : String(error)));
    control.signal?.addEventListener("abort", abort, { once: true });
    if (control.signal?.aborted) { abort(); return; }
    void createInferenceWorker().then(created => {
      worker = created;
      if (settled) { created.terminate(); return; }
      try {
        detach = created.listen(message => {
          if (settled) return;
          if (control.signal?.aborted) { abort(); return; }
          try {
            if (message.kind === "progress") control.onProgress?.(message.progress);
            else finish(message.result);
          } catch (error) { fail(error); }
        }, fail);
        created.postMessage(snapshot);
      } catch (error) { fail(error); }
    }, fail);
  });
}
