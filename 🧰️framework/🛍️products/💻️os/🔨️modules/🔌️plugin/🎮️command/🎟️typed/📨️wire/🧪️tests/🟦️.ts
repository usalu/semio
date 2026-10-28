import {test,expect} from "bun:test";
import {createHash} from "node:crypto";
import law from "./🔣️.json";

test("command text and Pack bytes agree with independent Buffer and SHA256",()=>{
 const text=Buffer.from(Array.from({length:law.textBytes},(_,i)=>97+i%26));
 const cases=[[Buffer.concat([Buffer.from(law.header),text]),law.textSha256],[Buffer.concat([Buffer.from(law.packPrefix),text,Buffer.from(law.packSuffix)]),law.packSha256]] as const;
 for(const [wire,sha]of cases){const pages=[];for(let offset=0;offset<wire.length;offset+=4096)pages.push(wire.subarray(offset,offset+4096));expect(pages.length).toBe(3);expect(createHash("sha256").update(Buffer.concat(pages)).digest("hex")).toBe(sha);}
 expect((law.packPrefix[6]&127)|(law.packPrefix[7]<<7)).toBe(law.textBytes);
});
