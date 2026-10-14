import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {BufferGeometry,Float32BufferAttribute} from "three";
import law from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
test("native mesh original pages preserve canonical geometry and all three nonempty label domains",()=>{
 expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);const geometry=new BufferGeometry();try{geometry.setAttribute("position",new Float32BufferAttribute(law.positions.flat(),3));geometry.setAttribute("normal",new Float32BufferAttribute(law.normals.flat(),3));geometry.setIndex(law.indices);geometry.computeBoundingBox();expect(geometry.boundingBox?.min.toArray()).toEqual([0,0,0]);expect(geometry.boundingBox?.max.toArray()).toEqual([1,1,0]);const view=new DataView(Uint8Array.from(law.geometry).buffer);for(let index=0;index<law.positions.flat().length;index++)expect(view.getFloat32(index*4,true)).toBe(geometry.getAttribute("position").array[index]);expect(law.geometry.length).toBe(128)}finally{geometry.dispose()}
 const db=new Database(":memory:");try{for(const labels of law.labels)for(const label of labels){const row=db.query("SELECT CAST(? AS BLOB) AS bytes").get(label) as {bytes:Uint8Array};expect(Array.from(row.bytes)).toEqual(Array.from(new TextEncoder().encode(label)))}}finally{db.close()}
});
