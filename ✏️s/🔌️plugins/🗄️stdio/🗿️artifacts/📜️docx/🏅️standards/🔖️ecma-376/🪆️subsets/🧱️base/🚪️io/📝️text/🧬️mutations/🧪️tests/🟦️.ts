import {parseInsertVmlPart as xlsxVml} from "../../../../../../../../../📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/🧬️schema/🧬️mutations/✒️insert-vml-part/🟦️.ts";
import xlsxVmlSchema from "../../../../../../../../../📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/🧬️schema/🧬️mutations/✒️insert-vml-part/🧬️schema/🔣️.json";
import {parseInsertVmlPart as pptxVml} from "../../../../../../../../../📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/🧬️schema/🧬️mutations/🖼️insert-vml-part/🟦️.ts";
import pptxVmlSchema from "../../../../../../../../../📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/🧬️schema/🧬️mutations/🖼️insert-vml-part/🧬️schema/🔣️.json";
import {parseInsertVmlPart as docxVml} from "../../../../../📏️strict/🧬️schema/🧬️mutations/✒️insert-vml-part/🟦️.ts";
import docxVmlSchema from "../../../../../📏️strict/🧬️schema/🧬️mutations/✒️insert-vml-part/🧬️schema/🔣️.json";
import {parseXmlDocumentJson} from "../../../../../../../../../📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🔣️json/🟦️.ts";
/** 🧪️ Neutral SetPart content and native framing agree with independent schema and UTF-8 oracles. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import vectors from "../../../../🧬️schema/🧬️mutations/📦set-part/🧫️fixtures/🔣️.json";
import schema from "../../../../🧬️schema/🧬️mutations/📦set-part/🧬️schema/🔣️.json";
import xmlSchema from "../../../../../../../../../📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔣️.json";
import {parseDocxSetPart} from "../../../../🧬️schema/🧬️mutations/📦set-part/🟦️.ts";
import {encodeDocxSetPartText,decodeDocxSetPartText} from "../🟦️.ts";
import {encodeDocxSetPartBinary,decodeDocxSetPartBinary} from "../../../💾️binary/🧬️mutations/🟦️.ts";
const validate=new Ajv({strict:false}).addSchema(xmlSchema).compile(schema);
for(const vector of vectors.valid)test(`DOCX typed SetPart ${vector.payload.kind} content owns its neutral payload`,()=>{expect(validate(vector)).toBe(true);const owned=parseDocxSetPart(vector.payload.kind==="xml"?{...vector,payload:{...vector.payload,document:parseXmlDocumentJson(vector.payload.document)}}:vector);expect(decodeDocxSetPartText(encodeDocxSetPartText(owned))).toEqual(owned);const text=encodeDocxSetPartText(owned);const independent=JSON.parse(Buffer.from(text.split(" payload=")[1]!,"hex").toString("utf8"));expect(parseDocxSetPart(independent.kind==="xml"?{...vector,payload:{...independent,document:parseXmlDocumentJson(independent.document)}}:{...vector,payload:independent})).toEqual(owned);const binary=encodeDocxSetPartBinary(owned);expect([...binary.subarray(0,2)]).toEqual([1,10]);expect(decodeDocxSetPartBinary(binary)).toEqual(owned);for(const malformed of[binary.subarray(0,binary.length-1),Uint8Array.from([...binary,0])])expect(()=>decodeDocxSetPartBinary(malformed)).toThrow();});
for(const payload of vectors.invalidPayloads)test(`DOCX typed SetPart rejects malformed ${JSON.stringify(payload)}`,()=>{const value={...vectors.valid[0]!,payload};expect(validate(value)).toBe(false);expect(()=>parseDocxSetPart(value)).toThrow();});
test("DOCX SetPart rejects a former encoded bytes mutation",()=>{const value={mutation:"setPart",path:"a.xml",content_type:"application/xml",bytes:[60,62]};expect(validate(value)).toBe(false);expect(()=>parseDocxSetPart(value)).toThrow();});

for(const [owner,parser,schema]of[["DOCX",docxVml,docxVmlSchema],["PPTX",pptxVml,pptxVmlSchema],["XLSX",xlsxVml,xlsxVmlSchema]]as const)test(`${owner} VML insertion accepts owned XML and refuses source markup`,()=>{const validate=new Ajv({strict:false}).addSchema(xmlSchema).compile(schema),native={path:"drawing.vml",document:vectors.valid[0]!.payload.document};expect(validate(native)).toBe(true);const owned={...native,document:parseXmlDocumentJson(native.document)};expect(parser(owned)).toEqual(owned);const encoded={path:"drawing.vml",markup:"<xml/>"};expect(validate(encoded)).toBe(false);expect(()=>parser(encoded)).toThrow();});
