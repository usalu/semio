import { pdfImageFromNativeJson,pdfStateFromNativeJson,pdfFormFromNativeJson,pdfShadingFromNativeJson,pdfPatternFromNativeJson } from "../../../../📝️text/📸️snapshot/🪪️native-json/🖼️resource/🟦️.ts";
/** 🖼️ Independent SQLite interpretation proves every image field and intrinsic codec relationship. */
import { expect,test } from "bun:test";
import { Database } from "bun:sqlite";
import type { PdfImage,PdfImageBody,PdfImageMask } from "../../../../../🧬️schema/📸️snapshot/🟦️.ts";
import { parsePdfImage,parsePdfExtGState,parsePdfFormXObject,parsePdfShading,parsePdfPattern } from "../../../../../🧬️schema/📸️snapshot/🟦️.ts";
import { PdfProjection,PdfReader } from "../../🧩️entity/🟦️.ts";
import { PDF17_SQLITE_SCHEMA } from "../../🧬️schema/🟦️.ts";
import { writePdfImage,readPdfImage,pdfResourceNumberColumns } from "../../🖼️resource/🟦️.ts";
import { writePdfState,readPdfState,writePdfForm,readPdfForm } from "../../🖼️resource/🎚️state/🟦️.ts";
import { writePdfShading,readPdfShading,writePdfPattern,readPdfPattern } from "../../🖼️resource/🌈️shading/🟦️.ts";
import { exportSqliteDatabase,importSqliteDatabase } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

test("Native JSON resource admission uses the canonical owned Binary64 domain",async()=>{
  const fixture=JSON.parse(await Bun.file(new URL("../../../../📝️text/📸️snapshot/🪪️native-json/🖼️resource/🧫️fixtures/🔣️.json",import.meta.url)).text());
  expect(parsePdfImage(pdfImageFromNativeJson(fixture.image)).decode).toEqual([{bits:0n},{bits:0x3ff0000000000000n}]);
  expect(parsePdfExtGState(pdfStateFromNativeJson(fixture.state)).lineWidth).toEqual({bits:0x3ff0000000000000n});
  expect(parsePdfFormXObject(pdfFormFromNativeJson(fixture.form)).bbox[2]).toEqual({bits:0x3ff0000000000000n});
  const shading=parsePdfShading(pdfShadingFromNativeJson(fixture.shading));if(shading.kind.kind!=="axial")throw new Error("Fixture shading");expect(shading.kind.coords[2]).toEqual({bits:0x3ff0000000000000n});
  const pattern=parsePdfPattern(pdfPatternFromNativeJson(fixture.pattern));if(pattern.kind.kind!=="tiling")throw new Error("Fixture pattern");expect(pattern.kind.xStep).toEqual({bits:0x3ff0000000000000n});
});

