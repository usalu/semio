/** 🧪️ Artifact-owned physical transport corpus and independent oracles. */
import {test,expect,afterAll} from "bun:test";
import Ajv from "ajv";
import {Buffer} from "node:buffer";
import fixture from "./🧫️fixtures/🔣️.json";
import {decodeRemodelingSnapshot,remodelingSnapshotToJsonText} from "./../../🟦️.ts";
import {defaultRemodelingSnapshot,parseRemodelingSnapshot} from "./../../../../../../🧬️schema/📸️snapshot/🟦️.ts";
const oracle=new Ajv({strict:false});
const word=(number:number):bigint=>{const bytes=Buffer.alloc(8);bytes.writeDoubleBE(number);return bytes.readBigUInt64BE()};
test("Remodeling JSON projects canonical defaults and decodes each exact scalar role",()=>{
 const owned=defaultRemodelingSnapshot(),text=remodelingSnapshotToJsonText(owned);
 expect(parseRemodelingSnapshot(decodeRemodelingSnapshot(JSON.parse(text)))).toEqual(owned);
 const changed=decodeRemodelingSnapshot({schema:"remodeling.scene",id:"literal",streams:[{id:"s",syncOffsetMs:fixture.word.number}]});
 expect(changed.streams[0]!.syncOffsetMs.bits).toBe(word(fixture.word.number));
 expect(()=>parseRemodelingSnapshot(JSON.parse(remodelingSnapshotToJsonText(changed)))).toThrow();
 expect(oracle.compile({const:{bits:fixture.word.bits}})(JSON.parse(remodelingSnapshotToJsonText(changed)).streams[0].syncOffsetMs)).toBe(true);
});
afterAll(()=>console.log("[DEBUG] Owned transport corpus completed: @semio-tech/remodel-remodeling-rs"));
