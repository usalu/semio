import assert from "node:assert/strict";
import {readFileSync,existsSync} from "node:fs";
import {applyPatch} from "fast-json-patch";
import stableStringify from "fast-json-stable-stringify";
import Ajv from "ajv";
import {Database} from "bun:sqlite";

/** 🌿️ Original VCS ownership laws use plain examples and independent JSONPatch/UTF8 references. */
export function testOriginalVcsClosure():void{
 assert.equal(existsSync(new URL("../🧬️schema/🔣️.json",import.meta.url)),false,"Testing examples cannot own schema authority");
 const fixture=JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
 const schema=JSON.parse(readFileSync(new URL("../../../../../../../🔨️modules/🌱️value/🗂️ordered/♻️retirement/🧬️schema/🔣️.json",import.meta.url),"utf8"));
 const ajv=new Ajv({strict:true,allErrors:true}).addSchema(schema),validateGrant=ajv.compile({$ref:schema.$id+"#/$defs/Grant"}),validateProgress=ajv.compile({$ref:schema.$id+"#/$defs/Progress"});let assertions=0;
 const contract=JSON.parse(readFileSync(new URL("../../../../../../../🔨️modules/🌱️value/♻️retirement/🧬️contract/🧬️schema/🔣️.json",import.meta.url),"utf8"));ajv.addSchema(contract);const validateDemand=ajv.compile({$ref:contract.$id+"#/$defs/Demand"});
 for(const row of fixture.closeAdmissionCases){assert.equal(validateDemand(row.demands),true);assertions++;}
 for(const field of ["copyBytes","capacityBytes","releaseBytes","depth"]){const invalid={...fixture.closeAdmissionCases[0].demands};delete invalid[field];assert.equal(validateDemand(invalid),false);assertions++;}
 for(const row of [...fixture.closeAdmissionCases,...fixture.closeReceiptCases]){assert.equal(validateGrant(row.grant),true);assertions++;}
 for(const field of ["maximumItems","maximumCopyBytes","maximumCapacityBytes","maximumReleaseBytes","maximumDepth"]){const invalid={...fixture.closeAdmissionCases[0].grant};delete invalid[field];assert.equal(validateGrant(invalid),false);assertions++;}
 for(const row of fixture.closeReceiptCases){assert.equal(validateProgress({...row.progress,complete:row.complete}),true);assertions++;}
 for(const field of ["copiedItems","copiedBytes","retainedCapacityBytes","releasedBytes"]){const invalid={...fixture.closeReceiptCases[0].progress,complete:fixture.closeReceiptCases[0].complete};delete invalid[field];assert.equal(validateProgress(invalid),false);assertions++;}
 assert.equal(validateProgress({...fixture.closeReceiptCases[0].progress,complete:fixture.closeReceiptCases[0].complete,depth:1}),false);assertions++;
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
 console.log("[DEBUG] original VCS admissionCases="+fixture.closeAdmissionCases.length+" receiptCases="+fixture.closeReceiptCases.length+" assertions="+assertions+" canonicalGrantProgressAjv=true wholeCorpusSchema=false SQLiteReference=true JSONPatch=true UTF8=true sourceReceivingOnly=true");
}
