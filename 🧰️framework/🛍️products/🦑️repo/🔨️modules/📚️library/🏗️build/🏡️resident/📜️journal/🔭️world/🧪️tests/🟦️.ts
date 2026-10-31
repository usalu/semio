import {verifyWorldWitnessV1 as verify} from "../../../../🔭️witness/🟨️.mjs";
import {residentWatcherV1} from "../../../🧪️tests/🟦️.ts";
import {createWatchSupervisorV1} from "../../../🔭️watch/🟨️.mjs";
import {POLICY_V1} from "../../../📨️protocol/🎚️policy/🟨️.mjs";
import {afterAll,beforeAll,expect,test} from "bun:test";
import {chmodSync,existsSync,lstatSync,mkdirSync,mkdtempSync,realpathSync,renameSync,rmSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {createChangeJournalClientV1} from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🧭️client/🟦️.ts";
import {checkWorldViaJournalV1,symlinkedDirectoriesV1,type WorldRowV1,type WorldVerdictV1} from "../🟦️.ts";
import {runChangeJournalServiceV1} from "../../🟦️.ts";

const binary=await residentWatcherV1();
let sandbox="",service:Awaited<ReturnType<typeof runChangeJournalServiceV1>>|undefined;

beforeAll(async()=>{
 sandbox=realpathSync(mkdtempSync(join(tmpdir(),"journal-world-")));
 service=await runChangeJournalServiceV1({root:sandbox,directory:join(sandbox,".journal"),fence:".fence",exclude:{rooted:[".journal","skipped"],names:["node_modules"]},binary,watcherPolicy:POLICY_V1,createSupervisor:createWatchSupervisorV1,residentBytes:()=>0});
});
afterAll(async()=>{await service?.stop("test");if(sandbox)rmSync(sandbox,{recursive:true,force:true});});

const identity=(file:string):string=>{try{const f=lstatSync(file);return[f.dev,f.ino,f.mode,f.size,f.mtimeMs,f.ctimeMs].join(":");}catch(error){return"!"+(error as NodeJS.ErrnoException).code;}};
const world=(files:string[]):WorldRowV1[]=>files.map(file=>["l",file,identity(file)] as WorldRowV1);

test("a world checked through the journal gets the verdict the stat check gives, for edits, touches, renames, removals, appearing paths, mode changes, links and unreported regions, with stat questions only for what the journal reports",async()=>{
 const client=createChangeJournalClientV1({directory:join(sandbox,".journal"),root:sandbox});
 const files:string[]=[];
 for(let directory=0;directory<20;directory++){mkdirSync(join(sandbox,"src","d"+directory),{recursive:true});for(let file=0;file<25;file++){const path=join(sandbox,"src","d"+directory,"f"+file+".ts");writeFileSync(path,"v0 "+path);files.push(path);}}
 mkdirSync(join(sandbox,"skipped"),{recursive:true});writeFileSync(join(sandbox,"skipped/s.ts"),"s");files.push(join(sandbox,"skipped/s.ts"));
 mkdirSync(join(sandbox,"real"),{recursive:true});writeFileSync(join(sandbox,"real/r.ts"),"r");symlinkSync(join(sandbox,"real"),join(sandbox,"linked"));files.push(join(sandbox,"linked/r.ts"));
 const absent=[join(sandbox,"src/d0/new.ts"),join(sandbox,"src/d1/missing/deep.ts")];
 await new Promise(resolve=>setTimeout(resolve,50));
 const cursor=await client.cursor() as {epoch:string;seq:number};
 const rows:WorldRowV1[]=[...world(files),...world(absent),["e","HOME","x"],["l","/etc/hosts",identity("/etc/hosts")]];
 const linked=await symlinkedDirectoriesV1(sandbox,rows.map(row=>row[1]).filter(subject=>subject.startsWith(sandbox)));
 expect(linked).toEqual(["linked"]);
 const stat=(subset:readonly WorldRowV1[])=>verify({observations:subset.filter(row=>row[0]!=="e")},{}) as Promise<WorldVerdictV1>;
 const ask=()=>checkWorldViaJournalV1({client,cursor,workspace:sandbox,rows,linked,stat});
 const quiet=await ask();
 expect(quiet).toMatchObject({via:"journal"});expect(quiet.verdict.holds).toBe(true);
 expect(quiet.statted).toBeLessThan(10);expect(quiet.journalled).toBeGreaterThan(450);
 const cases:[string,()=>void][]=[
  ["rewrite",()=>writeFileSync(files[3]!,"changed content")],
  ["touch",()=>{const at=new Date(Date.now()+5000);utimesSync(files[40]!,at,at);}],
  ["rename away",()=>renameSync(files[60]!,files[60]+".away")],
  ["appear",()=>writeFileSync(absent[0]!,"new")],
  ["mode",()=>chmodSync(files[90]!,0o600)],
  ["remove",()=>rmSync(files[120]!)],
  ["through the link",()=>writeFileSync(join(sandbox,"real/r.ts"),"changed through the target")],
  ["unreported region",()=>writeFileSync(join(sandbox,"skipped/s.ts"),"changed in an unreported region")],
 ];
 for(const[name,act]of cases){
  act();
  const through=await ask(),direct=await stat(rows);
  expect({name,via:through.via,holds:through.verdict.holds}).toEqual({name,via:"journal",holds:direct.holds});
  expect({name,holds:direct.holds}).toEqual({name,holds:false});
  if(name!=="through the link"&&name!=="unreported region")expect(through.statted).toBeLessThan(20);
 }
 expect(await checkWorldViaJournalV1({client,cursor:undefined,workspace:sandbox,rows,stat})).toMatchObject({via:"stat",refusal:"cursor",statted:rows.length});
});
