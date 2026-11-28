import {buildGraphScopeKeyV1,readBuildGraphSelectionV1,type BuildGraphSelectionV1} from "../🟦️.ts";

/** 🧺️ The one selection that serves two graph slice reads of the same tree and tool, or undefined when no single read can: two `tasks` selections that agree on `excludeDependencies` read the union of their tasks (the closure of a union is the union of the closures, so each asker finds its own tasks in it), as long as the union stays within `maximumTasks`; every other pair (`all`, `owners`, a differing dependency rule) stays two reads.
 * @see ../🧺️coalesce/🧪️tests/🟦️.ts */
export function coalesceBuildGraphSelectionsV1(current:BuildGraphSelectionV1,added:BuildGraphSelectionV1,maximumTasks:number):BuildGraphSelectionV1|undefined{
 if(current.kind!=="tasks"||added.kind!=="tasks"||(current.excludeDependencies===true)!==(added.excludeDependencies===true))return undefined;
 const tasks=new Map([...current.tasks,...added.tasks].map(task=>[JSON.stringify(task),task]));
 if(tasks.size>maximumTasks)return undefined;
 return readBuildGraphSelectionV1({kind:"tasks",tasks:[...tasks.values()],...current.excludeDependencies===true?{excludeDependencies:true}:{}});
}

/** 🛡️ Whether a read of `group` already contains what a read of `one` would produce and is worth waiting for: a `tasks` selection contains every `tasks` selection whose tasks it lists (one that follows dependencies also contains one that does not); every other selection contains only itself, because the read of the whole workspace is far longer than the scoped read of a task and must never be waited for by it.
 * @see ../🧺️coalesce/🧪️tests/🟦️.ts */
export function buildGraphSelectionCoversV1(group:BuildGraphSelectionV1,one:BuildGraphSelectionV1):boolean{
 if(group.kind!=="tasks"||one.kind!=="tasks")return buildGraphScopeKeyV1(group)===buildGraphScopeKeyV1(one);
 if(group.excludeDependencies===true&&one.excludeDependencies!==true)return false;
 const listed=new Set(group.tasks.map(task=>JSON.stringify(task)));
 return one.tasks.every(task=>listed.has(JSON.stringify(task)));
}
