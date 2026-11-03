import {test,expect} from "bun:test";
import Ajv from "ajv";
import {resolve} from "node:path";
import {cargoDependencyFetchArgumentsV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import flagViews from "./🧫️fixtures/🎯️flag-views/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
import "./📥️consumption/🟦️.ts";

test("neutral dependency acquisition preserves closed network grants and fetches the complete platform set planning reads",()=>{
 const validate=new Ajv({strict:false}).compile(schema),manifest=resolve(process.cwd(),fixture.manifest);
 for(const row of fixture.cases){expect(validate(row.network)).toBe(true);expect(cargoDependencyFetchArgumentsV1(manifest,row.args,row.network)).toEqual(["fetch","--locked","--manifest-path",manifest,...row.suffix]);}
 for(const network of [undefined,{}, {offline:false,ambient:true}]){expect(validate(network)).toBe(false);expect(()=>cargoDependencyFetchArgumentsV1(manifest,[],network as any)).toThrow("network grant");}
});

test("flag views are compared per target: the guard agrees with node:util parseArgs on every fixture case",async()=>{
  const {requireCompatibleCargoFlagViewsV1}=await import("../🟦️.ts"),{parseArgs}=await import("node:util"),rows=flagViews.cases;
  const oracle=(selections:readonly{args:string[];flags:string}[])=>selections.some((row,at)=>selections.slice(0,at).some(peer=>parseArgs({args:row.args.slice(1),options:{target:{type:"string"},bins:{type:"boolean"}},allowPositionals:true,strict:false}).values.target===parseArgs({args:peer.args.slice(1),options:{target:{type:"string"},bins:{type:"boolean"}},allowPositionals:true,strict:false}).values.target&&row.flags!==peer.flags));
  for(const row of rows){
    const run=()=>requireCompatibleCargoFlagViewsV1(row.selections.map(item=>item.args),row.selections.map(item=>({environment:{CARGO_ENCODED_RUSTFLAGS:item.flags,CARGO_ENCODED_RUSTDOCFLAGS:""}})));
    if(row.refused)expect(run,row.name).toThrow("Distinct Cargo target flag views");else expect(run,row.name).not.toThrow();
    expect(oracle(row.selections),row.name).toBe(row.refused);
  }
});
