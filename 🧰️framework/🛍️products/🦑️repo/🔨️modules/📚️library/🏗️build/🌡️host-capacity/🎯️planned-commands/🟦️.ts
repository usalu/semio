/** 🎯️ The commands whose graph is planned by the host service: their process only executes, so its capacity estimate never includes a planning, assembly or graph rebuild phase. */
export const PLANNED_COMMANDS_V1:readonly string[]=Object.freeze(["run","run-many","affected","exec","watch","inputs"]);

/** 🧮️ The global phase classes learned from the per-invocation planners that no longer exist in clients. */
export const PLANNER_PHASE_CLASSES_V1:readonly string[]=Object.freeze(["phase:assembly","phase:plan","phase:rebuild"]);

/** 🦀️ The commands that run an external tool and never plan a graph: their capacity estimate is that of the tool alone (its jobs times the learned footprint of one job), never the constant planning or assembly floor that graph runs learned, which would make a cargo run need more memory than the host can spare at any job count. */
export const PHASELESS_COMMANDS_V1:readonly string[]=Object.freeze(["cargo","command"]);
