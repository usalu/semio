/** 📬️ Independent XState custody oracle for the native ACK regression corpus. */
import { expect, test } from "bun:test";
import Ajv from "ajv";
import { assign, createActor, createMachine } from "xstate";
import fixture from "../🧫️fixtures/📬️ack/🔣️.json";
import schema from "../🧫️fixtures/📬️ack/📋️schema.json";

type Row = (typeof fixture.cases)[number];
const exactToken = (row: Row): boolean => (["receiver", "operation", "generation", "sequence", "attempt"] as const).every(key => row.initialToken[key] === row.ackToken[key]);

test("validates the permanent native ACK corpus with independent Ajv", () => {
    const validate = new Ajv().compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(new Set(fixture.cases.map(row => row.name)).size).toBe(fixture.cases.length);
});

for (const row of fixture.cases) test(row.name, () => {
    const refused = { guard: () => !row.sourceMatches, actions: assign({ advanceRefused: () => true }) };
    const funded = () => row.sourceMatches && row.deniedCurrency === "none";
    const actor = createActor(createMachine({
        initial: "AwaitingAck",
        context: { advanceRefused: false },
        states: {
            AwaitingAck: { on: { ACK: [
                { guard: () => exactToken(row) && row.lane === "terminal", target: "RetiringRetained" },
                { guard: () => exactToken(row), target: "PublishingRetained" },
            ], ADVANCE: refused } },
            PublishingRetained: { on: { ADVANCE: [refused, { guard: funded, target: "PublishingEmpty" }] } },
            RetiringRetained: { on: { ADVANCE: [refused, { guard: funded, target: "Retired" }] } },
            PublishingEmpty: {},
            Retired: { type: "final" },
        },
    }));
    actor.start();
    actor.send({ type: "ACK" });
    const acknowledged = actor.getSnapshot();
    const stageAfterAck = acknowledged.matches("AwaitingAck") ? "awaitingAck" : acknowledged.matches("RetiringRetained") ? "retiring" : "publishing";
    const retainedAfterAck = !acknowledged.matches("PublishingEmpty") && !acknowledged.matches("Retired");
    actor.send({ type: "ADVANCE" });
    const advanced = actor.getSnapshot();
    const retainedAfterAdvance = advanced.matches("AwaitingAck") || advanced.matches("PublishingRetained") || advanced.matches("RetiringRetained");
    expect({
        ackAccepted: !acknowledged.matches("AwaitingAck"),
        stageAfterAck,
        originalRetainedAfterAck: retainedAfterAck,
        presentedAfterAck: retainedAfterAck,
        originalRetainedAfterAdvance: retainedAfterAdvance,
        presentedAfterAdvance: retainedAfterAdvance,
        samePageAfterAdvance: retainedAfterAdvance,
        queueNextAllowed: advanced.matches("PublishingEmpty"),
        operationMustReachTerminal: advanced.matches("Retired"),
        advanceRefused: advanced.context.advanceRefused,
    }).toEqual(row.expected);
    actor.stop();
});
