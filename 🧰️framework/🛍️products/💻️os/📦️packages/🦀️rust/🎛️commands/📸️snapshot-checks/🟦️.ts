/** 📸️ Snapshot commands: route admission, codec receiving, native and sqlite admission, typed snapshot files and space history.
 *
 * @see ../../📜️script.ts — the package script that registers the commands of this module
 */
import { createIssuedResourceControlV1, scriptInvocationBudget } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runAdmittedCargoTestsV1 as runCargoTestsV1, runAdmittedCargoPackageTestsV1 } from "@semio-tech/process/cargo-test";
import { runBudgetedTestCommand } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { readCargoTestPolicyV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { runRepositoryTestCommand } from "@semio-tech/process/test-execution";
import { BundleScript } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { semioSchemaAjvV1 } from "@semio-tech/schema-ts/oracles/validator";
import type { SchemaCheck } from "../🧱️script-bases/🟦️.ts";
import { runtimeTypeEnvironmentV1 } from "../../../../../../🔨️modules/⚙️runtime/🏷️identity/🟦️.ts";
import { SqliteDatabaseV1 } from "../../../../../../🔨️modules/⚙️runtime/🗃️sqlite/🟦️.ts";
import { runtimeResolveV1 } from "../../../../../../🔨️modules/⚙️runtime/🚀️process/🟦️.ts";

export class SpaceHistorySqliteNativeTestScript extends BundleScript {
  async run(segments:string[]):Promise<void>{const {rest}=resolveTestLevel(segments);await runAdmittedCargoPackageTestsV1(["semio-framework-os-kernel"], this.repoRoot, this.invocation, ["--lib","sqlite_snapshot_framework_space_history_","--no-fail-fast",...rest,...(rest.includes("--")?[]:["--","--nocapture"])]);}
}

export class SpaceHistorySqliteSourceTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-space-history-sqlite-source accepts no arguments");
    const source = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts");
    await runBudgetedTestCommand(process.execPath, [runtimeResolveV1("typescript/bin/tsc", this.root), "--noEmit", "--strict", "--skipLibCheck", "--allowImportingTsExtensions", "--resolveJsonModule", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", runtimeTypeEnvironmentV1(), source], { cwd: this.repoRoot, budgetMs: 30000, throwOnFailure: true });
    await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot });
  }
}

