import {test,expect} from "bun:test";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/🔣️.json";
import {parseCargoCommandEntryPolicyV1,CargoCommandEntryOwnerV1} from "../🟦️.ts";
import {CargoController} from "../../../🟦️.ts";
import {runCargoCliCommand} from "../../🟦️.ts";

test("actual command entry requires authored scope and finite policy without defaults",()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture.policy)).toBe(true);expect(parseCargoCommandEntryPolicyV1(fixture.policy)).toEqual(fixture.policy);for(const key of fixture.missing){const value={...fixture.policy};delete value[key];expect(validate(value)).toBe(false);expect(()=>parseCargoCommandEntryPolicyV1(value)).toThrow();}for(const maximumElapsedMilliseconds of fixture.invalidDeadlines){const value={...fixture.policy,maximumElapsedMilliseconds};expect(validate(value)).toBe(false);expect(()=>parseCargoCommandEntryPolicyV1(value)).toThrow();}for(const {phase,key} of fixture.oversizedGrants){const value=structuredClone(fixture.policy);value.control[phase][key]++;expect(validate(value)).toBe(false);expect(()=>parseCargoCommandEntryPolicyV1(value)).toThrow();}
});
test("actual command entry owns real deadline progress yielding cancellation and retirement",async()=>{
 const events:string[]=[],owner=new CargoCommandEntryOwnerV1(fixture.policy,line=>events.push(line)),before=owner.remainingMilliseconds();expect(before).toBeGreaterThan(0);await new Promise<void>(accept=>setTimeout(accept,5));expect(owner.remainingMilliseconds()).toBeLessThan(before);const result=await runCargoCliCommand(owner.cli,async()=>{await new CargoController(owner.cli.operation,owner.cli.workspace).step("actual-command","Cargo.toml",1);return 7;});expect(result).toBe(7);expect(events.length).toBeGreaterThan(0);expect(owner.cli.workspace.state).toBe("retired");const refused=new CargoCommandEntryOwnerV1(fixture.policy,line=>events.push(line));refused.cli.cancelDiscovery();await expect(runCargoCliCommand(refused.cli,async()=>7)).rejects.toThrow();expect(refused.cli.workspace.state).toBe("retired");console.log("[DEBUG] actual command owner exercised finite deadline real progress yielding cancellation and funded retirement");
});

test("actual command entry preserves every original finite caller deadline",async()=>{
 const validate=new Ajv({strict:true}).compile(schema);for(const maximumElapsedMilliseconds of fixture.authoredDeadlines){const policy={...fixture.policy,maximumElapsedMilliseconds};expect(validate(policy)).toBe(true);expect(parseCargoCommandEntryPolicyV1(policy)).toEqual(policy);const progress:string[]=[],owner=new CargoCommandEntryOwnerV1(policy,line=>progress.push(line)),remaining=owner.remainingMilliseconds();expect(remaining).toBeGreaterThan(maximumElapsedMilliseconds-1000);expect(remaining).toBeLessThanOrEqual(maximumElapsedMilliseconds);await runCargoCliCommand(owner.cli,async()=>{await new CargoController(owner.cli.operation,owner.cli.workspace).step("original-deadline","Cargo.toml",1);});expect(owner.cli.workspace.state).toBe("retired");expect(progress.length).toBeGreaterThan(0);}console.log("[DEBUG] real original finite caller deadlines preserve all grants and funded retirement");
});
