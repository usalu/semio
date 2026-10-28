import {testNumericScratch} from './🟦️.ts';
import {numericScratchOutput,numericReservationOutput} from './oracle.mjs';
import {checkNumericReservation} from '../🎟️reservation/🧪️tests/🟦️.ts';
/** 🧮️ Runs original scalar-index semantics and funded retained ownership laws. */
export function runNumericScratchTests(){return{...testNumericScratch(numericScratchOutput),reservation:checkNumericReservation(numericReservationOutput)};}
