/** 🧪️ Native query owner lifecycle data is a closed multi-implementation contract. */
import {test,expect} from 'bun:test';
import Ajv from 'ajv/dist/2020';
import cases from './cases.json';
import schema from './schema.json';
test('closed original-native query lifecycle contract',()=>expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true));
