/** 🧪️ Contract: a staging directory of `<store>/v1/tmp` is retired only when no live owner is proven (process identity pid + start time, never the pid alone) and nothing below it was written for the schema-declared grace; a live owner's directory is never touched, whatever its age. */
import {afterAll,expect,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,rmSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {ownProcessIdentityV1} from "../../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts";
import policy from "../🔣️.json" with {type:"json"};
import {STAGING_TREES_POLICY_V1,createStagingTreeV1,parseStagingTreeNameV1,reclaimStagingTreesV1} from "../🟦️.ts";

const roots:string[]=[],hour=3_600_000;
const store=():string=>{const root=mkdtempSync(join(tmpdir(),"staging-trees-"));roots.push(root);mkdirSync(join(root,"v1","tmp"),{recursive:true});return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const tmpOf=(root:string):string=>join(root,"v1","tmp");
const fill=(directory:string,bytes:number,ageMs:number,now:number):void=>{mkdirSync(directory,{recursive:true});const file=join(directory,"units.bin");writeFileSync(file,Buffer.alloc(bytes,1));const at=(now-ageMs)/1000;utimesSync(file,at,at);utimesSync(directory,at,at);};

test("the grace and the walk bound are declared in the schema-checked JSON",()=>{
 expect(validateJsonSchemaSubset(policy.schema as never,policy.policy)).toEqual([]);
 expect(STAGING_TREES_POLICY_V1.graceMilliseconds).toBe(hour);
 expect(STAGING_TREES_POLICY_V1.fileGraceMilliseconds).toBeGreaterThanOrEqual(STAGING_TREES_POLICY_V1.graceMilliseconds);
});

test("a creator registers the owner identity in the directory name and the name parses back to it",()=>{
 const root=store(),path=createStagingTreeV1(root,"g1-m01","cold-units"),name=path.slice(tmpOf(root).length+1),identity=ownProcessIdentityV1();
 expect(existsSync(path)).toBe(true);
 expect(parseStagingTreeNameV1(name)).toEqual({originId:"g1-m01",label:"cold-units",pid:identity.pid,startMs:identity.startMs});
 expect(parseStagingTreeNameV1("procedural3d-cold-units-am00uln3")).toBeUndefined();
 expect(parseStagingTreeNameV1("g1-m01.x.12.0.0123456789abcdef")).toBeUndefined();
});

test("a directory of a live owner stays whatever its age, a dead owner's goes after the grace, a pid reused by another process counts as dead",async()=>{
 const root=store(),now=Date.now(),tmp=tmpOf(root),mine=ownProcessIdentityV1();
 const name=(pid:number,startMs:number,tag:string)=>`g1-m01.cold-units.${pid}.${startMs}.${tag.repeat(16).slice(0,16)}`;
 fill(join(tmp,name(mine.pid,mine.startMs,"a")),100,48*hour,now);
 fill(join(tmp,name(999_999_001,1_700_000_000_000,"b")),100,2*hour,now);
 fill(join(tmp,name(mine.pid,mine.startMs-10*hour,"c")),100,2*hour,now);
 fill(join(tmp,name(999_999_002,1_700_000_000_000,"d")),100,hour/2,now);
 const alive=(holder:{pid:number;startMs:number})=>holder.pid===mine.pid&&holder.startMs===mine.startMs;
 const outcome=await reclaimStagingTreesV1(root,{originId:"g1-m01",now,alive});
 expect(outcome.trees).toBe(2);
 expect(outcome.bytes).toBeGreaterThanOrEqual(200);
 expect(outcome.kept).toEqual({live:1,young:1,incomplete:0});
 expect(existsSync(join(tmp,name(mine.pid,mine.startMs,"a")))).toBe(true);
 expect(existsSync(join(tmp,name(999_999_002,1_700_000_000_000,"d")))).toBe(true);
 expect(existsSync(join(tmp,name(999_999_001,1_700_000_000_000,"b")))).toBe(false);
 expect(existsSync(join(tmp,name(mine.pid,mine.startMs-10*hour,"c")))).toBe(false);
});

test("a directory of another machine or without a registered owner is judged by its newest write below it",async()=>{
 const root=store(),now=Date.now(),tmp=tmpOf(root),alive=()=>true;
 fill(join(tmp,"procedural3d-cold-units-am00uln3"),50,5*hour,now);
 fill(join(tmp,"g1-m02.cold-units.1.1700000000000.0123456789abcdef"),50,5*hour,now);
 fill(join(tmp,"draw-guest-fcc8.21L2Yt"),50,2*hour,now);
 fill(join(tmp,"busy-writer"),50,5*hour,now);writeFileSync(join(tmp,"busy-writer","fresh.bin"),"x");utimesSync(join(tmp,"busy-writer"),(now-5*hour)/1000,(now-5*hour)/1000);
 const outcome=await reclaimStagingTreesV1(root,{originId:"g1-m01",now,alive});
 expect(outcome.trees).toBe(3);
 expect(outcome.kept).toEqual({live:0,young:1,incomplete:0});
 expect(existsSync(join(tmp,"busy-writer"))).toBe(true);
 expect(existsSync(join(tmp,"procedural3d-cold-units-am00uln3"))).toBe(false);
 expect(existsSync(join(tmp,"g1-m02.cold-units.1.1700000000000.0123456789abcdef"))).toBe(false);
});

test("a tree larger than the walk bound is kept, files and links are not this sweep's, and a dry run removes nothing",async()=>{
 const root=store(),now=Date.now(),tmp=tmpOf(root);
 fill(join(tmp,"big"),10,5*hour,now);for(let index=0;index<6;index++)writeFileSync(join(tmp,"big",`f${index}`),"x"),utimesSync(join(tmp,"big",`f${index}`),(now-5*hour)/1000,(now-5*hour)/1000);
 utimesSync(join(tmp,"big"),(now-5*hour)/1000,(now-5*hour)/1000);
 writeFileSync(join(tmp,"loose-file"),"x");symlinkSync(tmp,join(tmp,"link"));
 fill(join(tmp,"old"),10,5*hour,now);
 expect((await reclaimStagingTreesV1(root,{now,maximumEntries:4,dryRun:true})).kept.incomplete).toBe(1);
 expect(existsSync(join(tmp,"big"))).toBe(true);
 const dry=await reclaimStagingTreesV1(root,{now,dryRun:true});
 expect(dry.trees).toBe(2);expect(existsSync(join(tmp,"old"))).toBe(true);
 await reclaimStagingTreesV1(root,{now});
 expect(existsSync(join(tmp,"old"))).toBe(false);expect(existsSync(join(tmp,"loose-file"))).toBe(true);expect(existsSync(join(tmp,"link"))).toBe(true);
});

test("a store without a tmp directory yields nothing and is not created",async()=>{
 const root=mkdtempSync(join(tmpdir(),"staging-trees-"));roots.push(root);
 expect(await reclaimStagingTreesV1(root,{})).toEqual({trees:0,bytes:0,kept:{live:0,young:0,incomplete:0}});
 expect(existsSync(join(root,"v1"))).toBe(false);
});
