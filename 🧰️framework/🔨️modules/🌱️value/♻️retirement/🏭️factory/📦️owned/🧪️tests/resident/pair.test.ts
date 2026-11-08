import {expect,test} from "bun:test";
import law from "./pair.json";
import schema from "./pair.schema.json";
import {isDeepStrictEqual} from "node:util";
test("original pair law preserves independent value and issuer outputs",()=>{for(const[k,v]of Object.entries(law))expect(isDeepStrictEqual(schema.properties[k as keyof typeof schema.properties].const,v)).toBe(true);const original={value:law.body,issuer:{id:97}};const source={...original};const output={value:source.value,issuer:source.issuer};expect(output.issuer).toBe(original.issuer);expect(JSON.parse(JSON.stringify(output.value))).toBe(law.body);expect(law.maximumCopyBytes).toBe(4096);});
