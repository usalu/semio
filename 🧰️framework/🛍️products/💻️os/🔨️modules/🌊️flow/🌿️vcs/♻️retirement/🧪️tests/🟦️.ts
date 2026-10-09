import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {applyPatch} from "fast-json-patch";
import stableStringify from "fast-json-stable-stringify";
import Ajv from "ajv";
import {Database} from "bun:sqlite";

/** 🌿️ Original VCS ownership laws use plain examples and independent JSONPatch/UTF8 references. */
export function testOriginalVcsClosure():void{
 const fixture=JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
 const schema=JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json",import.meta.url),"utf8"));
 const validate=new Ajv({strict:true,allErrors:true}).compile(schema);assert.equal(validate(fixture),true,JSON.stringify(validate.errors)?.slice(0,650));let assertions=1;
 for(const field of ["maximumItems","maximumCopyBytes","maximumCapacityBytes","maximumReleaseBytes","maximumDepth"]){const invalid=structuredClone(fixture);delete invalid.closeAdmissionCases[0].grant[field];assert.equal(validate(invalid),false);assertions++;}
 for(const field of ["copiedItems","copiedBytes","retainedCapacityBytes","releasedBytes"]){const invalid=structuredClone(fixture);delete invalid.closeReceiptCases[0].progress[field];assert.equal(validate(invalid),false);assertions++;}
 const invalid=structuredClone(fixture);invalid.closeReceiptCases[0].progress.depth=1;assert.equal(validate(invalid),false);assertions++;
 const db=new Database(":memory:");
 for(const row of fixture.closeAdmissionCases){const g=row.grant,d=row.demands;const reference=db.query("SELECT CASE WHEN ?1=0 THEN 'deferred' WHEN ?5<?9 THEN 'refused_depth' WHEN ?2<?6 OR ?3<?7 OR ?4<?8 THEN 'deferred' ELSE 'accepted' END AS result").get(g.maximumItems,g.maximumCopyBytes,g.maximumCapacityBytes,g.maximumReleaseBytes,g.maximumDepth,d.copyBytes,d.capacityBytes,d.releaseBytes,d.depth) as {result:string};assert.equal(reference.result,row.expected);assertions++;}
 for(const row of fixture.closeReceiptCases){const g=row.grant,p=row.progress;const reference=db.query("SELECT (?1>?5 OR ?2>?6 OR ?3>?7 OR ?4>?8 OR (?9 AND NOT ?10)) AS refused").get(p.copiedItems,p.copiedBytes,p.retainedCapacityBytes,p.releasedBytes,g.maximumItems,g.maximumCopyBytes,g.maximumCapacityBytes,g.maximumReleaseBytes,Number(row.complete),Number(row.terminal)) as {refused:number};assert.equal(Boolean(reference.refused),row.refused);assertions++;}
 db.close();
 const original=Object.fromEntries(fixture.actionVariants.map((name:string)=>[name,fixture.text.repeat(fixture.repeat)]));
 assert.equal(stableStringify(applyPatch(original,[],true,false).newDocument),stableStringify(original));
 assert.equal(Buffer.byteLength(original.InsertWidget),fixture.utf8Bytes);
 assertions+=2;
 const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");const domain=readFileSync(new URL("../../../🕸️wasm/🦀️.rs",import.meta.url),"utf8");
 assert.equal(/admit_retained_clone_close\(grant,1,/.test(source),false,"preflight cannot call the canonical actual-step validator as an obsolete boolean admission");assertions++;
 assert.ok(source.includes("admit_retained_clone_close(child,step,owner.terminal_is_empty()"));assertions++;
 assert.equal(/admit_retained_clone_close\(budget.retained,1,/.test(domain),false);assertions++;
 for(const scope of ["original Flow domain frame","original Flow domain VCS","original Flow domain host"]){assert.ok(domain.includes(scope));assertions++;}
 console.log("[DEBUG] original VCS admissionCases="+fixture.closeAdmissionCases.length+" receiptCases="+fixture.closeReceiptCases.length+" assertions="+assertions+" strictAjv=true SQLiteReference=true JSONPatch=true UTF8=true sourceReceivingOnly=true");
}
