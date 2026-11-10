/** 🎯️ The commands whose graph is planned by the host service: their process only executes, so its capacity estimate never includes a planning, assembly or graph rebuild phase. */
export const PLANNED_COMMANDS_V1:readonly string[]=Object.freeze(["run","run-many","affected","exec","watch","inputs"]);

/** 🧮️ The global phase classes learned from the per-invocation planners that no longer exist in clients. */
export const PLANNER_PHASE_CLASSES_V1:readonly string[]=Object.freeze(["phase:assembly","phase:plan","phase:rebuild"]);
