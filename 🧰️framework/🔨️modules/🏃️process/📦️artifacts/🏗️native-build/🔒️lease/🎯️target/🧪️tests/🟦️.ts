import assert from "node:assert/strict";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {readFileSync,existsSync} from "node:fs";
import {join} from "node:path";
import {acquireCargoBuildLeaseV1} from "../../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import corpus from "./🔣️.json" with {type:"json"};
import corpusSchema from "./🧬️.json" with {type:"json"};

/** 🎯️ Independent SQLite binding and real original leases refuse another compiler store for an owned target. */
export async function proveCargoTargetStoreBindingV1(root:string,directory:string):Promise<void>{
    assert.ok(new Ajv({strict:true}).validate(corpusSchema,corpus));
    const database=new Database(":memory:");
    try{
        database.exec("CREATE TABLE targets (target TEXT PRIMARY KEY, store TEXT NOT NULL)");
        for(const [store,target,incomingStore,incomingTarget,expected] of corpus.sameStore){database.exec("DELETE FROM targets");database.query("INSERT INTO targets VALUES (?,?)").run(target as string,store as string);const existing=database.query("SELECT store FROM targets WHERE target=?").get(incomingTarget as string) as {store:string}|null;assert.equal(existing===null||existing.store===incomingStore,expected);}
    }finally{database.close();}
    const signal=new AbortController().signal,targetDirectory=join(root,"t"),buildDirectory=join(root,"b"),options={directory,buildDirectory,targetDirectory,args:["--profile","prototype"],signal};
    for(const args of corpus.foreignArguments){await assert.rejects(acquireCargoBuildLeaseV1({...options,args}),/target.*original|original.*target/i);assert.equal(existsSync(targetDirectory),false);assert.equal(existsSync(buildDirectory),false);}
    const first=await acquireCargoBuildLeaseV1(options);let foreign:Awaited<ReturnType<typeof acquireCargoBuildLeaseV1>>|undefined;
    try{
        let failure:unknown;
        try{foreign=await acquireCargoBuildLeaseV1({...options,buildDirectory:join(root,"other")});}catch(error){failure=error;}
        assert.ok(failure,"an admitted Cargo target must refuse a different compiler store before any output writer starts");
        const receipt=JSON.parse(readFileSync(join(targetDirectory,schema.definitions.Protocol.const.receipt),"utf8"));
        const validate=new Ajv({strict:true}).compile(schema);assert.ok(validate(receipt),JSON.stringify(validate.errors));assert.equal(receipt.buildDirectory,buildDirectory);assert.equal(receipt.targetDirectory,targetDirectory);
    }finally{foreign?.release();first.release();}
}
