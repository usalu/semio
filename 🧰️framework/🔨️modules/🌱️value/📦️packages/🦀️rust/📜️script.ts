#!/usr/bin/env repo
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "../../../🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {buildPreparationBudgetMs} from "../../../🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import assert from "node:assert/strict";
import { configuredExactCargoLawPolicyV1 } from "../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import { runAdmittedExactCargoLawsV1 } from "../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolve } from "node:path";
import { runOwnedCommand, resolveTypescriptCompilerV1 } from "../../../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { BundleScript, ScriptRouter, scriptInvocationBudget } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { readCargoTestPolicyV1 } from "../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";

import { createIssuedResourceControlV1 } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { readFileSync } from "node:fs";
import { semioSchemaAjvV1 } from "../../../🧬️schema/📦️packages/🟦️typescript/🔮️oracles/✅️validator/🟦️.ts";
import { runAdmittedCargoTestsV1 as runCargoTestsV1 } from "../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { resolveTestLevel } from "../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { runtimeTypeEnvironmentV1 } from "../../../⚙️runtime/🏷️identity/🟦️.ts";
import { SqliteDatabaseV1 } from "../../../⚙️runtime/🗃️sqlite/🟦️.ts";

/** 🔒️ Checks exclusive cursor transfer and shared quote admission against independent SQL. */
class ExclusiveRetirementSourceScript extends BundleScript{
 async run(args:string[]):Promise<void>{if(args.length)throw Error("Expected test-exclusive-retirement-source");await runOwnedCommand(process.execPath,["test",resolve(this.root,"../../♻️retirement/🔐️mutex/🧪️tests/🟦️.ts")],this.repoRoot,"value:exclusive-retirement:source",scriptInvocationBudget(this.invocation,30_000),{signal:this.invocation.control.signal});}
}

