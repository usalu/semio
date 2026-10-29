import{test,expect}from"bun:test";
import Ajv from"ajv";
import{find,mapValues,identity}from"lodash";
import{parseTree,getNodeValue}from"jsonc-parser";
import fixture from"./🔣️.json";
import schema from"./📋️schema.json";
import labelSchema from"../../🧬️schema/🏷️localized-label/🔣️.json";
import axes from"@semio-tech/ui/axes.json";
import{Locale,Terminology,LocalizedLabel,type LabelValue}from"../../🟦️.ts";
const value=(input:unknown):LabelValue=>typeof input==="string"?{kind:"string",value:input}:{kind:"object",entries:Object.entries(input as Record<string,unknown>).map(([key,child])=>[key,value(child)])};
test("genuine erased locale receiver corpus matches independent axes and JSON matrices",()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const accepts=new Ajv({strict:true}).compile(labelSchema);
 for(const row of fixture.cases){let actual:unknown,oracle:unknown;if(row.kind==="locale"){actual=Locale.parse(row.input).id;oracle=new Intl.Locale(row.input as string).language;}else if(row.kind==="terminology"){actual=Terminology.parse(row.input).id;oracle=find(axes.terminologies,item=>item.id===row.input)?.id;}else{expect(accepts(row.input),row.id).toBe(true);const label=LocalizedLabel.fromValue(value(row.input));actual=Object.fromEntries(axes.terminologies.map(terminology=>[terminology.id,Object.fromEntries(axes.locales.map(locale=>[locale.id,label.resolve(Terminology.parse(terminology.id),Locale.parse(locale.id))]))]));const tree=parseTree(JSON.stringify(row.input));expect(tree).toBeDefined();oracle=mapValues(getNodeValue(tree!),cells=>mapValues(cells,identity));}expect(actual,row.id).toEqual(row.expected);expect(actual,row.id).toEqual(oracle);}
});
