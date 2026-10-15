import {test,expect} from "bun:test";
import Ajv from "ajv";
import energyFixture from "../../../../🧪️tests/🧫️fixtures/🧬️mutations/🪪️owned-target/🔣️.json";
import referenceSchema from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/🔣️.json";
import {parseEnergyReferenceTarget} from "../../🟦️.ts";
const reference=new Ajv({strict:false}).addSchema(referenceSchema).compile({$ref:referenceSchema.$id+"#/$defs/ArtifactRef"});
test("Energy link targets admit owned identity fields and refuse native URI authority",()=>{for(const row of energyFixture.cases){expect(reference(row.target)).toBe(row.name==="empty-target-refuses"||row.valid);if(row.valid){const owned=parseEnergyReferenceTarget(row.target);expect(owned).toEqual(row.target as unknown as typeof owned);}else expect(()=>parseEnergyReferenceTarget(row.target)).toThrow();}});
