import {expect,test} from "bun:test";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🔣️.json" with {type:"json"};
import {readDescriptorEmitterReleaseReplyV1,descriptorEmitterReleaseJobAcceptsV1,type DescriptorEmitterReleaseRequestV1} from "../🟦️.ts";

test("the sealed release reply belongs to the same Original attempt and host",()=>{
 const valid=new Ajv({strict:true}).compile({definitions:schema.definitions,$ref:"#/definitions/Reply"});
 for(const row of fixture.replyCases){
  const reply={request:{...fixture.request,...row.requestDelta},evidence:{...fixture.evidence,...row.evidenceDelta}};
  expect(valid(reply),row.id).toBe(row.validShape);
  const read=()=>readDescriptorEmitterReleaseReplyV1(reply,fixture.request as DescriptorEmitterReleaseRequestV1);
  if(row.accepted)expect(read(),row.id).toEqual(expect.objectContaining(fixture.evidence));
  else expect(read,row.id).toThrow();
 }
});

test("only ordinary success and structured source restart admit a sealed release producer reply",()=>{
 for(const row of fixture.terminalCases)expect(descriptorEmitterReleaseJobAcceptsV1(row.exit,75,row.sealed)).toBe(row.accept);
});
