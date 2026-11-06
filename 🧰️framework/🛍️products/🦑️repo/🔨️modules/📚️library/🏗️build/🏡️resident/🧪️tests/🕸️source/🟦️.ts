import {test,expect} from "bun:test";
import fixture from "../🧫️fixtures/🕸️graph-source.json" with {type:"json"};
import {loadResidentGraphV1} from "../../🕸️source/🟦️.ts";
test("resident graph requests the actual current generation without a nonexistent inherited pin",async()=>{const original={} as never,control={} as never,expected={owned:true},result=await loadResidentGraphV1("root",original,control,async(...args)=>{expect(args).toEqual(["root",fixture.arguments,original,original,control,fixture.reuse,undefined,fixture.policy]);return expected as never;});expect(result).toBe(expected);});
