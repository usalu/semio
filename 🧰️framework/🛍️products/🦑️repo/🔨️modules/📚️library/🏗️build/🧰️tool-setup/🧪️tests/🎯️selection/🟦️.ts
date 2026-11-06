import {test,expect} from "bun:test";
import Ajv from "ajv";
import {parseToolSetupSelectionV1,selectToolRowsV1} from "../../⌨️command/🟦️.ts";
import cases from "./🔣️.json";
import schema from "./🧬️.json";

test("neutral targeted toolchain selections are closed and dispatch actual known rows",()=>{
 expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);
 for(const row of cases){const parse=()=>parseToolSetupSelectionV1(row.args);if(row.valid){const names=parse();expect(names).toEqual(row.names);if(names.length)expect(selectToolRowsV1(names,["bazel"]).map(row=>row.id)).toEqual(names)}else expect(parse).toThrow();}
});