/** 🪹️ Checks original page reservation and retained partial allocations against schema and SQL oracles. */
export class PagedReservationSourceScript extends BundleScript{
 async run(args:string[]):Promise<void>{if(args.length)throw Error("Expected test-paged-reservation-source");await runOwnedCommand(process.execPath,["test",resolve(this.root,"../../📋️list/🎟️reservation/🧪️tests/🟦️.ts")],this.repoRoot,"value:paged-reservation:source",scriptInvocationBudget(this.invocation,30_000),{signal:this.invocation.control.signal});}
}
/** 🌱️ Tests the actual neutral value package under its explicitly supplied native policy. */
/** 🧱️ Checks declared refusal authority without replacing the original process deadline. */
export class NativeLiteralRefusalSourceScript extends BundleScript{
 private childBudget():number{const remaining=this.invocation.control.remainingMilliseconds();if(remaining!==null&&remaining<1)throw Error("Original native refusal source deadline exhausted");return remaining===null?0:Math.floor(remaining);}
 async run(args:string[]):Promise<void>{
  if(args.length)throw Error("Expected test-native-literal-refusal-source");const test=resolve(this.root,"../../🛬️decode/⚠️refusal/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath,[resolveTypescriptCompilerV1(this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),test],this.repoRoot,"value:original-refusal:types",this.childBudget(),{signal:this.invocation.control.signal});
  await runOwnedCommand(process.execPath,["test",test],this.repoRoot,"value:original-refusal:source",this.childBudget(),{signal:this.invocation.control.signal});
 }
}
export class NativeRecipientSourceTestScript extends BundleScript{
 async run(args:string[]):Promise<void>{if(args.length)throw Error("Expected test-native-recipient-source");const test=resolve(this.root,"../../🛬️decode/🫴️recipient/🔁️continuation/🧪️tests/🟦️.ts");await runOwnedCommand(process.execPath, [resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),test], this.repoRoot, "value:recipient:types", scriptInvocationBudget(this.invocation, 30000), { signal: this.invocation.control.signal });await runOwnedCommand(process.execPath, ["test",test], this.repoRoot, "value:recipient:source", scriptInvocationBudget(this.invocation, 30000), { signal: this.invocation.control.signal });}
}
/** 🔐️ Executes original detached recipient identity and real retained physical custody laws. */
export class NativeReceivingOriginalSourceScript extends BundleScript{
 async run(args:string[]):Promise<void>{if(args.length)throw Error("Expected test-native-receiving-original-source");const tests=[resolve(this.root,"../../🛬️decode/🏭️construction/🧾️result/🧪️tests/🟦️.ts"),resolve(this.root,"../../🛬️decode/🧳️owned/🧳️original/🧪️tests/🟦️.ts"),resolve(this.root,"../../🛬️decode/🧳️owned/📥️default/🧪️tests/🟦️.ts")];for(const test of tests){await runOwnedCommand(process.execPath,[resolveTypescriptCompilerV1(this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),test],this.repoRoot,"value:receiving-original:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});await runOwnedCommand(process.execPath,["test",test],this.repoRoot,"value:receiving-original:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});}}
}
export class NativeRecipientTestScript extends BundleScript{
 async run(args:string[]):Promise<void>{
    const cargoPhase=createIssuedResourceControlV1(this.invocation,"cargo:script-entry",0);if(args.length)throw Error("Expected test-native-recipient");await runCargoTestsV1({ ...({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-value-tests"],cwd:this.root,extraArgs:["--lib","native_local_original","--","--nocapture"]}), signal: this.invocation.control.signal, remainingMilliseconds: () => this.invocation.control.remainingMilliseconds() }, readCargoTestPolicyV1(process.env), this.repoRoot, this.invocation,cargoPhase);}
}
/** 🧮️ Exercises the existing complete neutral value package route. */
export class TestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const cargoPhase=createIssuedResourceControlV1(this.invocation,"cargo:script-entry",0);
    if (args[0] === "portable") {
      if (args.length !== 1) throw Error("Expected test portable");
      await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../🔁️codec/🧪️tests/🛬️controlled/🟦️.ts")], this.repoRoot, "value:portable:construction", scriptInvocationBudget(this.invocation,15_000), { signal: this.invocation.control.signal });
      await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../🛬️decode/🧪️tests/🟦️.ts")], this.repoRoot, "value:portable:decoding", scriptInvocationBudget(this.invocation,30_000), { signal: this.invocation.control.signal });
      return;
    }
    const { rest } = resolveTestLevel(args);
    await runCargoTestsV1({ ...({ manifestPath: resolve(this.root, "Cargo.toml"), packages: ["semio-framework-value-tests"], cwd: this.root, extraArgs: rest }), signal: this.invocation.control.signal, remainingMilliseconds: () => this.invocation.control.remainingMilliseconds() }, readCargoTestPolicyV1(process.env), this.repoRoot, this.invocation,cargoPhase);
  }
}
/** 🛬️ Exercises canonical borrowed construction, allocation admission and owner retirement. */
/** 🕸️ Executes closed Int64 scalar transport and actual test-only GraphQL mapping. */
export class GraphqlInt64SourceTestScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length)throw Error("Expected test-graphql-int64-source");
  const tests=resolve(this.root,"../../🧬️schema/🔗️graphql/🔢️int64/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath, ["test","--timeout","30000",tests], this.repoRoot, "value:graphql:int64:source", scriptInvocationBudget(this.invocation, 30000), { signal: this.invocation.control.signal });
  await runOwnedCommand(process.execPath, [resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),tests], this.repoRoot, "value:graphql:int64:types", scriptInvocationBudget(this.invocation, 30000), { signal: this.invocation.control.signal });
 }
}
/** 🗂️ Checks full retirement categories and the independent ordered payload oracle. */
export class OrderedRetirementSourceTestScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length)throw Error("Expected test-ordered-retirement-source");
  const tests=resolve(this.root,"../../🗂️ordered/♻️retirement/🧪️tests/🟦️.ts");
  const paged=resolve(this.root,"../../📋️list/🧪️tests/📋️list/🟦️.ts");
  await runOwnedCommand(process.execPath, [resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),tests,paged], this.repoRoot, "value:ordered:retirement:types", scriptInvocationBudget(this.invocation, 30000), { signal: this.invocation.control.signal });
  await runOwnedCommand(process.execPath, ["test",tests,paged], this.repoRoot, "value:ordered:retirement:source", scriptInvocationBudget(this.invocation, 30000), { signal: this.invocation.control.signal });
 }
}
export class ControlledValueTestScript extends BundleScript {
  async run(args:string[]):Promise<void>{
    const cargoPhase=createIssuedResourceControlV1(this.invocation,"cargo:script-entry",0);
    const {rest}=resolveTestLevel(args);
    await runCargoTestsV1({ ...({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-value-tests"],cwd:this.root,extraArgs:["--lib","controlled_value_",...rest]}), signal: this.invocation.control.signal, remainingMilliseconds: () => this.invocation.control.remainingMilliseconds() }, readCargoTestPolicyV1(process.env), this.repoRoot, this.invocation,cargoPhase);
  }
}
/** 🛫️ Exercises explicit borrowed output construction and cumulative encoding admission. */
export class ControlledEncodingTestScript extends BundleScript {
 async run(args:string[]):Promise<void>{
    const cargoPhase=createIssuedResourceControlV1(this.invocation,"cargo:script-entry",0);const{rest}=resolveTestLevel(args);await runCargoTestsV1({ ...({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-value-tests"],cwd:this.root,extraArgs:["--lib","controlled_value_encoding_",...rest]}), signal: this.invocation.control.signal, remainingMilliseconds: () => this.invocation.control.remainingMilliseconds() }, readCargoTestPolicyV1(process.env), this.repoRoot, this.invocation,cargoPhase);}
}
/** 🏷️ Validates the canonical type corpus and direct lower ownership. */
export class TypeOwnershipTestScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length)throw Error("Expected test-type-ownership");
  const absence=resolve(this.root,"../../🏷️type/🧪️tests/🚮️absence/🟦️.ts");
  const source=resolve(this.root,"../../🏷️type/🟦️.ts"),tests=resolve(this.root,"../../🏷️type/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath, [resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),source,tests,absence], this.repoRoot, "value:type:types", scriptInvocationBudget(this.invocation,15000), { signal: this.invocation.control.signal });
  await runOwnedCommand(process.execPath, ["test",tests], this.repoRoot, "value:type:ownership", scriptInvocationBudget(this.invocation,15000), { signal: this.invocation.control.signal });
  await runOwnedCommand(process.execPath, ["test","--timeout","30000",absence], this.repoRoot, "value:type:absence", scriptInvocationBudget(this.invocation, 30000), { signal: this.invocation.control.signal });
 }
}
/** ⚠️ Checks typed refusal schema, independent paths and actual portable decoder errors. */
export class RefusalPortableTestScript extends BundleScript {
 async run(args: string[]): Promise<void> {
  if (args.length) throw Error("Expected test-refusal-portable");
  const test = resolve(this.root, "../../⚠️refusal/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath, [resolve(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--noUncheckedIndexedAccess", "--skipLibCheck", "--allowImportingTsExtensions", "--resolveJsonModule", "--esModuleInterop", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", runtimeTypeEnvironmentV1(), test], this.repoRoot, "value:refusal:types", scriptInvocationBudget(this.invocation, 30000), { signal: this.invocation.control.signal });
  await runOwnedCommand(process.execPath, ["test", "--timeout", "30000", test], this.repoRoot, "value:refusal:portable", scriptInvocationBudget(this.invocation, 30000), { signal: this.invocation.control.signal });
 }
}
/** 🚫️ Proves controlled refusal transport against the closed schema and independent wire owners. */
export class RefusalCodecTestScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length)throw Error("Expected test-refusal-codec");
  const test=resolve(this.root,"../../⚠️refusal/🔁️codec/🧪️tests/🟦️.ts");
  const source=resolve(this.root,"../../⚠️refusal/🔁️codec/🟦️.ts");
  await runOwnedCommand(process.execPath, [resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),source,test], this.repoRoot, "value:refusal:codec:types", scriptInvocationBudget(this.invocation, 30000), { signal: this.invocation.control.signal });
  await runOwnedCommand(process.execPath, ["test","--timeout","30000",test], this.repoRoot, "value:refusal:codec:portable", scriptInvocationBudget(this.invocation, 30000), { signal: this.invocation.control.signal });
 }
}
/** 🎚️ Checks neutral decode discovery and the original schema and SQLite reference corpus. */
export class DecodeOwnershipTestScript extends BundleScript {
 async run(args: string[]): Promise<void> {
  if (args.length) throw Error("Expected test-decode-ownership");
  const tests = [resolve(this.root, "../../🛬️decode/🧪️tests/🧩️ownership/🟦️.ts"), resolve(this.root, "../../🛬️decode/🧪️tests/🟦️.ts")];
  await runOwnedCommand(process.execPath, [resolve(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--skipLibCheck", "--allowImportingTsExtensions", "--resolveJsonModule", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", runtimeTypeEnvironmentV1(), ...tests], this.repoRoot, "value:decode:types", scriptInvocationBudget(this.invocation, 30000), { signal: this.invocation.control.signal });
  await runOwnedCommand(process.execPath, ["test", "--timeout", "30000", resolve(this.root, "../../🛬️decode/🧪️tests/🧩️ownership/🟦️.ts"), resolve(this.root, "../../🛬️decode/🧪️tests/🟦️.ts")], this.repoRoot, "value:decode:ownership", scriptInvocationBudget(this.invocation, 30000), { signal: this.invocation.control.signal });
 }
}

/** 🏭️ Checks the complete neutral shared factory copy and cancellation matrix with its independent oracle. */
export class SharedFactoryRetirementTestScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length)throw Error("Expected test-shared-factory-retirement");
  await runOwnedCommand(process.execPath,["test",resolve(this.root,"../../♻️retirement/🔗️arc-custody/🏭️factory/🧪️tests/🟦️.ts")],this.repoRoot,"value:shared-factory:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
 }
}

/** 🔗️ Validates exact original issuer custody through an independent SQLite ownership journal. */
export class FactoryOwnedSourceCheckScript extends BundleScript {
 async run(args:string[]):Promise<void> {
  if(args.length)throw Error("test-factory-owned-source accepts no arguments");
  const {Database}={ Database: SqliteDatabaseV1 };
  const root=resolve(this.root,"../../♻️retirement/🏭️factory/📦️owned");
  const law=JSON.parse(readFileSync(resolve(root,"🧪️tests/🧫️fixtures/🔗️source/🔣️.json"),"utf8")) as {cases:Array<{name:string;text:string;capacity:number;route:string;refuseOnce:boolean;expected:{payloadRetireCalls:number;refusals:number}}>;sourceRefusals:string[];projectionRefusals:string[];transferRefusals:string[]};
  const schema=JSON.parse(readFileSync(resolve(root,"🧬️schema/🔗️source/🔣️.json"),"utf8"));
  const validate=semioSchemaAjvV1({strict:true,allErrors:true}).compile(schema);assert(validate(law),JSON.stringify(validate.errors));
  const database=new Database(":memory:");
  try {
   database.exec("CREATE TABLE custody(id TEXT PRIMARY KEY,payload TEXT,issuer TEXT,retained INTEGER,calls INTEGER,refusals INTEGER,refuse_once INTEGER)");
   for(const row of law.cases) {
    assert(Buffer.byteLength(row.text)<=row.capacity);
    database.query("INSERT INTO custody VALUES(?,?,?,1,0,0,?)").run(row.name,row.text,row.name,Number(row.refuseOnce));
    const original=database.query("SELECT payload,issuer FROM custody WHERE id=?").get(row.name);
    if(row.refuseOnce) {
     database.query("UPDATE custody SET calls=calls+1,refusals=refusals+1,refuse_once=0 WHERE id=? AND retained=1 AND refuse_once=1").run(row.name);
     assert.deepEqual(database.query("SELECT payload,issuer FROM custody WHERE id=? AND retained=1").get(row.name),original);
    }
    database.query("UPDATE custody SET calls=calls+1,retained=0 WHERE id=? AND retained=1 AND refuse_once=0").run(row.name);
    const actual=database.query("SELECT calls AS payloadRetireCalls,refusals FROM custody WHERE id=? AND retained=0").get(row.name);
    assert.deepEqual(actual,row.expected,row.name);
    assert.deepEqual(database.query("SELECT payload,issuer FROM custody WHERE id=?").get(row.name),original);
   }
   for(const [scope,axes] of [["source",law.sourceRefusals],["projection",law.projectionRefusals],["transfer",law.transferRefusals]] as const) {
    for(const axis of axes) {
     const values=[Number(axis!=="items"),Number(axis!=="copy"),Number(axis!=="capacity"),Number(axis!=="depth")];
     const row=database.query("SELECT CASE WHEN ?=1 AND ?=1 AND (?=1 OR ?!='source') AND ?=1 THEN 1 ELSE 0 END AS visits").get(values[0],values[1],values[2],scope,values[3]) as {visits:number};
     assert.equal(row.visits,0,`${scope} denied ${axis} must preserve custody before provider or callback work`);
    }
   }
  } finally { database.close(); }
 }
}

/** 🔮️ Validates generic clone fixture shapes and independently rebuilds their values through SQLite JSON. */
export class CompoundDemandCheckScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length)throw Error("test-compound-demands accepts no arguments");const{Database}={ Database: SqliteDatabaseV1 };
  const root=resolve(this.root,"../../🧬️retained-clone/🔮️demands");const law=JSON.parse(readFileSync(resolve(root,"🧪️tests/🧫️fixtures/🔣️.json"),"utf8")) as {cases:Array<{name:string;kind:string;source:unknown}>};const schema=JSON.parse(readFileSync(resolve(root,"🧬️schema/🔣️.json"),"utf8"));const validate=semioSchemaAjvV1({strict:true,allErrors:true}).compile(schema);assert(validate(law),JSON.stringify(validate.errors));
  const database=new Database(":memory:");
  try{for(const row of law.cases){
   const input=JSON.stringify(row.source);let sql:string;
   switch(row.kind){
    case"vec-scalar":case"vec-string":case"array-string":sql="SELECT json_group_array(value) AS value FROM json_each(?)";break;
    case"record":sql="SELECT json_object('name',json_extract(?1,'$.name'),'values',json(json_extract(?1,'$.values'))) AS value";break;
    case"enum":sql="SELECT CASE WHEN json_extract(?1,'$.kind')='unit' THEN json_object('kind','unit') ELSE json_object('kind','record','label',json_extract(?1,'$.label'),'values',json(json_extract(?1,'$.values'))) END AS value";break;
    case"option-box":sql="SELECT json_quote(json_extract(?,'$')) AS value";break;
    case"tuple":sql="SELECT json_array(json_extract(?1,'$[0]'),json(json_extract(?1,'$[1]'))) AS value";break;
    default:throw Error(`Unknown compound fixture ${row.kind}`);
   }
   const reference=database.query(sql).get(input) as {value:string};assert.deepEqual(JSON.parse(reference.value),row.source,row.name);
  }}finally{database.close();}
 }
}

