import {test,expect} from "bun:test";
import Ajv from "ajv";
import {resolve} from "node:path";
import {cargoDependencyFetchArgumentsV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";

test("neutral dependency acquisition preserves closed network grants and fetches the complete platform set planning reads",()=>{
 const validate=new Ajv({strict:false}).compile(schema),manifest=resolve(process.cwd(),fixture.manifest);
 for(const row of fixture.cases){expect(validate(row.network)).toBe(true);expect(cargoDependencyFetchArgumentsV1(manifest,row.args,row.network)).toEqual(["fetch","--locked","--manifest-path",manifest,...row.suffix]);}
 for(const network of [undefined,{}, {offline:false,ambient:true}]){expect(validate(network)).toBe(false);expect(()=>cargoDependencyFetchArgumentsV1(manifest,[],network as any)).toThrow("network grant");}
 console.log("[DEBUG] Complete locked dependency fetches and original offline grants agree with neutral fixtures/Ajv");
});
