/** 🧺️ Contract: the selections of concurrent graph slice reads merge into one read exactly when the closure of the union serves each asker, and a read that already holds a selection serves it. The rows are language-agnostic; the second oracle recomputes the union with plain set arithmetic on task identities.
 * @see ../🟦️.ts */
import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {buildGraphSelectionCoversV1,coalesceBuildGraphSelectionsV1} from "../🟦️.ts";
import {readBuildGraphSelectionV1} from "../../🟦️.ts";

const rows=JSON.parse(readFileSync(new URL("./🔣️.json",import.meta.url),"utf8")) as {unions:{name:string;current:any;added:any;maximumTasks:number;union:any}[];covers:{name:string;group:any;one:any;covers:boolean}[]};

test("the declared rows say which selections merge and what the merged selection is",()=>{
 for(const row of rows.unions)expect(coalesceBuildGraphSelectionsV1(readBuildGraphSelectionV1(row.current),readBuildGraphSelectionV1(row.added),row.maximumTasks)??null,row.name).toEqual(row.union===null?null:readBuildGraphSelectionV1(row.union));
});

test("the declared rows say which selection a read already contains",()=>{
 for(const row of rows.covers)expect(buildGraphSelectionCoversV1(readBuildGraphSelectionV1(row.group),readBuildGraphSelectionV1(row.one)),row.name).toBe(row.covers);
});

test("a merged selection contains both of its parts and the merge is commutative and idempotent",()=>{
 const tasks=["a:test","b:build","c:lint","d:test"].map(id=>({project:id.split(":")[0]!,target:id.split(":")[1]!}));
 for(const left of tasks)for(const right of tasks){
  const a=readBuildGraphSelectionV1({kind:"tasks",tasks:[left]}),b=readBuildGraphSelectionV1({kind:"tasks",tasks:[right]}),union=coalesceBuildGraphSelectionsV1(a,b,16)!;
  expect(buildGraphSelectionCoversV1(union,a)&&buildGraphSelectionCoversV1(union,b)).toBe(true);
  expect(coalesceBuildGraphSelectionsV1(b,a,16)).toEqual(union);
  expect(coalesceBuildGraphSelectionsV1(union,a,16)).toEqual(union);
  const oracle=new Set([left,right].map(task=>task.project+":"+task.target));
  expect(new Set((union as {tasks:{project:string;target:string}[]}).tasks.map(task=>task.project+":"+task.target))).toEqual(oracle);
 }
});