/** 📽️ Validates immutable root custody with strict schema and an independent SQLite alias journal. */
export class ImmutableProjectionCheckScript extends BundleScript{
 async run(args:string[]):Promise<void>{
  assert(args.length===0||args.length===1&&["source","native"].includes(args[0]!),"test-immutable-projection accepts source, native or no arguments");const{Database}={ Database: SqliteDatabaseV1 };
  const root=resolve(this.root,"../../📽️projection");const law=JSON.parse(readFileSync(resolve(root,"🧪️tests/🧫️fixtures/🔣️.json"),"utf8")) as{cases:Array<{name:string;items:number}>;cancelCuts:number[];expected:{normalTurns:number;rootBirths:number;payloadClones:number;sourceIdentityPreserved:boolean;transferRequiresGrant:boolean}};const validate=semioSchemaAjvV1({strict:true,allErrors:true}).compile(JSON.parse(readFileSync(resolve(root,"🧬️schema/🔣️.json"),"utf8")));assert(validate(law),JSON.stringify(validate.errors));
  const db=new Database(":memory:");try{
   db.exec("PRAGMA foreign_keys=ON;CREATE TABLE roots(id INTEGER PRIMARY KEY,payload BLOB);CREATE TABLE owners(kind TEXT PRIMARY KEY,root INTEGER REFERENCES roots(id))");
   for(const row of law.cases){for(const cut of law.cancelCuts){
    db.exec("DELETE FROM owners;DELETE FROM roots");db.query("INSERT INTO roots VALUES(1,zeroblob(?))").run(row.items);db.query("INSERT INTO owners VALUES('source',1)").run();
    const original=db.query("SELECT id,length(payload) AS items FROM roots").get();let turns=0;
    if(cut>0){db.query("INSERT INTO owners SELECT 'binding',root FROM owners WHERE kind='source'").run();turns++;}
    if(cut>1){db.query("INSERT INTO owners SELECT 'candidate',root FROM owners WHERE kind='source'").run();turns++;}
    for(const axis of ["items","copy","depth"]){const admit=db.query("SELECT CASE WHEN ?=1 AND ?=1 AND ?=1 THEN 1 ELSE 0 END AS admit").get(Number(axis!=="items"),Number(axis!=="copy"),Number(axis!=="depth")) as{admit:number};assert.equal(admit.admit,0);assert.deepEqual(db.query("SELECT id,length(payload) AS items FROM roots").get(),original);}
    if(cut>2)db.query("UPDATE owners SET kind='output' WHERE kind='candidate'").run();
    const births=db.query("SELECT count(*)-1 AS count FROM roots").get()as{count:number};assert.equal(births.count,law.expected.rootBirths);assert.equal(turns,Math.min(cut,law.expected.normalTurns));assert.equal((db.query("SELECT count(*) AS count FROM owners WHERE kind='source' AND root=1").get()as{count:number}).count,1);
    if(cut>1)assert.deepEqual(db.query("SELECT id,length(payload) AS items FROM roots JOIN owners ON roots.id=owners.root WHERE kind=?").get(cut>2?"output":"candidate"),original);
    db.query("DELETE FROM owners WHERE kind!='source'").run();assert.deepEqual(db.query("SELECT id,length(payload) AS items FROM roots").get(),original);
   }}
   const extended=law as typeof law&{refusalCases:Array<{excessCopyBytes:number;ready:boolean;aliasesAfterCapture:number;aliasesAfterClose:number}>;composition:{fields:string[];epoch:number;maximumTurns:number;payloadClones:number;sourceRootIdentityPreserved:boolean}};
   db.exec("DELETE FROM owners;DELETE FROM roots;CREATE TABLE admission(ready INTEGER);INSERT INTO admission VALUES(0)");
   for(const row of extended.refusalCases){db.query("INSERT INTO roots VALUES(1,zeroblob(4096))").run();db.query("INSERT INTO owners VALUES('source',1)").run();db.query("INSERT INTO owners SELECT 'unadmitted',root FROM owners WHERE kind='source'").run();db.query("UPDATE admission SET ready=1 WHERE ?<=0").run(row.excessCopyBytes);assert.equal((db.query("SELECT ready FROM admission").get()as{ready:number}).ready,Number(row.ready));assert.equal((db.query("SELECT count(*) AS count FROM owners").get()as{count:number}).count,row.aliasesAfterCapture);db.query("DELETE FROM owners WHERE kind='unadmitted'").run();assert.equal((db.query("SELECT count(*) AS count FROM owners").get()as{count:number}).count,row.aliasesAfterClose);db.exec("DELETE FROM owners;DELETE FROM roots");}
   db.exec("CREATE TABLE records(scope TEXT,field TEXT,root INTEGER REFERENCES roots(id),scalar INTEGER,PRIMARY KEY(scope,field))");
   for(const row of law.cases){db.exec("DELETE FROM records;DELETE FROM roots");db.query("INSERT INTO roots VALUES(1,zeroblob(?)),(2,zeroblob(?))").run(row.items,row.items);db.query("INSERT INTO records VALUES('source','epoch',NULL,?),('source','primary',1,NULL),('source','secondary',2,NULL)").run(extended.composition.epoch);const original=db.query("SELECT field,root,scalar FROM records WHERE scope='source' ORDER BY field").all();db.query("INSERT INTO records SELECT 'candidate',field,root,scalar FROM records WHERE scope='source'").run();assert.deepEqual(db.query("SELECT field,root,scalar FROM records WHERE scope='candidate' ORDER BY field").all(),original);assert.equal((db.query("SELECT count(*) AS count FROM roots").get()as{count:number}).count,2);assert.equal((db.query("SELECT sum(length(payload)) AS bytes FROM roots").get()as{bytes:number}).bytes,row.items*2);for(const axis of["items","copy","depth"]){db.query("UPDATE records SET scope='output' WHERE scope='candidate' AND ?=1 AND ?=1 AND ?=1").run(Number(axis!=="items"),Number(axis!=="copy"),Number(axis!=="depth"));assert.deepEqual(db.query("SELECT field,root,scalar FROM records WHERE scope='candidate' ORDER BY field").all(),original);}db.query("UPDATE records SET scope='output' WHERE scope='candidate'").run();assert.deepEqual(db.query("SELECT field,root,scalar FROM records WHERE scope='output' ORDER BY field").all(),original);db.query("DELETE FROM records WHERE scope='output'").run();assert.deepEqual(db.query("SELECT field,root,scalar FROM records WHERE scope='source' ORDER BY field").all(),original);}
  }finally{db.close();}
  if(args[0]==="native"){const p=readCargoTestPolicyV1(process.env);await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,p.buildBudgetMs>0?p.buildBudgetMs:buildPreparationBudgetMs())},cwd:this.repoRoot,manifestPaths:{"semio-framework-value-snapshot-projection":resolve(this.root,"../../📽️projection/📦️packages/🦀️rust/Cargo.toml")},groups:[{package:"semio-framework-value-snapshot-projection",target:{kind:"lib",name:"semio_framework_value_snapshot_projection"},laws:["snapshot_projection::tests::quick_immutable_snapshot_projection_original_root_and_every_prefix","snapshot_projection::tests::quick_immutable_snapshot_projection_bad_receipt_keeps_unadmitted_original_owner","snapshot_projection::tests::quick_immutable_snapshot_projection_composed_record_every_prefix"]}]});}
 }
}

