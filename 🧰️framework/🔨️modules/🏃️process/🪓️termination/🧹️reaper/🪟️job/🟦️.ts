import schema from "../🧬️schema/🔣️.json" with { type: "json" };

/** 📐️ The constants of the Windows job object calls.
 * @see https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_extended_limit_information
 * @see ../🧬️schema/🔣️.json */
export const JOB_OBJECT_V1 = schema.definitions.JobObject.const;

/** 🔌️ The kernel calls the job table needs; the real ones are [[windowsJobKernelV1]], tests pass a recorder. A handle is opaque. */
export interface JobKernelV1 {
  readonly create: () => unknown | undefined;
  readonly arm: (job: unknown, killOnClose: boolean) => boolean;
  readonly assign: (job: unknown, pid: number) => boolean;
  readonly terminate: (job: unknown) => void;
  readonly close: (job: unknown) => void;
}

/** 🧰️ One job object per registered child: its handle, held by the sentinel only. */
export interface JobTableV1 {
  readonly add: (pid: number) => boolean;
  readonly stop: (pid: number) => boolean;
  readonly release: (pid: number) => void;
  readonly stopAll: () => number;
  readonly size: () => number;
}

/** 🧰️ The table of jobs by child pid: `add` creates a kill-on-close job and assigns the child (a failed step closes the job and returns `false`); `stop` ends the whole tree of the child and closes the job; `release` disarms and closes the job of a child that ended on its own, so nothing is killed; `stopAll` ends every job (the parent is gone). */
export function createJobTableV1(kernel: JobKernelV1): JobTableV1 {
  const jobs = new Map<number, unknown>();
  const drop = (pid: number, kill: boolean): boolean => {
    const job = jobs.get(pid);
    if (job === undefined) return false;
    jobs.delete(pid);
    try { if (kill) kernel.terminate(job); else kernel.arm(job, false); } catch {}
    try { kernel.close(job); } catch {}
    return true;
  };
  return {
    add(pid) {
      if (jobs.has(pid)) return true;
      let job: unknown | undefined;
      try { job = kernel.create(); } catch { job = undefined; }
      if (job === undefined) return false;
      let held = false;
      try { held = kernel.arm(job, true) && kernel.assign(job, pid); } catch { held = false; }
      if (!held) { try { kernel.close(job); } catch {} return false; }
      jobs.set(pid, job);
      return true;
    },
    stop: (pid) => drop(pid, true),
    release: (pid) => { drop(pid, false); },
    stopAll() {
      let count = 0;
      for (const pid of [...jobs.keys()]) if (drop(pid, true)) count++;
      return count;
    },
    size: () => jobs.size,
  };
}

/** 🪟️ The real kernel calls over `bun:ffi` and kernel32, `undefined` off Windows or where the library cannot be bound. */
export async function windowsJobKernelV1(): Promise<JobKernelV1 | undefined> {
  if (process.platform !== "win32") return undefined;
  try {
    const { dlopen, FFIType, ptr } = await import("bun:ffi");
    const { symbols } = dlopen("kernel32.dll", {
      CreateJobObjectW: { args: [FFIType.ptr, FFIType.ptr], returns: FFIType.ptr },
      SetInformationJobObject: { args: [FFIType.ptr, FFIType.i32, FFIType.ptr, FFIType.u32], returns: FFIType.i32 },
      OpenProcess: { args: [FFIType.u32, FFIType.i32, FFIType.u32], returns: FFIType.ptr },
      AssignProcessToJobObject: { args: [FFIType.ptr, FFIType.ptr], returns: FFIType.i32 },
      TerminateJobObject: { args: [FFIType.ptr, FFIType.u32], returns: FFIType.i32 },
      CloseHandle: { args: [FFIType.ptr], returns: FFIType.i32 },
    });
    const constants = JOB_OBJECT_V1;
    return {
      create: () => symbols.CreateJobObjectW(null, null) ?? undefined,
      arm: (job, killOnClose) => {
        const information = new Uint8Array(constants.extendedLimitInformationBytes);
        new DataView(information.buffer).setUint32(constants.limitFlagsOffset, killOnClose ? constants.killOnJobClose : 0, true);
        return symbols.SetInformationJobObject(job as never, constants.extendedLimitInformationClass, ptr(information), information.length) !== 0;
      },
      assign: (job, pid) => {
        const process = symbols.OpenProcess(constants.processSetQuota | constants.processTerminate, 0, pid);
        if (process === null) return false;
        try { return symbols.AssignProcessToJobObject(job as never, process) !== 0; } finally { symbols.CloseHandle(process); }
      },
      terminate: (job) => { symbols.TerminateJobObject(job as never, constants.terminateExitCode); },
      close: (job) => { symbols.CloseHandle(job as never); },
    };
  } catch {
    return undefined;
  }
}
