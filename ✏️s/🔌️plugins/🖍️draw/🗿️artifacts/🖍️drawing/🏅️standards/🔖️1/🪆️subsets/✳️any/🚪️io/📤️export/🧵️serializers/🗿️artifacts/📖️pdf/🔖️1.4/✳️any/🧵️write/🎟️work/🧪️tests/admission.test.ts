/** 🧭️ The actual path formatter must resume the same bounded candidate while real storage admission blocks. */
import {test,expect} from "bun:test";
import {PdfWriteJob} from "../../🟦️.ts";
import {input,finish,retire} from "./owned.test.ts";
import law from "../📐️.json";
import {bank} from "../../../../../../../../../../🧬️schema/🎬️scene/🗺️plan-model/🎟️owned/🧪️tests/owned.test.ts";
test("PDF path formatter preserves the exact candidate across genuine allocator admission",()=>{const source=input(),node=source.nodes.at(0);if(node===undefined||node.content.kind!=="path")throw Error("Original path admission input absent");const segments=node.content.segments;expect<unknown[]>(Array.from({length:segments.length},(_,at)=>segments.at(at))).toEqual(law.pathAdmissionSegments);const w=bank(),job=new PdfWriteJob(source,{viewBox:[0,0,2,1],maxOutputBytes:65536,maxWork:1e9});let primary:unknown,failed=false;try{finish(job,w);}catch(error){primary=error;failed=true;}const errors:unknown[]=[];if(failed)errors.push(primary);try{if(failed)job.cancel();retire(job,w);}catch(error){errors.push(error);}if(errors.length)throw errors.length===1?errors[0]:new AggregateError(errors,"Actual path candidate and original-bank cleanup failures");});