/** 📦️ Checks explicit owned field projection against SQLite copying before genuine native custody laws. */
export class OwnedProjectionCheckScript extends BundleScript{
 async run(args:string[]):Promise<void>{
  assert(args.length===1&&["source","native"].includes(args[0]!),"test-owned-projection requires source or native");
  const root=resolve(this.root,"../../📽️projection"),law=JSON.parse(readFileSync(resolve(root,"🧪️tests/🧫️fixtures/📦️owned/🔣️.json"),"utf8"))as{outputHandoffs:Array<{ready:boolean;closing:boolean;hasOutput:boolean;items:number;outputBytes:number;copyBudget:number;depth:number;admitted:boolean;retained:boolean}>;nestedBodyQuotes:Array<{body:number;headers:number[];payload:number;spent:number}>;cases:Array<{id:string;schema:string;count:number;octet:number;optionalPayload:boolean}>;cancelCuts:number[]};
  const validate=semioSchemaAjvV1({strict:true,allErrors:true}).compile(JSON.parse(readFileSync(resolve(root,"🧬️schema/📦️owned.json"),"utf8")));assert(validate(law),JSON.stringify(validate.errors));
  const{Database}={ Database: SqliteDatabaseV1 },db=new Database(":memory:");try{
   db.exec("CREATE TABLE original(schema TEXT,bytes BLOB,optional BLOB);CREATE TABLE candidate(schema TEXT,bytes BLOB,optional BLOB)");
   for(const row of law.cases){db.exec("DELETE FROM original;DELETE FROM candidate");db.query("INSERT INTO original VALUES(?,?,?)").run(row.schema,Buffer.alloc(row.count,row.octet),row.optionalPayload?Buffer.alloc(row.count,row.octet):null);db.exec("INSERT INTO candidate SELECT schema,bytes,optional FROM original");const original=db.query("SELECT schema,bytes,optional FROM original").get()as{schema:string;bytes:Uint8Array;optional:Uint8Array|null};const candidate=db.query("SELECT schema,bytes,optional FROM candidate").get()as{schema:string;bytes:Uint8Array;optional:Uint8Array|null};assert.equal(candidate.schema,row.schema);assert.deepEqual([...candidate.bytes],Array(row.count).fill(row.octet));assert.deepEqual(candidate.optional===null?null:[...candidate.optional],row.optionalPayload?Array(row.count).fill(row.octet):null);assert.deepEqual(original,candidate);db.exec("DELETE FROM candidate");assert.deepEqual(db.query("SELECT schema,bytes,optional FROM original").get(),original);}
  }finally{db.close();}
  for(const row of law.outputHandoffs){const admitted=row.ready&&!row.closing&&row.hasOutput&&row.items>0&&row.copyBudget>=2*row.outputBytes&&row.depth>=1;assert.equal(admitted,row.admitted);assert.equal(row.hasOutput&&!admitted,row.retained);const db=new Database(":memory:");try{const result=db.query("SELECT (?1 AND NOT ?2 AND ?3 AND ?4>0 AND ?5>=2*?6 AND ?7>=1) admitted, (?3 AND NOT (?1 AND NOT ?2 AND ?3 AND ?4>0 AND ?5>=2*?6 AND ?7>=1)) retained").get(Number(row.ready),Number(row.closing),Number(row.hasOutput),row.items,row.copyBudget,row.outputBytes,row.depth)as{admitted:number;retained:number};assert.equal(Boolean(result.admitted),row.admitted);assert.equal(Boolean(result.retained),row.retained);}finally{db.close();}}
  for(const row of law.nestedBodyQuotes){const header=row.headers.reduce((sum,width)=>sum+width,0);const db=new Database(":memory:");try{const expected=db.query("SELECT max(0,?1-?2) payload").get(row.body,header)as{payload:number};assert.equal(expected.payload,row.payload);assert.equal(row.spent,row.payload+header);assert(row.spent<=row.body);}finally{db.close();}}
  if(args[0]==="native"){const p=readCargoTestPolicyV1(process.env);await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,p.buildBudgetMs>0?p.buildBudgetMs:buildPreparationBudgetMs()),lawMilliseconds:scriptInvocationBudget(this.invocation,p.assertionBudgets[p.level]>0?p.assertionBudgets[p.level]:300000)},cwd:this.repoRoot,nativeEnv:{RUST_MIN_STACK:p.rustMinStack},manifestPaths:{"semio-framework-value-snapshot-projection":resolve(this.root,"../../📽️projection/📦️packages/🦀️rust/Cargo.toml")},groups:[{"package":"semio-framework-value-snapshot-projection","target":{"kind":"lib","name":"semio_framework_value_snapshot_projection"},"cargoArgs":["--locked"],"laws":["snapshot_projection::tests::owned::owned_snapshot_projection_composes_real_string_and_byte_fields_with_exact_custody"]}]});}
 }
}

/** 🔎️ Validates immutable ranked table traversal and record output against original SQLite rows. */
export class PersistentTraversalCheckScript extends BundleScript{
 async run(args:string[]):Promise<void>{
  assert(args.length===1&&["source","native"].includes(args[0]!),"test-persistent-traversal requires source or native");
  const root=resolve(this.root,"../../📋️list/🔗️persistent/🔎️traversal");
  const law:typeof import("../../📋️list/🔗️persistent/🔎️traversal/🧪️tests/🧫️fixtures/🔣️.json")=JSON.parse(readFileSync(resolve(root,"🧪️tests/🧫️fixtures/🔣️.json"),"utf8"));
  const{default:Ajv2020}=await import("ajv/dist/2020"),ajv=new Ajv2020({strict:true}),validate=ajv.compile<typeof law>(JSON.parse(readFileSync(resolve(root,"🧬️schema/🔣️.json"),"utf8")));
  assert(validate(law),JSON.stringify(validate.errors));assert(!validate({...law,stride:0}));assert(!validate({...law,cases:[]}));
  const{Database}={ Database: SqliteDatabaseV1 },db=new Database(":memory:");let assertions=3;
  try{
   db.exec("CREATE TABLE original(position INTEGER PRIMARY KEY,rank INTEGER,label TEXT)");
   for(const row of law.cases){
    const count=row.fullLeaves*16+row.tailLeaves.reduce((sum:number,length:number)=>sum+length,0);assert.equal(count,row.rowCount);let pages=row.fullLeaves+row.tailLeaves.length,all=pages;while(pages>1){pages=Math.ceil(pages/16);all+=pages;}assert.equal(all,row.pageCount);
    db.exec("DELETE FROM original");db.transaction(()=>{for(let index=0;index<count;index++)db.query("INSERT INTO original VALUES(?,?,?)").run(index,index*law.stride,law.prefix+index)})();
    const output=db.query("SELECT rank,label FROM original ORDER BY position").all();assert.deepEqual(output,Array.from({length:count},(_,index)=>({rank:index*law.stride,label:law.prefix+index})));
    let position=0;for(const skip of law.queries){const requested=position+skip;const actual=db.query("SELECT rank,label FROM original ORDER BY position LIMIT 1 OFFSET ?").get(requested);assert.deepEqual(actual,requested<count?{rank:requested*law.stride,label:law.prefix+requested}:null);position=requested<count?requested+1:count;assertions++;}
    assertions+=3;
   }
  }finally{db.close();}
  if(args[0]==="native"){const p=readCargoTestPolicyV1(process.env);await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,p.buildBudgetMs>0?p.buildBudgetMs:buildPreparationBudgetMs()),lawMilliseconds:scriptInvocationBudget(this.invocation,p.assertionBudgets[p.level]>0?p.assertionBudgets[p.level]:300000)},cwd:this.repoRoot,nativeEnv:{RUST_MIN_STACK:p.rustMinStack},manifestPaths:{"semio-framework-value-persistent":resolve(this.root,"../../📋️list/🔗️persistent/📦️packages/🦀️rust/Cargo.toml")},groups:[{"package":"semio-framework-value-persistent","target":{"kind":"lib","name":"semio_framework_value_persistent"},"cargoArgs":["--locked"],"laws":["list::persistent::tests::traversal::persistent_table_traversal_preserves_original_rows_and_linear_zero_heap_scan","list::persistent::tests::traversal::persistent_table_traversal_projects_borrowed_records_and_controlled_wire_without_payload_clone"]}]});}
 }
}

