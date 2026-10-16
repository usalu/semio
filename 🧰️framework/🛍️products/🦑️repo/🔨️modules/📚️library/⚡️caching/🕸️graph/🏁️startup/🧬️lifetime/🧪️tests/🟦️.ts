import{createIssuedResourceControlV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🔣️.json" with {type:"json"};
import {createNxDaemonLifetimeV1,type NxDaemonLifetimeEventV1} from "../🟦️.ts";

function sqliteOracle(events:readonly Readonly<{type:string;resource?:string;id?:string}>[]){
 const db=new Database(":memory:");db.exec("create table state(phase text, parentHeld integer, server integer);insert into state values('awaiting-acquisition',1,0);create table loans(name text primary key);create table children(kind text,id text,primary key(kind,id));");let accepted=true;
 try{for(const event of events){db.exec("begin");try{const state=db.query("select * from state").get() as {phase:string;parentHeld:number;server:number};if(state.phase==="terminal")throw Error("settled");let query="",args:(string|number)[]=[];
  switch(event.type){
   case "acquire":if(state.phase!=="awaiting-acquisition")throw Error("phase");query="insert into loans values(?)";args=[event.resource!];break;
   case "acquired":query="update state set phase='acquired' where phase='awaiting-acquisition' and (select count(*) from loans)=4";break;
   case "server-open":query="update state set server=1 where phase='acquired' and server=0";break;
   case "ready":query="update state set phase='ready' where phase='acquired' and server=1";break;
   case "parent-exit":query="update state set parentHeld=0,phase=case when phase='awaiting-acquisition' then 'draining' else phase end where parentHeld=1";break;
   case "drain":query="update state set phase='draining'";break;
   case "server-close":query="update state set server=0 where phase='draining' and server=1";break;
   case "worker-open":if(!['acquired','ready'].includes(state.phase))throw Error("phase");query="insert into children values('worker',?)";args=[event.id!];break;
   case "request-open":if(state.phase!=="ready")throw Error("phase");query="insert into children values('request',?)";args=[event.id!];break;
   case "worker-close":case "request-close":query="delete from children where kind=? and id=?";args=[event.type.split('-')[0]!,event.id!];break;
   case "terminal":query="update state set phase='terminal',parentHeld=0 where phase='draining' and server=0 and (select count(*) from children)=0";break;
   default:throw Error("event");
  }
  if(db.query(query).run(...args).changes!==1)throw Error("refused");if(event.type==="terminal")db.exec("delete from loans");db.exec("commit");
 }catch{db.exec("rollback");accepted=false;break;}}
 const state=db.query("select phase,parentHeld from state").get() as {phase:string;parentHeld:number};return {accepted,phase:state.phase,parentHeld:!!state.parentHeld,loans:(db.query("select count(*) as n from loans").get() as {n:number}).n,workers:(db.query("select count(*) as n from children where kind='worker'").get() as {n:number}).n,requests:(db.query("select count(*) as n from children where kind='request'").get() as {n:number}).n};
 }finally{db.close();}
}

/** 🧬️ Language-neutral transitions agree with independent SQLite constraints before actual daemon integration. */
test("detached daemon root custody follows exact acquisition and terminal transitions",()=>{
 const ajv=new Ajv({strict:true}),validateBirth=ajv.compile(schema),validateEvent=ajv.compile(schema.$defs.Event);expect(validateBirth(fixture.birth)).toBe(true);
 for(const row of fixture.cases){const lifetime=createNxDaemonLifetimeV1(fixture.birth);let accepted=true;for(const event of row.events){expect(validateEvent(event)).toBe(true);try{lifetime.record(event as NxDaemonLifetimeEventV1);}catch{accepted=false;break;}}
  const state=lifetime.snapshot(),actual={accepted,phase:String(state.phase),parentHeld:state.parentHeld,loans:state.loans.length,workers:state.workers.length,requests:state.requests.length},expected={accepted:row.accepted,phase:row.phase,parentHeld:row.parentHeld,loans:row.loans,workers:row.workers,requests:row.requests};expect(actual).toEqual(expected);expect(sqliteOracle(row.events)).toEqual(expected);
 }
 console.log("[DEBUG] Daemon neutral lifecycle/SQLite transition agreement "+fixture.cases.length+" cases; model is not service authority or physical image proof");
});

/** 🚧️ Wire values cannot smuggle runtime authority, alternate scopes or malformed lifecycle messages. */
test("daemon lifetime descriptor and events are closed before Source issuance",()=>{
 const validate=new Ajv({strict:true}).compile(schema),birth=fixture.birth;for(const [value,wireAccepted] of [[{...birth,deadlineEpochMilliseconds:null},false],[{...birth,repositoryRoot:""},false],[{...birth,toolingIdentity:"unknown"},false],[{...birth,toolkitRoot:birth.toolkitRoot+"/../alternate"},true],[{...birth,parentWitness:"123"},false]] as const){expect(validate(value)).toBe(wireAccepted);expect(()=>createNxDaemonLifetimeV1(value)).toThrow();}
 const lifetime=createNxDaemonLifetimeV1(birth);for(const value of [{type:"acquire",resource:"graph"},{type:"ready",original:{}},{type:"worker-open",id:"0"},{type:"release",resource:"sdk"}])expect(()=>lifetime.record(value as NxDaemonLifetimeEventV1)).toThrow();expect(lifetime.snapshot().loans).toEqual([]);expect(lifetime.snapshot().phase).toBe("awaiting-acquisition");
 const snapshot=lifetime.snapshot();expect(Object.isFrozen(snapshot)).toBe(true);expect(Object.isFrozen(snapshot.loans)).toBe(true);expect(snapshot.birth.repositoryRoot).not.toBe(snapshot.birth.workspaceRoot);
});

import {readNxDaemonServiceHandoffV1} from "../../🚦️service/🟦️.ts";
/** 🔐️ A structural descriptor cannot replace the live Original parent witness at service birth. */
test("daemon service refuses unissued or changed parent handoff before image acquisition",async()=>{
 for(const value of [undefined,"{}",JSON.stringify({birth:fixture.birth,witness:{}}),JSON.stringify({birth:fixture.birth,witness:{nonce:fixture.birth.parentWitness},original:{}})])expect(()=>readNxDaemonServiceHandoffV1(value)).toThrow();
});

import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import {parse} from "@babel/parser";
/** 🏁️ Independent JavaScript parsing verifies actual detached entry orders ownership before Native imports. */
test("authored daemon entry enters service custody before compile-cache and server imports",()=>{
 const patch=readFileSync(resolve(import.meta.dir,"../../../../🩹️patches/nx@23.2.0.patch"),"utf8"),section=patch.split("diff --git a/dist/src/daemon/server/start.js b/dist/src/daemon/server/start.js\n")[1]?.split("diff --git ")[0];expect(section).toBeDefined();
 const source=section!.split("\n").filter(line=>(line.startsWith("+")&&!line.startsWith("+++ "))||line.startsWith(" ")).map(line=>line.slice(1)).join("\n"),ast=parse(source,{sourceType:"script"}),serialized=JSON.stringify(ast);expect(serialized).toContain("withNxDaemonServiceV1");expect(source.indexOf("withNxDaemonServiceV1")).toBeLessThan(source.indexOf('require("../../utils/enable-compile-cache")'));expect(source.indexOf("withNxDaemonServiceV1")).toBeLessThan(source.indexOf('require("./server")'));expect(source).toContain("waitForTerminal");
});

import {mkdtempSync,mkdirSync,rmSync} from "node:fs";
import {join} from "node:path";
import {boundedNxNamespaceV1} from "../../../../💾️storage/🔒️enforcement/🔌️nx/🟦️.ts";
import {boundedNxSocketNamespaceV1} from "../../../../💾️storage/🔒️enforcement/🔌️nx/🟦️.ts";
import socketSchema from "../../../../💾️storage/🔒️enforcement/🔌️nx/🧬️schema/🔌️socket.json" with {type:"json"};
import socketFixture from "./🔌️socket.json" with {type:"json"};
/** 🔌️ A closed neutral namespace and an independent digest oracle constrain real socket byte paths. */
test("daemon sockets use a short funded namespace for each watched root and reject byte overflow",()=>{
 const validate=new Ajv({strict:true}).compile(socketSchema),tooling="a".repeat(64),socketRoot=resolve("/cache/l/m"),roots=new Set<string>();for(const row of socketFixture.cases){const workspaceRoot=row.workspaceRoot.startsWith("/")?resolve(row.workspaceRoot):row.workspaceRoot,selection={version:1,toolingIdentity:tooling,workspaceRoot,graphIdentity:row.graphIdentity,implementationIdentity:"d".repeat(64),watcherIdentity:row.watcherIdentity};if(!row.accepted){expect(()=>boundedNxSocketNamespaceV1(socketRoot,tooling,workspaceRoot,row.graphIdentity,"d".repeat(64),row.watcherIdentity??undefined)).toThrow();continue;}expect(validate(selection)).toBe(true);const namespace=boundedNxSocketNamespaceV1(socketRoot,tooling,workspaceRoot,row.graphIdentity,"d".repeat(64),row.watcherIdentity??undefined),key=new Bun.CryptoHasher("sha256").update(JSON.stringify([tooling,workspaceRoot.replaceAll("\\","/"),row.graphIdentity,"d".repeat(64),row.watcherIdentity])).digest("hex").slice(0,24);expect(namespace.environment.NX_SOCKET_DIR).toBe(join(socketRoot,"s",key));expect(namespace.environment.SEMIO_NX_SOCKET_STORAGE).toBe(socketRoot);expect(namespace.environment.NX_DAEMON_SOCKET_DIR).toBeUndefined();expect(Buffer.byteLength(join(namespace.environment.NX_SOCKET_DIR!,"d.sock"))).toBeLessThanOrEqual(95);roots.add(namespace.environment.NX_SOCKET_DIR!);}expect(roots.size).toBe(6);expect(()=>boundedNxSocketNamespaceV1("/"+"🦀".repeat(22),tooling,"/workspace/first",null,"d".repeat(64))).toThrow();
});
/** 🗂️ Independent watched workspaces and activation bindings cannot collide within one real funded data root. */
test("daemon data namespace binds watched root and closed watcher identity",async()=>receiveScriptProcessInvocation(process.env,original=>withRepositoryCacheStoreV1(process.cwd(),"diagnostics",original,createIssuedResourceControlV1(original,"cache:store-entry",0),async store=>{
 const directory=mkdtempSync(join(store.root,"dn-")),root=join(directory,"image"),first=join(directory,"first"),second=join(directory,"second");for(const path of[root,first,second])mkdirSync(path);
 try{const identity="a".repeat(64),one=boundedNxNamespaceV1(root,root,identity,first,null,"d".repeat(64)),two=boundedNxNamespaceV1(root,root,identity,second,null,"d".repeat(64)),watch=boundedNxNamespaceV1(root,root,identity,first,null,"d".repeat(64),"1".repeat(24));expect(one.environment.NX_WORKSPACE_DATA_DIRECTORY).not.toBe(two.environment.NX_WORKSPACE_DATA_DIRECTORY);expect(watch.environment.NX_WORKSPACE_DATA_DIRECTORY).not.toBe(one.environment.NX_WORKSPACE_DATA_DIRECTORY);expect(()=>boundedNxNamespaceV1(root,root,identity,first,null,"d".repeat(64),"../escape")).toThrow();expect(one.environment.NX_CACHE_DIRECTORY).toBe(two.environment.NX_CACHE_DIRECTORY);}
 finally{rmSync(directory,{recursive:true,force:true});}
})));

import {receiveScriptProcessInvocation} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {withRepositoryCacheStoreV1} from "../../../../💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";

/** 🚪️ Independent JavaScript parsing keeps public daemon controls inside the same original service route. */
test("public daemon start and stop preserve original service ownership",()=>{
 const patch=readFileSync(resolve(import.meta.dir,"../../../../🩹️patches/nx@23.2.0.patch"),"utf8"),client=patch.split("diff --git a/dist/src/daemon/client/client.js b/dist/src/daemon/client/client.js\n")[1]!.split("diff --git ")[0]!,entry=patch.split("diff --git a/dist/src/command-line/daemon/daemon.js b/dist/src/command-line/daemon/daemon.js\n")[1]?.split("diff --git ")[0];
 expect(entry).toBeDefined();expect(client).toContain("await this.requestShutdown()");expect(client).toContain("-                process.kill(pid, 'SIGTERM');");expect(entry).toContain("+        await daemonClient.startDaemonIfNecessary();");
 const body=client.split("+    async stop() {")[1]?.split("\n+    }")[0];expect(body).toBeDefined();const ast=parse("async function stop(){"+body!.split("\n").filter(line=>line.startsWith("+")).map(line=>line.slice(1)).join("\n")+"}",{sourceType:"script"});expect(JSON.stringify(ast)).toContain("withNxDaemonStartupV1");
});
