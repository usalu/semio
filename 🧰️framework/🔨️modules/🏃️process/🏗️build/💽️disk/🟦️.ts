import { diskFullOfV1, isDiskFullV1, type DiskFullV1 } from "../../🆓️free-space/💽️disk-full/🟦️.ts";
import { PROCESS_TEXTS_V1, refuseProcessV1 } from "../../📖️texts/🟦️.ts";
import { handleTextV1 } from "../🗣️language/📖️catalog/🟦️.ts";
import type { RepoErrorV1 } from "../🗣️language/🚨️error/🟦️.ts";

/** 🧹️ The command that frees space on a full disk; it needs no free byte of its own. */
export const DISK_RECLAIM_COMMAND_V1 = "repo cache cas gc";

const UNITS = ["B", "KiB", "MiB", "GiB", "TiB"] as const;

/** 🔢️ Writes a byte count with the largest binary unit that keeps it at one or more (`0 B`, `512 KiB`, `1.25 GiB`); the one form of every disk refusal. */
export function formatDiskBytesV1(bytes: number): string {
  let value = Math.max(0, bytes), unit = 0;
  while (value >= 1024 && unit < UNITS.length - 1) { value /= 1024; unit++; }
  return `${unit === 0 ? Math.round(value) : Math.round(value * 100) / 100} ${UNITS[unit]}`;
}

/** 💽️ The refusal of a failure that says the disk is full: it names the partition, the free bytes and the command that frees space, in both languages. @see ../../🆓️free-space/💽️disk-full/🟦️.ts */
export const refuseDiskFullFactsV1 = (facts: DiskFullV1, cause?: unknown): RepoErrorV1 => facts.freeBytes === undefined
  ? refuseProcessV1("failed", "disk.full-unmeasured", { partition: facts.partition, command: DISK_RECLAIM_COMMAND_V1 }, { cause })
  : refuseProcessV1("failed", "disk.full", { partition: facts.partition, free: formatDiskBytesV1(facts.freeBytes), command: DISK_RECLAIM_COMMAND_V1 }, { cause });

/** 💽️ Turns a full-disk failure (`ENOSPC`, `EDQUOT`, `database or disk is full`) into the typed refusal; any other error answers `undefined`. */
export function refuseDiskFullV1(error: unknown, options: Parameters<typeof diskFullOfV1>[1] = {}): RepoErrorV1 | undefined {
  const facts = isDiskFullV1(error) ? diskFullOfV1(error, options) : undefined;
  return facts === undefined ? undefined : refuseDiskFullFactsV1(facts, error);
}

/** 🚧️ What a partition holds against the used share a write or a build may start at. */
export type DiskLowSpaceV1 = Readonly<{ partition: string; freeBytes: number; totalBytes: number; limitFraction: number }>;

/** 🚧️ The refusal of a build or cache write that would start on a partition above its used share: it names the partition, the free bytes, the used and the allowed percent and the command that frees space. */
export const refuseLowSpaceV1 = (facts: DiskLowSpaceV1): RepoErrorV1 => refuseProcessV1("refused", "disk.low-space", {
  partition: facts.partition,
  free: formatDiskBytesV1(facts.freeBytes),
  usedPercent: Math.round((1 - facts.freeBytes / Math.max(1, facts.totalBytes)) * 1000) / 10,
  limitPercent: Math.round(facts.limitFraction * 1000) / 10,
  command: DISK_RECLAIM_COMMAND_V1,
});

/** 📣️ The progress line of a command that waits for the cache collection because its partition is above the used share it may start at: it names the partition, how full it is and the limit, in the language asked for. */
export const diskCollectingLineV1 = (facts: Readonly<{ partition: string; usedFraction: number; limitFraction: number }>, language: string): string => handleTextV1(PROCESS_TEXTS_V1, "disk.collecting", language, {
  partition: facts.partition,
  usedPercent: Math.round(facts.usedFraction * 1000) / 10,
  limitPercent: Math.round(facts.limitFraction * 1000) / 10,
});

/** 🚪️ What the upfront admission of a build decides on: the partition that holds the image backing, its free bytes, the planned new space of the build, the largest single batch that must fit whole, the reserve it keeps, the blocks the image already holds without using (slack) and the shares of the plan that must fit and of the slack that counts. */
export type DiskAdmissionV1 = Readonly<{ partition: string; hostFreeBytes: number; plannedBytes: number; minimumBytes: number; reserveBytes: number; slackBytes: number; planShareBasisPoints: number; slackCreditBasisPoints: number }>;

/** 🚪️ The new host space a build asks for: the planned footprint scaled by the share that persists (never below the largest single batch), less the credited part of the image slack that later writes reuse without growing the backing file. Pure and language-agnostic. */
export function diskAdmissionNeedBytesV1(facts: Pick<DiskAdmissionV1, "plannedBytes" | "minimumBytes" | "slackBytes" | "planShareBasisPoints" | "slackCreditBasisPoints">): Readonly<{ needBytes: number; creditBytes: number; planBytes: number }> {
  const planBytes = Math.max(facts.minimumBytes, Math.ceil(facts.plannedBytes * facts.planShareBasisPoints / 10000)), creditBytes = Math.min(planBytes, Math.floor(Math.max(0, facts.slackBytes) * facts.slackCreditBasisPoints / 10000));
  return Object.freeze({ needBytes: planBytes - creditBytes, creditBytes, planBytes });
}

/** 🚪️ The bytes the partition lacks for the build and its reserve: zero when it fits, so a build that cannot finish stops before it starts instead of at its end. Pure and language-agnostic. */
export function diskAdmissionShortfallBytesV1(facts: DiskAdmissionV1): number {
  return Math.max(0, diskAdmissionNeedBytesV1(facts).needBytes + facts.reserveBytes - facts.hostFreeBytes);
}

/** 🚪️ The bilingual refusal of a build the partition cannot take: it names the partition, its free bytes, the planned and the credited bytes, the reserve, what is missing and the command that frees space. */
export const refuseDiskAdmissionV1 = (facts: DiskAdmissionV1): RepoErrorV1 => {
  const need = diskAdmissionNeedBytesV1(facts);
  return refuseProcessV1("refused", "disk.admission-insufficient", { partition: facts.partition, free: formatDiskBytesV1(facts.hostFreeBytes), need: formatDiskBytesV1(need.needBytes), plan: formatDiskBytesV1(need.planBytes), credit: formatDiskBytesV1(need.creditBytes), reserve: formatDiskBytesV1(facts.reserveBytes), missing: formatDiskBytesV1(diskAdmissionShortfallBytesV1(facts)), command: DISK_RECLAIM_COMMAND_V1 });
};
