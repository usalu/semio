import type { VizChartTextOutput } from "../../../🚪️io/📝️text/💡️inferences/🟦️.ts";
/** 🏠️ Chart inference owns worker lifetime, cancellation and atomic publication. */
import type {VizChartSnapshot} from "../../../🧬️schema/📸️snapshot/🟦️.ts";
import type {VizChartInferenceProgress,VizChartInferenceControl} from "../../../🧬️schema/💡️inferences/🟦️.ts";
type WorkerMessage = { readonly kind: "progress"; readonly progress: VizChartInferenceProgress } | { readonly kind: "result"; readonly result: VizChartTextOutput };
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
export async function inferVizChart(snapshot: VizChartSnapshot, control: VizChartInferenceControl = {}): Promise<VizChartTextOutput> {
  const diagnostic = (code: string, message: string): VizChartTextOutput => ({ tikz: "", diagnostics: [{ code, path: "chart", message }], complete: false });
  const cancelled = () => diagnostic("print.chart.cancelled", "chart inference cancelled");
  if (control.signal?.aborted) return cancelled();
  return new Promise(resolve => {
    let settled = false, worker: InferenceWorker | undefined, detach: (() => void) | undefined;
    const finish = (result: VizChartTextOutput) => {
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
