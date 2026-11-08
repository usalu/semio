import {expect,test} from "bun:test";
import {isDeepStrictEqual} from "node:util";
import law from "./resident.json";
import schema from "./resident.schema.json";
test("native original slot schema matches the independent JSON value oracle",()=>{for(const[k,v]of Object.entries(law))expect(isDeepStrictEqual(schema.properties[k as keyof typeof schema.properties].const,v)).toBe(true);expect(JSON.parse(JSON.stringify(law.value))).toBe(law.value);expect(law.cancellationCuts.at(-1)).toBe(law.maximumCopyBytes);});
