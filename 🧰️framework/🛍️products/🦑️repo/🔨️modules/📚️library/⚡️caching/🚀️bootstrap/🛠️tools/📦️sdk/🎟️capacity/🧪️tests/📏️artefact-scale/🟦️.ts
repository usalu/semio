import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import headroomSource from"../../../../🧹️retention/📏️headroom/🟦️.ts"with{type:"text"};
import toolsSource from"../../../../🟦️.ts"with{type:"text"};
import formulaSource from"../../🧮️formula/🟦️.ts"with{type:"text"};
import{isRepoErrorV1,renderRepoErrorV1}from"../../../../../../../../../../../🔨️modules/🏃️process/🏗️build/🗣️language/🚨️error/🟦️.ts";
import snapshotPolicy from"../../../../🔏️authorizers/📦️snapshot/🔣️policy.json"with{type:"json"};
import derivativeSchema from"../../../📥️source/🔗️derivative/🧬️schema/🔣️.json"with{type:"json"};
import policy from"../../📋️policy.json"with{type:"json"};
import schema from"../../🧬️schema.json"with{type:"json"};
const formula=()=>import("../../🧮️formula/🟦️.ts");
const real={version:1 as const,entries:11701,copyAllocatedBytes:23171072,inputBytes:262144};
const oracle=(row:{entries:number;copyAllocatedBytes:number;inputBytes:number})=>{const db=new Database(":memory:");try{return(db.query("select ?1+(?2+?3)*?4+?5+2*?6+?7+?8+?9+?10 as bytes").get(row.copyAllocatedBytes,row.entries,policy.publicationEntries,policy.metadataBytesPerEntry,snapshotPolicy.maximumSnapshotBytes,snapshotPolicy.maximumOutputBytes,row.inputBytes,derivativeSchema.definitions.Policy.const.maximumIndexBytes,policy.maximumDerivativePatchBytes,policy.maximumDerivativeReceiptBytes)as{bytes:number}).bytes;}finally{db.close();}};
test("the real descriptor values of the Nx SDK derivative are funded, not refused",async()=>{
 const{nxSdkDerivativeHeadroomBytesV1}=await formula();expect(nxSdkDerivativeHeadroomBytesV1(real)).toBe(oracle(real));
});
test("the largest declared input set is funded under the declared artefact sizes",async()=>{
 const{nxSdkDerivativeHeadroomBytesV1}=await formula(),row={...real,inputBytes:policy.maximumInputBytes};expect(nxSdkDerivativeHeadroomBytesV1(row)).toBe(oracle(row));
});
test("the clone publication charge is the declared artefact sizes, not a fixed number",async()=>{
 const{nxSdkCloneHeadroomBytesV1}=await formula(),row={version:1 as const,entries:11701,copyAllocatedBytes:23171072},expected=row.copyAllocatedBytes+(row.entries+policy.publicationEntries)*policy.metadataBytesPerEntry+2*snapshotPolicy.maximumSnapshotBytes+2*snapshotPolicy.maximumOutputBytes+policy.maximumInputBytes;expect(nxSdkCloneHeadroomBytesV1(row)).toBe(expected);
});
test("no fixed publication ceiling is declared that an artefact policy can outgrow",()=>{
 expect("maximumPublicationBytes"in policy).toBe(false);expect(JSON.stringify(schema)).not.toContain("maximumPublicationBytes");
});
test("a headroom beyond the declared ceiling is a typed bilingual refusal naming sdk, derivative, need, ceiling and free bytes",async()=>{
 const{nxSdkDerivativeHeadroomBytesV1}=await formula();let caught:unknown;try{nxSdkDerivativeHeadroomBytesV1({version:1,entries:200000,copyAllocatedBytes:0,inputBytes:0},123456789);}catch(error){caught=error;}
 expect(isRepoErrorV1(caught)).toBe(true);const need=String(oracle({entries:200000,copyAllocatedBytes:0,inputBytes:0}));
 for(const language of["en","de"]){const text=renderRepoErrorV1(caught as never,language);expect(text).not.toMatch(/Internal error|Interner Fehler/u);for(const part of["nx","derivative",need,String(policy.maximumHeadroomBytes),"123456789"])expect(text).toContain(part);}
});
test("the ceiling of the tools source never throws an untyped error for artefact sizes",()=>{
 expect(formulaSource).not.toContain("byte ceiling exceeded");expect(formulaSource).not.toContain("headroom capacity exceeded");
});
test("the formula spells no artefact size: every ceiling is read from the declared policies",()=>{
 expect(formulaSource.match(/\b\d{6,}\b/gu)).toBeNull();
});
test("every artefact-size refusal of the SDK publication chain is the typed refusal, never an internal error",()=>{
 expect(headroomSource).not.toContain("Released managed tooling capacity required");expect(headroomSource).toContain("sdk.derivative-capacity");expect(toolsSource).not.toContain("SDK derivative compiler publication byte ceiling exceeded");expect(toolsSource).toContain("sdk.derivative-capacity");expect(toolsSource).toContain("nxSdkDerivativeHeadroomBytesV1(material,Number(facts.bavail*facts.bsize))");
});
