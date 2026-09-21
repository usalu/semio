export type BrowserAssetCancellationPort = {
  readonly hasFetch: () => boolean;
  readonly hasPageImageDecode: () => boolean;
  readonly responseCurrent: () => boolean;
  readonly abortFetch: () => void;
  readonly cancelPageImageDecode: () => void;
  readonly returnResponseOwner: () => boolean;
};

export type BrowserAssetCancellationStep = "idle" | "waiting" | "returned";

/** 🧵️ Refuses a stale async continuation before it can mutate a returned response owner. */
export function assertBrowserAssetResponseContinuation(controller: AbortController, responseCurrent: () => boolean): void {
  controller.signal.throwIfAborted();
  if (responseCurrent()) return;
  controller.abort();
  controller.signal.throwIfAborted();
}

export class BrowserAssetCancellationCursor {
  private handback: "idle" | "waiting" | "returned" = "idle";

  step(port: BrowserAssetCancellationPort): BrowserAssetCancellationStep {
    if (this.handback === "returned") return "returned";
    if (this.handback === "idle" && !port.hasFetch() && !port.hasPageImageDecode()) return "idle";
    if (this.handback === "idle" && port.responseCurrent()) return "idle";
    port.abortFetch();
    port.cancelPageImageDecode();
    this.handback = "waiting";
    if (!port.returnResponseOwner()) return "waiting";
    this.handback = "returned";
    return "returned";
  }

  releaseReturned(): boolean {
    if (this.handback !== "returned") return false;
    this.handback = "idle";
    return true;
  }

  pollAdmitted(): boolean {
    return this.handback === "idle";
  }
}
