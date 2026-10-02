/** 🔗️ Unrestricted owned identity admits literal fields independently of URI formatting. */
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import {parseArtifactRef} from "../../🟦️.ts";
import contract from "../../🔣️.json";
import fixture from "../🧫️fixtures/🔣️.json";

test("owned reference strings remain literal under schema and independent SQL admission",()=>{
  const ajv=new Ajv({strict:false});ajv.addSchema(contract);const admit=ajv.compile({$ref:`${contract.$id}#/$defs/ArtifactRef`});
  const sql=new Database(":memory:");sql.run("CREATE TABLE reference(artifact_id TEXT NOT NULL,artifact_kind TEXT NOT NULL,standard TEXT NOT NULL,subset TEXT NOT NULL)");
  try{for(const value of fixture.references){expect(admit(value)).toBe(true);const actual=parseArtifactRef(value);expect(actual).toEqual(value);sql.run("DELETE FROM reference");sql.run("INSERT INTO reference VALUES(?,?,?,?)",actual.artifactId,actual.dialect.artifactKind,actual.dialect.standard,actual.dialect.subset);expect(sql.query("SELECT artifact_id AS artifactId,artifact_kind AS artifactKind,standard,subset FROM reference").get()).toEqual({artifactId:value.artifactId,...value.dialect});}
    for(const invalid of [{artifactId:"",dialect:{artifactKind:"",standard:""}},{artifactId:2,dialect:{artifactKind:"",standard:"",subset:""}}]){expect(admit(invalid)).toBe(false);expect(()=>parseArtifactRef(invalid)).toThrow();}
  }finally{sql.close();}
});
