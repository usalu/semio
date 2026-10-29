/** 🧪️ Current original host custody is checked against neutral omissions and independent schema/patch libraries. */
import {test,expect} from "bun:test";
import Ajv from "ajv";
import jsonpatch from "fast-json-patch";
import schema from "../📐️.json";
import fixture from "../🔣️.json";
import {toolJobOriginalReservedHostEvidence,toolJobOriginalReservedHostExact} from "../🟦️.ts";
import {toolJobFrameworkReservedRoutesExact} from "../../../🏭️factory/🟦️.ts";
import {toolJobPuzzleReservedRoutesExact,toolJobPuzzle2dReservedRoutesExact} from "../../🟦️.ts";
import {toolJobPluginInstanceSource} from "../../../📤️publication/🟦️.ts";
import {toolJobProductionSource} from "../../../📋️inventory/🟦️.ts";
import {toolJobPuzzleReservedRoutesSelfTests} from "../../../../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️tool-job-puzzle-reserved-routes/🟦️.ts";
const source=toolJobProductionSource(toolJobPluginInstanceSource(process.cwd()));
const evidence=toolJobOriginalReservedHostEvidence(source);
const validate=new Ajv({strict:true,allErrors:true}).compile(schema);
const replace=(text:string,needle:string,replacement:string):string=>{
 const pattern=new RegExp([...needle.replace(/\s+/g,"")].map(character=>character.replace(/[.*+?^${}()|[\]\\]/g,"\\$&")).join("\\s*"),"g");
 expect(pattern.test(text)).toBe(true);
 pattern.lastIndex=0;
 return text.replace(pattern,replacement);
};
for(const row of fixture.cases){
 test(row.name,()=>{
  expect(fixture.schema).toBe("semio.original-reserved-host.source.v1");
  const candidate=row.gate===null?source:replace(source,row.needle!,row.replacement!);
  const actual=toolJobOriginalReservedHostEvidence(candidate);
  const reference=row.gate===null?evidence:jsonpatch.applyPatch(evidence,[{op:"replace",path:"/"+row.gate,value:false}],true,false).newDocument;
  expect(validate(reference)).toBe(row.accepted);
  expect(validate(actual)).toBe(row.accepted);
  expect(toolJobOriginalReservedHostExact(candidate)).toBe(row.accepted);
  expect(toolJobFrameworkReservedRoutesExact(candidate)).toBe(row.accepted);
  if(row.gate!==null)expect(actual[row.gate]).toBe(false);
 });
}
test("actual framework routes keep exact registry and original configuration/import workers",()=>{
 expect(toolJobFrameworkReservedRoutesExact(source)).toBe(true);
 expect(toolJobFrameworkReservedRoutesExact(replace(source,"pending_configuration_source.is_some_and(|source|!source.matches(cx))","false"))).toBe(false);
 expect(toolJobFrameworkReservedRoutesExact(replace(source,"(source.0,source.1)!=(cx.operation(),cx.generation())","false"))).toBe(false);
});
test("puzzle positive host and every guarded omission use the current original route",()=>{
 const checked=toolJobPuzzleReservedRoutesSelfTests({toolJobPuzzleReservedRoutesExact,toolJobPuzzle2dReservedRoutesExact});
 expect(checked).toBeGreaterThan(30);
 console.log(JSON.stringify({puzzleSourceCases:checked,neutralHostCases:fixture.cases.length}));
});
