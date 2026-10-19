import { hostContinuations } from "@semio-tech/framework-async/continuation";
import { ValueError, type SqliteDatabaseOptions, type SqliteDatabaseProgress } from "../../🟦️.ts";

/** 🚦️ Semantic entity progress plus physical page progress. */
export interface ArtifactSqliteProgress { readonly phase: SqliteDatabaseProgress["phase"] | "projectSnapshot" | "reconstructSnapshot"; readonly completed: number; readonly total: number }
/** 🛑️ Shared cancellation, bounds and progress for typed snapshot projections. */
export interface ArtifactSqliteOptions extends Omit<SqliteDatabaseOptions, "onProgress"> { readonly onProgress?: (progress: ArtifactSqliteProgress) => void }

/** ⏱️ Publish semantic entity progress and yield to cancellation at bounded intervals. */
export async function artifactSqliteCheckpoint(options: ArtifactSqliteOptions, phase: "projectSnapshot" | "reconstructSnapshot" | "indexTables", completed: number, total: number, yieldToEventLoop = true): Promise<void> {
  cancellation(options);
  options.onProgress?.({ phase, completed, total });
  cancellation(options);
  if (yieldToEventLoop && completed % 256 === 0) await hostContinuations.yieldContinuation();
  cancellation(options);
}

/** 🛑️ Throws a canceled {@link ValueError} once the caller's signal aborted. */
export function cancellation(options: ArtifactSqliteOptions): void {
  if (options.signal?.aborted) throw new ValueError("canceled", "Artifact SQLite projection cancelled");
}
