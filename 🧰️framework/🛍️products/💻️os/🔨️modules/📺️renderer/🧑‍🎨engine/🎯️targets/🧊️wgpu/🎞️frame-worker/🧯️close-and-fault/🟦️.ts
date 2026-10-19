import { assetAbort, bootAbort, closed, closeOwner, closing, frameTurns, frameTurnTasks, interactiveJobs, jobsCloseComplete, lifecycle, mediaRegistry, mediaRetirement, originalFrameSource, pendingFault, runtime, runtimeCloseComplete, scope, setAssetAbort, setClosed, setCloseOwner, setClosing, setFailed, setFrameTurnTasks, setJobsCloseComplete, setMediaRetirement, setPendingFault, setRuntimeCloseComplete } from "../🧵️state/🟦️.ts";
import { ownedStep, post } from "../⏱️steps-and-reporting/🟦️.ts";
import { cancelPageImageDecode } from "../🖼️page-image-decode/🟦️.ts";

async function closeRuntime(): Promise<void> {
  await mediaRetirement;
  await originalFrameSource?.close();
  for (;;) {
    ownedStep("close-step", () => {
      if (closeOwner === "runtime" && !runtimeCloseComplete) {
        setRuntimeCloseComplete(runtime ? runtime.closeStep() : true);
        setCloseOwner("jobs");
      } else if (!jobsCloseComplete) {
        setJobsCloseComplete(interactiveJobs ? interactiveJobs.closeStep() : true);
        setCloseOwner("runtime");
      } else if (!runtimeCloseComplete) {
        setCloseOwner("runtime");
      }
    });
    if (runtimeCloseComplete && jobsCloseComplete && (frameTurns?.terminalIsEmpty() ?? true)) break;
    await new Promise<void>((resolve) => setTimeout(resolve, 0));
  }
  if (pendingFault) post({ kind: "fault", lifecycle, code: pendingFault.code, detail: pendingFault.detail,originalFailure:pendingFault.originalFailure },pendingFault.originalFailure?[pendingFault.originalFailure.bytes.buffer as ArrayBuffer]:[]);
  post({ kind: "closed", lifecycle });
  frameTurnTasks?.close();
  setFrameTurnTasks(undefined);
  setClosed(true);
  scope.close();
}

export function beginClose(): void {
  if (closed || closing) return;
  setClosing(true);
  bootAbort.abort();
  setMediaRetirement(mediaRegistry.close());
  setFailed(pendingFault !== undefined);
  setRuntimeCloseComplete(runtime === undefined);
  setJobsCloseComplete(interactiveJobs === undefined);
  frameTurns?.beginClose();
  assetAbort?.abort();
  setAssetAbort(undefined);
  cancelPageImageDecode();
  if (runtime) {
    try {
      ownedStep("asset-abort", () => runtime!.abortAssetResponse());
    } catch (error) {
      (pendingFault ?? setPendingFault({ code: "asset-abort-fault", detail: error instanceof Error ? error.message : String(error) }));
    }
  }
  interactiveJobs?.close();
  void closeRuntime();
}

export function fault(code: string, detail: string): void {
  requestFault(code, detail);
}

export function requestFault(code: string, detail: string): void {
  if (closed || pendingFault) return;
  setPendingFault({ code, detail });
  setFailed(true);
  beginClose();
}
