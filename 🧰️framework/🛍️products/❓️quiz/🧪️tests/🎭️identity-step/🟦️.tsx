/** 🎭️ The identity step says what a choice means before it is made — a pseudonym or name is public and is the whole key
 * to its learner, there are no passwords, anonymous progress lives in this browser only, the quizzes are for fun — and
 * explains a refused pseudonym precisely. The client decides with the core's own `normalizeHandle` (the policy the
 * proctor enforces, held to its Rust twin and its oracles by the core's cases) and explains by asking that function
 * about the parts of the handle, so every fault of the shared vectors must be one the core refuses and no accepted
 * handle may have one. A learner is registered by a command and recalled by a read; losing a handle to someone faster
 * asks again and continues as its holder. A sign-up the proctor refuses for now — the network's sign-up allowance is
 * spent, the roster is full — is said as what it is, in both languages (shared vectors): what happened, whether it
 * passes by itself or the operator must act, and how to go on; a spent allowance is never sent again by the client on
 * its own, nor before the wait the proctor named is over, while recalling a pseudonym keeps working.
 *
 * @see ../../🧫️fixtures/🎭️identity-step/🔣️.json
 * @see ../🪪️identity-shapes — the handle policy of both cores
 */

import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { decodeCommandEnvelope, decodeQueryEnvelope, encodeCommandOutcome, encodeQueryResult, type CommandEnvelope, type CommandOutcome, type HttpResponse, type HttpTransport } from "@semio-tech/framework-server";
import { handleActorId, normalizeHandle, type HandleView, type IdentifyLearnerCommand, type Query, type Rejection } from "@semio-tech/quiz";
import { IdentityScreen, ProctorClient, ProctorThrottled, QUIZ_LOCALES, QuizSession, failureProblem, handleFault, handleFaultMessage, localStore, memoryStorageOrigin, quizInstance, quizText, type HandleFault, type SessionFailure } from "@semio-tech/quiz-react";
import step from "../../🧫️fixtures/🎭️identity-step/🔣️.json";

type Locale = (typeof QUIZ_LOCALES)[number];
type FaultVector = { readonly kind: string; readonly length?: number; readonly characters?: readonly (readonly number[])[] };

interface Fixture {
  readonly faults: readonly { readonly name: string; readonly typed: readonly number[]; readonly fault: FaultVector | null }[];
  readonly messages: readonly ({ readonly fault: FaultVector } & Readonly<Record<Locale, string>>)[];
  readonly refusals: { readonly allowance: string; readonly vectors: readonly ({ readonly id: string; readonly failure: unknown } & Readonly<Record<Locale, string>>)[] };
}

const fixture: Fixture = step;
const TENANT = "identity";
const TIMING = { minMs: 1, maxMs: 4 };
const encoder = new TextEncoder();
const decoder = new TextDecoder();
const typed = (points: readonly number[]): string => String.fromCodePoint(...points);

function fault(vector: FaultVector | null): HandleFault | undefined {
  if (vector === null) return undefined;
  if (vector.kind === "characters") return { kind: "characters", characters: (vector.characters ?? []).map(typed) };
  if (vector.kind === "long") return { kind: "long", length: vector.length ?? 0 };
  return { kind: vector.kind as "empty" | "letter" };
}

function reply(status: number, body: unknown): HttpResponse {
  const payload = JSON.stringify(body);
  return { status, text: async () => payload, bytes: async () => encoder.encode(payload) };
}

