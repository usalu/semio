import {test,expect} from "bun:test";
import Ajv from "ajv";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
type Row=Record<string,unknown>;
const apply=(state:Row,mutation:Row)=>{const{kind,...payload}=mutation;const post={...state,...structuredClone(payload)};const inverse:Row={kind};for(const field of Object.keys(payload))inverse[field]=structuredClone(state[field]);return{post,inverse};};
test("field-set apply cases agree with an independent exchange oracle",()=>{
 expect(new Ajv({strict:true,allowUnionTypes:true}).compile(schema)(fixture)).toBe(true);
 for(const row of fixture.cases){
  const forward=apply(row.base,row.mutation);
  expect(forward.post).toEqual(row.post);
  expect(forward.inverse).toEqual(row.inverse);
  const back=apply(forward.post,forward.inverse);
  expect(back.post).toEqual(row.base);
  expect(back.inverse).toEqual(row.mutation);
 }
 console.log("[DEBUG] independent JS exchange oracle reproduces every post state and inverse row of the Rust apply edit fixture");
});
