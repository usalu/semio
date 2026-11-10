/** 🛡️ Contract: no change of the tree is ever missed by the directory stamps. An independent Python implementation builds the fixture, records the facts, applies each mutation (add, remove, rename, replace a file by a directory and back, retarget a symlink, rename over an entry, touch) and says which facts still hold; the stamp-based doubt must flag every fact that no longer holds. */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {createDirectoryStampsV1} from "../../🗂️directory-stamps/🟨️.mjs";
import {createEpochDoubtV1} from "../../🔎️doubt/🟨️.mjs";

const oracle=fileURLToPath(new URL("./🐍️.py",import.meta.url)),python=process.env.SEMIO_PYTHON??"python3";
const run=(...args:string[])=>{const result=spawnSync(python,["-I",oracle,...args],{encoding:"utf8",maxBuffer:1<<26});expect(result.status,result.stderr).toBe(0);return result.stdout;};
const MUTATIONS=["add","add-candidate","remove","rename","replace-file-with-dir","retarget-symlink","rename-over","replace-dir-with-file","touch","nested-add","remove-link-target","replace-link-target-with-dir","add-missing-tree","add-deep-missing-tree","none"];

for(const mutation of MUTATIONS)test("stamps miss no change: "+mutation,async()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"stamp-sound-")));
 try{
  run("build",root);
  const rows=JSON.parse(run("rows",root)) as {kind:string;path:string;fact:unknown}[],file=join(root,"stamps.json");
  const stamps=createDirectoryStampsV1({file}),doubt=createEpochDoubtV1({stamps});
  await doubt.published({rows:rows as never,observedAt:Date.now()});
  run("mutate",root,mutation);
  const rowsFile=join(root,"rows.json");writeFileSync(rowsFile,JSON.stringify(rows));
  const truth=JSON.parse(run("truth",rowsFile)) as boolean[];
  const fresh=createEpochDoubtV1({stamps:createDirectoryStampsV1({file})}),flags=Array.from(await fresh.classify(rows as never));
  const missed=rows.filter((_,at)=>!truth[at]&&flags[at]===0).map(row=>row.kind+" "+row.path.replace(root,""));
  expect(missed,"changed facts the stamps vouched for").toEqual([]);
  const changed=truth.filter(value=>!value).length,vouched=flags.filter(flag=>flag===0).length;
  if(!["none","touch","rename-over"].includes(mutation))expect(changed,"the mutation must change a fact").toBeGreaterThan(0);
  if(mutation==="none")expect(vouched,"an untouched tree is mostly vouched for").toBeGreaterThan(rows.length/2);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a change inside the racy window of the stamp is never vouched for",async()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"stamp-racy-")));
 try{
  run("build",root);run("mutate",root,"touch");
  const rows=JSON.parse(run("rows",root)) as {kind:string;path:string;fact:unknown}[],file=join(root,"stamps.json");
  await createEpochDoubtV1({stamps:createDirectoryStampsV1({file})}).published({rows:rows as never,observedAt:Date.now()});
  run("mutate",root,"add");
  const rowsFile=join(root,"rows.json");writeFileSync(rowsFile,JSON.stringify(rows));
  const truth=JSON.parse(run("truth",rowsFile)) as boolean[],flags=Array.from(await createEpochDoubtV1({stamps:createDirectoryStampsV1({file})}).classify(rows as never));
  expect(rows.filter((_,at)=>!truth[at]&&flags[at]===0)).toEqual([]);
 }finally{rmSync(root,{recursive:true,force:true});}
});