/** 🛂️ A proctor that answers the two calls of an identification from a script and records them. */
function scriptedProctor(script: { readonly handle: (handle: string) => HandleView | Rejection; readonly register: (command: IdentifyLearnerCommand, envelope: CommandEnvelope) => Rejection | undefined }, now?: () => number) {
  const calls: string[] = [];
  const envelopes: CommandEnvelope[] = [];
  const transport: HttpTransport = {
    send: async (request) => {
      if (request.method === "GET" && request.path === "/instance") return reply(200, quizInstance());
      const body = JSON.parse(typeof request.body === "string" ? request.body : decoder.decode(request.body)) as unknown;
      if (request.path === "/queries") {
        const query = JSON.parse(decoder.decode(decodeQueryEnvelope(body).arguments)) as Query;
        if (query.type !== "handle") return reply(404, { kind: "notFound", message: query.type });
        calls.push(`handle:${query.handle}`);
        const answer = script.handle(query.handle);
        if (typeof answer === "string") return reply(400, { kind: "invalid", message: `${answer}: the handle is outside the policy` });
        return reply(200, encodeQueryResult({ kind: "snapshot", value: encoder.encode(JSON.stringify(answer)), frontier: null }));
      }
      const envelope = decodeCommandEnvelope(body);
      envelopes.push(envelope);
      const command = JSON.parse(decoder.decode(envelope.payload)) as IdentifyLearnerCommand;
      calls.push(`register:${command.identity.kind === "anonymous" ? "anonymous" : command.identity.handle}`);
      const rejection = script.register(command, envelope);
      const receipt = { commandId: envelope.commandId, actor: envelope.target, revision: 1, acceptedAt: envelope.clientHlc };
      const identity = command.identity.kind === "anonymous" ? command.identity : { kind: command.identity.kind, handle: normalizeHandle(command.identity.handle)!.display };
      const outcome: CommandOutcome =
        rejection === undefined
          ? { status: "accepted", receipt, events: [{ stream: envelope.target, seq: 1, hlc: envelope.clientHlc, kind: "quiz.learner-registered", payload: encoder.encode(JSON.stringify({ type: "learner-registered", learner: command.learner, identity, at: 1 })) }], frontier: null }
          : { status: "rejected", receipt, reason: { kind: "invalid", detail: rejection }, notices: [{ code: rejection, message: rejection }] };
      return reply(200, encodeCommandOutcome(outcome));
    },
  };
  const session = new QuizSession({ proctor: new ProctorClient(() => transport, TENANT), store: localStore(memoryStorageOrigin().tab(), TENANT), timing: TIMING, now });
  return { session, calls, envelopes };
}

const signal = (): AbortSignal => new AbortController().signal;
const HOLDER = "b".repeat(32);

describe("🎭️ explaining a refused pseudonym", () => {
  for (const vector of fixture.faults) {
    it(`finds ${vector.fault === null ? "no fault" : `the fault "${vector.fault.kind}"`} in ${vector.name}`, () => {
      const text = typed(vector.typed);
      expect(handleFault(text)).toEqual(fault(vector.fault));
      expect(normalizeHandle(text.normalize("NFC")) === undefined).toBe(vector.fault !== null);
    });
  }

  for (const vector of fixture.messages) {
    it(`says what is wrong with a handle that is "${vector.fault.kind}" in both languages`, () => {
      for (const locale of QUIZ_LOCALES) expect(handleFaultMessage(fault(vector.fault)!, quizText(locale))).toBe(vector[locale]);
    });
  }
});

