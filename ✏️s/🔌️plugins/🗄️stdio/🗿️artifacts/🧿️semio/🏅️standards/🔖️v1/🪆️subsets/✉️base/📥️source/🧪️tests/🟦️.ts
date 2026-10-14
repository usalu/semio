import {readFileSync} from "node:fs";
import {dirname,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import {type ToolJobMemberStoreProducerSources, type toolJobMemberStoreProducerExact} from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🏠️owners/🏭️member/📥️source/🟦️.ts";
import domainSchema from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🏠️owners/🏭️member/📥️source/🧬️schema/🔣️.json" with {type:"json"};
import baseSchema from "../../🧬️schema/🔣️.json" with {type:"json"};
import fixtures from "./🧫️fixtures/🔣️.json" with {type:"json"};

/** 🧪️ Checks this instance’s actual producer/table binding under the original Generic source contract. */
export function semioMemberStoreProducerSourceSelfTests(check:typeof toolJobMemberStoreProducerExact,sources:ToolJobMemberStoreProducerSources):number{
 const root=resolve(dirname(fileURLToPath(import.meta.url)),"../../../../../../../../../../..");
 const declaration=JSON.parse(readFileSync(resolve(root,"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/📥️source/🔣️.json"),"utf8"));
 const ajv=new Ajv({strict:true,allErrors:true});ajv.addKeyword({keyword:"x-semio-formats",valid:true});ajv.addSchema(domainSchema);
 const validate=ajv.compile(baseSchema.$defs.SemioMemberStoreProducerSourceDeclaration),validateSources=ajv.getSchema(domainSchema.$id+"#/$defs/ToolJobMemberStoreProducerSources")!;
 if(!validate(declaration)||sources.extensions.length!==1||!check(sources))throw Error("[semio member producer source] actual explicit extension refused");
 for(const row of fixtures.cases){
  const candidate=structuredClone(sources),extension=candidate.extensions[0];if(!extension)throw Error("[semio member producer source] actual extension missing");
  const body=extension[row.change.role as "producer"|"members"];
  let at=-1;for(let count=0;count<=(("occurrence" in row.change?row.change.occurrence:0)??0);count++)at=body.source.indexOf(row.change.from,at+1);
  if(at<0)throw Error("[semio member producer source] current mutation absent: "+row.id);
  (body as {source:string}).source=body.source.slice(0,at)+row.change.to+body.source.slice(at+row.change.from.length);
  if(check(candidate)!==row.accepted)throw Error("[semio member producer source] binding mutation mismatch: "+row.id);
 }
 for(const row of fixtures.sourceWrappers){const candidate=structuredClone(sources),extension=candidate.extensions[0];if(!extension)throw Error("actual extension missing");const body=extension[row.role as "producer"|"members"];(body as {source:string}).source=row.prefix+body.source+row.suffix;if(check(candidate)!==row.accepted)throw Error("[semio member producer source] production-root table refusal mismatch: "+row.id);}
 for(const row of fixtures.inputCases){const candidate=structuredClone(sources),extension=candidate.extensions[0];if(!extension)throw Error("actual extension missing");const owner=row.path.slice(0,-1).reduce((value,key)=>(value as Record<string,unknown>)[key],extension as unknown),key=row.path.at(-1);if(key===undefined)throw Error("input case path missing");(owner as Record<string,unknown>)[key]=row.value;if((validateSources(candidate)&&validate(extension.declaration))!==row.accepted||check(candidate)!==row.accepted)throw Error("[semio member producer source] independent/production closed input refusal mismatch: "+row.id);}
 return fixtures.cases.length+fixtures.sourceWrappers.length+fixtures.inputCases.length+2;
}
