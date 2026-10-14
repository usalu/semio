import {test,expect} from "bun:test";
import Ajv from "ajv";
import {resolve as oracleResolve} from "pathe";
import {cargoObservationOutputV1} from "../../🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import vectors from "./🧫️fixtures/🔣️.json";
test("immutable compiler observation output stays inside its explicit owner",()=>{
 const validate=new Ajv({strict:true}).compile(schema);
 for(const row of vectors.vectors){const input={version:1,directory:row.directory};if(row.accepted){expect(validate(input)).toBe(true);expect(cargoObservationOutputV1(input,vectors.buildDirectory)).toBe(oracleResolve(row.directory));}else expect(()=>cargoObservationOutputV1(input,vectors.buildDirectory)).toThrow();}
 for(const value of [{directory:"/compiler/semio-cargo-observations/x"},{version:1,directory:"/compiler/semio-cargo-observations/x",extra:true}]){expect(validate(value)).toBe(false);expect(()=>cargoObservationOutputV1(value,vectors.buildDirectory)).toThrow();}
});
