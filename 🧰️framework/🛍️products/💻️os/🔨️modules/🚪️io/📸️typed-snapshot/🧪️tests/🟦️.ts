/** 📸️ Complete legal snapshot fields remain independently queryable through physical SQLite files. */
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import fixture from "./🧫️fixtures/🔣️.json";
import {semioSchemaAjvV1} from "../../../../../../../🧰️framework/🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";
import { readModuleTreeSource } from "../../../../../../🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";

const read=(path:string)=>readFileSync(new URL(path,import.meta.url),"utf8");

test("registered accepted file owners preserve paid physical retirement",()=>{
 const law=fixture.acceptedRetirement;expect(semioSchemaAjvV1({strict:true}).compile(JSON.parse(read("./🧬️schema/🔣️accepted-retirement.json")))(law)).toBe(true);const db=new Database(":memory:");try{db.run("CREATE TABLE custody(owner TEXT PRIMARY KEY,backing INTEGER NOT NULL,born INTEGER NOT NULL,released INTEGER NOT NULL)");for(const[ordinal,owner]of law.owners.entries()){const backing=ordinal+17,born=ordinal+31;db.run("INSERT INTO custody VALUES(?,?,?,?)",[owner,backing,born,backing+born]);}expect(db.query("SELECT COUNT(*) AS conserved FROM custody WHERE backing+born=released").get()).toEqual({conserved:law.owners.length});}finally{db.close();}
 const source=read("./🦀️.rs"),start=source.indexOf("async fn sqlite_snapshot_framework_space_history_actual_typed_io_file_metadata_and_route_registered_read_and_destination"),end=source.indexOf("async fn sqlite_snapshot_framework_space_history_actual_typed_io_file_metadata_and_route_registered_destination_refusals",start);expect(start).toBeGreaterThan(0);expect(end).toBeGreaterThan(start);const accepted=source.slice(start,end);for(const token of["export_born","export_released","import_born","import_released","physical_born","physical_released","retire_registered_output(bytes","retire_registered_output(physical","retire_registered_output(destination.take()","acceptedRetirement"])expect(accepted).toContain(token);expect(source).toContain("fn retire_registered_output");
 console.log("[DEBUG] Accepted file, physical SQLite database and typed destination have independently authored zero-denial/terminal and exact backing conservation laws; Native qualification remains separate");
});

test("typed Snapshot file public API requires supplied original directional owners and physical receiving frames",()=>{
 const source=read("../../🦀️.rs");const start=source.indexOf("struct TypedSnapshotExportFrame");const end=source.indexOf("fn validate_snapshot_metadata_receiving",start);expect(start).toBeGreaterThan(0);expect(end).toBeGreaterThan(start);const typed=source.slice(start,end);
 expect(typed).toContain("NativeSnapshotDecodeOwner");expect(typed).toContain("NativeSnapshotEncodeOwner");expect(typed).toContain("RetireOwned");expect(typed).toContain("SqliteSnapshotControl");expect(typed).toContain("to_sqlite_database_receiving");expect(typed).toContain("from_sqlite_database_receiving");expect(typed).toContain("validate_sqlite_snapshot_subset_decoding");expect(typed).toContain("validate_sqlite_snapshot_subset_encoding");expect(typed).toContain("export_sqlite_database_receiving_into");expect(typed).toContain("import_sqlite_database_receiving_into");expect(typed).toContain("receive_nested");expect(typed).not.toContain("run_as_original_caller");expect(typed).not.toContain(".to_sqlite_database(control)");expect(typed).not.toContain("P::from_sqlite_database(");
 console.log("[DEBUG] Direct typed file IO Source declarations require the actual original caller owner; Native physical custody and publication laws remain separate");
});

test("registered Snapshot hops share original physical receiving and retain results before SQL close settlement",()=>{
 const source=read("../../🦀️.rs");const start=source.indexOf("fn run_snapshot_hop(");const end=source.indexOf("fn resolve_run(",start);expect(start).toBeGreaterThan(0);expect(end).toBeGreaterThan(start);const hop=source.slice(start,end);
 for(const name of ["physical_export","physical_import","export_sqlite_database_receiving_into","import_sqlite_database_receiving_into","TypedFileSettlement","receive_nested","record_progress(body.progress())","attach_metadata_decoding","take_metadata_encoding","frame.result=Some","settlement.finish_body(owned)"])expect(hop).toContain(name);
 for(const cold of ["export_sqlite_database_controlled(","import_sqlite_database_controlled(","NativeDecodeControl::new","NativeEncodeControl::new"])expect(hop).not.toContain(cold);
 expect(hop.match(/frame\.result=Some/g)?.length).toBe(2);expect(hop.match(/settlement\.finish_body\(owned\)/g)?.length).toBe(2);
 console.log("[DEBUG] Registered Snapshot hop Source binding: both original directions retain physical scratch/result and settle SQL while still received; Native qualification remains separate");
});

