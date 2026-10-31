/** 🛑️ The existing preparation entry must not advance under an actual zero-authority caller ledger. */
import {test,expect} from "bun:test";
import Ajv from "ajv";
import {binary64} from "@semio-tech/framework/value/ieee754";
import {Database} from "bun:sqlite";
import {DocumentSceneJob} from "../../🟦️.ts";
import {parseDrawingArtifact} from "../../../../🟦️.ts";
import {RetainedWorkLedger} from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts";
import rows from "../../../../../🧪️tests/🧫️fixtures/🎬️scene/📋️prepare/🔣️.json";
import law from "./📐️.json";
import schema from "./🔣️.json";
function lift(value:unknown):unknown{return typeof value==="number"?binary64(value):Array.isArray(value)?value.map(lift):value&&typeof value==="object"?Object.fromEntries(Object.entries(value).map(([key,item])=>[key,lift(item)])):value;}
test("actual Drawing preparation zero authority preserves public semantic progress before any new API",()=>{expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);const row=rows.find(value=>value.name===law.zeroAuthority.source);if(row===undefined)throw Error("Original preparation source missing");const source=parseDrawingArtifact({...row.document,layers:lift(row.document.layers)}),layers=source.layers,assets=source.assets,before=structuredClone(row.document),job=new DocumentSceneJob(source,row.limits),work=new RetainedWorkLedger(law.zeroAuthority.grant),db=new Database(":memory:"),errors:unknown[]=[];try{const actual=Reflect.apply(job.advance,job,[law.zeroAuthority.items,work]);const expected=db.query("SELECT ? AS work").get(law.zeroAuthority.expectedWork);expect({work:actual.work}).toEqual(expected);expect(work.receipt).toEqual(law.zeroAuthority.expectedReceipt);}catch(error){errors.push(error);}finally{try{job.cancel();expect(source.layers).toBe(layers);expect(source.assets).toBe(assets);expect(row.document).toEqual(before);}catch(error){errors.push(error);}db.close();}if(errors.length)throw errors.length===1?errors[0]:new AggregateError(errors,"Preparation behavioral assertion and genuine cancellation failures");});