/** 🧬️ Verifies plain derived retirement widths against the neutral SQLite corpus. */
export class PlainElementRetirementSourceScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length)throw Error("test-plain-element-retirement-source accepts no arguments");
  const source=resolve(this.root,"../../♻️retirement/🧪️tests/🧮️plain-elements/🟦️.ts");
  await runOwnedCommand(process.execPath,[resolveTypescriptCompilerV1(this.root),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),source],this.repoRoot,"value:plain-element-retirement:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"value:plain-element-retirement:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
 }
}
/** 🎟️ Counts original nested owner quotes independently of SQLite path leaves. */
export class OriginalRetirementDemandScript extends BundleScript{
 async run(args:string[]):Promise<void>{
  assert(args.length===1&&["source","native"].includes(args[0]!),"test-original-retirement-demand requires source or native");
  const root=resolve(this.root,"../../♻️retirement/🎟️demand"),law=JSON.parse(readFileSync(resolve(root,"🧪️tests/🧫️fixtures/🔣️.json"),"utf8"))as typeof import("../../♻️retirement/🎟️demand/🧪️tests/🧫️fixtures/🔣️.json");const{default:Ajv}=await import("ajv/dist/2020"),v=new Ajv({strict:true}).compile<typeof law>(JSON.parse(readFileSync(resolve(root,"🧬️schema/🔣️.json"),"utf8")));assert(v(law),JSON.stringify(v.errors));assert(!v({...law,expected:{...law.expected,leafVisitsPerQuote:4}}));
  const{Database}={ Database: SqliteDatabaseV1 },db=new Database(":memory:");let assertions=2;try{db.exec("create table nodes(family integer,level integer,parent integer)");for(const[index,family]of law.families.entries()){for(let level=0;level<=family.sealedDepth;level++)db.query("insert into nodes values(?,?,?)").run(index,level,level===0?null:level-1);for(const body of law.bodyHints){const leaves=db.query("with recursive path(level) as(select level from nodes where family=? and parent is null union all select n.level from nodes n join path p on n.parent=p.level where n.family=?) select count(*) as visits from path where level=?").get(index,index,family.sealedDepth)as{visits:number};assert.equal(leaves.visits,law.expected.leafVisitsPerQuote);assert(Number.isSafeInteger(body));assertions+=2;}}}finally{db.close();}
  if(args[0]==="native"){const p=readCargoTestPolicyV1(process.env);await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,p.buildBudgetMs>0?p.buildBudgetMs:buildPreparationBudgetMs()),lawMilliseconds:scriptInvocationBudget(this.invocation,p.assertionBudgets[p.level]>0?p.assertionBudgets[p.level]:300000)},cwd:this.repoRoot,nativeEnv:{RUST_MIN_STACK:p.rustMinStack},manifestPaths:{"semio-framework-value-tests":resolve(this.root,"../../🧪️tests/🥣️companion/📦️packages/🦀️rust/Cargo.toml")},groups:[{"package":"semio-framework-value-tests","target":{"kind":"lib","name":"semio_framework_value_tests"},"cargoArgs":["--locked"],"laws":["original_demand_tests::original_retirement_combined_quote_visits_terminal_leaf_once_with_actual_body_hint"]}]});}
 }
}



/** 🔀️ Checks schema-declared FEM record and tuple variant receiving laws against independent validators. */
export class NativePayloadEnumScript extends BundleScript{
 async run(args:string[]):Promise<void>{
  assert(args.length===1&&["source","native"].includes(args[0]!),"test-native-payload-enum requires source or native");const source=resolve(this.root,"../../✨️derive/🛬️receive/🔀️enum/🧪️tests/🟦️.ts");await runOwnedCommand(process.execPath,[resolveTypescriptCompilerV1(this.root),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),source],this.repoRoot,"value:payload-enum:types",scriptInvocationBudget(this.invocation,120000),{signal:this.invocation.control.signal});(await import("../../✨️derive/🛬️receive/🔀️enum/🧪️tests/🟦️.ts")).nativePayloadEnumOracle();if(args[0]==="native"){const p=readCargoTestPolicyV1(process.env);await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,p.buildBudgetMs>0?p.buildBudgetMs:buildPreparationBudgetMs())},cwd:this.repoRoot,manifestPaths:{"semio-framework-value-native-owned":resolve(this.root,"../../🛬️decode/🧳️owned/📦️packages/🦀️rust/Cargo.toml")},groups:[{"package":"semio-framework-value-native-owned","target":{"kind":"lib","name":"semio_framework_value_native_owned"},"laws":["owned::payload_enum_tests::native_payload_enum_receives_exact_declared_variants_and_closes_every_original_frontier"]}]});}
 }
}


/** ⚖️ Verifies borrowed immutable Debug and value equality against SQLite. */
class BorrowedObservationScript extends BundleScript{
 async run(args:string[]):Promise<void>{
  assert(args.length===1&&["source","native"].includes(args[0]!),"test-borrowed-observations source|native");
  const oracle=await import("../../📋️list/🔗️persistent/⚖️observation/🧪️tests/🟦️.ts");oracle.borrowedObservationOracle();
  if(args[0]==="native"){const p=readCargoTestPolicyV1(process.env);await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,p.buildBudgetMs>0?p.buildBudgetMs:buildPreparationBudgetMs()),lawMilliseconds:scriptInvocationBudget(this.invocation,p.assertionBudgets[p.level]>0?p.assertionBudgets[p.level]:300000)},cwd:this.repoRoot,nativeEnv:{RUST_MIN_STACK:p.rustMinStack},manifestPaths:{"semio-framework-value-persistent":resolve(this.root,"../../📋️list/🔗️persistent/📦️packages/🦀️rust/Cargo.toml")},groups:[{package:"semio-framework-value-persistent",target:{kind:"lib",name:"semio_framework_value_persistent"},cargoArgs:["--locked"],laws:["list::persistent::tests::borrowed_observations::borrowed_immutable_observations_preserve_original_rows_without_clone_or_heap"]}]});}
 }
}

/** 🚚️ Checks bounded bitwise clone work at its defining split owner under the original process. */
class BulkVecRetainedCheckScript extends BundleScript{
 async run(args:string[]):Promise<void>{
  assert(args.length===1&&["source","native"].includes(args[0]!),"test-bulk-vec-retained requires source or native");
  const source=resolve(this.root,"../../🧬️retained-clone/🚚️bulk-vec/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath,[resolveTypescriptCompilerV1(this.root),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),source],this.repoRoot,"value:bulk-clone:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"value:bulk-clone:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
  if(args[0]==="native"){const policy=readCargoTestPolicyV1(process.env);await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,policy.buildBudgetMs>0?policy.buildBudgetMs:buildPreparationBudgetMs())},cwd:this.repoRoot,manifestPaths:{"semio-framework-value-retained-clone":resolve(this.root,"../../🧬️retained-clone/🧬️contract/📦️packages/🦀️rust/Cargo.toml")},groups:[{"package":"semio-framework-value-retained-clone","target":{"kind":"lib","name":"semio_framework_value_retained_clone"},"laws":["retained_clone::bulk_vec_tests::bulk_vector_clone_equals_the_clone_oracle_for_every_bitwise_element_shape","retained_clone::bulk_vec_tests::bulk_vector_clone_turn_count_is_bytes_over_page","retained_clone::bulk_vec_tests::bulk_vector_clone_capacity_accounting_is_exact","retained_clone::bulk_vec_tests::bulk_vector_clone_cancels_at_every_turn_and_releases_everything","retained_clone::bulk_vec_tests::bulk_run_elements_obeys_every_grant_axis","retained_clone::bulk_vec_tests::bulk_array_zero_sized_work_is_bounded_and_every_prefix_closes","retained_clone::bulk_vec_tests::bulk_clone_logical_body_survives_expanded_physical_headers"]}]});}
 }
}
/** 📩️ Verifies paid original child receiving, immutable sealing and every controlled cancellation frontier. */
class NativeSharedReceivingCheckScript extends BundleScript{
 async run(args:string[]):Promise<void>{
  assert(args.length===1&&["source","native"].includes(args[0]!),"test-native-shared-receive requires source or native");
  const source=resolve(this.root,"../../🛬️decode/🧳️owned/🔗️sealed-shared-receiver/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),source],this.repoRoot,"value:native-shared-receive:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"value:native-shared-receive:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
  if(args[0]==="native"){const policy=readCargoTestPolicyV1(process.env);await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,policy.buildBudgetMs>0?policy.buildBudgetMs:buildPreparationBudgetMs())},cwd:this.repoRoot,manifestPaths:{"semio-framework-value-native-owned":resolve(this.root,"../../🛬️decode/🧳️owned/📦️packages/🦀️rust/Cargo.toml")},groups:[{"package":"semio-framework-value-native-owned","target":{"kind":"lib","name":"semio_framework_value_native_owned"},"cargoArgs":["--locked"],"laws":["owned::shared::tests::native_shared_original_receive_exact_prefix_cancellation_and_transfer","owned::shared::tests::native_shared_original_callback_refusal_retains_paid_child_authority","owned::shared::tests::native_record32_single_active_child_fits4096_and_preserves_every_original_prefix","owned::shared::tests::native_flattened_original_record_preserves_source_schema_and_all_partial_custody"]}]});}
 }
}
/** 📬️ Verifies source-bound immutable key admission and sparse original ranked reuse. */
class NamedRootReceivingCheckScript extends BundleScript{
 async run(args:string[]):Promise<void>{
  assert(args.length===1&&["source","native"].includes(args[0]!),"test-named-root-receive requires source or native");
  const source=resolve(this.root,"../../📋️list/🔺️edit/🔑️named/📇️index/🌳️root/🛬️receive/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"value:named-root-receive:source",scriptInvocationBudget(this.invocation,120000),{signal:this.invocation.control.signal});
  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),source],this.repoRoot,"value:named-root-receive:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
  if(args[0]==="native"){const p=readCargoTestPolicyV1(process.env);await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,p.buildBudgetMs),lawMilliseconds:scriptInvocationBudget(this.invocation,p.assertionBudgets[p.level])},cwd:this.repoRoot,nativeEnv:{RUST_MIN_STACK:p.rustMinStack},manifestPaths:{"semio-framework-value-edit":resolve(this.root,"../../📋️list/🔺️edit/📦️packages/🦀️rust/Cargo.toml")},groups:[{"package":"semio-framework-value-edit","target":{"kind":"lib","name":"semio_framework_value_edit"},"cargoArgs":["--locked"],"laws":["named_edit::index::tests::transfer::receiving::original_named_receiving_publishes_independent_roots_and_closes_every_paid_cut","named_edit::index::tests::transfer::receiving::original_named_receiving_checks_full_utf8_under_original_decoder_quantum"]}]});}
 }
}
/** 📇️ Original immutable key admission and controlled directory transfer share the actual public planner. */
class NamedKeyIndexCheckScript extends BundleScript{
 async run(args:string[]):Promise<void>{
  assert(args.length===1&&["source","native"].includes(args[0]!),"test-named-key-index requires source or native");
  const source=resolve(this.root,"../../📋️list/🔺️edit/🔑️named/📇️index/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"value:named-key-index:source",scriptInvocationBudget(this.invocation,120000),{signal:this.invocation.control.signal});
  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),source],this.repoRoot,"value:named-key-index:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
  if(args[0]==="native"){const p=readCargoTestPolicyV1(process.env);await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,p.buildBudgetMs>0?p.buildBudgetMs:buildPreparationBudgetMs()),lawMilliseconds:scriptInvocationBudget(this.invocation,p.assertionBudgets[p.level]>0?p.assertionBudgets[p.level]:300000)},cwd:this.repoRoot,nativeEnv:{RUST_MIN_STACK:p.rustMinStack},manifestPaths:{"semio-framework-value-edit":resolve(this.root,"../../📋️list/🔺️edit/📦️packages/🦀️rust/Cargo.toml")},groups:[{"package":"semio-framework-value-edit","target":{"kind":"lib","name":"semio_framework_value_edit"},"cargoArgs":["--locked"],"laws":["named_edit::index::tests::transfer::key_preserving_persistent_producer_transfers_original_directory_and_all_paid_cuts","named_edit::index::tests::transfer::key_preserving_persistent_producer_checks_complete_original_utf8_and_foreign_sources","named_edit::index::tests::admitted_named_index_preserves_original_opaque_keys_ranked_sources_and_cancelled_custody","named_edit::index::tests::admitted_named_index_reuses_persistent_originals_without_base_allocation_or_resorting","named_edit::tests::named_edit_plan_matches_portable_keys_and_physical_custody","named_edit::tests::named_edit_plan_long_prefixes_and_cancellation_are_bounded"]}]});}
 }
}


