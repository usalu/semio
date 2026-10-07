import {gisMapInferenceRequestToJson} from "../../../../../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/💡️inferences/🌐️hub/🟦️.ts";
import {sealGisMapInferenceJobRequestV1} from "../../../../../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧬️schema/🟦️.ts";
import {testLayoutDocumentContractOracle} from "../../../../../../../../../✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document/🟦️.ts";
import {testProgramDocumentContract} from "../../../../../../../../../✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️document-contract/🟦️.ts";
import {testResumableQueryOracle} from "../../../../../../../../../✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor/🧪️tests/🪜️resumable-query/🟦️.ts";
/** 🧪️ Physical TypeScript codecs preserve neutral fixtures while canonical schema guards refuse native transport scalars. */
import {test,expect,afterAll} from "bun:test";
import Ajv from "ajv";
import {Buffer} from "node:buffer";
import fixture from "./🧫️fixtures/🔣️.json";
import {parseFormsJsonValue,formsValueJson} from "../../../../../../../../../✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🔣️json/🟦️.ts";
import {testFormsResponseExport} from "../../../../../../../../../✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/📨️response/📤️export/🧪️tests/🟦️.ts";
import {parseSemioGraphJsonValue,semioGraphJsonValue} from "../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🚪️io/📝️text/📸️snapshot/🔣️json/🟦️.ts";
import {pdfCosFromNativeJson,pdfFunctionFromNativeJson} from "../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🪪️native-json/🟦️.ts";
import {parsePdfObject,parsePdfFunction} from "../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts";
import {schema as pdfNativeSchema} from "../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🪪️native-json/🧬️schema/🟦️.ts";
import {decodeSetLineProtobuf} from "../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/✏️set-line/🟦️.ts";
import {decodeRemodelingSnapshot,remodelingSnapshotToJsonText} from "../../../../../../../../../✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🔣️json/🟦️.ts";
import {defaultRemodelingSnapshot,parseRemodelingSnapshot} from "../../../../../../../../../✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts";
import {block3dSnapshotFromJsonText,block3dSnapshotToJsonText} from "../../../../../../../../../✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🔣️json/🟦️.ts";

const oracle=new Ajv({strict:false});
const word=(number:number):bigint=>{const bytes=Buffer.alloc(8);bytes.writeDoubleBE(number);return bytes.readBigUInt64BE()};
test("Forms JSON and response exports follow the neutral transport corpus",()=>{
 expect(formsValueJson(parseFormsJsonValue(fixture.forms.value))).toBe(fixture.forms.json);
 expect(oracle.compile({const:fixture.forms.value})(JSON.parse(formsValueJson(parseFormsJsonValue(fixture.forms.value))))).toBe(true);
 testFormsResponseExport();
});
test("Graph JSON preserves nonfinite words and ordered literal fields",()=>{
 const owned=parseSemioGraphJsonValue(fixture.graph);
 expect(owned.nodes[0]!.position.x.bits).toBe(0x8000000000000000n);
 expect(owned.nodes[0]!.width.bits).toBe(0x7ff8000000000042n);
 expect(oracle.compile({const:fixture.graph})(semioGraphJsonValue(owned))).toBe(true);
});
test("PDF native JSON converts only at IO and canonical guards require owned words",()=>{
 const cos=pdfCosFromNativeJson(fixture.pdf.cos),fn=pdfFunctionFromNativeJson(fixture.pdf.function);
 expect(parsePdfObject(cos)).toEqual(cos);
 expect(parsePdfFunction(fn)).toEqual(fn);
 expect(()=>parsePdfObject(fixture.pdf.cos)).toThrow();
 expect(()=>parsePdfFunction(fixture.pdf.function)).toThrow();
 expect(fn.kind).toBe("exponential");if(fn.kind==="exponential")expect(fn.n.bits).toBe(word(fixture.word.number));
 const native={$defs:pdfNativeSchema.$defs,$ref:"#/$defs/PdfFunction"};expect(oracle.compile(native)(fixture.pdf.function)).toBe(true);
});
test("TXT physical protobuf field decoding preserves Unicode and rejects truncated frames",()=>{
 const bytes=Uint8Array.from(fixture.txt.bytes),value=decodeSetLineProtobuf(bytes);
 expect(oracle.compile({const:fixture.txt.expected})(value)).toBe(true);
 expect(Buffer.from(bytes.subarray(4)).toString("utf8")).toBe(fixture.txt.expected.text);
 expect(()=>decodeSetLineProtobuf(bytes.subarray(0,bytes.length-1))).toThrow();
});
test("Remodeling JSON projects canonical defaults and decodes each exact scalar role",()=>{
 const owned=defaultRemodelingSnapshot(),text=remodelingSnapshotToJsonText(owned);
 expect(parseRemodelingSnapshot(decodeRemodelingSnapshot(JSON.parse(text)))).toEqual(owned);
 const changed=decodeRemodelingSnapshot({schema:"remodeling.scene",id:"literal",streams:[{id:"s",syncOffsetMs:fixture.word.number}]});
 expect(changed.streams[0]!.syncOffsetMs.bits).toBe(word(fixture.word.number));
 expect(()=>parseRemodelingSnapshot(JSON.parse(remodelingSnapshotToJsonText(changed)))).toThrow();
 expect(oracle.compile({const:{bits:fixture.word.bits}})(JSON.parse(remodelingSnapshotToJsonText(changed)).streams[0].syncOffsetMs)).toBe(true);
});
test("Block 3D physical JSON resolves words before canonical snapshot admission",()=>{
 const owned=block3dSnapshotFromJsonText(JSON.stringify(fixture.block)),json=JSON.parse(block3dSnapshotToJsonText(owned));
 expect(json.vortices[0].radius.bits).toBe(word(fixture.block.vortices[0]!.radius).toString(16).padStart(16,"0"));
 expect(block3dSnapshotFromJsonText(JSON.stringify(json))).toEqual(owned);
 expect(oracle.compile({const:json})(JSON.parse(block3dSnapshotToJsonText(owned)))).toBe(true);
});

afterAll(()=>console.log("[DEBUG] Physical TypeScript codecs: neutral transport laws and independent scalar/schema oracles completed"));

test("Program snapshot and diff codecs retain the independent document corpus",()=>testProgramDocumentContract(),{timeout:60000});
test("Layout snapshot and diff codecs retain the independent document and Drawing corpus",()=>testLayoutDocumentContractOracle());
test("Jack typed query ownership and graph mutations retain independent SQLite and JSON1 laws",()=>testResumableQueryOracle(),{timeout:60000});

test("GIS semantic requests acquire JSON byte admission only at transport IO",()=>{
 const row=fixture.gisInference,value=sealGisMapInferenceJobRequestV1(row.requestId,row.lifetimeMs),json=gisMapInferenceRequestToJson(value);
 expect(json).toBe(row.json);expect(Buffer.byteLength(json,"utf8")).toBe(row.bytes);
 expect(oracle.compile({const:JSON.parse(row.json)})(JSON.parse(json))).toBe(true);
 expect(()=>gisMapInferenceRequestToJson({...value,requestId:"x".repeat(10000)})).toThrow("oversized-request");
});
