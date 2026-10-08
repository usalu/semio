import Ajv from "ajv/dist/2020.js";
/** 🔮️ Schema oracle answers expose only first-party booleans across the test library boundary. */
export interface SurfaceSchemaOracleV1 { input(value:unknown):boolean; surface(value:unknown):boolean; batch(value:unknown):boolean }
/** 📐️ Strict independent schema compilation declares exactly two intentional annotation keywords. */
export function createSurfaceSchemaOracleV1(schema:unknown):SurfaceSchemaOracleV1 {
 const ajv=new Ajv({strict:true,allErrors:true});
 ajv.addKeyword({keyword:"x-semio-numeric-policy",schemaType:"object",metaSchema:{type:"object",additionalProperties:false,required:["positions","indices","faceOrder","faceComparison","failedTriangulation","barsAndBeams"],properties:{positions:{const:"binary64"},indices:{const:"u32"},faceOrder:{const:"unspecified"},faceComparison:{const:"cyclic-oriented-multiset"},failedTriangulation:{const:"document-owner-omits-region"},barsAndBeams:{const:"document-owner-contributes-no-region"}}}});
 ajv.addKeyword({keyword:"x-semio-operation-contract",schemaType:"object",metaSchema:{type:"object",additionalProperties:false,required:["mandatory","ownership","progress","partialResult","scheduling","phases","refusal"],properties:{mandatory:{const:"explicit-caller-authority"},ownership:{const:"semio_framework_value::retained_clone::RetainedCloneGrant"},progress:{const:"semio_framework_value::retained_clone::RetainedCloneProgress"},partialResult:{const:"semio_framework_value::retained_clone::RetainedCloneStep"},scheduling:{const:"semio_framework_job::StepContext"},phases:{const:["admission","triangulation","extrusion","tetrahedralization","boundary","encoding","retirement"]},refusal:{const:"no-default-or-demand-derived-grant"}}}});
 if(schema===null||typeof schema!=="object"||!("$id"in schema)||schema.$id!=="https://json.schemas.assets.semio-tech.com/s/fem/mesh/surface/component.json")throw Error("Surface oracle requires the canonical first-party schema identity");
 ajv.addSchema(schema as Record<string,unknown>);const input=ajv.compile({$ref:schema.$id+"#/$defs/RegionSurfaceInputV1"}),surface=ajv.compile({$ref:schema.$id+"#/$defs/RegionVolumeSurfaceV1"}),batch=ajv.compile({$ref:schema.$id+"#/$defs/RegionVolumeSurfaceBatchV1"});
 const accept=(validate:typeof input,value:unknown):boolean=>{const result=validate(value);if(typeof result!=="boolean")throw Error("Surface schema oracle must return a synchronous first-party answer");return result;};
 return{input:value=>accept(input,value),surface:value=>accept(surface,value),batch:value=>accept(batch,value)};
}
