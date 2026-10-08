import {expect,test} from "bun:test";
import fixture from "../../../../🧫️fixtures/🏷️metadata.json";
import {parseId3v1Tag,parseId3v2Tag} from "../../🟦️.ts";
test("MP3 neutral metadata is semantic rather than native representation",async()=>{
 const rust=await Bun.file(new URL("../../🏷️metadata/🦀️.rs",import.meta.url)).text();
 expect(rust).not.toMatch(/pub raw: Vec<u8>|pub data: Vec<u8>/);
 expect(parseId3v1Tag(fixture.id3v1)).toEqual(fixture.id3v1);
 expect<unknown>(parseId3v2Tag(fixture.id3v2)).toEqual(fixture.id3v2);
 console.log("[DEBUG] MP3 neutral typed metadata snapshot admitted without native metadata octets");
});
test("MP3 actual native metadata codecs consume semantic bodies",async()=>{const native=await Bun.file(new URL("../../../../🚪️io/🦀️.rs",import.meta.url)).text();expect(native).not.toMatch(/frame\.data|tag\.raw/);});

import Ajv from "ajv";
import schema from "../../🔣️.json";
import {parseId3Frame,type Mp3Snapshot} from "../../🟦️.ts";
import {parseMp3Mutation,mp3MutationDiff,applyMp3Diff,inverseMp3Diff} from "../../../🧬️mutations/🟦️.ts";
test("MP3 neutral typed mutations and inverses replay named metadata with independent Ajv",()=>{
 const base:Mp3Snapshot={schema:"stdio.mp3",frames:[],id3v1:parseId3v1Tag(fixture.id3v1),id3v2:parseId3v2Tag(fixture.id3v2)};
 const ajv=new Ajv({strict:false});expect(ajv.validate(schema,base)).toBe(true);
 const mutation=parseMp3Mutation({mutation:"setId3v1",id3v1:{...fixture.id3v1,title:fixture.mutationTitle}});const diff=mp3MutationDiff(mutation);const next=applyMp3Diff(base,diff);expect(next.id3v1!.title).toBe(fixture.mutationTitle);expect(applyMp3Diff(next,inverseMp3Diff(base,diff))).toEqual(base);expect(base.id3v1!.title).toBe("Café");
 for(const frame of [{id:"TIT2",content:{kind:"opaque",bytes:[0,65]}},{id:"COMM",content:{kind:"comment",language:"zz",description:"",text:"x"}},{id:"GEOB",content:{kind:"opaque",bytes:[]}},{id:"TIT2",content:{kind:"text",values:["a\0b"]}}]){expect(()=>parseId3Frame(frame)).toThrow();expect(ajv.validate(schema,{...base,id3v2:{frames:[frame]}})).toBe(false);}
 expect(()=>parseId3v1Tag({raw:Array(128).fill(0)})).toThrow();expect(ajv.validate(schema,{...base,id3v1:{raw:[84,65,71]}})).toBe(false);expect(()=>parseId3v2Tag({majorVersion:3,minorVersion:0,flags:0,frames:[]})).toThrow();console.log("[DEBUG] Neutral metadata mutation/inverse carries named title and independent Ajv rejects native-state aliases");
});
