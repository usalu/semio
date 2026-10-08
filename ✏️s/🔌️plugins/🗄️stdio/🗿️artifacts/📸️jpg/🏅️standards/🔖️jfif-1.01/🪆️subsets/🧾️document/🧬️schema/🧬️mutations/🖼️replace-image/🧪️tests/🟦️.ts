import {expect,test} from "bun:test";
import Ajv from "ajv";
import fixture from "../🧫️fixtures/🔣️.json";
import snapshotSchema from "../../../📸️snapshot/🔣️.json";
import payloadSchema from "../🧬️schema/🔣️.json";
import type {JpgImage,JpgSnapshot} from "../../../📸️snapshot/🟦️.ts";
test("JPEG image intent owns imported content and excludes native observations",()=>{
 const image=structuredClone(fixture.importedImage) as JpgImage;const snapshot:JpgSnapshot={schema:"stdio.jpg",image};
 const ajv=new Ajv({strict:false});ajv.addSchema(snapshotSchema);expect(ajv.validate(snapshotSchema,snapshot)).toBe(true);expect(ajv.validate(payloadSchema,{image})).toBe(true);
 for(const field of fixture.excludedNativeFields){expect(ajv.validate(snapshotSchema,{...snapshot,[field]:0})).toBe(false);expect(ajv.validate(payloadSchema,{image:{...image,[field]:0}})).toBe(false);}
 expect(snapshot.image.otherSegments.map(segment=>segment.marker)).toEqual([225,254,225]);
 console.info("[DEBUG] JPEG neutral imported image validated independently by Ajv");
});
