import{expect,test}from "bun:test";
import{Database}from "bun:sqlite";
import Ajv from "ajv";
import{resolve}from "node:path";
import fixture from "./🔣️.json";
import schema from "./🧬️.json";
const plugin=resolve(import.meta.dir,"../..");
const compact=(value:string)=>value.replace(/\s+/g,"");
function method(source:string,name:string):string{
 const start=source.indexOf("fn "+name+"(");expect(start,"actual method "+name).toBeGreaterThanOrEqual(0);
 let at=source.indexOf("{",start),depth=1,end=at+1;while(depth&&end<source.length){if(source[end]==="{")depth++;else if(source[end]==="}")depth--;end++;}expect(depth).toBe(0);return compact(source.slice(at+1,end-1));
}
test("two authoring modes and cancellation agree with independent SQLite",()=>{
 const valid=new Ajv({strict:true}).compile(schema);expect(valid(fixture),JSON.stringify(valid.errors)).toBe(true);
 const db=new Database(":memory:");try{for(const row of fixture.modes){const result=db.query("SELECT CASE WHEN ?1=1 OR ?2=1 THEN char(111,119,110,101,100) WHEN ?3=1 THEN char(98,111,114,114,111,119,101,100) ELSE char(114,101,102,117,115,101) END AS mode, CASE WHEN ?4=0 AND (?1=1 OR ?2=1 OR ?3=1) THEN 1 ELSE 0 END AS followChildren").get(+row.identity,+row.completion,+row.original,+row.cancelled)as{mode:string;followChildren:number};expect(result.mode).toBe(row.mode);expect(Boolean(result.followChildren)).toBe(row.followChildren);}}finally{db.close();}
});
test("genuine original emission borrows and revalidates the parent without minting continuation",async()=>{
 const source=await Bun.file(resolve(plugin,"🧵️instance/🧾️typed-operations/📥️staging/📬️publication/🏃️mounted-run-and-pump/🦀️.rs")).text();
 const borrowed=method(source,fixture.borrowedMethod);
 expect(borrowed).toContain("mounted."+fixture.validator+"(cx)?");
 const advance=await Bun.file(resolve(plugin,"🧵️instance/🧾️typed-operations/📥️staging/📬️publication/🔁️stall-witness-and-unit-advance/🦀️.rs")).text();
 const revalidate=method(advance,"revalidate_mounted_publication");expect(revalidate).toContain("mounted."+fixture.validator+"(cx)?");expect(revalidate).toContain("revalidate_interaction_on_document_change(meta,identity)");
 const unit=method(advance,"advance_typed_operation_publication_unit");expect(unit).toContain("mounted."+fixture.predicate+"()");expect(unit).toContain("revalidate_mounted_publication(&mutmounted,identity,cx)");
 const pump=method(source,"publish_mounted_typed_operation_pump");expect(pump).toContain("ifcx.is_cancelled(){ifletSome(lease)=mounted.cancellation_lease.as_ref(){lease.cancel();}}");expect(pump).toContain("lease.publication_is_cancelled()");expect(pump).toContain("!acknowledged_page&&!cancelled");
 expect(borrowed.indexOf("lease.publication_is_cancelled()")).toBeLessThan(borrowed.indexOf("follow_derivable_children"));
 expect(revalidate.indexOf("cx.is_cancelled()")).toBeLessThan(revalidate.indexOf("letmeta=&mounted.meta"));
 expect(borrowed).toContain("publish_mounted_typed_operation_run_admitted(mounted,identity,cx,retained)");
 expect(borrowed).toContain("follow_derivable_children(&mut*self.stores,identity)");
 expect(borrowed).not.toContain("NativeEncodeControl::new");expect(borrowed).not.toContain("EntityIdentityAuthority::new");
 expect(borrowed).not.toContain("take_authoring_identity");
 const owned=method(source,fixture.ownedMethod);expect(owned).toContain("take_authoring_identity");expect(owned).toContain("EntityIdentityAuthority::resume");
});

test("native fixture reaches actual intake, pump and paid ACK without a substituted producer",async()=>{
 const source=await Bun.file(resolve(import.meta.dir,"🦀️.rs")).text();for(const port of ["dispatch_original_emission","advance_typed_operation_publication","OriginalFixtureScope::admit","observe_heap_allocations_on_this_thread","original_lifecycle_delta","present_fixture_mounted_page","close_original_retained_step","acknowledge_result_page","take_typed_operation_preparation_receipt","take_typed_operation_retirement_receipt"])expect(source).toContain(port);for(const bypass of ["install_authoring_identity","NativeEncodeControl::new","MountedTypedCommandFullOperation {","renew_turn","renew_payload_page_grant","retire_fixture_acknowledged_mounted_page"])expect(source).not.toContain(bypass);
});
