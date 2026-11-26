import { workerData } from "node:worker_threads";
import {processPidExistsV1} from "../../../../../../../../../../🔨️modules/🏃️process/🔖️identity/🟨️.mjs";

/** 🐕️ The watchdog thread of a farm worker: a closed channel to the read is only noticed by an event loop that runs, and the inference of a project holds the loop of the worker for minutes, so this thread (a loop of its own) looks every `watchMilliseconds` whether the read that started the worker is alive and, on POSIX, still its parent (a dead parent hands its children to pid 1), and ends the whole process when it is not, whatever it computes.
 * @see ../🧪️tests/🪦️busy-orphan/🟦️.ts */
const { parentPid, watchMilliseconds = 1000, windows = process.platform === "win32" } = workerData;
const alive = pid => processPidExistsV1(pid);
setInterval(() => { if (!alive(parentPid) || !windows && process.ppid !== parentPid) { try { process.kill(process.pid, "SIGKILL"); } catch {} process.exit(1); } }, watchMilliseconds);
