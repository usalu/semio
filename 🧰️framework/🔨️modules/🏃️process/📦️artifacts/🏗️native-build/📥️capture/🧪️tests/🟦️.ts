import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import Ajv from "ajv";
import {readCompletedCargoCaptureMetadataV1} from "../📋️contract/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import sourcePolicy from "../../../📤️publication/📥️sources/🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🔣️.json" with {type:"json"};

test("completed Cargo capture accepts large executables within the bounded source inventory", () => {
  const {valid, maximumObservationBytes, extents} = fixture;
  const acceptsMetadata = new Ajv({strict:true}).compile(schema);
  const maximumBytes = sourcePolicy.properties.maximumBytes.const;
  const db = new Database(":memory:");
  try {
    const fitsInventory = db.query("select sum(value) <= ?2 as fits from json_each(?1)");
    for (const {fileBytes, schemaAccepts, captureAccepts} of extents) {
      const files = fileBytes.map((bytes, index) => ({...valid.files[index]!, bytes}));
      const metadata = {...valid, files};
      expect(acceptsMetadata(metadata)).toBe(schemaAccepts);
      expect(fitsInventory.get(JSON.stringify(fileBytes), maximumBytes)).toEqual({fits:Number(captureAccepts)});
      const read = () => readCompletedCargoCaptureMetadataV1(metadata, maximumObservationBytes);
      if (captureAccepts) expect(read().files).toEqual(files);
      else expect(read).toThrow();
    }
  } finally {
    db.close();
  }
});