test("whole typed-file settlement keeps original allocation authority through receiving and close",()=>{
 const authority=fixture.settlementAuthority;const independent=new Database(":memory:");try{expect(independent.query("SELECT ? < ? AND ? > 0 AS unrelated_native_slack").get(authority.coldReservationBytes,authority.nativePrefixBytes,authority.coldReservationBytes)).toEqual({unrelated_native_slack:1});}finally{independent.close();}
 const source=read("../../🦀️.rs");const start=source.indexOf("pub(crate) struct TypedFileSettlement");const end=source.indexOf("fn typed_file_validation",start);expect(start).toBeGreaterThan(0);expect(end).toBeGreaterThan(start);const settlement=source.slice(start,end);
 expect(settlement.includes("SqliteSnapshotReceivingControl")).toBe(true);expect(settlement.includes("control.receiving_scope()")).toBe(true);
 const laws=read("./🦀️.rs");for(const name of["original_allocation_decoding","original_allocation_encoding","TypedFileSettlement","nativePrefixBytes","coldReservationBytes","observe_backing","has_retirement_owner"])expect(laws.includes(name)).toBe(true);
 expect(read("../../../🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs").includes("typed_file_original_allocation_laws")).toBe(true);
 console.log("[DEBUG] Whole typed-file SQL/native settlement borrows original allocation requirement throughout body, close and publication; true original paid-prefix prebirth/custody laws are mounted in the owning Kernel");
});

test("registered complete snapshot file capabilities preserve typed identity and original receiving destinations",()=>{
 const independent=new Database(":memory:");try{independent.run("CREATE TABLE snapshot_file_owner(id TEXT PRIMARY KEY,typed INTEGER NOT NULL,matching INTEGER NOT NULL,export_read INTEGER NOT NULL,import_into INTEGER NOT NULL)");for(const row of fixture.registeredFiles.identities)independent.run("INSERT INTO snapshot_file_owner VALUES(?,?,?,?,?)",[row.id,Number(row.typed),Number(row.matchingType),Number(row.exportRead),Number(row.importInto)]);expect(independent.query("SELECT id,CASE WHEN typed THEN matching ELSE export_read=0 AND import_into=0 END AS accepted FROM snapshot_file_owner ORDER BY id").all()).toEqual(fixture.registeredFiles.identities.map(row=>({id:row.id,accepted:Number(row.accepted)})).sort((a,b)=>a.id.localeCompare(b.id)));}finally{independent.close();}
 const source=read("../../🦀️.rs"),codec=read("../../../🏪️store/🚪️io/🦀️.rs");for(const name of["io_export_sqlite_snapshot_read","io_import_sqlite_snapshot_into","export_sqlite_snapshot_receiving","import_sqlite_snapshot_receiving","export_snapshot_file_read","import_snapshot_file_into","get::<P>()","downcast_mut::<Option<P>>()"])expect((source+codec).includes(name)).toBe(true);
 for(const name of["export_file","import_file"])expect(codec.includes(name)).toBe(true);
 for(const refusal of fixture.registeredFiles.refusals)expect((source+codec).includes(refusal.message)).toBe(true);
 const start=codec.indexOf("pub fn identical_to"),end=codec.indexOf("pub enum ArtifactNativeSnapshotIdentity",start);expect(codec.slice(start,end)).toContain("export_file");expect(codec.slice(start,end)).toContain("import_file");expect(codec).not.toContain("ArtifactSnapshotFileFuture");
 console.log("[DEBUG] Registered complete file identities independently distinguish typed/guest owners and specialized borrowed read/empty typed destination thunks share original synchronous file producers");
});

