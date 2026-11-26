/** 🧪️ Contract: a read that meets a crate without an owner row repairs the registry itself and goes on, advises only commands the catalogue declares, and says why when no repair is possible.
 * @see ../🟦️.ts */
import {afterEach,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import catalogue from "../../../../⚙️operations/🔣️catalog.json" with {type:"json"};
import graphSchema from "../../../🧬️schema/🔣️.json" with {type:"json"};
import generationSchema from "../../../🕰️generations/🧬️schema/🔣️.json" with {type:"json"};
import {renderGenerationMessageV1} from "../../../🕰️generations/🔌️load/🟨️.mjs";
import {withImplementationPinV1} from "../../../../📥️inputs/🔏️revision/🟨️.mjs";
import {buildRepairCommandV1} from "../../../../⚙️operations/🔤️options/🟦️.ts";
import {ownerRegistryRefusalV1,settleOwnerRegistryV1,withOwnerRegistryCustodyV1} from "../🟦️.ts";

const dirs:string[]=[];
afterEach(()=>{for(const dir of dirs.splice(0))rmSync(dir,{recursive:true,force:true});});
const scratch=()=>{const root=realpathSync(mkdtempSync(join(tmpdir(),"registry-follow-")));dirs.push(root);return root;};
const write=(root:string,path:string,body:string)=>{mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),body);};
const registryOf=(root:string)=>join(root,"📋️build-owners.json");
const rows=(root:string)=>(JSON.parse(readFileSync(registryOf(root),"utf8")).owners as {name:string;root:string}[]).map(row=>row.name);
const fault=(path:string)=>Object.assign(Error(`Native dependency has no build project owner: ${path} (required by p:build; candidates=1)`),{code:"BUILD_SOURCE_AUTHORITY"});
const signal=new AbortController().signal;
const settle=(root:string,failing:()=>string|undefined)=>withImplementationPinV1(()=>settleOwnerRegistryV1(root,()=>withOwnerRegistryCustodyV1(signal,async contain=>contain(async()=>{const path=failing();if(path!==undefined)throw fault(path);return "settled";}))));

const resolves=(line:string):boolean=>{
 const [frontend,script,id,...lane]=line.split(" ");
 if(frontend!=="repo")return false;
 if(script==="./📜️script.ts")return id==="verify"&&lane[0]==="build-owners"&&(catalogue as {repairs:{command:string}[]}).repairs.some(row=>row.command===line);
 return (catalogue.operations as {name:string}[]).some(row=>row.name===script);
};
const advised=(text:string)=>[...text.matchAll(/`(repo [^`]+)`/gu)].map(match=>match[1]!);

test("the repair command comes from the catalogue and every advised command resolves, in English and German",()=>{
 const command=buildRepairCommandV1("build-owners-sync");
 expect(command).toBe("repo ./📜️script.ts verify build-owners sync");
 expect(resolves(command)).toBe(true);
 expect(()=>buildRepairCommandV1("no-such-repair")).toThrow("Declared repair required");
 for(const language of ["en","de"] as const){
  const refusal=ownerRegistryRefusalV1({roots:["a/b"],rounds:2,moved:false,cause:Error("c")}).message;
  const pending=renderGenerationMessageV1("pendingCrate",{path:"a/b/Cargo.toml",command},language);
  for(const text of [refusal,pending]){
   expect(text).not.toContain("workspace:verify");
   expect(text).not.toContain("{command}");
   expect(text).toContain(command);
  }
  expect(advised(refusal).every(resolves)).toBe(true);
  expect(advised(refusal).length).toBe(2);
 }
 for(const table of [graphSchema.definitions.OwnerRegistryRefusal.const,generationSchema.definitions.Messages.const.en,generationSchema.definitions.Messages.const.de] as Record<string,unknown>[])for(const value of Object.values(table))if(typeof value==="string")for(const line of advised(value.replaceAll("{command}",command)))expect(resolves(line),line).toBe(true);
});

test("one refusal repairs every ownerless crate of the tree, however many a peer wrote, and the read goes on",async()=>{
 const root=scratch();
 for(let index=0;index<9;index++)write(root,`crates/c${index}/Cargo.toml`,`[package]\nname = "crate-${index}"\n`);
 write(root,"📋️build-owners.json",JSON.stringify({version:1,owners:[],catalogProviders:[]}));
 let attempts=0;
 await expect(settle(root,()=>++attempts===1?"crates/c0":undefined)).resolves.toBe("settled");
 expect(attempts).toBe(2);
 expect(rows(root)).toEqual(Array.from({length:9},(_,index)=>`crate-${index}`));
});

test("a sweep adds rows only: rows of vanished or moved modules are left to the canonical sync",async()=>{
 const root=scratch();
 write(root,"crates/new/Cargo.toml",'[package]\nname = "crate-new"\n');
 write(root,"📋️build-owners.json",JSON.stringify({version:1,owners:[{name:"gone",root:"gone",manifest:"gone/Cargo.toml",packages:[]}],catalogProviders:[]}));
 await settle(root,(()=>{let first=true;return ()=>first?(first=false,"crates/new"):undefined;})());
 expect(rows(root)).toEqual(["crate-new","gone"]);
});

test("a repair that cannot add a row refuses at once with the reason, the repair command and the error the repair met",async()=>{
 const root=scratch();
 write(root,"a/Cargo.toml",'[workspace]\nmembers = []\n');
 write(root,"b/Cargo.toml",'[package]\nname = "taken"\n');
 write(root,"c/Cargo.toml",'[package]\nname = "taken"\n');
 write(root,"📋️build-owners.json",JSON.stringify({version:1,owners:[{name:"taken",root:"b",manifest:"b/Cargo.toml",packages:[]}],catalogProviders:[]}));
 for(const [path,reason] of [["missing/rust","missing/rust/Cargo.toml is absent"],["a","declares no [package] name"],["c","already belongs to the row at b"]] as const){
  let caught:any;
  try{await settle(root,()=>path);}catch(error){caught=error;}
  expect(caught?.code).toBe("BUILD_OWNER_REGISTRY_INCONSISTENT");
  expect(caught.message).toContain(reason);
  expect(caught.message).toContain(buildRepairCommandV1("build-owners-sync"));
 }
});

test("a repair that throws is reported in the refusal instead of being swallowed",async()=>{
 const root=scratch();
 write(root,"crates/c0/Cargo.toml",'[package]\nname = "crate-0"\n');
 write(root,"📋️build-owners.json","{not json");
 let caught:any;
 try{await settle(root,()=>"crates/c0");}catch(error){caught=error;}
 expect(caught?.code).toBe("BUILD_OWNER_REGISTRY_INCONSISTENT");
 expect(caught.message).toContain("the repair failed");
});
