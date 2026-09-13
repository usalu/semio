import type { Duplex } from "node:stream";

export const LOCAL_BOOTSTRAP_FRAME_MAX = 16 * 1024;
export const LOCAL_BOOTSTRAP_DEADLINE_MS = 15_000;

/** 📡️ Reads one length-prefixed JSON value while bounding retained input and outstanding work. */
export class LocalFrameReader {
  private retained = Buffer.alloc(0);
  private readonly waiters: Array<{ resolve: (value: Record<string, any>) => void; reject: (error: Error) => void; timer: ReturnType<typeof setTimeout> }> = [];

  constructor(
    private readonly pipe: Duplex,
    private readonly frameMaximumBytes = LOCAL_BOOTSTRAP_FRAME_MAX,
    private readonly label = "local bootstrap",
  ) {
    pipe.on("data", (chunk: Buffer) => {
      if (this.retained.length + chunk.length > this.frameMaximumBytes * 2) return this.fail(new Error(`${this.label} retained input exceeded fixed bound`));
      this.retained = Buffer.concat([this.retained, chunk]);
      this.drain();
    });
    pipe.once("error", () => this.fail(new Error(`${this.label} endpoint failed`)));
    pipe.once("end", () => this.fail(new Error(`${this.label} endpoint reached EOF`)));
    pipe.once("close", () => this.fail(new Error(`${this.label} endpoint closed`)));
  }

  read(deadlineMs = LOCAL_BOOTSTRAP_DEADLINE_MS): Promise<Record<string, any>> {
    if (!Number.isSafeInteger(deadlineMs) || deadlineMs <= 0) return Promise.reject(new Error("local bootstrap frame deadline invalid"));
    if (this.waiters.length >= 8) return Promise.reject(new Error("local bootstrap outstanding read bound exceeded"));
    return new Promise((resolveRead, rejectRead) => {
      const waiter = {
        resolve: resolveRead,
        reject: rejectRead,
        timer: setTimeout(() => {
          const index = this.waiters.indexOf(waiter);
          if (index >= 0) this.waiters.splice(index, 1);
          rejectRead(new Error("local bootstrap frame deadline exceeded"));
        }, deadlineMs),
      };
      this.waiters.push(waiter);
      this.drain();
    });
  }

  private drain(): void {
    while (this.waiters.length > 0 && this.retained.length >= 4) {
      const length = this.retained.readUInt32BE(0);
      if (length === 0 || length + 4 > this.frameMaximumBytes) return this.fail(new Error(`${this.label} frame exceeded fixed bound`));
      if (this.retained.length < length + 4) return;
      const bytes = this.retained.subarray(4, length + 4);
      this.retained = this.retained.subarray(length + 4);
      const waiter = this.waiters.shift()!;
      clearTimeout(waiter.timer);
      try {
        waiter.resolve(JSON.parse(bytes.toString("utf8")));
      } catch {
        waiter.reject(new Error(`${this.label} frame was not JSON`));
      }
    }
  }

  private fail(error: Error): void {
    for (const waiter of this.waiters.splice(0)) {
      clearTimeout(waiter.timer);
      waiter.reject(error);
    }
    this.retained.fill(0);
    this.retained = Buffer.alloc(0);
  }
}

/** ✉️ Writes one bounded length-prefixed JSON value and clears its transient bytes. */
export function writeLocalFrame(pipe: Duplex, value: object): Promise<void> {
  const bytes = Buffer.from(JSON.stringify(value));
  if (bytes.length === 0 || bytes.length + 4 > LOCAL_BOOTSTRAP_FRAME_MAX) throw new Error("local bootstrap frame exceeded fixed bound");
  const frame = Buffer.allocUnsafe(bytes.length + 4);
  frame.writeUInt32BE(bytes.length, 0);
  bytes.copy(frame, 4);
  bytes.fill(0);
  return new Promise((resolveWrite, rejectWrite) => {
    pipe.write(frame, (error?: Error | null) => {
      frame.fill(0);
      if (error) rejectWrite(new Error("local bootstrap write failed"));
      else resolveWrite();
    });
  });
}
