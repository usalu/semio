import {afterAll,expect,test} from "bun:test";
import {spawn} from "node:child_process";
import {mkdtempSync,readdirSync,readFileSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {enterRestartScopeV1,restartExpectedV1,settleRestartCustodiesV1,staleImplementationV1} from "../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import {parkGrantForRestartV1} from "../../🟦️.ts";

const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(tmpdir(),"hr-"));roots.push(root);return root;};
afterAll(async()=>{await settleRestartCustodiesV1();for(const root of roots)rmSync(root,{recursive:true,force:true});});
const grants=(directory:string)=>readdirSync(join(directory,"grants")),queue=(directory:string)=>readdirSync(join(directory,"queue"));
const run=(script:string,environment:Record<string,string>)=>new Promise<{code:number|null;out:string[];err:string}>(accept=>{
 const child=spawn(process.execPath,[script],{env:{...process.env,...environment},stdio:["ignore","pipe","pipe"]});
 let out="",err="";child.stdout.on("data",chunk=>{out+=chunk;});child.stderr.on("data",chunk=>{err+=chunk;});
 child.on("close",code=>accept({code,out:out.split("\n").filter(Boolean),err}));
});
const script=fileURLToPath(new URL("./📜️child/🟦️.ts",import.meta.url));

test("the restart wrapper hands the grant of a stale command to its real replacement process, which reports it as adopted and runs once",async()=>{
 const directory=fresh(),result=await run(script,{HC_DIRECTORY:directory,HC_MEGABYTES:"64",LANG:"en_US.UTF-8"});
 const rows=result.out.map(row=>JSON.parse(row));
 expect(result.code).toBe(0);
 expect(rows.length).toBe(4);
 expect(rows[0]).toMatchObject({adopted:false,attempt:null});
 expect(rows[1]).toMatchObject({adopted:true,id:rows[0].id,attempt:"1",ppid:rows[0].pid,queue:0});
 expect(rows[1].pid).not.toBe(rows[0].pid);expect(rows[1].grantPid).toBe(rows[1].pid);
 expect(rows[2]).toMatchObject({outcome:"completed",pid:rows[1].pid});
 expect(rows[3]).toMatchObject({outcome:"reexecuted",code:0,pid:rows[0].pid});
 expect(result.err).toContain("[capacity] keeping the run grant");expect(result.err).toContain("[capacity] took over the run grant");expect(result.err).not.toContain("waiting");
 expect(grants(directory)).toEqual([]);expect(queue(directory)).toEqual([]);
},60000);

test("a grant is parked only for a failure a fresh process cures and only while an outermost restart wrapper is open; the door's gate passes that predicate",async()=>{
 const stale=staleImplementationV1("This command runs an outdated implementation"),ordinary=Error("ordinary failure");
 expect(restartExpectedV1()).toBe(false);expect(await parkGrantForRestartV1(stale)).toBe(false);
 const nested=enterRestartScopeV1(false);expect(restartExpectedV1()).toBe(false);expect(await parkGrantForRestartV1(stale)).toBe(false);
 const outermost=enterRestartScopeV1(true);
 expect(restartExpectedV1()).toBe(true);expect(await parkGrantForRestartV1(stale)).toBe(true);expect(await parkGrantForRestartV1(ordinary)).toBe(false);
 outermost();expect(restartExpectedV1()).toBe(false);nested();
 const source=readFileSync(fileURLToPath(new URL("../../🟦️.ts",import.meta.url)),"utf8");
 expect(source).toContain("withHostCapacityV1({park:parkGrantForRestartV1,...ports,");
});