/** 🛂️ Verifies public route admission before original payload transfer or native backing effects. */
export class SnapshotRouteAdmissionScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length!==1||segments[0]===undefined||!['source','native'].includes(segments[0]))throw Error('test-snapshot-route-admission requires source or native');
  const owner=join(this.repoRoot,'🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io/🧪️tests/🪶️transfer/🛂️route');assert.equal(existsSync(join(owner,'🧬️schema/🔣️.json')),false,'Testing examples have no schema authority');const fixture=JSON.parse(readFileSync(join(owner,'🧪️tests/🧫️fixtures/🔣️.json'),'utf8'));const ajv=semioSchemaAjvV1({strict:true,allErrors:true});
  for(const path of ['🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/🔣️.json','🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json'])ajv.addSchema(JSON.parse(readFileSync(join(this.repoRoot,path),'utf8')));
  const route:SchemaCheck=ajv.compile({$ref:"https://json.schemas.assets.semio-tech.com/framework/io/schema.json#/$defs/IoRoute"});for(const row of fixture.cases)assert(route(row.route),JSON.stringify(route.errors));const grant:SchemaCheck=ajv.compile(JSON.parse(readFileSync(join(this.repoRoot,'🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🌐️wire/🧬️schema/🔣️.json'),'utf8')));assert(grant(fixture.original.grant),JSON.stringify(grant.errors));
  const identities=['empty-original-route','disconnected-original-route','sqlite-intermediate-original-route','sqlite-nonexact-original-route','sqlite-self-original-route'];
  assert.deepEqual(fixture.cases.map((row:{id:string})=>row.id),identities);
  const duplicate=structuredClone(fixture);duplicate.cases[1].id=duplicate.cases[0].id;
  const forged=structuredClone(fixture);forged.cases[0].cause='unowned replacement cause';
  const enlarged=structuredClone(fixture);enlarged.original.grant.maximumItems=1;
  const{Database}={ Database: SqliteDatabaseV1 };const oracle=new Database(':memory:');try{
   oracle.run("CREATE TABLE original_cases(id TEXT PRIMARY KEY,cause TEXT NOT NULL CHECK((id IN ('empty-original-route','disconnected-original-route') AND cause='io_run: route must contain connected hops') OR (id IN ('sqlite-intermediate-original-route','sqlite-nonexact-original-route') AND cause='io_run: SQLite snapshots are exact endpoint transfers and cannot be intermediate hops') OR (id='sqlite-self-original-route' AND cause='io_run: inconsistent SQLite snapshot descriptor'))) WITHOUT ROWID");
   for(const row of fixture.cases)oracle.run('INSERT INTO original_cases VALUES(?,?)',row.id,row.cause);
   assert.equal((oracle.query('SELECT count(*) count FROM original_cases').get()as{count:number}).count,identities.length);
   assert.throws(()=>oracle.run('INSERT INTO original_cases VALUES(?,?)',duplicate.cases[1].id,duplicate.cases[1].cause));
   assert.throws(()=>oracle.run('UPDATE original_cases SET cause=? WHERE id=?',forged.cases[0].cause,forged.cases[0].id));
   assert.deepEqual(oracle.query('SELECT id,cause FROM original_cases ORDER BY id').all(),fixture.cases.map((row:{id:string;cause:string})=>({id:row.id,cause:row.cause})).sort((left:{id:string},right:{id:string})=>left.id.localeCompare(right.id)));
   oracle.run('BEGIN');oracle.query('DELETE FROM original_cases WHERE id=?').run(fixture.cases[3].id);assert.notEqual((oracle.query('SELECT count(*) count FROM original_cases').get()as{count:number}).count,identities.length);oracle.run('ROLLBACK');
   oracle.run('CREATE TABLE original_authority(axis TEXT PRIMARY KEY,remaining INTEGER NOT NULL CHECK(remaining=0)) WITHOUT ROWID');for(const[axis,remaining]of Object.entries(fixture.original.grant)){if(typeof remaining!=="number")throw Error("Numeric original authority required");oracle.run('INSERT INTO original_authority VALUES(?,?)',[axis,remaining]);}
   assert.throws(()=>oracle.run('UPDATE original_authority SET remaining=? WHERE axis=?',enlarged.original.grant.maximumItems,'maximumItems'));
   assert.deepEqual(oracle.query('SELECT axis,remaining FROM original_authority ORDER BY axis').all(),Object.entries(fixture.original.grant).map(([axis,remaining])=>({axis,remaining})).sort((left,right)=>left.axis.localeCompare(right.axis)));
   oracle.run('CREATE TABLE original_hops(ordinal INTEGER PRIMARY KEY,source TEXT,destination TEXT,is_sqlite INTEGER)');for(const row of fixture.cases){oracle.run('DELETE FROM original_hops');for(const[index,hop]of row.route.hops.entries()){const coordinate=(value:{artifactKind:string;standard:string;subset:string})=>JSON.stringify([value.artifactKind,value.standard,value.subset]);const sqlite=(value:{artifactKind:string;standard:string;subset:string})=>value.artifactKind==='framework.io.sqlite-snapshot'&&value.standard==='1'&&value.subset==='*';oracle.run('INSERT INTO original_hops VALUES(?,?,?,?)',index,coordinate(hop.from),coordinate(hop.into),Number(sqlite(hop.from)||sqlite(hop.into)));}const observed=oracle.query("SELECT CASE WHEN count(*)=0 OR EXISTS(SELECT 1 FROM original_hops a JOIN original_hops b ON b.ordinal=a.ordinal+1 WHERE a.destination<>b.source) THEN 'io_run: route must contain connected hops' WHEN max(is_sqlite)=1 AND (count(*)<>1 OR ?<>'Exact') THEN 'io_run: SQLite snapshots are exact endpoint transfers and cannot be intermediate hops' WHEN max(is_sqlite)=1 AND min(source=destination)=1 THEN 'io_run: inconsistent SQLite snapshot descriptor' ELSE NULL END cause FROM original_hops").get(row.route.fidelity)as{cause:string|null};assert.equal(observed.cause,row.cause,row.id);assert.equal(observed.cause===null,row.accepted,row.id);}}finally{oracle.close();}
  if(segments[0]==='native')await runAdmittedCargoPackageTestsV1(['semio-framework-os-kernel'],this.repoRoot,this.invocation,['--test','sqlite_snapshot_native_admission','sqlite_snapshot_whole_hop_original_route_refusals','--no-fail-fast','--success-output','immediate']);
 }
}

