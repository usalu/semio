import assert from 'node:assert/strict';
import fixture from './🧫️fixtures/🔣️.json' with {type:'json'};
import {DRAWING_CHILD_EDIT_PATH,DRAWING_CHILD_EDIT_FILL,DRAWING_CHILD_PATCH_LAYER} from '../🟦️.ts';
import {DRAWING_VECTOR_SOURCE} from '../../../🧬️schema/🧵️vector/🟦️.ts';
import {DRAWING_ADMITTER} from '../../../🧬️schema/✅️validation/🟦️.ts';
/** 🪆️ Registered Drawing commands retain the genuine editor dialect and its mandatory admission owner. */
export function testDrawingChildRegistrations(equal:(a:unknown,b:unknown)=>boolean){const rows=[DRAWING_CHILD_EDIT_PATH,DRAWING_CHILD_EDIT_FILL,DRAWING_CHILD_PATCH_LAYER];for(let index=0;index<rows.length;index++){const row=rows[index]!;assert.ok(equal(row.identity,{...fixture.dialect,editorId:fixture.editorId,commandId:fixture.commands[index]}));assert.equal(row.admission,DRAWING_ADMITTER);assert.equal(row.vector,DRAWING_VECTOR_SOURCE);assert.ok(Object.isFrozen(row));assert.ok(Object.isFrozen(row.identity));}return{registrations:rows.length,originalAdmitters:rows.length,originalVectors:rows.length};}