/** ♻️ Verifies original decoder retirement custody after semantic cancellation under exact caller grants. */
class NativeRetirementLoanCheckScript extends BundleScript{
 async run(args:string[]):Promise<void>{
  assert(args.length===1&&["source","native"].includes(args[0]!),"test-native-retirement-loan requires source or native");
  const source=resolve(this.root,"../../🛬️decode/♻️retirement/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"value:native-retirement-loan:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),source],this.repoRoot,"value:native-retirement-loan:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
  if(args[0]==="native"){const policy=readCargoTestPolicyV1(process.env);await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,policy.buildBudgetMs>0?policy.buildBudgetMs:buildPreparationBudgetMs())},cwd:this.repoRoot,manifestPaths:{"semio-framework-value-native-decoding":resolve(this.root,"../../🛬️decode/📦️packages/🦀️rust/Cargo.toml")},groups:[{"package":"semio-framework-value-native-decoding","target":{"kind":"lib","name":"semio_framework_value_native_decoding"},"laws":["native_decoding::retirement_loan::tests::native_retirement_loan_preserves_original_cancelled_ledger_and_grant","native_decoding::retirement_loan::tests::native_retirement_loan_closes_original_returned_payload_after_cancellation"]}]});}
 }
}


/** 🧳️ Verifies original typed child receiving into fixed persistent pages without a cold array conversion. */
class NativePersistentReceivingCheckScript extends BundleScript{
 async run(args:string[]):Promise<void>{
  assert(args.length===1&&["source","native"].includes(args[0]!),"test-native-persistent-receive requires source or native");
  const source=resolve(this.root,"../../🛬️decode/🧳️owned/📋️list/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),source],this.repoRoot,"value:native-persistent-receive:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"value:native-persistent-receive:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
  if(args[0]==="native"){const policy=readCargoTestPolicyV1(process.env);await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,policy.buildBudgetMs>0?policy.buildBudgetMs:buildPreparationBudgetMs())},cwd:this.repoRoot,manifestPaths:{"semio-framework-value-native-owned":resolve(this.root,"../../🛬️decode/🧳️owned/📦️packages/🦀️rust/Cargo.toml")},groups:[{"package":"semio-framework-value-native-owned","target":{"kind":"lib","name":"semio_framework_value_native_owned"},"laws":["owned::list_tests::native_persistent_list_receives_original_rows_and_preserves_all_cancelled_custody","owned::list_tests::native_persistent_list_later_row_refusal_retains_original_partial_owner","owned::list_tests::native_persistent_list_original_callback_refusal_preserves_partial_receiving","owned::list_tests::native_persistent_list_receiving_fits_original_interaction_quantum","owned::list_tests::native_persistent_list_original_vector_owner_fits_quantum_and_keeps_custody","owned::original::tests::native_receiving_original_keeps_cancellation_and_denied_transfers_owned","owned::original::tests::native_receiving_original_shell_requires_funded_input_and_receiver_custody"]}]});}
 }
}


/** 📥️ Verifies original streamed row adoption, fixed page carries and funded persistent cancellation. */
class PersistentBuildCheckScript extends BundleScript{
 async run(args:string[]):Promise<void>{
  assert(args.length===1&&["source","native"].includes(args[0]!),"test-persistent-build requires source or native");
  const source=resolve(this.root,"../../📋️list/🔗️persistent/📥️build/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),source],this.repoRoot,"value:persistent-build:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"value:persistent-build:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
  if(args[0]==="native"){const policy=readCargoTestPolicyV1(process.env);await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,policy.buildBudgetMs>0?policy.buildBudgetMs:buildPreparationBudgetMs())},cwd:this.repoRoot,manifestPaths:{"semio-framework-value-persistent":resolve(this.root,"../../📋️list/🔗️persistent/📦️packages/🦀️rust/Cargo.toml")},groups:[{"package":"semio-framework-value-persistent","target":{"kind":"lib","name":"semio_framework_value_persistent"},"laws":["list::persistent::build_tests::persistent_owned_builder_preserves_ranked_originals_and_every_receiving_frontier","list::persistent::build_tests::persistent_owned_builder_refuses_missing_and_excess_without_consuming_input","list::persistent::build_tests::persistent_owned_builder_cancels_every_preparation_action_frontier","list::persistent::build_tests::persistent_owned_builder_constructor_fits_authored_interaction_quantum"]}]});}
 }
}


/** 🔏️ Verifies original typed sealed children use their caller's funded cleanup frontier. */
class SealedFrontierCheckScript extends BundleScript{
 async run(args:string[]):Promise<void>{
  assert(args.length===1&&["source","native"].includes(args[0]!),"test-sealed-frontier requires source or native");
  const source=resolve(this.root,"../../♻️retirement/🔗️arc-custody/🔒️sealed/🚚️frontier/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),source],this.repoRoot,"value:sealed-frontier:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"value:sealed-frontier:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
  if(args[0]==="native"){const policy=readCargoTestPolicyV1(process.env);await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,policy.buildBudgetMs>0?policy.buildBudgetMs:buildPreparationBudgetMs())},cwd:this.repoRoot,manifestPaths:{"semio-framework-value-shared-issuance":resolve(this.root,"../../♻️retirement/🔗️arc-custody/🔒️sealed/📦️packages/🦀️rust/Cargo.toml")},groups:[{"package":"semio-framework-value-shared-issuance","target":{"kind":"lib","name":"semio_framework_value_shared_issuance"},"laws":["frontier_tests::original_sealed_owned_payload_hands_native_child_to_caller_frontier","tests::sealed_shared_original_aliases_preserve_exact_payload_and_funded_header","tests::sealed_shared_original_alias_return_slots_transfer_parallel_custody_without_heap","tests::sealed_shared_stale_slot_refuses_a_distinct_original_issuance_at_same_address"]}]});}
 }
}

