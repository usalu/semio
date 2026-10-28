import {expect,test} from 'bun:test';
import {encodePackValue} from '../../../🎁️packs/🟦️.ts';
import {stagedFormCensusSteps,stagedFormWireBytes,STAGED_FORM_LIMITS} from './🟦️.ts';
import wireCases from './wire-cases.json';
import requestCases from './request-cases.json';

test('standalone staged wire reservation covers actual production Pack framing',()=>{
 const census=(value:unknown)=>{const steps=stagedFormCensusSteps(value);for(;;){const next=steps.next();if(next.done)return next.value;}};
 expect(stagedFormWireBytes()).toBe(wireCases.frameworkMaximumBytes);
 for(const value of [...wireCases.values,...requestCases.valid]){const measured=census(value);expect(encodePackValue(value).length).toBeLessThanOrEqual(stagedFormWireBytes({...STAGED_FORM_LIMITS,utf8Bytes:measured.utf8Bytes,nodes:measured.nodes,depth:measured.depth}));}
 const many=Object.fromEntries(Array.from({length:16000},(_,index)=>[String(index),'v'+index]));
 const measured=census(many);expect(encodePackValue(many).length).toBeLessThanOrEqual(stagedFormWireBytes({...STAGED_FORM_LIMITS,utf8Bytes:measured.utf8Bytes,nodes:measured.nodes,depth:measured.depth}));
});
