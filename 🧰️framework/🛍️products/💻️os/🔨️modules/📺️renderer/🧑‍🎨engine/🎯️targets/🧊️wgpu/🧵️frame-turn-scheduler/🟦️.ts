export type FrameTurnSchedule = (callback: () => void) => void;
export type FrameTurnStep = () => boolean;
export type FrameTurnCloseStep = () => boolean;
export type FrameTurnOwner = "frame" | "assetDecode";

/** @emoji 🚏️ Owns one unthrottled Worker task. A MessagePort task is not subject to the nested and
 * background timer clamps that can strand a retained frame between phases. */
export class WorkerTurnTaskQueue {
  private readonly channel = new MessageChannel();
  private callback: (() => void) | undefined;
  private closed = false;

  constructor() {
    this.channel.port1.onmessage = () => {
      const callback = this.callback;
      this.callback = undefined;
      callback?.();
    };
    this.channel.port1.start();
    this.channel.port2.start();
  }

  readonly schedule: FrameTurnSchedule = (callback) => {
    if (this.closed) throw new Error("frame turn task owner is closed");
    if (this.callback) throw new Error("frame turn task credits exceeded");
    this.callback = callback;
    this.channel.port2.postMessage(0);
  };

  close(): void {
    if (this.closed) return;
    this.closed = true;
    this.callback = undefined;
    this.channel.port1.onmessage = null;
    this.channel.port1.close();
    this.channel.port2.close();
  }
}

/** @emoji 🔢️ Advances the Worker-owned frame result sequence without saturation or reuse. */
export function nextFrameSequence(current: number): number {
  if (!Number.isSafeInteger(current) || current < 0 || current >= Number.MAX_SAFE_INTEGER) throw new Error("frame output sequence exhausted");
  return current + 1;
}

/** @emoji 🧵️ Retains one frame owner inside the dedicated Worker. Each scheduled callback performs at
 * most one frame unit or one close unit, and another callback is armed only while that owner remains
 * pending. */
export class FrameTurnScheduler {
  private scheduled = false;
  private framePending = false;
  private assetDecodePending = false;
  private lastDispatched: FrameTurnOwner | undefined;
  private closing = false;
  private terminal = false;

  constructor(
    private readonly schedule: FrameTurnSchedule,
    private readonly frameStep: FrameTurnStep,
    private readonly closeStep: FrameTurnCloseStep,
    private readonly assetDecodeStep?: FrameTurnStep,
  ) {}

  request(owner: FrameTurnOwner = "frame"): void {
    if (this.closing || this.terminal) return;
    if (owner === "frame") this.framePending = true;
    else this.assetDecodePending = true;
    this.arm();
  }

  requestRuntimeWake(): void {
    if (this.closing || this.terminal) return;
    this.framePending = true;
    this.assetDecodePending = true;
    this.arm();
  }

  beginClose(): void {
    if (this.closing || this.terminal) return;
    this.closing = true;
    this.framePending = false;
    this.assetDecodePending = false;
    this.arm();
  }

  terminalIsEmpty(): boolean {
    return this.terminal;
  }

  private arm(): void {
    if (this.scheduled || this.terminal) return;
    this.scheduled = true;
    this.schedule(() => this.run());
  }

  private run(): void {
    this.scheduled = false;
    if (this.closing) {
      this.terminal = this.closeStep();
      if (!this.terminal) this.arm();
      return;
    }
    const owner = this.nextOwner();
    if (!owner) return;
    this.lastDispatched = owner;
    if (owner === "frame") {
      this.framePending = false;
      if (this.frameStep()) this.framePending = true;
    } else {
      this.assetDecodePending = false;
      if (this.assetDecodeStep?.()) this.assetDecodePending = true;
    }
    if (this.framePending || this.assetDecodePending) this.arm();
  }

  private nextOwner(): FrameTurnOwner | undefined {
    if (this.framePending && this.assetDecodePending) return this.lastDispatched === "frame" ? "assetDecode" : "frame";
    if (this.framePending) return "frame";
    if (this.assetDecodePending) return "assetDecode";
    return undefined;
  }
}
