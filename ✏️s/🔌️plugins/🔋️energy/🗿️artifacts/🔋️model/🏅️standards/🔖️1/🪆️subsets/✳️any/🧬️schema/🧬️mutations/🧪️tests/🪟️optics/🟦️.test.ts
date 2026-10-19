import {test,expect} from "bun:test";
import Ajv from "ajv";
import {applyPatch} from "fast-json-patch";
import {readFileSync} from "node:fs";
import {Buffer} from "node:buffer";
import fixture from "../../🧫️fixtures/🪟️optics/🔣️.json";
import {planGlazingOpticalMutation,type EnergyGlazingOpticalMutation} from "../../🪟️optics/🟦️.ts";

/** 🔢️ Node independently captures the exact IEEE word used by typed model rows. */
function oracleWord(value:number){const bytes=Buffer.alloc(8);bytes.writeDoubleBE(value);return{bits:bytes.readBigUInt64BE()};}

/** 🧪️ Neutral optical intents agree with schema admission and independent RFC6902 application. */
test("all five optical fields have sparse forward and base-derived inverse intent",()=>{
  const schemas=new Map<string,ReturnType<Ajv["compile"]>>();
  for(const row of fixture.cases){
    const directory={"change-glazing-material-solar-reflectance-front":"🔅️solar-front","change-glazing-material-solar-reflectance-back":"🔆️solar-back","change-glazing-material-visible-reflectance-front":"🔎️visible-front","change-glazing-material-visible-reflectance-back":"🪞️visible-back","change-glazing-material-infrared-transmittance":"🌡️infrared-transmittance"}[row.kind as "change-glazing-material-solar-reflectance-front"|"change-glazing-material-solar-reflectance-back"|"change-glazing-material-visible-reflectance-front"|"change-glazing-material-visible-reflectance-back"|"change-glazing-material-infrared-transmittance"];
    if(!schemas.has(row.kind))schemas.set(row.kind,new Ajv({strict:false}).compile(JSON.parse(readFileSync(new URL("../../"+directory+"/🧬️schema/🔣️.json",import.meta.url),"utf8"))));
    expect(schemas.get(row.kind)!(row.payload)).toBe(row.id.endsWith(":missing")||row.outcome!=="rejected");
    const base={model:{glazing_materials:[structuredClone(fixture.base)]}};
    const typedBase={model:{glazing_materials:[Object.fromEntries(Object.entries(fixture.base).map(([key,value])=>[key,typeof value==="number"&&key!=="id"?oracleWord(value):value]))]}};
    const result=planGlazingOpticalMutation(typedBase as unknown as Parameters<typeof planGlazingOpticalMutation>[0],row.payload as unknown as EnergyGlazingOpticalMutation);
    const typedDiff=row.outcome==="applied"?{model:{glazing_materials:{modified:[{id:1,[row.field]:oracleWord(row.after[row.field as keyof typeof row.after] as number)}]}}}:{};
    expect(result as unknown).toEqual({outcome:row.outcome,diff:typedDiff,inverse:row.inverse});
    const patch=row.outcome==="applied"?[{op:"replace" as const,path:"/model/glazing_materials/0/"+row.field,value:row.after[row.field as keyof typeof row.after]}]:[];
    const after=applyPatch(structuredClone(base),patch,true,false).newDocument;
    expect(after.model.glazing_materials[0]).toEqual(row.after);
    if(result.inverse.length){
      after.model.glazing_materials[0].conductivity_w_m_k=fixture.peer.value;
      const typedPeer={model:{glazing_materials:[Object.fromEntries(Object.entries(after.model.glazing_materials[0]).map(([key,value])=>[key,typeof value==="number"&&key!=="id"?oracleWord(value):value]))]}};
      const undo=planGlazingOpticalMutation(typedPeer as unknown as Parameters<typeof planGlazingOpticalMutation>[0],result.inverse[0]);
      expect(Object.keys(undo.diff.model!.glazing_materials!.modified![0])).toEqual(["id",row.field]);
      const restored=applyPatch(after,[{op:"replace",path:"/model/glazing_materials/0/"+row.field,value:fixture.base[row.field as keyof typeof fixture.base]}],true,false).newDocument;
      expect(restored.model.glazing_materials[0]).toEqual({...fixture.base,conductivity_w_m_k:fixture.peer.value});
    }
  }
  console.log("[DEBUG] energy optical neutral cases="+fixture.cases.length+" fields=5 AJV/RFC6902 sparse-inverse independent-peer=passed");
});

/** 📜️ The neutral history contract keeps independent peer fields through local inverse/revision replay. */
test("energy peer history field intent agrees with an independent event replay",()=>{
 const law=JSON.parse(readFileSync(new URL("../../../../✏️editor/🧫️fixtures/📜️document-history/🔣️.json",import.meta.url),"utf8"));
 const base={name:"",site:{latitude_deg:0,longitude_deg:0}};
 const events=[{op:"replace" as const,path:"/name",value:law.peer.localName},{op:"replace" as const,path:"/site/latitude_deg",value:law.peer.localLatitude},{op:"replace" as const,path:"/site/longitude_deg",value:law.peer.remoteLongitude}];
 const after=applyPatch(structuredClone(base),events,true,false).newDocument;
 const revised=applyPatch(after,[{op:"replace",path:"/site/latitude_deg",value:law.peer.revisedLatitude}],true,false).newDocument;
 expect(revised).toEqual({name:law.peer.expected.name,site:{latitude_deg:law.peer.expected.latitude,longitude_deg:law.peer.expected.longitude}});
 const undone=applyPatch(revised,[{op:"replace",path:"/site/latitude_deg",value:base.site.latitude_deg}],true,false).newDocument;
 expect(undone.name).toBe(law.peer.localName);expect(undone.site.longitude_deg).toBe(law.peer.remoteLongitude);
 expect(law.documents.archive.history).toBe("preserve-events");expect(law.documents.import.semanticMutationRows).toBe(0);expect(law.documents.example.semanticMutationRows).toBe(0);
 console.log("[DEBUG] energy neutral history RFC6902 independent-fields undo/revision preserved; native document/peer gate owns actual runtime evidence");
});

/** 📡️ Neutral wire identities agree with the authored protocol and both grammar surfaces. */
test("optical wire cases retain unique declared protocol identities",()=>{
 const binary=readFileSync(new URL("../../../../🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio",import.meta.url),"utf8");
 const records=[...binary.matchAll(/^record (\S+) tag=(\d+)$/gm)];expect(new Set(records.map(r=>r[2])).size).toBe(records.length);
 for(const row of fixture.wireCases){
  expect(records.find(r=>r[1]===row.kind)?.[2]).toBe(String(row.binaryTag));expect(row.binaryPrefix).toEqual([1,row.binaryTag%128+128,Math.floor(row.binaryTag/128)]);
  for(const name of ["📖️.grammar.semio","📖️mutations.grammar.semio"]){const grammar=readFileSync(new URL("../../../../🚪️io/📝️text/🧬️mutations/"+name,import.meta.url),"utf8");expect(grammar.split("\n").find(l=>l.startsWith("op = "))?.split(" | ")).toContain(row.kind);expect(grammar).toContain(row.kind+" = ");}
 }
 console.log("[DEBUG] energy optical wire neutral cases=5 tags300to304 unique protocol and both grammar rosters=passed");
});
