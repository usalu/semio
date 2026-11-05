/** 🔒️ Revision preconditions share neutral cases with native metadata admission. */
import {test,expect} from 'bun:test';
import Ajv from 'ajv/dist/2020.js';
import cases from './cases.json' with {type:'json'};
import schema from './🔣️.json' with {type:'json'};
test('staged revision cases admit exactly the original document frontier',()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);for(const row of cases.cases)expect(row.expected===null||row.expected===row.current&&row.instance===row.live).toBe(row.accepted);});
