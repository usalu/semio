import type { CargoProvenanceSessionV1 } from "../../🟦️.ts";

export const RELEASE_PROVENANCE_COMPLETION_V1 = "provenance:seal";

/** 🧾️ Closes the actual observed invocations before release can publish any of their outputs. */
export async function closeReleaseProvenanceV1(
  session: CargoProvenanceSessionV1,
  observations: readonly (Promise<unknown> | undefined)[],
  recheck: () => void | Promise<void>,
  outcome: Readonly<{ failed: boolean; linked: number; browserRelease: boolean }>,
): Promise<boolean> {
  if (outcome.failed && (!outcome.linked || !outcome.browserRelease)) return false;
  const settled = await Promise.allSettled(observations);
  const refusal = settled.find(row => row.status === "rejected");
  if (refusal?.status === "rejected") throw refusal.reason;
  await recheck();
  await session.seal();
  await recheck();
  await session.close();
  await recheck();
  return true;
}
