/** 🧫️ Original GDEF glyph eligibility agrees with independently parsed class records. */
import {test,expect} from "bun:test";
import Ajv from "ajv";
import opentype from "opentype.js";
import {FontFaceAdmissionJob} from "../../../🟦️.ts";
import {builtinFontLocations} from "../../../📇️catalog/🟦️.ts";
import {FontGlyphClassCursor} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import rows from "../🧫️fixtures/🔣️.json";
const limits={maxFontBytes:67108864,maxTables:128,maxGlyphs:262144,maxPoints:1048576,maxContours:65536,maxComponents:262144,maxDepth:32,maxSegments:1048576,maxWork:1e9};
test("shared original GDEF classes match independent OpenType",async()=>{expect(new Ajv({strict:true}).compile(schema)(rows)).toBe(true);let queries=0;for(const source of [...builtinFontLocations("Anta"),...builtinFontLocations("Noto Emoji")]){const bytes=new Uint8Array(await Bun.file(source.url).arrayBuffer()),parsed=opentype.parse(bytes.buffer),admission=new FontFaceAdmissionJob(bytes,limits);while(!admission.advance(4096).done){}const face=admission.result(),table=(parsed.tables as any).gdef?.classDef;for(const row of rows){for(const scalar of row.text){const glyph=parsed.charToGlyphIndex(scalar),cursor=new FontGlyphClassCursor(face,glyph);let turns=0;while(!cursor.step()){expect(()=>cursor.result()).toThrow();expect(++turns).toBeLessThan(100);}let expected=0;if(table?.format===1&&glyph>=table.startGlyph&&glyph<table.startGlyph+table.classes.length)expected=table.classes[glyph-table.startGlyph];if(table?.format===2){const range=table.ranges.find((range:any)=>glyph>=range.start&&glyph<=range.end);if(range)expected=range.classId;}expect(cursor.result()).toBe(expected);expect(face.bytes).toBe(bytes);queries++;}}const moved=admission.intoRetirement();while(!moved.job.advance(1).done){}}console.log(`[DEBUG] Original GDEF classes queries=${queries}: independent OpenType marks/base/ligature authority`);},120000);
