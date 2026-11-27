import { AsyncLocalStorage } from "node:async_hooks";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

const REFUSAL = schema.$defs.refusal.const, marks = new Set(), scopes = new AsyncLocalStorage();

/** 🚧️ Marks this whole process as the parent of a read for as long as `alive()` answers true (a farm exists); answers the function that removes the mark. A read marks only its own computation with {@link withReadParentScopeV1}, so the other flows of the same process (a generation answering a question while the read runs) are never refused. While marked, {@link assertNotReadParentV1} refuses every computation that belongs to a farm child.
 * @see ./🧬️schema/🔣️.json */
export function markReadParentV1(alive = () => true) {
  const mark = { alive };
  marks.add(mark);
  return () => { marks.delete(mark); };
}

/** 🚧️ Runs `body` as the parent of one read: everything it starts, however deep and however late, is refused the computations of a farm child once the read arms its scope with the liveness of its farm (`scope.arm(alive)`, answers the function that disarms it). A flow of the same process that the read did not start is not in the scope.
 * @param {(scope: { arm: (alive: () => boolean) => () => void }) => Promise<unknown>} body */
export function withReadParentScopeV1(body) {
  const scope = readScopeV1();
  return scopes.run(scope, () => body(scope));
}

/** 🪶️ The scope object of one read, made where no other variable lives: every async context the read starts keeps this object alive for as long as a timer, a pending promise or a cached continuation holds the context, and a closure environment shared with the read body would keep the body, its options and its inputs alive with it, wave after wave. */
function readScopeV1() {
  const scope = { alive: undefined, arm(alive) { scope.alive = alive; return () => { if (scope.alive === alive) scope.alive = undefined; }; } };
  return scope;
}

function liveV1(alive) { try { return alive() === true; } catch { return false; } }

/** 🚧️ Whether the running flow is in the parent of a read that has a farm. */
export function readParentV1() {
  const scoped = scopes.getStore()?.alive;
  if (scoped !== undefined && liveV1(scoped)) return true;
  for (const mark of marks) if (liveV1(mark.alive)) return true;
  return false;
}

/** 🛑️ Refuses a computation of kind `kind` (`infer`, `walk`, `resolve`, `derive`, `refs`) on `what` when the running flow is in the parent of a read that has a farm; nothing otherwise. */
export function assertNotReadParentV1(kind, what = "") {
  if (marks.size === 0 && scopes.getStore()?.alive === undefined) return;
  if (!readParentV1()) return;
  const text = language => REFUSAL[language].replace("{kind}", kind).replace("{what}", String(what).slice(0, 160));
  throw Object.assign(new Error(text("en") + " / " + text("de")), { code: REFUSAL.code, kind, what: String(what).slice(0, 160) });
}

const looks = new AsyncLocalStorage(), observed = { computations: 0, records: 0 };

/** 🔭️ Runs `body` as one look of the take pass: while the running flow is in the parent of a read that has a farm, every observed computation and every observed record inside it is counted ({@link readParentObservedCountsV1}); a look is a job of a farm child, so the count must stay zero.
 * @see ../../🕸️graph/📥️inference/📦️project-nodes/🔭️look-queue/🧪️tests/🟦️.ts */
export function withTakeLookScopeV1(body) { return looks.run(true, body); }

/** 🔭️ Counts one observed `computation` (`memoizedObservedComputationV1`: a directory visit, a closure walk) or `record` (a recorded file-system fact such as `existsSync`) that runs in the parent of a read inside a look of the take pass; nothing otherwise. */
export function noteObservedInReadParentV1(kind) { if (looks.getStore() === true && readParentV1()) observed[kind]++; }

/** 🔭️ How many observed computations and records ran in the parent of this process inside looks of the take pass so far. */
export function readParentObservedCountsV1() { return Object.freeze({ ...observed }); }

/** ⏳️ The line a queued job says while no child can take it, in both languages. */
export function queuedJobTextV1(project, reason) {
  const text = language => schema.$defs.waits.const[language].replace("{project}", project).replace("{reason}", reason);
  return text("en") + " / " + text("de");
}