/** 🫴️ Checks neutral codec custody and independently reads its literal SQL owner. */
export class SnapshotCodecReceivingScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length!==1||segments[0]===undefined||!['source','native'].includes(segments[0]))throw Error('test-snapshot-codec-receiving requires source or native');
  const owner=join(this.repoRoot,'🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🪶️native-retirement');
  const fixture=JSON.parse(readFileSync(join(owner,'🧪️tests/🧫️fixtures/🔣️.json'),'utf8'));
  const actualGrant=JSON.parse(readFileSync(join(this.repoRoot,'🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🌐️wire/🧬️schema/🔣️.json'),'utf8')),validateGrant=semioSchemaAjvV1({strict:true,allErrors:true}).compile(actualGrant);assert(validateGrant(fixture.original.grant),JSON.stringify(validateGrant.errors));
  assert(validateGrant(fixture.original.defaultRefusalGrant),JSON.stringify(validateGrant.errors));
  const{Database}={ Database: SqliteDatabaseV1 };const reference=new Database(':memory:');try{reference.run(readFileSync(join(owner,'🧬️schema/🗄️.sql'),'utf8'));reference.run('INSERT INTO retained_value VALUES(1,7)');assert.deepEqual(reference.query('SELECT id,value FROM retained_value').all(),[{id:1,value:7}]);assert.equal((reference.query('PRAGMA integrity_check').get()as {integrity_check:string}).integrity_check,'ok');}finally{reference.close();}
  assert.equal(fixture.custodyCases.find((row:{id:string})=>row.id==='import-refused-frame-birth').callerRetainsDatabase,true);
  assert.equal(fixture.custodyCases.find((row:{id:string})=>row.id==='import-refused-release').recipientOccupied,true);
  const ordered=new Database(':memory:');try{ordered.run('CREATE TABLE original_values(ordinal INTEGER PRIMARY KEY,value INTEGER)');for(const row of fixture.relocationCases){ordered.run('DELETE FROM original_values');for(const[index,value]of row.values.entries())ordered.run('INSERT INTO original_values VALUES(?,?)',index,value);assert.deepEqual(ordered.query<{value:number},[]>('SELECT value FROM original_values ORDER BY ordinal').all().map(value=>value.value),row.values);assert.deepEqual([...row.original,...row.replacement].sort((a:number,b:number)=>a-b),[...row.values].sort((a:number,b:number)=>a-b));if(row.completed)assert.deepEqual(row.original,row.values);}}finally{ordered.close();}
  const defaults=new Database(':memory:');try{defaults.run('CREATE TABLE refusal(ordinal INTEGER PRIMARY KEY,operation TEXT,kind TEXT,cause TEXT)');for(const[index,row]of fixture.defaultRefusalCases.entries())defaults.run('INSERT INTO refusal VALUES(?,?,?,?)',index,row.operation,row.kind,row.cause);assert.deepEqual(defaults.query('SELECT operation,kind,cause FROM refusal ORDER BY ordinal').all(),fixture.defaultRefusalCases.map((row:{operation:string;kind:string;cause:string})=>({operation:row.operation,kind:row.kind,cause:row.cause})));}finally{defaults.close();}
  if(segments[0]==='native')await runAdmittedCargoPackageTestsV1(['semio-framework-os-kernel'],this.repoRoot,this.invocation,['--test','sqlite_snapshot_native_admission','sqlite_snapshot_codec_','--no-fail-fast','--success-output','immediate']);
 }
}

export class SnapshotNativeAdmissionTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runRepositoryTestCommand(process.execPath, ["test", join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🪶️native-decoding/🧪️tests/🚪️public/🧩️ownership/🟦️.ts"), join(this.repoRoot, "🧰️framework/🔨️modules/🌱️value/🛬️decode/🧪️tests/🟦️.ts"), join(this.repoRoot, "🧰️framework/🔨️modules/🗣️dsl/📖️grammar/📡️literal/🧪️tests/🟦️.ts"), join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🧪️tests/🚦️controlled/🟦️.ts"), join(this.repoRoot, "🧰️framework/🔨️modules/🚪️io/🧬️schema/🔗️reference/🧪️tests/🟦️.ts")], { cwd: this.repoRoot });
    if (segments.length === 1 && segments[0] === "portable") return;
    const { rest } = resolveTestLevel(segments);
    await runAdmittedCargoPackageTestsV1(["semio-framework-os-kernel"], this.repoRoot, this.invocation, ["--test", "sqlite_snapshot_native_admission", ...rest, ...(rest.includes("--no-fail-fast") ? [] : ["--no-fail-fast"])]);
  }
}

/** 🪶️ Verifies native OS SQLite snapshot admission under its exact owner. */
export class SnapshotSqliteAdmissionScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
 if(segments.length)throw Error("test-snapshot-sqlite-admission accepts no arguments");
 await runAdmittedCargoPackageTestsV1(["semio-framework-os-kernel"], this.repoRoot, this.invocation, ["--test","sqlite_snapshot_native_admission"]);
 }
}

