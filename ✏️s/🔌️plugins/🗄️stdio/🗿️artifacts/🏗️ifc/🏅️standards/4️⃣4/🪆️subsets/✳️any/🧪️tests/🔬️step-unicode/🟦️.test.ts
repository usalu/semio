import {test as register} from "node:test";
import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import iconv from "iconv-lite";
import {decodeIfc4, encodeIfc4} from "../../🚪️io/🟦️.ts";

export const records: {name: string; passed: boolean; error?: string}[] = [];
export const checks: Promise<void>[] = [];
const frame = (literal:string) => "ISO-10303-21;HEADER;FILE_DESCRIPTION((''),'2;1');FILE_NAME('','',(''),(''),'','','');FILE_SCHEMA(('IFC4'));ENDSEC;DATA;#1=IFCTHING('" + literal + "');ENDSEC;END-ISO-10303-21;";
const vectors: {valid: {literal:string;text:string}[]; invalid:string[]} = JSON.parse(readFileSync(new URL("../../../../../../../../📇️registry/🧬️contract/📐️part21/🧪️tests/🧫️fixtures/🔡️unicode.json", import.meta.url), "utf8"));

/** 🔡️ Compares language-neutral STEP escapes with an independent Unicode codec. */
export function checkUnicode(): void {
  for (const item of vectors.valid) {
    const external = iconv.decode(Buffer.from(item.literal.slice(4, -4), "hex"), item.literal[2] === "2" ? "utf16-be" : "utf32-be");
    assert.equal(external, item.text);
    const snapshot = decodeIfc4(frame(item.literal));
    const value = snapshot.entities[0]!.args[0]!;
    assert.equal(value.kind, "string");
    if (value.kind === "string") assert.equal(value.value, external);
    assert.deepEqual(decodeIfc4(encodeIfc4(snapshot)), snapshot);
  }
  for (const literal of vectors.invalid) assert.throws(() => decodeIfc4(frame(literal)), literal);
}
checks.push(register("STEP Unicode directives match iconv-lite and reject malformed scalar sequences", () => {
  try {checkUnicode(); records.push({name: "unicode", passed: true});} catch(error) {records.push({name: "unicode", passed: false, error:String(error)}); throw error;}
}));
