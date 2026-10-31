import {createRequire} from 'node:module';
const require=createRequire(import.meta.url),{sortBy}=require('lodash'),schema=require('../🧬️schema/🔣️.json'),validate=new(require('ajv'))({strict:false}).compile(schema);
if(!validate(require('./📜️fixtures/🔣️.json')))throw Error('Independent numeric scratch schema refusal');
/** 🔬️ Independent ordered observation over a host Map uses test-only lodash sorting. */
export function numericScratchOutput(operations){const state=new Map();return operations.map(row=>{const previous=state.get(row.key);switch(row.kind){case'set':state.set(row.key,row.value);return previous??null;case'get':return previous??null;case'remove':state.delete(row.key);return previous??null;case'first':{const first=sortBy(Array.from(state.entries()),row=>row[0])[0];if(first)state.delete(first[0]);return first??null;}case'reset':state.clear();return null;default:throw Error('oracle.operation');}});}

/** 🎟️ Independent test-only ordering for funded reservation observations. */
export function numericReservationOutput(rows){return sortBy(rows,row=>row[0]);}
