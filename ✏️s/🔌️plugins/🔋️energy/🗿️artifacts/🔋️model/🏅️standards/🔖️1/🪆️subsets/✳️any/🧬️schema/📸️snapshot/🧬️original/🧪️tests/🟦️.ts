import { expect, test } from "bun:test";
import Ajv from "ajv";
import publicSchema from "../../🔣️.json";
import modelSchema from "../../⚡️model/🔣️.json";
import diffSchema from "../../../🔺️diff/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import law from "./🧫️fixtures/🔣️.json";
import genesisLaw from "../🌱️genesis/🧪️tests/🔣️.json";
import { Database } from "bun:sqlite";
import modelLaw from "../../../../🧪️tests/🧫️fixtures/📸️snapshot/🧳️state/🔣️.json";

test("original Energy document preserves the sole model and declared link slots", () => {
  const validate = new Ajv({strict: true}).compile(schema);
  const document = {...law.document, model: modelLaw.model};
  expect(validate(document)).toBe(true);
  expect(Object.keys(JSON.parse(JSON.stringify(document))).sort()).toEqual([...law.persistedFields].sort());
  const before = JSON.stringify(document);
  for (const field of ["structure", "zones"]) expect(validate({...document, [field]: {}})).toBe(false);
  for (const field of law.persistedFields) {
    const missing: Record<string, unknown> = {...document};
    delete missing[field];
    expect(validate(missing)).toBe(false);
  }
  expect(JSON.stringify(document)).toBe(before);
});

test("only a funded source advance establishes the original close body", () => {
  const db = new Database(":memory:");
  db.run("CREATE TABLE owner (body INTEGER)");
  db.run("INSERT INTO owner VALUES (NULL)");
  for (const [body, funded] of [[genesisLaw.bodyPolicy.unfunded, false], [genesisLaw.bodyPolicy.admitted, true]] as const) {
    db.query("UPDATE owner SET body = ? WHERE body IS NULL AND ?").run(body, funded ? 1 : 0);
    expect(db.query("SELECT body FROM owner").get()).toEqual({body: funded ? genesisLaw.bodyPolicy.admitted : null});
  }
  expect(db.query("SELECT body = ? AS accepted FROM owner").get(genesisLaw.bodyPolicy.foreign)).toEqual({accepted: 0});
  expect(db.query("SELECT body FROM owner").get()).toEqual({body: genesisLaw.bodyPolicy.admitted});
  db.close();
});

test("public native Snapshot and prepared branch schemas describe the same sole model authority",()=>{const ajv=new Ajv({strict:false});ajv.addSchema(modelSchema);const validate=ajv.compile(publicSchema),delta=ajv.compile(diffSchema);const document={...law.document,model:modelLaw.model};expect(validate(document)).toBe(true);for(const field of ["structure","zones"])expect(validate({...document,[field]:{}})).toBe(false);expect(delta({model:{}})).toBe(true);for(const[branch,spec]of Object.entries(diffSchema.properties.model.properties)){const value=Object.fromEntries(spec.required.map(k=>[k,document.model[k]]));expect(delta({model:{[branch]:value}})).toBe(true);expect(delta({model:{[branch]:{}}})).toBe(false);}expect(delta({model:{unknown:{}}})).toBe(false);expect(delta({model:{geometry:{zones:{removed:[]}}}})).toBe(false);});
