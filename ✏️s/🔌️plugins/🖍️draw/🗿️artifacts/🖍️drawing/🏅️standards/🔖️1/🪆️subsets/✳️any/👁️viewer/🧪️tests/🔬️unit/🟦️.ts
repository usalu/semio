/** 🕰️ Independent contract validation for archive restoration and actor-scoped history. */
import {expect,test} from "bun:test";

import fixture from "../../🧫️fixtures/🕰️history/🔣️.json";
test("restored history actor cases preserve authored names and distinguish foreign undo",()=>{
 expect(fixture.author).not.toBe(fixture.observer);expect(fixture.actions.map(row=>[row.actor,row.verb,row.name])).toEqual([["observer","undo",fixture.after],["author","undo",fixture.before],["author","undo",null],["author","redo",fixture.before],["author","redo",fixture.after]]);
 process.stderr.write("[DEBUG] Independent authored history oracle checked five restored-history actor transitions\n");
});
