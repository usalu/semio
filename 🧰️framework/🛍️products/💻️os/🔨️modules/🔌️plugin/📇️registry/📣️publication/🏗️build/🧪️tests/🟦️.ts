import {test,expect} from "bun:test";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json";
import fixtures from "./🧫️fixtures/🔣️.json";
import {loadCatalogBuildProvidersV1,catalogBuildContributionForManifestV1} from "../🟦️.ts";
import publication from "../../🧬️schema/🔣️.json";
import contribution from "../🧬️schema/📣️contribution/🔣️.json";
const ajv=new Ajv({strict:true}).addSchema(publication),validate=ajv.compile(schema),reference=ajv.compile(contribution);
for(const row of fixtures.cases)test(row.name,async()=>{
 expect(validate(fixtures.providers)).toBe(true);
 const ports={readText:()=>{throw Error("unexpected source read");},kind:(path:string)=>path===fixtures.providers[0]!.ownerRoot?row.ownerKind:row.sourceKind,load:async()=>({catalogBuildContributionV1:()=>row.contribution})};
 const oracle=row.ownerKind===null?"empty":row.sourceKind!=="file"||!reference(row.contribution)?"refused":"admitted";
 if("reject" in row){expect(oracle).toBe("refused");await expect(loadCatalogBuildProvidersV1(fixtures.providers,ports)).rejects.toThrow();}
 else{const supplied=await loadCatalogBuildProvidersV1(fixtures.providers,ports);expect(supplied.length).toBe(row.count);expect(catalogBuildContributionForManifestV1(supplied,"applications/design/plugins/demo/Cargo.toml")?.project).toBe(row.count?"design-composition":undefined);}
 console.log("[DEBUG] Build catalog provider",row.name,oracle);
});
test("absent application providers never inspect owner paths",async()=>{expect(await loadCatalogBuildProvidersV1(fixtures.noProviders,{readText:()=>{throw Error("unexpected source read");},kind:()=>{throw Error("unexpected owner read");},load:async()=>{throw Error("unexpected owner import");}})).toEqual([]);});


test("providers consume declarations through the original source reader",async()=>{
 const row=fixtures.cases.find(row=>row.name==="valid-owner")!,reads:string[]=[],text=JSON.stringify(row.contribution),provided=await loadCatalogBuildProvidersV1(fixtures.providers,{kind:path=>path===fixtures.providers[0]!.ownerRoot?"directory":"file",readText:path=>{reads.push(path);return text;},load:async()=>({catalogBuildContributionV1:(source:{readText(path:string):string})=>JSON.parse(source.readText(row.contribution.inputs![0]!))})});
 expect(reference(provided[0])).toBe(true);expect(provided).toEqual([JSON.parse(text)]);expect(reads).toEqual(row.contribution.inputs!);
});
