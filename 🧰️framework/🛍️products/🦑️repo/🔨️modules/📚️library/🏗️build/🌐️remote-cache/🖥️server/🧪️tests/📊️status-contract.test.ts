/** 📊️ Checks the served counter ledger against both schema validators. */
import {expect,test} from "bun:test";
import {mkdtempSync,rmSync} from "node:fs";
import {join} from "node:path";
import Ajv from "ajv";
import {startRemoteCacheServerV1} from "../🟦️.ts";
import {requireRemoteCacheShapeV1} from "../../🟦️.ts";
import schema from "../../../🏃️execution/🧬️schema/🌐️remote-cache.json";
import cases from "./📊️status-contract.json";

test("served ledger status is closed and independently valid",async()=>{
 const folder=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"cache-status-"));
 const server=await startRemoteCacheServerV1({directory:folder,host:"127.0.0.1",port:0,maximumBytes:1048576,maximumEntryBytes:65536,maximumAgeDays:30,defaultInstance:"main",tokens:{}});
 try{const response=await fetch(`http://127.0.0.1:${server.port}/status`),status=await response.json(),validate=new Ajv({strict:false}).compile({...schema,$ref:"#/definitions/ServerStatus"});expect(response.status).toBe(200);expect(status.session).toBeDefined();expect(status.ledger).toBeDefined();expect(validate(status)).toBe(true);expect(()=>requireRemoteCacheShapeV1("ServerStatus",status,"closed")).not.toThrow();const withoutLedger=structuredClone(status);delete withoutLedger.session;delete withoutLedger.ledger;expect(validate(withoutLedger)).toBe(true);expect(()=>requireRemoteCacheShapeV1("ServerStatus",withoutLedger,"closed")).not.toThrow();for(const row of cases){const invalid=structuredClone(status);const keys=row.path.split("."),leaf=keys.pop()!;let target=invalid;for(const key of keys)target=target[key];target[leaf]=row.value;expect(validate(invalid),row.name).toBe(false);expect(()=>requireRemoteCacheShapeV1("ServerStatus",invalid,"closed"),row.name).toThrow();}}
 finally{await server.stop();rmSync(folder,{recursive:true,force:true});}
});
