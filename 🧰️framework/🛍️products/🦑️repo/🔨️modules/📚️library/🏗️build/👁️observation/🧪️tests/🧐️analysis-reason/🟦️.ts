/** 🧐️ Law: the first failing row of a stale record that is an analysis fact says why it fails (`why`), so a read can tell an edited input from an unregistered provider or a different parser revision; the verdict and the reason are one function, a fact that holds names nothing.
 * @see ../../🟨️.mjs `sourceAnalysisFailureV1` `firstFailingRowV1` */
import{expect,test}from"bun:test";
import portableRowsSchema from "../../../🪪️graph-state/📦️project-records/🧳️portable/🧬️schema/🔣️.json";
import{createHash}from"node:crypto";
import{mkdtempSync,realpathSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";

const observation=await import("../../🟨️.mjs") as any;
const revision=createHash("sha256").update("analysis-reason-parser").digest("hex");
const provider=observation.registerSourceAnalysisV1({id:"build:analysis-reason:v1:"+createHash("sha256").update("law").digest("hex"),revision,analyze:(_path:string,bytes:Buffer)=>({length:bytes.length})});
const root=mkdtempSync(join(realpathSync(tmpdir()),"analysis-reason-")),file=join(root,"input.txt");
writeFileSync(file,"one");
await new Promise(done=>setTimeout(done,2200));
const cachePath=join(root,"np","analysed.json");
await observation.observeInference({cachePath,key:["analysed"]},async()=>({value:observation.observedSourceAnalysisV1(provider,file)}));
const rows=()=>observation.projectInferenceObservationsV1(cachePath).filter((row:{kind:string})=>row.kind==="analysis");

test("an analysis fact that holds names no reason, an edited input is a changed replay result",async()=>{
 expect(rows().length).toBe(1);
 expect((await observation.verifyStoredProjectRecordV1({cachePath,key:["analysed"]})).verdict).toBe("holds");
 expect(await observation.firstFailingRowV1(rows(),undefined)).toBeUndefined();
 writeFileSync(file,"two, longer");
 const said=await observation.verifyStoredProjectRecordV1({cachePath,key:["analysed"]});
 expect(said.verdict).toBe("stale");expect(said.staleRow.kind).toBe("analysis");expect(said.staleRow.why).toMatch(/^replay-result [0-9a-f]{10}>[0-9a-f]{10}; judge \d+ of pid \d+$/u);
 writeFileSync(file,"one");
});

test("an unregistered parser role and a different parser revision are told apart; another id of a registered role and revision is no reason",async()=>{
 await new Promise(done=>setTimeout(done,2200));
 const [row]=rows();
 const resolved=await observation.firstFailingRowV1([{...row,fact:{...row.fact,provider:"build:analysis-reason:v1:"+"0".repeat(64)}}],undefined);
 expect(resolved,"another id of the registered role and revision is the same parser").toBeUndefined();
 const portable="build:analysis-reason:v1:"+portableRowsSchema.definitions.Policy.const.ownerToken;
 expect(await observation.firstFailingRowV1([{...row,fact:{...row.fact,provider:portable}}],undefined)).toBeUndefined();
 const unknown=await observation.firstFailingRowV1([{...row,fact:{...row.fact,provider:"build:analysis-reason-other:v1:"+"0".repeat(64)}}],undefined);
 expect(unknown.why.startsWith("provider-unregistered build:analysis-reason-other:v1:")).toBe(true);
 expect(unknown.why).toContain("; registered ");expect(unknown.why).toContain(", same role none");
 const other=await observation.firstFailingRowV1([{...row,fact:{...row.fact,revision:"f".repeat(64)}}],undefined);
 expect(other.why).toBe("revision "+"f".repeat(12)+" "+revision.slice(0,12));
 const unreadable=await observation.firstFailingRowV1([{...row,path:join(root,"gone.txt"),fact:{...row.fact,source:{...row.fact.source,size:1}}}],undefined);
 expect(unreadable.why.startsWith("unreadable")).toBe(true);
});

test("a row of another kind has no reason and the source keeps verdict and reason in one function",async()=>{
 const source=(await Bun.file(new URL("../../🟨️.mjs",import.meta.url)).text());
 expect(source).toContain("function sourceAnalysisFactHoldsV1(path,fact){return sourceAnalysisFailureV1(path,fact)===undefined;}");
});