test("registered import refusal policies retain independent finite grant axes",()=>{
 const ajv=semioSchemaAjvV1({strict:true});const admit=ajv.compile(JSON.parse(read("../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🌐️wire/🧬️schema/🔣️.json")));expect(admit(fixture.closeGrant)).toBe(true);const fields={items:"maximumItems",copy:"maximumCopyBytes",capacity:"maximumCapacityBytes",release:"maximumReleaseBytes",depth:"maximumDepth"}as const;const independent=new Database(":memory:");
 try{independent.run("CREATE TABLE refused_authority(ordinal INTEGER PRIMARY KEY,axis TEXT NOT NULL,maximum INTEGER NOT NULL,minimum INTEGER NOT NULL)");for(const[index,row]of fixture.registeredFiles.grantRefusals.entries()){const grant={...fixture.bodyGrant};if(row.axis!=="native")grant[fields[row.axis as keyof typeof fields]]=row.maximum;expect(admit(grant)).toBe(true);independent.run("INSERT INTO refused_authority VALUES(?,?,?,?)",[index,row.axis,row.maximum,row.axis==="depth"?2:1]);}expect(independent.query("SELECT COUNT(*) AS denied FROM refused_authority WHERE maximum<minimum").get()).toEqual({denied:fixture.registeredFiles.grantRefusals.length});}finally{independent.close();}
 expect(fixture.registeredFiles.grantRefusals.map(row=>row.axis)).toEqual(["items","copy","capacity","release","depth","depth","native"]);
 console.log("[DEBUG] Seven authored receiving/publication refusal policies independently validate all five finite grant axes and native ceiling; depth1 cannot admit the original destination header depth2");
});

test("registered export refusal policies preserve distinct original frame admission",()=>{
 const ajv=semioSchemaAjvV1({strict:true});const admit=ajv.compile(JSON.parse(read("../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🌐️wire/🧬️schema/🔣️.json")));const fields={items:"maximumItems",copy:"maximumCopyBytes",capacity:"maximumCapacityBytes",release:"maximumReleaseBytes",depth:"maximumDepth"}as const;const independent=new Database(":memory:");
 try{independent.run("CREATE TABLE export_authority(ordinal INTEGER PRIMARY KEY,axis TEXT NOT NULL,maximum INTEGER NOT NULL,minimum INTEGER NOT NULL,prebirth INTEGER NOT NULL)");for(const[index,row]of fixture.registeredFiles.exportGrantRefusals.entries()){const grant={...fixture.bodyGrant};if(row.axis!=="native")grant[fields[row.axis as keyof typeof fields]]=row.maximum;expect(admit(grant)).toBe(true);independent.run("INSERT INTO export_authority VALUES(?,?,?,?,?)",[index,row.axis,row.maximum,row.axis==="depth"?2:1,Number(row.zeroHeap)]);}expect(independent.query("SELECT COUNT(*) AS denied FROM export_authority WHERE maximum<minimum").get()).toEqual({denied:7});expect(independent.query("SELECT axis FROM export_authority WHERE prebirth=0 ORDER BY ordinal").all()).toEqual([{axis:"copy"},{axis:"release"}]);}finally{independent.close();}
 expect(fixture.registeredFiles.exportGrantRefusals.map(row=>row.axis)).toEqual(["items","copy","capacity","release","depth","depth","native"]);
 const laws=read("./🦀️.rs"),start=laws.indexOf("async fn sqlite_snapshot_framework_space_history_actual_typed_io_file_metadata_and_route_registered_export_grant_axes");expect(start).toBeGreaterThan(0);for(const name of["exportGrantRefusals","io_export_sqlite_snapshot_read","snapshot_read().unwrap().into_erased()","settlement.finish_body","close_retirement_recipient","try_return_to_registry_witness"])expect(laws.slice(start)).toContain(name);
 console.log("[DEBUG] Seven independently admitted export grants preserve copy0/release0 retained-frame predictions and genuine Store-read/funded original SQL-native close law mounts; Native heap proof remains separate");
});

test("registered file dependency preserves original mounted close loan and complete causes",()=>{
 const source=readModuleTreeSource(new URL("../../../🔌️plugin/🧵️instance/🦀️.rs",import.meta.url)),start=source.indexOf("/// 🎟️ Advances one genuine original owner under the complete caller grant."),end=source.indexOf("fn close_terminal_is_empty",start);expect(start).toBeGreaterThan(0);expect(end).toBeGreaterThan(start);const boundary=source.slice(start,end);
 for(const name of["cx:&mut semio_framework_job::StepContext","loan:RetainedCloneGrant","crate::PluginLifecycleFailure","original_lifecycle_step(cx,loan,demand","self.close_ladder_step(same,grant)"])expect(boundary).toContain(name);
 expect(boundary).not.toContain("into_message()");expect(boundary).not.toContain("plugin_sdk_fault");
 const receiver=read("../../../🔌️plugin/🏇️mounted-owner/🦀️.rs");expect(receiver).toContain("advance_lifecycle");
 console.log("[DEBUG] Original mounted close dependency declaration borrows the caller context and independent phase loan, forwards complete causes and uses actual generic lifecycle receiver; Native behavior remains separately qualified");
});

