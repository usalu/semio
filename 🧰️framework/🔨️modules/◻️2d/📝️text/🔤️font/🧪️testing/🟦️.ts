import {FontWorkingMapping} from '../🧵️shape/🧩️context/🪄️apply/🏪️workspace/🟦️.ts';
import {RetainedWorkLedger} from '../../../../🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts';
import fixture from '../🎟️work/🧪️tests/🧫️fixtures/🔣️.json' with {type:'json'};
/** 🧪️ Explicit test authority is independent of child demand and never used by production. */
export const fontTestWork=(maximumItems:number)=>new RetainedWorkLedger({...fixture.typographyOriginal,maximumItems});
/** 🧬️ A genuine owned glyph sequence built through the funded workspace, for tests that start from authored glyphs. */
export function fontGlyphSequence(values:readonly any[]){const workspace=new FontWorkingMapping(0,Math.max(1,values.length));while(!workspace.admit(fontTestWork(4096))){}for(const value of values)while(!workspace.insert(value,fontTestWork(4096))){}while(!workspace.compactStep(fontTestWork(4096))){}return workspace.take(fontTestWork(4096))!.glyphs;}
