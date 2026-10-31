/** 🔐️ A structural lookalike or copied loan is not a defining issuer's current observation token. */
import {test,expect} from "bun:test";
import Ajv from "ajv";
import {RetainedWorkBuffer} from "../../🟦️.ts";
import {wallet,run,owning,zero} from "./admission-fixture.ts";
import law from "./admission-provenance.json";
import schema from "./admission-provenance.schema.json";
test("unissued and copied accounting loans refuse before owned storage or untrusted method reads",()=>{expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);for(const kind of law.cases){const owner=new RetainedWorkBuffer<number>(8),funding=wallet();let touched=0;owning(owner,funding,()=>{const actual=run(funding,(work,real)=>{const token=kind==="copied-live-loan"?{...real}:Object.freeze({get actualWork(){touched++;return()=>0;},get commitActualWork(){touched++;return()=>{};}});return Reflect.apply(work.slot,work,[owner,token]);});expect(actual.error).toBeInstanceOf(Error);expect(actual.work.receipt).toEqual(zero());expect(actual.cpu).toBe(law.cpu);expect(owner.capacity).toBe(0);expect(owner.backingFor(0)).toBeUndefined();expect(touched).toBe(0);});}});
