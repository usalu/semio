import {mkdirSync,mkdtempSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {expect,test} from "bun:test";
import budgetPolicy from "../../../🔒️enforcement/📦️budget/🔣️policy.json" with {type:"json"};
import fixtures from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {cacheAreaHomeV1} from "../../🟦️.ts";

for(const row of fixtures.cases)test(`🏠️ home of ${fixtures.area}: ${row.name}`,()=>{
 const workspace=realpathSync(mkdtempSync(join(tmpdir(),"semio-home-")));
 try{
  const partition=(budgetPolicy.partitions as Record<string,{directory:string}>).dependencies!.directory;
  if(row.partitionDirectory)mkdirSync(join(workspace,partition),{recursive:true});
  const expected=process.platform==="darwin"?row.darwin:row.other;
  if(expected==="detached")expect(()=>cacheAreaHomeV1(workspace,fixtures.area)).toThrow(expect.objectContaining({name:"CacheAreaDetachedError"}));
  else expect(cacheAreaHomeV1(workspace,fixtures.area).directory.startsWith(join(workspace,partition))).toBe(true);
 }finally{rmSync(workspace,{recursive:true,force:true});}
});