/** 🏠️ Verifies immutable list root cloning through actual original pointer and physical retirement witnesses. */
class ImmutableRootCloneCheckScript extends BundleScript{
 async run(args:string[]):Promise<void>{
  assert(args.length===1&&["source","native"].includes(args[0]!),"test-immutable-root-clone requires source or native");
  const source=resolve(this.root,"../../🧬️retained-clone/🔗️sealed-shared-list/📋️list/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),source],this.repoRoot,"value:immutable-root-clone:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"value:immutable-root-clone:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
   if(args[0]==="native"){const policy=readCargoTestPolicyV1(process.env);await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,policy.buildBudgetMs>0?policy.buildBudgetMs:buildPreparationBudgetMs())},cwd:this.repoRoot,manifestPaths:{"semio-framework-value-persistent":resolve(this.root,"../../📋️list/🔗️persistent/📦️packages/🦀️rust/Cargo.toml"),"semio-framework-value-tests":resolve(this.root,"../../🧪️tests/🥣️companion/📦️packages/🦀️rust/Cargo.toml")},groups:[{"package":"semio-framework-value-persistent","target":{"kind":"lib","name":"semio_framework_value_persistent"},"laws":["immutable_clone::list_tests::persistent_list_retained_clone_preserves_original_root_without_payload_copy"]},{"package":"semio-framework-value-tests","target":{"kind":"lib","name":"semio_framework_value_tests"},"laws":["retained_clone::source_custody_tests::retained_clone_owned_source_shared_alias_keeps_original_identity_in_every_close_order"]}]});}
 }
}

/** 🌳️ Verifies single-visit original close quotes and genuine paid nested owner retirement. */
class CloseDemandCheckScript extends BundleScript{
 async run(args:string[]):Promise<void>{
  assert(args.length===1&&["source","native"].includes(args[0]!),"test-close-demand requires source or native");
  const source=resolve(this.root,"../../🧬️retained-clone/♻️close/🎟️demand/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),source],this.repoRoot,"value:close-demand:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"value:close-demand:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
  if(args[0]==="native"){const policy=readCargoTestPolicyV1(process.env);await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,policy.buildBudgetMs>0?policy.buildBudgetMs:buildPreparationBudgetMs())},cwd:this.repoRoot,manifestPaths:{"semio-framework-value-tests":resolve(this.root,"../../🧪️tests/🥣️companion/📦️packages/🦀️rust/Cargo.toml")},groups:[{"package":"semio-framework-value-tests","target":{"kind":"lib","name":"semio_framework_value_tests"},"laws":["retained_clone::close_demand_tests::original_nested_close_preserves_paid_source_and_cancellation","retained_clone::close_demand_tests::derived_original_record_and_enum_preflight_all_axes_and_close_exact_custody"]}]});}
 }
}

/** 🟰️ Verifies immutable scalar-buffer equality under original paged copy grants. */
class TypedEqualityCheckScript extends BundleScript{
 async run(args:string[]):Promise<void>{
  assert(args.length===1&&["source","native"].includes(args[0]!),"test-retained-equality requires source or native");
  const source=resolve(this.root,"../../🌳️tree/🟰️equal/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),source],this.repoRoot,"value:typed-equality:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"value:typed-equality:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
  if(args[0]==="native"){const policy=readCargoTestPolicyV1(process.env);await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,policy.buildBudgetMs>0?policy.buildBudgetMs:buildPreparationBudgetMs())},cwd:this.repoRoot,manifestPaths:{"semio-framework-value-persistent":resolve(this.root,"../../📋️list/🔗️persistent/📦️packages/🦀️rust/Cargo.toml")},groups:[{"package":"semio-framework-value-persistent","target":{"kind":"lib","name":"semio_framework_value_persistent"},"laws":["retained_equality::tests::retained_typed_equality_matches_portable_original_fields","retained_equality::tests::retained_typed_equality_cancellation_keeps_original_leases_and_child_frontiers"]}]});}
 }
}

class SliceEqualityCheckScript extends BundleScript{
 async run(args:string[]):Promise<void>{
  assert(args.length===1&&["source","native"].includes(args[0]!),"test-slice-equality requires source or native");
  const source=resolve(this.root,"../../📋️list/🟰️equal/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),source],this.repoRoot,"value:slice-equality:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"value:slice-equality:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
  if(args[0]==="native"){const policy=readCargoTestPolicyV1(process.env);await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,policy.buildBudgetMs>0?policy.buildBudgetMs:buildPreparationBudgetMs())},cwd:this.repoRoot,manifestPaths:{"semio-framework-value-persistent":resolve(this.root,"../../📋️list/🔗️persistent/📦️packages/🦀️rust/Cargo.toml")},groups:[{"package":"semio-framework-value-persistent","target":{"kind":"lib","name":"semio_framework_value_persistent"},"laws":["slice_equality::tests::retained_slice_equality_matches_portable_bytes_without_allocations","slice_equality::tests::retained_slice_equality_wide_typed_pages_and_cancellation_are_bounded"]}]});}
 }
}

/** 🧵️ Verifies neutral named edits, exact paged keys and genuine original directory custody. */
class OriginalVectorConcatCheckScript extends BundleScript{
 async run(args:string[]):Promise<void>{
  assert(args.length===1&&["source","native"].includes(args[0]!),"test-original-vector-concat requires source or native");
  const source=resolve(this.root,"../../📋️list/🪢️concat/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),source],this.repoRoot,"value:original-vector-concat:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"value:original-vector-concat:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
  if(args[0]==="native"){const policy=readCargoTestPolicyV1(process.env);await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,policy.buildBudgetMs>0?policy.buildBudgetMs:buildPreparationBudgetMs())},cwd:this.repoRoot,manifestPaths:{"semio-framework-value-persistent":resolve(this.root,"../../📋️list/🔗️persistent/📦️packages/🦀️rust/Cargo.toml")},groups:[{"package":"semio-framework-value-persistent","target":{"kind":"lib","name":"semio_framework_value_persistent"},"laws":["original_vector_concat::tests::original_vector_concat_preserves_original_rows_and_cancelled_custody","original_vector_concat::tests::original_vector_concat_wide_payload_and_chunk_directory_remain_bounded"]}]});}
 }
}

class NamedEditCheckScript extends BundleScript{
 async run(args:string[]):Promise<void>{
  assert(args.length===1&&["source","native"].includes(args[0]!),"test-named-edit requires source or native");
  const source=resolve(this.root,"../../📋️list/🔺️edit/🔑️named/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),source],this.repoRoot,"value:named-edit:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"value:named-edit:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
  if(args[0]==="native"){const policy=readCargoTestPolicyV1(process.env);await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,policy.buildBudgetMs>0?policy.buildBudgetMs:buildPreparationBudgetMs())},cwd:this.repoRoot,manifestPaths:{"semio-framework-value-edit":resolve(this.root,"../../📋️list/🔺️edit/📦️packages/🦀️rust/Cargo.toml")},groups:[{"package":"semio-framework-value-edit","target":{"kind":"lib","name":"semio_framework_value_edit"},"laws":["named_edit::tests::named_edit_plan_matches_portable_keys_and_physical_custody","named_edit::tests::named_edit_plan_long_prefixes_and_cancellation_are_bounded"]}]});}
 }
}

/** 🔑️ Checks original UTF8 key pages against SQLite and genuine zero-heap native cursors. */
class ExactKeyLookupCheckScript extends BundleScript{
 async run(args:string[]):Promise<void>{
  assert(args.length===1&&["source","native"].includes(args[0]!),"test-exact-key-lookup requires source or native");
  const source=resolve(this.root,"../../🔑️lookup/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),source],this.repoRoot,"value:exact-key:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"value:exact-key:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
  if(args[0]==="native"){const policy=readCargoTestPolicyV1(process.env);await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,policy.buildBudgetMs>0?policy.buildBudgetMs:buildPreparationBudgetMs())},cwd:this.repoRoot,manifestPaths:{"semio-framework-value-registry":resolve(this.root,"../../📋️registry/📦️packages/🦀️rust/Cargo.toml")},groups:[{"package":"semio-framework-value-registry","target":{"kind":"lib","name":"semio_framework_value_registry"},"laws":["exact_key_lookup::tests::exact_key_lookup_matches_portable_opaque_identity_without_heap_work","exact_key_lookup::tests::exact_key_lookup_long_prefixes_and_cancellation_preserve_originals"]}]});}
 }
}

