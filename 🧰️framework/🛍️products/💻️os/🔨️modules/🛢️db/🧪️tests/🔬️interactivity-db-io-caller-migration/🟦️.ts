import { interactivityDbIoCallerMigrationFailures } from "../../../../../../../📜️script.ts";

/** 🧪️ Executes interactivity db io caller migration policy assertions. */
export function interactivityDbIoCallerMigrationSelfTests(): void {
  const good = [["caller.rs", "submit_db_io_task pool.try_submit(Lane::Io, job) DbIoPageWriter DbIoPages"]] as const;
  if (interactivityDbIoCallerMigrationFailures(good).length !== 0) throw new Error("[verify interactivity] DB caller migration self-test rejected a typed caller.");
  for (const forbidden of ["run_blocking_op", "DbIoRequest", "DbIoPages::try_new", "DbIoPages::try_range", ".into_vec()", "MemoryStorage::default()"])
    if (interactivityDbIoCallerMigrationFailures([["mutated.rs", `${good[0][1]} ${forbidden}`]]).length === 0) throw new Error(`[verify interactivity] DB caller migration self-test missed ${forbidden}.`);
}