describe("🎭️ the identity step", () => {
  it("says before the choice that names are public, that there are no passwords, where anonymous progress lives and that it is for fun", () => {
    const told = {
      en: [/shown publicly – on the leaderboard and to the others who are online/u, /There are no passwords: anyone who enters the same pseudonym or name continues as that learner/u, /stays on this device only: it is lost when you switch identity or clear this browser's data/u, /are for fun: the others can see what you answer, by design/u, /Recommended\./u],
      de: [/wird öffentlich angezeigt – in der Rangliste und für die anderen, die online sind/u, /Es gibt keine Passwörter: Wer dasselbe Pseudonym oder denselben Namen eingibt, macht als diese Person weiter/u, /bleibt nur auf diesem Gerät: Er geht verloren, wenn du die Identität wechselst oder die Daten dieses Browsers löschst/u, /sind zum Spaß da: Die anderen können sehen, was du antwortest/u, /Empfohlen\./u],
    };
    for (const locale of QUIZ_LOCALES) {
      const { session } = scriptedProctor({ handle: (handle) => ({ display: handle }), register: () => undefined });
      const { container, unmount } = render(<IdentityScreen session={session} text={quizText(locale)} />);
      for (const sentence of told[locale]) expect(container.textContent).toMatch(sentence);
      unmount();
    }
  });

  it("refuses a pseudonym outside the policy on the device, says exactly why and keeps the learner in the field", async () => {
    const { session, calls } = scriptedProctor({ handle: (handle) => ({ display: handle }), register: () => undefined });
    const user = userEvent.setup();
    render(<IdentityScreen session={session} text={quizText("en")} />);
    await user.click(screen.getByRole("radio", { name: "Pseudonym" }));
    const field = screen.getByRole("textbox", { name: "Your pseudonym" });
    expect(field.getAttribute("maxlength")).toBeNull();
    expect(document.getElementById(field.getAttribute("aria-describedby")!.split(" ")[0]!)?.textContent).toBe("1 to 64 characters: Latin letters, digits, single spaces between words and ' . _ - (at least one letter or digit).");
    await user.click(field);
    await user.paste("Ada 😀!");
    await user.click(screen.getByRole("button", { name: "Continue" }));
    const alert = screen.getByRole("alert");
    expect(alert.textContent).toBe("Not allowed here: “😀” “!”. Use Latin letters, digits, single spaces between words and ' . _ -");
    expect(field.getAttribute("aria-invalid")).toBe("true");
    expect(field.getAttribute("aria-describedby")?.split(" ")[0]).toBe(alert.id);
    expect(document.activeElement).toBe(field);
    expect(calls).toEqual([]);
    await user.clear(field);
    await user.type(field, "  Ada   Lovelace ");
    await user.click(screen.getByRole("button", { name: "Continue" }));
    await waitFor(() => expect(session.getSnapshot().state.learner?.identity).toEqual({ kind: "pseudonym", handle: "Ada Lovelace" }));
    expect(calls).toEqual(["handle:  Ada   Lovelace ", "register:  Ada   Lovelace "]);
  });
});

describe("🎭️ registering and recalling", () => {
  it("registers an anonymous learner in its own stream and tries a fresh id when the proctor already knows the first", async () => {
    let known = 1;
    const { session, calls, envelopes } = scriptedProctor({ handle: () => "handle-invalid", register: () => (known-- > 0 ? "learner-exists" : undefined) });
    expect(await session.identify({ kind: "anonymous" }, signal())).toBeUndefined();
    expect(calls).toEqual(["register:anonymous", "register:anonymous"]);
    const learners = envelopes.map((envelope) => (JSON.parse(decoder.decode(envelope.payload)) as IdentifyLearnerCommand).learner);
    expect(new Set(learners).size).toBe(2);
    expect(envelopes.map((envelope) => envelope.target)).toEqual(learners.map((learner) => ({ tenant: TENANT, kind: "quiz-learner", id: learner })));
    expect(session.getSnapshot().state.learner).toEqual({ id: learners[1], identity: { kind: "anonymous" } });
  });

  it("recalls the holder of a claimed pseudonym by a read and writes nothing", async () => {
    const { session, calls } = scriptedProctor({ handle: () => ({ display: "Ada", holder: { learner: HOLDER, identity: { kind: "pseudonym", handle: "Ada" } } }), register: () => "handle-claimed" });
    expect(await session.identify({ kind: "name", handle: "ada" }, signal())).toBeUndefined();
    expect(calls).toEqual(["handle:ada"]);
    expect(session.getSnapshot().state.learner).toEqual({ id: HOLDER, identity: { kind: "pseudonym", handle: "Ada" } });
    expect(session.getSnapshot().state.step).toEqual({ screen: "home" });
  });

  it("registers a free pseudonym through the actor of its handle key", async () => {
    const { session, calls, envelopes } = scriptedProctor({ handle: () => ({ display: "Zoë O'Brien" }), register: () => undefined });
    expect(await session.identify({ kind: "pseudonym", handle: "Zoë  O’Brien" }, signal())).toBeUndefined();
    expect(calls).toEqual(["handle:Zoë  O’Brien", "register:Zoë  O’Brien"]);
    expect(envelopes[0]!.target).toEqual({ tenant: TENANT, kind: "quiz-handle", id: handleActorId("zoë o'brien") });
    expect(session.getSnapshot().state.learner?.identity).toEqual({ kind: "pseudonym", handle: "Zoë O'Brien" });
  });

  it("asks again who holds a pseudonym someone else claimed in between, and continues as that learner", async () => {
    let asked = 0;
    const { session, calls } = scriptedProctor({
      handle: () => (asked++ === 0 ? { display: "Ada" } : { display: "Ada", holder: { learner: HOLDER, identity: { kind: "pseudonym", handle: "Ada" } } }),
      register: () => "handle-claimed",
    });
    expect(await session.identify({ kind: "pseudonym", handle: "Ada" }, signal())).toBeUndefined();
    expect(calls).toEqual(["handle:Ada", "register:Ada", "handle:Ada"]);
    expect(session.getSnapshot().state.learner?.id).toBe(HOLDER);
  });

  it("reports what the proctor refuses: a handle outside its policy, a full roster", async () => {
    const refusing = scriptedProctor({ handle: () => "handle-invalid", register: () => undefined });
    expect(await refusing.session.identify({ kind: "pseudonym", handle: "Ada" }, signal())).toEqual({ kind: "rejected", rejection: "handle-invalid" });
    expect(refusing.calls).toEqual(["handle:Ada"]);
    const full = scriptedProctor({ handle: (handle) => ({ display: handle }), register: () => "roster-full" });
    expect(await full.session.identify({ kind: "pseudonym", handle: "Ada" }, signal())).toEqual({ kind: "rejected", rejection: "roster-full" });
    expect(await full.session.identify({ kind: "anonymous" }, signal())).toEqual({ kind: "rejected", rejection: "roster-full" });
    expect(full.session.getSnapshot().state.learner).toBeUndefined();
  });
});

describe("🎭️ a sign-up the proctor refuses for now", () => {
  for (const vector of fixture.refusals.vectors) {
    it(`says in both languages what happened and how to go on: ${vector.id}`, () => {
      for (const locale of QUIZ_LOCALES) expect(failureProblem(vector.failure as SessionFailure, quizText(locale))).toEqual({ message: vector[locale] });
    });
  }

  it("does not send a sign-up again by itself when the network's allowance is spent, asks for none before the wait is over, and still recalls a pseudonym", async () => {
    let now = 1_000_000;
    let spent = true;
    const proctor = scriptedProctor(
      {
        handle: (handle) => (handle === "Ada" ? { display: "Ada", holder: { learner: HOLDER, identity: { kind: "pseudonym", handle: "Ada" } } } : { display: handle }),
        register: () => {
          if (spent) throw new ProctorThrottled(35_912, fixture.refusals.allowance);
          return undefined;
        },
      },
      () => now,
    );
    const { session, calls } = proctor;
    expect(await session.identify({ kind: "anonymous" }, signal())).toEqual({ kind: "waiting", allowance: "sign-up", retryAfterMs: 35_912 });
    expect(calls).toEqual(["register:anonymous"]);
    expect(session.proctor.reachability()).toBe("reachable");
    now += 10_000;
    expect(await session.identify({ kind: "anonymous" }, signal())).toEqual({ kind: "waiting", allowance: "sign-up", retryAfterMs: 25_912 });
    expect(await session.identify({ kind: "pseudonym", handle: "Zoe" }, signal())).toEqual({ kind: "waiting", allowance: "sign-up", retryAfterMs: 25_912 });
    expect(calls).toEqual(["register:anonymous", "handle:Zoe"]);
    expect(session.getSnapshot().state.learner).toBeUndefined();
    now += 25_912;
    spent = false;
    expect(await session.identify({ kind: "pseudonym", handle: "Zoe" }, signal())).toBeUndefined();
    expect(calls).toEqual(["register:anonymous", "handle:Zoe", "handle:Zoe", "register:Zoe"]);
    expect(session.getSnapshot().state.learner?.identity).toEqual({ kind: "pseudonym", handle: "Zoe" });

    const returning = scriptedProctor({ handle: () => ({ display: "Ada", holder: { learner: HOLDER, identity: { kind: "pseudonym", handle: "Ada" } } }), register: () => { throw new ProctorThrottled(35_912, fixture.refusals.allowance); } }, () => now);
    expect(await returning.session.identify({ kind: "anonymous" }, signal())).toMatchObject({ kind: "waiting" });
    expect(await returning.session.identify({ kind: "pseudonym", handle: "Ada" }, signal())).toBeUndefined();
    expect(returning.calls).toEqual(["register:anonymous", "handle:Ada"]);
    expect(returning.session.getSnapshot().state.learner?.id).toBe(HOLDER);
  });

  it("shows the refusal at the form as an alert, with nothing left running and the form ready for the next try", async () => {
    for (const locale of QUIZ_LOCALES) {
      const { session, calls } = scriptedProctor({ handle: (handle) => ({ display: handle }), register: () => { throw new ProctorThrottled(35_912, fixture.refusals.allowance); } });
      const user = userEvent.setup();
      const { container, unmount } = render(<IdentityScreen session={session} text={quizText(locale)} />);
      const submit = screen.getByRole("button", { name: quizText(locale)("quiz.identity.submit") });
      await user.click(submit);
      const alert = await screen.findByRole("alert");
      expect(alert.textContent).toBe(fixture.refusals.vectors[0]![locale]);
      expect(screen.queryByRole("status")).toBeNull();
      expect(container.querySelector("form")?.getAttribute("aria-busy")).toBe("false");
      expect((submit as HTMLButtonElement).disabled).toBe(false);
      expect(calls).toEqual(["register:anonymous"]);
      await user.click(submit);
      await waitFor(() => expect(container.querySelector("form")?.getAttribute("aria-busy")).toBe("false"));
      expect(calls).toEqual(["register:anonymous"]);
      expect(screen.getByRole("alert").textContent).toMatch(locale === "en" ? /try again in about \d+ s/u : /in etwa \d+ s noch einmal/u);
      await user.click(screen.getByRole("radio", { name: quizText(locale)("quiz.identity.pseudonym") }));
      const field = screen.getByRole("textbox", { name: quizText(locale)("quiz.identity.handlePseudonym") });
      await user.type(field, "Zoe");
      await user.click(submit);
      const refused = await screen.findByRole("alert");
      expect(calls).toEqual(["register:anonymous", "handle:Zoe"]);
      expect(field.getAttribute("aria-invalid")).toBe("false");
      expect(field.getAttribute("aria-describedby")?.split(" ")).not.toContain(refused.id);
      unmount();
    }
  });
});
