import {RetainedWorkLedger} from '../../../../🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts';
import fixture from '../🎟️work/🧪️tests/🧫️fixtures/🔣️.json' with {type:'json'};
/** 🧪️ Explicit test authority is independent of child demand and never used by production. */
export const fontTestWork=(maximumItems:number)=>new RetainedWorkLedger({...fixture.typographyOriginal,maximumItems});
