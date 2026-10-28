/** 🚪️ Standard export identity remains separate from narrow clause execution type checks. */
import {test,expect} from 'bun:test';
import {executeIfc4x3Requirements as standardEndpoint} from '../../../🟦️.ts';
import {executeIfc4x3Requirements} from './🟦️.ts';
test('actual standard endpoint exposes selected execution',()=>{expect(standardEndpoint).toBe(executeIfc4x3Requirements);});
