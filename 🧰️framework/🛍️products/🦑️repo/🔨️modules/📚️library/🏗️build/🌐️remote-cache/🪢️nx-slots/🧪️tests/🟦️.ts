import {afterAll,expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {Database} from "bun:sqlite";
import {existsSync,mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {nxTaskSlotV1,readNxTaskRowsV1,registerNxSlotsV1,supersedeNxCacheV1,type NxTaskRowV1} from "../🟦️.ts";
import {slotIdV1} from "../../../🪢️slots/🟦️.ts";

const python=testPythonV1(),scratch:string[]=[];
const directory=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`nx-slot-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});

function database(path:string,rows:{hash:string;project:string;target:string;configuration:string|null;created?:string;accessed?:string}[]):void{
 const db=new Database(path);
 db.run("CREATE TABLE task_details (hash TEXT PRIMARY KEY NOT NULL, project TEXT NOT NULL, target TEXT NOT NULL, configuration TEXT)");
 db.run("CREATE TABLE cache_outputs (hash TEXT PRIMARY KEY NOT NULL, code INTEGER NOT NULL, size INTEGER NOT NULL, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, accessed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP)");
 for(const row of rows){
  db.run("INSERT INTO task_details VALUES (?,?,?,?)",[row.hash,row.project,row.target,row.configuration]);
  if(row.created)db.run("INSERT INTO cache_outputs VALUES (?,0,1,?,?)",[row.hash,row.created,row.accessed??row.created]);
 }
 db.close();
}

test.skipIf(!python)("the task rows read from an Nx workspace database equal an independent Python sqlite3 read",async()=>{
 const folder=directory("db"),path=join(folder,"nx.db");
 database(path,[{hash:"111",project:"@semio/a",target:"build",configuration:null,created:"2026-11-13 10:00:00",accessed:"2026-11-13 11:00:00"},{hash:"222",project:"@semio/a",target:"build",configuration:"production",created:"2026-11-13 12:00:00"},{hash:"333",project:"@semio/b",target:"test",configuration:null}]);
 const ours=await readNxTaskRowsV1(path);
 const run=Bun.spawnSync([python!,"-I","-c","import sqlite3,json,sys,datetime;c=sqlite3.connect('file:'+sys.argv[1]+'?mode=ro',uri=True);rows=c.execute('select d.hash,d.project,d.target,d.configuration,c.created_at,c.accessed_at from task_details d left join cache_outputs c on c.hash=d.hash order by d.hash').fetchall();ms=lambda v:None if v is None else int(datetime.datetime.strptime(v,'%Y-%m-%d %H:%M:%S').replace(tzinfo=datetime.timezone.utc).timestamp()*1000);print(json.dumps([[r[0],r[1],r[2],r[3],ms(r[4]),ms(r[5])] for r in rows]))",path],{stdout:"pipe",stderr:"pipe"});
 expect(run.exitCode,run.stderr.toString()).toBe(0);
 const model=JSON.parse(run.stdout.toString()) as [string,string,string,string|null,number|null,number|null][];
 expect([...ours].sort((a,b)=>a.hash<b.hash?-1:1).map(row=>[row.hash,row.project,row.target,row.configuration,row.createdAt,row.accessedAt])).toEqual(model);
});

const row=(hash:string,project:string,at:number|null,target="build",configuration:string|null=null):NxTaskRowV1=>({hash,project,target,configuration,createdAt:at,accessedAt:at});

test("the local Nx cache keeps the newest entry of every task slot; a hit makes an older entry current; the grace and unlisted directories are respected",async()=>{
 const cache=directory("cache");
 for(const hash of["h1","h2","h3","h4","h5","orphan"]){mkdirSync(join(cache,hash,"outputs"),{recursive:true});writeFileSync(join(cache,hash,"outputs","file"),"x".repeat(10));writeFileSync(join(cache,hash+".commit"),"");}
 const rows=[row("h1","a",1000),row("h2","a",2000),row("h3","a",3000,"build","production"),row("h4","b",1000,"test"),{...row("h5","b",500,"test"),accessedAt:4000},row("gone","c",100)];
 const forgotten:string[][]=[];
 const dry=await supersedeNxCacheV1({cacheDirectory:cache,rows,platform:"p",nowMs:5000,graceMilliseconds:100,dryRun:true,forget:async hashes=>{forgotten.push([...hashes]);}});
 expect(dry).toMatchObject({dryRun:true,scanned:5,current:3,removed:["h1","h4"],bytes:20});expect(existsSync(join(cache,"h1"))).toBe(true);expect(forgotten).toEqual([]);
 const applied=await supersedeNxCacheV1({cacheDirectory:cache,rows,platform:"p",nowMs:5000,graceMilliseconds:100,forget:async hashes=>{forgotten.push([...hashes]);}});
 expect(applied.removed).toEqual(["h1","h4"]);expect(forgotten).toEqual([["h1","h4"]]);
 for(const gone of["h1","h4"]){expect(existsSync(join(cache,gone))).toBe(false);expect(existsSync(join(cache,gone+".commit"))).toBe(false);}
 for(const kept of["h2","h3","h5","orphan"])expect(existsSync(join(cache,kept)),kept).toBe(true);
 const within=await supersedeNxCacheV1({cacheDirectory:cache,rows:[row("h2","a",2000),row("h3","a",2050)].map(r=>({...r,configuration:null})),platform:"p",nowMs:2100,graceMilliseconds:100,dryRun:true});
 expect(within.removed,"superseded 50 ms ago, inside the grace").toEqual([]);expect(within.retained).toBe(1);
});

test("platforms never supersede each other and the slot id follows the documented construction",()=>{
 const task={project:"a",target:"build",configuration:null};
 expect(nxTaskSlotV1(task,"darwin-arm64")).toBe(slotIdV1("nx-task",["a","build","","darwin-arm64"]));
 expect(nxTaskSlotV1(task,"darwin-arm64")).not.toBe(nxTaskSlotV1(task,"linux-x64"));
 expect(nxTaskSlotV1({...task,configuration:"production"},"p")).not.toBe(nxTaskSlotV1(task,"p"));
});

test("registering slots posts the claim in batches, carries the token, and never fails a build",async()=>{
 const calls:{url:string;body:{entries:{kind:string;key:string;slot:string}[]};authorization:string|null}[]=[];
 const fake=(async(url:string,init:RequestInit)=>{calls.push({url,body:JSON.parse(String(init.body)),authorization:new Headers(init.headers).get("authorization")});return Response.json({claimed:1,missing:0});}) as unknown as typeof fetch;
 const rows=Array.from({length:1500},(_,index)=>({hash:`h${index}`,project:"a",target:"build",configuration:null}));
 const result=await registerNxSlotsV1({endpoint:"http://cache.test",instance:"main",token:"secret",rows:[...rows,{hash:"BAD HASH",project:"a",target:"b",configuration:null}],platform:"p",fetch:fake});
 expect(result).toEqual({claimed:2,missing:0});expect(calls.map(call=>call.body.entries.length)).toEqual([1024,476]);
 expect(calls[0]!.url).toBe("http://cache.test/main/v1/slots");expect(calls[0]!.authorization).toBe("Bearer secret");expect(calls[0]!.body.entries[0]).toEqual({kind:"nx",key:"h0",slot:nxTaskSlotV1(rows[0]!,"p")});
 expect(await registerNxSlotsV1({endpoint:"http://x",instance:"main",rows:[],fetch:fake})).toEqual({claimed:0,missing:0});
 const refused=(async()=>new Response("no",{status:401})) as unknown as typeof fetch,broken=(async()=>{throw Error("down");}) as unknown as typeof fetch;
 expect(await registerNxSlotsV1({endpoint:"http://x",instance:"main",rows,fetch:refused})).toBeUndefined();expect(await registerNxSlotsV1({endpoint:"http://x",instance:"main",rows,fetch:broken})).toBeUndefined();
});