/** ✏️ Verifies shared strict indexed admission, header streaming and original directory custody. */
class IndexedEditCheckScript extends BundleScript{
 async run(args:string[]):Promise<void>{
  assert(args.length===1&&["source","native"].includes(args[0]!),"test-indexed-edit requires source or native");
  const source=resolve(this.root,"../../📋️list/🔺️edit/🧪️tests/🟦️.ts");
  const sources=[source,resolve(this.root,"../../📋️list/🔺️edit/🚚️commit/🧪️tests/🟦️.ts"),resolve(this.root,"../../📋️list/🔺️edit/🔑️named/🧪️tests/🟦️.ts")];
  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),...sources],this.repoRoot,"value:indexed-edit:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
  await runOwnedCommand(process.execPath,["test",...sources],this.repoRoot,"value:indexed-edit:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
  if(args[0]==="native"){const policy=readCargoTestPolicyV1(process.env);await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,policy.buildBudgetMs>0?policy.buildBudgetMs:buildPreparationBudgetMs())},cwd:this.repoRoot,manifestPaths:{"semio-framework-value-edit":resolve(this.root,"../../📋️list/🔺️edit/📦️packages/🦀️rust/Cargo.toml")},groups:[{"package":"semio-framework-value-edit","target":{"kind":"lib","name":"semio_framework_value_edit"},"laws":["indexed_edit::tests::indexed_edit_plan_matches_portable_coordinates_and_original_custody","indexed_edit::tests::indexed_edit_plan_wide_order_and_cancellation_remain_bounded","indexed_collection_commit::tests::indexed_collection_commit_preserves_original_payloads_and_exact_cancellation","indexed_collection_commit::tests::indexed_collection_commit_copies_large_authored_fields_without_cloning_survivors","named_edit::tests::named_edit_plan_matches_portable_keys_and_physical_custody","named_edit::tests::named_edit_plan_long_prefixes_and_cancellation_are_bounded"]}]});}
 }
}


/** 🗝️ Executes exact registry admission, reuse and foreign-owner laws under original process authority. */
class FixedKeySlotsCheckScript extends BundleScript{
 async run(args:string[]):Promise<void>{
  assert(args.length===1&&["source","native"].includes(args[0]!),"test-fixed-key-slots requires source or native");
  const source=resolve(this.root,"../../📋️registry/🎫️slots/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),source],this.repoRoot,"value:fixed-slots:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"value:fixed-slots:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
  if(args[0]==="native"){
   const policy=readCargoTestPolicyV1(process.env);
   await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,policy.buildBudgetMs>0?policy.buildBudgetMs:buildPreparationBudgetMs())},cwd:this.repoRoot,manifestPaths:{"semio-framework-value-registry":resolve(this.root,"../../📋️registry/📦️packages/🦀️rust/Cargo.toml")},groups:[{"package":"semio-framework-value-registry","target":{"kind":"lib","name":"semio_framework_value_registry"},"laws":["fixed_key_slots::tests::fixed_key_slots_match_portable_admission_and_stale_release"]}]});
  }
 }
}


/** 🎫️ Exercises shared scoped request owners with portable and independent consumer laws. */
class FixedRequestOwnersCheckScript extends BundleScript{
 async run(args:string[]):Promise<void>{
  assert(args.length===1&&["source","native"].includes(args[0]!),"test-fixed-request-owners requires source or native");
  const source=resolve(this.root,"../../📋️registry/🎫️requests/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),source],this.repoRoot,"value:request-owners:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"value:request-owners:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
  if(args[0]==="native"){const policy=readCargoTestPolicyV1(process.env);await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,policy.buildBudgetMs>0?policy.buildBudgetMs:buildPreparationBudgetMs())},cwd:this.repoRoot,manifestPaths:{"semio-framework-value-registry":resolve(this.root,"../../📋️registry/📦️packages/🦀️rust/Cargo.toml"),"semio-framework-value-edit":resolve(this.root,"../../📋️list/🔺️edit/📦️packages/🦀️rust/Cargo.toml")},groups:[{"package":"semio-framework-value-registry","target":{"kind":"lib","name":"semio_framework_value_registry"},"laws":["fixed_key_slots::tests::fixed_key_slots_match_portable_admission_and_stale_release","fixed_request_owners::tests::fixed_request_owners_match_portable_consumer_transitions","fixed_request_owners::tests::request_text_and_terminals_match_portable_boundaries","fixed_request_owners::tests::request_selector_preserves_original_binding_until_verified_promotion","fixed_request_owners::tests::original_source_lookup_matches_portable_reordered_operands","fixed_request_owners::tests::provisional_request_consumers_match_portable_live_and_pending_outputs"]},{"package":"semio-framework-value-edit","target":{"kind":"lib","name":"semio_framework_value_edit"},"laws":["indexed_edit::tests::indexed_edit_plan_matches_portable_coordinates_and_original_custody","indexed_edit::tests::indexed_edit_plan_wide_order_and_cancellation_remain_bounded"]}]});}
 }
}


/** 🚶️ Shared UTF8 scalar traversal and original buffers retain explicit caller work authority. */
class RetainedUtf8CheckScript extends BundleScript{
 async run(args:string[]):Promise<void>{
  assert(args.length===1&&["source","native"].includes(args[0]!),"test-retained-utf8 requires source or native");const source=resolve(this.root,"../../📝️text/🚶️utf8/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),source],this.repoRoot,"value:utf8:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"value:utf8:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
  if(args[0]==="native"){const policy=readCargoTestPolicyV1(process.env),manifest=resolve(this.root,"Cargo.toml");assert.equal(resolve(policy.manifestPath),manifest);await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,policy.buildBudgetMs>0?policy.buildBudgetMs:buildPreparationBudgetMs()),lawMilliseconds:scriptInvocationBudget(this.invocation,policy.assertionBudgets[policy.level]>0?policy.assertionBudgets[policy.level]:300000)},nativeEnv:{RUST_MIN_STACK:policy.rustMinStack},cwd:this.repoRoot,manifestPaths:{"semio-framework-value":resolve(this.root,"Cargo.toml")},groups:[{"package":"semio-framework-value","target":{"kind":"lib","name":"semio_framework_value"},"cargoArgs":["--locked"],"laws":["retained_utf8::tests::original_utf8_cursor_pages_match_portable_scalar_offsets_without_allocations","retained_utf8::tests::original_utf8_buffer_funds_transfer_and_every_cancellation_frontier"]}]});}
 }
}

await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dirname).register("test-native-payload-enum",NativePayloadEnumScript).register("test-native-retirement-loan",NativeRetirementLoanCheckScript).register("test-native-shared-receive",NativeSharedReceivingCheckScript).register("test-native-persistent-receive",NativePersistentReceivingCheckScript).register("test-retained-equality",TypedEqualityCheckScript).register("test-persistent-build",PersistentBuildCheckScript).register("test-borrowed-observations",BorrowedObservationScript).register("test-retained-utf8",RetainedUtf8CheckScript).register("test-sealed-frontier",SealedFrontierCheckScript).register("test-immutable-root-clone",ImmutableRootCloneCheckScript).register("test-close-demand",CloseDemandCheckScript).register("test-bulk-vec-retained",BulkVecRetainedCheckScript).register("test-plain-element-retirement-source",PlainElementRetirementSourceScript).register("test-paged-reservation-source",PagedReservationSourceScript).register("test-exclusive-retirement-source",ExclusiveRetirementSourceScript).register("test-original-retirement-demand",OriginalRetirementDemandScript).register("test-persistent-traversal",PersistentTraversalCheckScript).register("test-slice-equality",SliceEqualityCheckScript).register("test-original-vector-concat",OriginalVectorConcatCheckScript).register("test-named-root-receive",NamedRootReceivingCheckScript).register("test-named-key-index",NamedKeyIndexCheckScript).register("test-named-edit",NamedEditCheckScript).register("test-exact-key-lookup",ExactKeyLookupCheckScript).register("test-indexed-edit",IndexedEditCheckScript).register("test-fixed-request-owners",FixedRequestOwnersCheckScript).register("test-fixed-key-slots",FixedKeySlotsCheckScript).register("test", TestScript).register("test-owned-projection",OwnedProjectionCheckScript).register("test-immutable-projection",ImmutableProjectionCheckScript).register("test-compound-demands",CompoundDemandCheckScript).register("test-factory-owned-source", FactoryOwnedSourceCheckScript).register("test-shared-factory-retirement",SharedFactoryRetirementTestScript).register("test-native-literal-refusal-source",NativeLiteralRefusalSourceScript).register("test-native-recipient-source",NativeRecipientSourceTestScript).register("test-native-receiving-original-source",NativeReceivingOriginalSourceScript).register("test-native-recipient",NativeRecipientTestScript).register("test-decode-ownership", DecodeOwnershipTestScript).register("test-graphql-int64-source",GraphqlInt64SourceTestScript).register("test-type-ownership", TypeOwnershipTestScript).register("test-controlled-construction", ControlledValueTestScript).register("test-controlled-encoding",ControlledEncodingTestScript).register("test-refusal-portable", RefusalPortableTestScript).register("test-refusal-codec",RefusalCodecTestScript).register("test-ordered-retirement-source",OrderedRetirementSourceTestScript), { invocation: original, ...({ defaultCommand: "test" }) }));