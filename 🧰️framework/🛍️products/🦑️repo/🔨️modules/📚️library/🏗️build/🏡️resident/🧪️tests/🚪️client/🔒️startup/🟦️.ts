import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {existsSync,lstatSync,mkdtempSync,readFileSync,readdirSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,resolve} from "node:path";
import {setTimeout as pause} from "node:timers/promises";
import {fileURLToPath} from "node:url";
import schema from "../../../🚪️client/🛰️plan/🧬️schema/🔣️.json" with {type:"json"};
import cases from "./🔣️.json" with {type:"json"};

const runner=fileURLToPath(new URL("./📜️script.ts",import.meta.url)),validate=new Ajv({strict:true}).compile(schema.definitions.StartupCustodyCase);
for(const row of cases)test("resident startup custody "+row.mode,async()=>{
 expect(validate(row)).toBe(true);
 const root=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"s-")),children:ReturnType<typeof Bun.spawn>[]=[],name=Buffer.from(root).toString("hex").slice(-64),db=new Database(":memory:");
 const path=(name:string)=>join(root,name),write=(name:string,value="")=>writeFileSync(path(name),value),read=(name:string)=>readFileSync(path(name),"utf8"),exists=(name:string)=>existsSync(path(name));
 const wait=async(...files:string[])=>{for(let end=performance.now()+3500;!files.some(exists);){if(performance.now()>end)throw Error("Fixture waiting for "+files.join(", ")+"; found "+readdirSync(root).join(", "));await pause(10);}};
 const launch=(role:string,mode="")=>{const child=Bun.spawn([process.execPath,runner,role,name,mode],{cwd:root,stdout:"ignore",stderr:"inherit"});children.push(child);return child;};
 const lockIdentity=(lock:string)=>{const stat=lstatSync(lock,{bigint:true});return [stat.dev,stat.ino].map(String);};
 try{
  const a=launch("a",row.mode==="live-empty"?"held-empty":row.mode==="owner-timeout"?"no-socket":row.mode==="restart"?"restart":"");
  await wait("a.start","a.outcome");expect(exists("a.start")).toBe(true);
  const lock=resolve(root,read("a.start"));
  expect(lstatSync(lock).isFile()).toBe(true);expect(lstatSync(lock).size).toBe(0);
  if(process.platform!=="win32"){expect(lstatSync(lock).mode&0o777).toBe(0o600);expect(lstatSync(dirname(lock)).mode&0o777).toBe(0o700);}
  if(row.mode==="late-release")renameSync(lock,lock+".old");
  await pause(35);
  launch("b",row.mode==="waiter-cancel"?"cancel":row.mode==="restart"?"restart":"");
  await wait("b.waiting","b.start","b.outcome","b.enter");
  if(row.mode==="restart"){
   await pause(150);
   write("a.go");await wait("a.outcome");await wait("b.outcome","b.start");
   expect(exists("b.start")).toBe(false);expect(read("b.outcome")).toBe("joined");
  }else if(row.mode==="late-release"){
   expect(exists("b.start")).toBe(true);
   const current=lockIdentity(lock);write("a.go","fail");
   await wait("a.outcome");expect(read("a.outcome")).toBe("fixture released");
   expect(lockIdentity(lock)).toEqual(current);
   write("b.go");await wait("b.outcome");expect(read("b.outcome")).toBe("started");
  }else{
   expect(exists("b.waiting")).toBe(true);expect(exists("b.start")).toBe(false);
   if(row.mode==="abandoned"||row.mode==="owner-timeout"){
    const original=lockIdentity(lock);
    if(row.mode==="abandoned"){a.kill("SIGKILL");await a.exited;}
    else{await wait("a.outcome");expect(read("a.outcome")).toBe("timeout");}
    await wait("b.start","b.outcome");expect(lockIdentity(lock)).toEqual(original);
    write("b.go");await wait("b.outcome");expect(read("b.outcome")).toBe("started");
   }else{
    if(row.mode==="waiter-cancel"){write("b.cancel");await wait("b.outcome");expect(read("b.outcome")).toBe("cancelled");expect(a.exitCode).toBe(null);}
    write("a.go");await wait("a.outcome");expect(read("a.outcome")).toBe("started");
    if(row.mode!=="waiter-cancel"){await wait("b.outcome");expect(read("b.outcome")).toBe("joined");}
   }
  }
  expect(db.query("select count(*) starts from json_each(?1) where value like '%.start'").get(JSON.stringify(readdirSync(root)))).toEqual({starts:row.starts});
 }finally{
  write("done");for(const child of children)if(child.exitCode===null)child.kill("SIGKILL");
  await Promise.all(children.map(child=>child.exited));db.close();rmSync(root,{recursive:true,force:true});
 }
},12000);
