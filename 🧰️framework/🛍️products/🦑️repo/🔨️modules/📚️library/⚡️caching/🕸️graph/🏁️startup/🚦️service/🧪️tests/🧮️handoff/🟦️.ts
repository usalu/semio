/** 🚦️ Contract: the daemon service handoff is admitted only as a bounded, closed record of exactly a birth and a witness, refused with the authored message at the first stage that fails, in agreement with an independent Python classifier over the wires of `./🔣️.json`; the request helpers refuse every use outside an acquired service. */
import {expect,test,setDefaultTimeout} from "bun:test";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {completeNxDaemonShutdownV1,NX_DAEMON_SERVICE_BIRTH_ENV_V1,readNxDaemonServiceHandoffV1,requestNxDaemonShutdownV1,retainNxDaemonWorkerV1,withNxDaemonRequestV1,withNxDaemonServiceV1} from "../../🟦️.ts";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"./🐍️oracle/🐍️.py");
const wireOf=(row:(typeof corpus.wires)[number]):string|undefined=>{
 if(row.filler===null)return row.wire??undefined;
 const unit=(row.filler as {multibyte?:boolean}).multibyte?"é":"a",wrapper=row.filler.wrapper,overhead=Buffer.byteLength(wrapper.replace("%s","")),size=Buffer.byteLength(unit),room=row.filler.bytes-overhead;
 return wrapper.replace("%s",unit.repeat(Math.floor(room/size))+"a".repeat(room%size));
};
const ask=(rows:unknown[]):string[]=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify({maximumBytes:corpus.maximumBytes,wires:rows}),encoding:"utf8",maxBuffer:64*1024*1024});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const refusal=(action:()=>unknown):Error|undefined=>{try{action();return undefined;}catch(error){return error as Error;}};

test("the corpus is closed by its schema and the environment name is the one the finite client sets",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,wires:[{...corpus.wires[0],kind:"other"}]})).toBe(false);
 expect(NX_DAEMON_SERVICE_BIRTH_ENV_V1).toBe(corpus.envName);
});

test("every wire is refused with the authored message at the stage the Python classifier names",()=>{
 const wires=corpus.wires.map(wireOf),answers=ask(wires.map(wire=>wire===undefined?null:wire));
 for(const[index,row]of corpus.wires.entries()){
  const error=refusal(()=>readNxDaemonServiceHandoffV1(wires[index])),label=row.name;
  expect(error,label).toBeDefined();expect(answers[index],label).toBe(row.kind);
  if(row.kind==="syntax")expect(error!.name,label).toBe("SyntaxError");else expect(error!.message,label).toBe(row.error);
 }
 expect(new Set(corpus.wires.map(row=>row.kind)).size).toBe(4);
});

test("the service entry and its request helpers refuse every use outside an acquired service",async()=>{
 if(typeof process.send!=="function")await expect(withNxDaemonServiceV1({},async()=>1)).rejects.toThrow("One original IPC-owned daemon service required");
 expect(()=>withNxDaemonRequestV1(async()=>1)).toThrow("caching.daemon.service-required");
 expect(()=>requestNxDaemonShutdownV1("because")).toThrow("caching.daemon.service-required");
 expect(()=>retainNxDaemonWorkerV1("w")).toThrow("caching.daemon.service-required");
 await expect(completeNxDaemonShutdownV1()).rejects.toThrow("caching.daemon.service-required");
});
