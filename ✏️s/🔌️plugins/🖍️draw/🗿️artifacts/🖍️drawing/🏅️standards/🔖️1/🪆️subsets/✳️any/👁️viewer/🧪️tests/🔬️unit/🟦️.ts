/** 🕰️ Independent contract validation for archive restoration and actor-scoped history. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import fixture from "../../🧫️fixtures/🕰️history/🔣️.json";
import schema from "../../🧬️schema/🕰️history/🔣️.json";
test("restored history actor cases preserve authored names and distinguish foreign undo",()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);expect(fixture.author).not.toBe(fixture.observer);expect(fixture.actions.map(row=>[row.actor,row.verb,row.name])).toEqual([["observer","undo",fixture.after],["author","undo",fixture.before],["author","undo",null],["author","redo",fixture.before],["author","redo",fixture.after]]);
 for(const patch of [{layerDelta:-1},{layerDelta:2},{actor:"unspecified"},{verb:"remove"},{name:5}]){const invalid=structuredClone(fixture);Object.assign(invalid.actions[0]!,patch);expect(validate(invalid)).toBe(false);}
 process.stderr.write("[DEBUG] Independent Ajv validated five restored-history actor transitions and refused invalid action contracts\n");
});