/** 📸️ Executes complete legal snapshots through independent SQLite files before original Native custody qualification. */
export class TypedSnapshotFileSourceScript extends BundleScript{
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw Error("test-typed-snapshot-file-source accepts no arguments");
  const source=join(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io/📸️typed-snapshot/🧪️tests/🟦️.ts");
  await runBudgetedTestCommand(process.execPath,[runtimeResolveV1("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),source],{cwd:this.repoRoot,budgetMs:30000,throwOnFailure:true});
  await runRepositoryTestCommand(process.execPath,["test",source],{cwd:this.repoRoot});
 }
}

/** 🪶️ Executes the actual complete builtin snapshot file law through supplied original receiving callers. */
export class TypedSnapshotFileNativeScript extends BundleScript{
 async run(segments:string[]):Promise<void>{
    const cargoPhase=createIssuedResourceControlV1(this.invocation,"cargo:script-entry",0);
  if(segments.length)throw Error("test-typed-snapshot-file-native accepts no arguments");
  await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-os-kernel"],cwd:this.repoRoot,extraArgs:["--lib","sqlite_snapshot_framework_space_history_actual_typed_io_file_metadata_and_route","--no-fail-fast","--success-output","immediate"],signal:this.invocation.control.signal,remainingMilliseconds:()=>this.invocation.control.remainingMilliseconds()},readCargoTestPolicyV1(process.env), this.repoRoot, this.invocation,cargoPhase);
 }
}

/** 🛂️ Qualifies borrowed seven-table History validation under original directional receiving authority. */
export class SpaceHistoryValidationScript extends BundleScript{
 async run(segments:string[]):Promise<void>{
    const cargoPhase=createIssuedResourceControlV1(this.invocation,"cargo:script-entry",0);
  if(segments.length!==1||!["source","native"].includes(segments[0]!))throw Error("test-space-history-validation requires source or native");
  if(segments[0]==="source"){
   const source=join(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🛂️validation/🧪️tests/🟦️.ts");
   await runBudgetedTestCommand(process.execPath,[runtimeResolveV1("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),source],{cwd:this.repoRoot,budgetMs:30000,throwOnFailure:true});
   await runRepositoryTestCommand(process.execPath,["test",source],{cwd:this.repoRoot});
  }else await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-os-kernel"],cwd:this.repoRoot,extraArgs:["--lib","sqlite_snapshot_framework_space_history_borrowed_validation_","--no-fail-fast","--success-output","immediate"],signal:this.invocation.control.signal,remainingMilliseconds:()=>this.invocation.control.remainingMilliseconds()},readCargoTestPolicyV1(process.env), this.repoRoot, this.invocation,cargoPhase);
 }
}

export class StableIdentityCheckScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length > 1 || (args.length && args[0] !== "--native")) throw Error("stable-identity-check accepts only --native");
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🪡️list-delta");
    const fixtures = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🪪️stable-identity/🔣️.json"), "utf8")) as { vectors: { id: string; base: { id: string; value: number }[]; patches: { target: string; row: { id: string; value: number } }[]; accepted: boolean; expected: { id: string; value: number }[] }[] };
    const { Database } = { Database: SqliteDatabaseV1 };
    for (const vector of fixtures.vectors) {
      const database = new Database(":memory:");
      try {
        database.run("CREATE TABLE rows (position INTEGER PRIMARY KEY, id TEXT UNIQUE NOT NULL, value INTEGER NOT NULL)");
        database.run("CREATE TRIGGER immutable_identity BEFORE UPDATE OF id ON rows WHEN NEW.id <> OLD.id BEGIN SELECT RAISE(ABORT, 'identity changed'); END");
        vector.base.forEach((row, position) => database.run("INSERT INTO rows VALUES (?, ?, ?)", [position, row.id, row.value]));
        let accepted = true;
        database.run("BEGIN");
        try {
          for (const patch of vector.patches) {
            const result = database.run("UPDATE rows SET id = ?, value = ? WHERE id = ?", [patch.row.id, patch.row.value, patch.target]);
            if (result.changes !== 1) throw Error("missing target");
          }
          database.run("COMMIT");
        } catch {
          accepted = false;
          database.run("ROLLBACK");
        }
        assert.equal(accepted, vector.accepted, vector.id);
        assert.deepEqual(database.query("SELECT id, value FROM rows ORDER BY position").all(), vector.expected, vector.id);
      } finally {
        database.close();
      }
    }
    const nested = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🪆️nested-inverse/🔣️.json"), "utf8")) as { vectors: { id: string; base: { id: string; value: number; label: string; parts: { id: string; weight: number }[] }[]; patches: { id: string; patch: { parts: { removed: { id: string; index: number }[]; inserted: { index: number; row: { id: string; weight: number } }[] } } }[]; expected: unknown }[] };
    for (const vector of nested.vectors) {
      const database = new Database(":memory:");
      try {
        database.run("CREATE TABLE parts (parent TEXT NOT NULL, position INTEGER NOT NULL, id TEXT UNIQUE NOT NULL, weight INTEGER NOT NULL)");
        vector.base.forEach(row => row.parts.forEach((part, position) => database.run("INSERT INTO parts VALUES (?, ?, ?, ?)", [row.id, position, part.id, part.weight])));
        const insert = (parent: string, index: number, row: { id: string; weight: number }): void => {
          database.run("UPDATE parts SET position = position + 1 WHERE parent = ? AND position >= ?", [parent, index]);
          database.run("INSERT INTO parts VALUES (?, ?, ?, ?)", [parent, index, row.id, row.weight]);
        };
        const remove = (parent: string, index: number, id: string): void => {
          assert.equal(database.run("DELETE FROM parts WHERE parent = ? AND position = ? AND id = ?", [parent, index, id]).changes, 1);
          database.run("UPDATE parts SET position = position - 1 WHERE parent = ? AND position > ?", [parent, index]);
        };
        const inverse: (() => void)[] = [];
        for (const mutation of vector.patches) {
          for (const row of mutation.patch.parts.removed) {
            const before = database.query("SELECT id, weight FROM parts WHERE parent = ? AND position = ?").get(mutation.id, row.index) as { id: string; weight: number };
            remove(mutation.id, row.index, row.id);
            inverse.push(() => insert(mutation.id, row.index, before));
          }
          for (const row of mutation.patch.parts.inserted) {
            insert(mutation.id, row.index, row.row);
            inverse.push(() => remove(mutation.id, row.index, row.row.id));
          }
        }
        const projection = () => vector.base.map(row => ({ ...row, parts: database.query("SELECT id, weight FROM parts WHERE parent = ? ORDER BY position").all(row.id) }));
        assert.deepEqual(projection(), vector.expected, vector.id);
        inverse.reverse().forEach(replay => replay());
        assert.deepEqual(projection(), vector.base, vector.id + " inverse");
      } finally {
        database.close();
      }
    }
    if (args[0] === "--native") {
      await runAdmittedCargoPackageTestsV1(["semio-framework-os-kernel"], this.repoRoot, this.invocation, ["--lib", "-E", "test(stable_identity_) | test(nested_inverse_)", "--no-fail-fast"]);
    }
  }
}

export class SnapshotReceivingNativeScript extends BundleScript{
 async run(args:string[]):Promise<void>{
  if(args.length)throw Error("Expected test-snapshot-receiving-native");
  await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-os-kernel"],cwd:this.repoRoot,extraArgs:["--lib","original_receiving_","--no-fail-fast","--success-output","immediate"],signal:this.invocation.control.signal,remainingMilliseconds:()=>this.invocation.control.remainingMilliseconds()},readCargoTestPolicyV1(process.env),this.repoRoot,this.invocation,createIssuedResourceControlV1(this.invocation,"cargo:script-entry",0));
 }
}

export class OriginalSnapshotReceivingScript extends BundleScript{
 async run(segments:string[]):Promise<void>{
  const mode=segments[0];if(segments.length!==1||(mode!=="source"&&mode!=="native"))throw Error("test-original-snapshot-receiving source|native");
  const sources=[join(this.repoRoot,"🧰️framework/🔨️modules/🎒️pack/🌱️value/🛬️decode/🫴️receiving/🧪️tests/🟦️.ts"),join(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/📸️native-snapshot/🫴️receiving/📜️pair/🧪️tests/🟦️.ts"),join(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🛬️receive/🧪️tests/🟦️.ts")];
  for(const source of sources){
  await runBudgetedTestCommand(process.execPath,[runtimeResolveV1("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),source],{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,0),signal:this.invocation.control.signal,throwOnFailure:true});
  await runBudgetedTestCommand(process.execPath,["test",source],{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,0),signal:this.invocation.control.signal,throwOnFailure:true});
  }
  if(mode==="native")await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-os-kernel"],cwd:this.repoRoot,extraArgs:["--lib","original_snapshot_pack_receiving_preserves_same_source_control_and_paid_close","--no-fail-fast","--success-output","immediate"],signal:this.invocation.control.signal,remainingMilliseconds:()=>this.invocation.control.remainingMilliseconds()},readCargoTestPolicyV1(process.env),this.repoRoot,this.invocation,createIssuedResourceControlV1(this.invocation,"cargo:script-entry",0));
  if(mode==="native")await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-os-kernel"],cwd:this.repoRoot,extraArgs:["--lib","original_child_receive_same_caller_all_partial_prefixes_and_literal_target","--no-fail-fast","--success-output","immediate"],signal:this.invocation.control.signal,remainingMilliseconds:()=>this.invocation.control.remainingMilliseconds()},readCargoTestPolicyV1(process.env),this.repoRoot,this.invocation,createIssuedResourceControlV1(this.invocation,"cargo:script-entry",0));
 }
}
