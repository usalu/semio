/** 🪪️ Law: the providers project is read as two fields of its configuration, never as a file. An edit of any other line of the configuration holds every row that names the providers project (so no project is inferred again), an edit of the name or of the designation it declares does not hold, and a configuration that does not declare the designation is refused. */
import {expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {captureObservedSourceInputsV1,observationsHold} from "../../../../👁️observation/🟨️.mjs";
import {libraryBootstrap} from "../../🟨️.mjs";
import {providerProjectOfConfigV1} from "../🟨️.mjs";

const write=(path:string,value:unknown)=>writeFileSync(path,typeof value==="string"?value:JSON.stringify(value));
const declared=(extra:Record<string,unknown>={},name="providers",designation="🔣️.json")=>({name,metadata:{semio:{scriptProviders:designation}},targets:{build:{executor:"repo:script"}},...extra});

test("the providers project is observed as fields: any other edit holds, the name or the designation does not, a missing designation is refused",async()=>{
 await libraryBootstrap;
 const root=mkdtempSync(join(realpathSync(tmpdir()),"provider-project-")),config=join(root,"📋️project.json");
 try{
  mkdirSync(root,{recursive:true});write(config,declared());
  const recorded=await captureObservedSourceInputsV1(async()=>providerProjectOfConfigV1(config,"🔣️.json"),undefined,false) as any;
  expect(recorded.value).toBe("providers");
  const rows=recorded.observations as {kind:string;path:string}[];
  expect(rows.some(row=>row.kind==="file"&&row.path===config),"no content row of the configuration").toBe(false);
  expect(rows.filter(row=>row.kind==="field"&&row.path===config).length).toBe(2);
  expect(await observationsHold(rows as never)).toBe(true);
  write(config,JSON.stringify(declared({description:"edited",targets:{build:{executor:"repo:script"},test:{executor:"repo:script"}}}),null,4)+"\n");
  expect(await observationsHold(rows as never),"a body-only edit of the configuration").toBe(true);
  write(config,declared({},"renamed"));
  expect(await observationsHold(rows as never),"the name changed").toBe(false);
  write(config,declared({},"providers","other.json"));
  expect(await observationsHold(rows as never),"the designation changed").toBe(false);
  expect(()=>providerProjectOfConfigV1(config,"🔣️.json"),"a project that declares another designation").toThrow();
  write(config,{name:"providers"});
  expect(()=>providerProjectOfConfigV1(config,"🔣️.json"),"no designation declared").toThrow();
  write(config,"{not json");
  expect(()=>providerProjectOfConfigV1(config,"🔣️.json")).toThrow();
 }finally{rmSync(root,{recursive:true,force:true});}
});