test("PDF image resources preserve logical samples and foreign artifact references, masks and complete semantic fields",async()=>{
  const fixture=JSON.parse(await Bun.file(new URL("../../🖼️resource/🧫️fixtures/🔣️.json",import.meta.url)).text(),(key,value)=>key==="bits"?BigInt(value):value);
  for(const [index,body]of(fixture.bodies as PdfImageBody[]).entries()){
    const mask:PdfImageMask|null=fixture.masks[index%fixture.masks.length];const value:PdfImage={...fixture.image,body,mask,matte:index%2===0?null:[]};
    const out=await PdfProjection.create(PDF17_SQLITE_SCHEMA,{},pdfResourceNumberColumns);const root=await writePdfImage(out,value);const bytes=await exportSqliteDatabase(await out.finish());const sql=Database.deserialize(bytes);
    expect(sql.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(sql.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(sql.query("SELECT resource_name,CAST(width AS TEXT) AS width,body_kind,matte_present FROM pdf_image").get()).toEqual({resource_name:value.id,width:"4294967295",body_kind:body.kind,matte_present:index%2});
    const reader=await PdfReader.create(await importSqliteDatabase(bytes),PDF17_SQLITE_SCHEMA,{},pdfResourceNumberColumns);expect(await readPdfImage(reader,root)).toEqual(value);await reader.finish();sql.close();
  }
},{timeout:30_000});

test("PDF resource import rejects independently authored partial tuples and mixed mask fields",async()=>{
  const stateFixture=JSON.parse(await Bun.file(new URL("../../🖼️resource/🎚️state/🧫️fixtures/🔣️.json",import.meta.url)).text(),(key,value)=>key==="bits"?BigInt(value):value);
  const shadeFixture=JSON.parse(await Bun.file(new URL("../../🖼️resource/🌈️shading/🧫️fixtures/🔣️.json",import.meta.url)).text(),(key,value)=>key==="bits"?BigInt(value):value);
  const out=await PdfProjection.create(PDF17_SQLITE_SCHEMA,{},pdfResourceNumberColumns);const state=await writePdfState(out,stateFixture.states[1]);const shade=await writePdfShading(out,shadeFixture.shadings[0]);
  const bytes=await exportSqliteDatabase(await out.finish());
  const font=Database.deserialize(bytes);font.exec("UPDATE pdf_ext_g_state SET font_name=NULL");const fontReader=await PdfReader.create(await importSqliteDatabase(font.serialize()),PDF17_SQLITE_SCHEMA,{},pdfResourceNumberColumns);await expect(readPdfState(fontReader,state)).rejects.toThrow("both name and size");font.close();
  const mask=Database.deserialize(bytes);mask.exec("UPDATE pdf_ext_g_state SET soft_mask_group='Foreign'");const maskReader=await PdfReader.create(await importSqliteDatabase(mask.serialize()),PDF17_SQLITE_SCHEMA,{},pdfResourceNumberColumns);await expect(readPdfState(maskReader,state)).rejects.toThrow("unrelated scalar");mask.close();
  const domain=Database.deserialize(bytes);domain.exec("UPDATE pdf_function_shading SET domain_xmin=1,domain_xmin_bits=4607182418800017408,domain_xmin_class='finite'");const domainReader=await PdfReader.create(await importSqliteDatabase(domain.serialize()),PDF17_SQLITE_SCHEMA,{},pdfResourceNumberColumns);await expect(readPdfShading(domainReader,shade)).rejects.toThrow("entirely present or absent");domain.close();
},{timeout:30_000});

test("PDF shading and patterns expose every variant as explicit SQL entities",async()=>{
  const fixture=JSON.parse(await Bun.file(new URL("../../🖼️resource/🌈️shading/🧫️fixtures/🔣️.json",import.meta.url)).text(),(key,value)=>key==="bits"?BigInt(value):value);
  const out=await PdfProjection.create(PDF17_SQLITE_SCHEMA,{},pdfResourceNumberColumns);const shades:bigint[]=[];const patterns:bigint[]=[];
  for(const value of fixture.shadings)shades.push(await writePdfShading(out,value));for(const value of fixture.patterns)patterns.push(await writePdfPattern(out,value));
  const bytes=await exportSqliteDatabase(await out.finish());const sql=Database.deserialize(bytes);
  expect(sql.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(sql.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect(sql.query("SELECT kind FROM pdf_shading ORDER BY id").all()).toEqual([{kind:"functionBased"},{kind:"axial"},{kind:"radial"},{kind:"mesh"}]);
  expect(sql.query("SELECT r.artifact_id AS artifactId,CAST(m.bits_per_coordinate AS TEXT) AS width FROM pdf_mesh_shading m JOIN pdf_artifact_reference r ON r.id=m.artifact_reference_id").get()).toEqual({artifactId:"fixture:mesh",width:"4294967295"});
  const reader=await PdfReader.create(await importSqliteDatabase(bytes),PDF17_SQLITE_SCHEMA,{},pdfResourceNumberColumns);
  for(const[index,key]of shades.entries())expect(await readPdfShading(reader,key)).toEqual(fixture.shadings[index]);for(const[index,key]of patterns.entries())expect(await readPdfPattern(reader,key)).toEqual(fixture.patterns[index]);await reader.finish();sql.close();
},{timeout:30_000});

test("PDF graphics state and forms preserve absent and empty relations and exact IEEE words",async()=>{
  const fixture=JSON.parse(await Bun.file(new URL("../../🖼️resource/🎚️state/🧫️fixtures/🔣️.json",import.meta.url)).text(),(key,value)=>key==="bits"?BigInt(value):value);
  const out=await PdfProjection.create(PDF17_SQLITE_SCHEMA,{},pdfResourceNumberColumns);const keys:bigint[]=[];
  for(const state of fixture.states)keys.push(await writePdfState(out,state));const form=await writePdfForm(out,fixture.form);
  const bytes=await exportSqliteDatabase(await out.finish());const sql=Database.deserialize(bytes);
  expect(sql.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect(sql.query("SELECT resource_name,blend_present,soft_mask_kind,backdrop_present FROM pdf_ext_g_state ORDER BY id").all()).toEqual([
    {resource_name:"Absent",blend_present:0,soft_mask_kind:null,backdrop_present:null},{resource_name:"Empty",blend_present:1,soft_mask_kind:"none",backdrop_present:null},{resource_name:"Alpha",blend_present:1,soft_mask_kind:"alpha",backdrop_present:null},{resource_name:"LuminosityAbsent",blend_present:0,soft_mask_kind:"luminosity",backdrop_present:0},{resource_name:"LuminosityEmpty",blend_present:0,soft_mask_kind:"luminosity",backdrop_present:1}
  ]);
  const reader=await PdfReader.create(await importSqliteDatabase(bytes),PDF17_SQLITE_SCHEMA,{},pdfResourceNumberColumns);
  for(const [index,key]of keys.entries())expect(await readPdfState(reader,key)).toEqual(fixture.states[index]);expect(await readPdfForm(reader,form)).toEqual(fixture.form);await reader.finish();sql.close();
},{timeout:30_000});
