/** 📂️ Runs selection-only file intake without lending mutable placement to the browser. */
import { sendOriginalMedia, type OriginalMediaFrame, type OriginalMediaSource } from "./original-media-sender.ts";
import { type OriginalMediaExchange, type OriginalMediaRequest } from "./original-media-wire.ts";

export async function pickOriginalMedia(
  request: OriginalMediaRequest,
  exchange: OriginalMediaExchange,
  select: () => Promise<readonly OriginalMediaSource[]>,
  current: () => boolean,
  transfer: (file: OriginalMediaSource) => Readonly<{
    signal: AbortSignal;
    progress(completed: number, total: number): void;
    finish(): void;
  }>,
): Promise<"complete" | "cancelled"> {
  let task: ReturnType<typeof transfer> | undefined;
  let cancel: Promise<void> | undefined;
  const owned = () => current() && exchange.current() && !task?.signal.aborted;
  const cancelIssued = () => cancel ??= exchange.cross({ kind: "cancel", loan: request.loan }, undefined, () => true);
  const sink = (frame: OriginalMediaFrame, bytes?: Uint8Array) =>
    frame.kind === "cancel" ? cancelIssued() : exchange.cross(frame, bytes, owned);
  try {
    if (!owned()) { await cancelIssued(); return "cancelled"; }
    const selected = await select();
    if (!owned() || selected.length === 0) { await cancelIssued(); return "cancelled"; }
    if (selected.length !== 1) throw new Error("original-media.selection-count");
    const file = selected[0]!;
    task = transfer(file);
    await sendOriginalMedia(request.loan, file, request.maximumFileBytes, sink, {
      signal: task.signal, current: owned, progress: task.progress,
    });
    return "complete";
  } catch (failure) {
    const retirementIssued = cancel !== undefined;
    try { await cancelIssued(); }
    catch (error) {
      if (!retirementIssued) throw new AggregateError([failure, error], "original-media.transfer-close");
      throw failure;
    }
    if (!owned() && !(failure instanceof AggregateError)) return "cancelled";
    throw failure;
  } finally {
    task?.finish();
  }
}
