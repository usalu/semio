export type ArtifactCreationReadyOpeningOutcomeV1 = "opened" | "failed" | "retired";

export interface ArtifactCreationReadyOpeningPortV1<TTarget, TDisposition extends Readonly<{ committed: boolean }>> {
  readonly current: () => boolean;
  readonly prepare: () => Promise<TTarget | null>;
  readonly open: (target: TTarget) => Promise<TDisposition | null>;
  readonly publish: (target: TTarget, disposition: TDisposition) => void;
  readonly release: (target: TTarget, disposition: TDisposition | null) => Promise<void>;
  readonly failed: (error: unknown) => void;
}

/** 🌱️ Keeps durable Ready retryable while one private app target is admitted, opened and published exactly once. */
export async function runArtifactCreationReadyOpeningV1<TTarget, TDisposition extends Readonly<{ committed: boolean }>>(
  port: ArtifactCreationReadyOpeningPortV1<TTarget, TDisposition>,
): Promise<ArtifactCreationReadyOpeningOutcomeV1> {
  if (!port.current()) return "retired";
  let target: TTarget | null = null;
  let disposition: TDisposition | null = null;
  let released = false;
  const releaseOnce = async (openingError?: unknown): Promise<unknown> => {
    if (target === null || released) return openingError;
    released = true;
    try {
      await port.release(target, disposition);
      return openingError;
    } catch (releaseError) {
      return openingError === undefined ? releaseError : new AggregateError([openingError, releaseError], "artifact opening and release failed");
    }
  };
  try {
    target = await port.prepare();
    if (target === null) throw new Error("artifact opening target unavailable");
    if (!port.current()) {
      await releaseOnce();
      return "retired";
    }
    disposition = await port.open(target);
    if (disposition === null || !disposition.committed) throw new Error("artifact document opening retired");
    if (!port.current()) {
      await releaseOnce();
      return "retired";
    }
    port.publish(target, disposition);
    return "opened";
  } catch (error) {
    error = await releaseOnce(error);
    if (!port.current()) return "retired";
    port.failed(error);
    return "failed";
  }
}
