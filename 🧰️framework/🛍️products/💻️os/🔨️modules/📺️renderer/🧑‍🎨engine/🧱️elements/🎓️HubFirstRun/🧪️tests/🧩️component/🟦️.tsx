// #region 🧲️Header
/** @emoji 🎓️ Laws for the hub first-run walkthrough contract and pane (ticket
 * `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END` slice D2, G12 ranked item #12). Anchors are checked
 * against the shared element-id grammar this repo already owns (`ELEMENT_ID_PATTERN`, the same
 * regex `assertElementId` enforces at every call site) and against the `os.hub.` namespace
 * `🔐️HubSignIn`/`🏘️SpaceBrowser` publish — not against a fixture copy of those ids, and deliberately
 * not against their rendered DOM, which would need both panes' full prop surface here. Honest
 * consequence, recorded rather than hidden: renaming a control in those panes does **not** turn this
 * file red, it silently degrades the step to a centered screen step at runtime. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { cleanup, fireEvent, render, screen } from "@semio-tech/ui-react/test";
import { ELEMENT_ID_PATTERN, UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX, readStoredIntroductionSeen, setUiLocale, writeStoredIntroductionSeen } from "@semio-tech/ui-react";
import { afterEach, beforeEach, describe, expect, it } from "vitest";
import {
  HUB_FIRST_RUN_ANCHORS_V1,
  HUB_FIRST_RUN_INTRODUCTION_ID_V1,
  HUB_FIRST_RUN_SEEN_VALUE_V1,
  HUB_FIRST_RUN_STAGES_V1,
  HUB_FIRST_RUN_STORAGE_KEY_V1,
  HUB_FIRST_RUN_TEXT_V1,
  hubFirstRunIntroductionV1,
  hubFirstRunSeenV1,
  hubFirstRunShouldStartV1,
  hubFirstRunStageV1,
  hubFirstRunStateFromV1,
  hubFirstRunStepIndexV1,
  hubFirstRunTextV1,
  markHubFirstRunSeenV1,
  type HubFirstRunStateV1,
} from "../../../../../../📇️directory/🎓️first-run/🟦️.ts";
import type { HubConnectionStorageV1, HubSignInTransportV1 } from "../../../../../../📇️directory/🔐️sign-in/🟦️.ts";
import { HubFirstRun, hubFirstRunStageAtIndex } from "../../🟦️.tsx";
import { HubWorkspace } from "../../../🔗️HubConnection/🏛️workspace/🟦️.tsx";
import type { HubConnectionPortV1 } from "../../../🔗️HubConnection/🟦️.tsx";
// #endregion 🔌️Adapters

//#region 🔌️jsdom polyfills
// `UIIntroduction` measures its anchor and info box through ResizeObserver; jsdom does not implement
// it. Same shim `🧪️tests/🔬️engine-contract` installs for the renderer hosts.
class ResizeObserverMock {
  observe() {}
  unobserve() {}
  disconnect() {}
}
if (!globalThis.ResizeObserver) globalThis.ResizeObserver = ResizeObserverMock as unknown as typeof ResizeObserver;
if (!Element.prototype.scrollIntoView) Element.prototype.scrollIntoView = () => {};
//#endregion 🔌️jsdom polyfills

afterEach(cleanup);
beforeEach(async () => {
  await setUiLocale("en");
});

//#region 🧫️Doubles
function memoryStorage(seed?: string): HubConnectionStorageV1 & { readonly entries: Map<string, string> } {
  const entries = new Map<string, string>();
  if (seed !== undefined) entries.set(HUB_FIRST_RUN_STORAGE_KEY_V1, seed);
  return { entries, getItem: (key) => entries.get(key) ?? null, setItem: (key, value) => void entries.set(key, value) };
}

/** 💥️ A store whose every operation throws — a private-mode browser, a blocked origin, a full quota. */
const throwingStorage: HubConnectionStorageV1 = {
  getItem: () => {
    throw new Error("storage denied");
  },
  setItem: () => {
    throw new Error("storage denied");
  },
};

const SIGNED_OUT: HubFirstRunStateV1 = { signedIn: false, spaceCount: 0, openSpaceId: null, canInvite: false };
const NO_SPACE: HubFirstRunStateV1 = { signedIn: true, spaceCount: 0, openSpaceId: null, canInvite: false };
const IN_SPACE: HubFirstRunStateV1 = { signedIn: true, spaceCount: 1, openSpaceId: "spc_1", canInvite: true };
const VIEWER_IN_SPACE: HubFirstRunStateV1 = { signedIn: true, spaceCount: 1, openSpaceId: "spc_1", canInvite: false };
//#endregion 🧫️Doubles

//#region 🎓️Contract
describe("hub first-run contract", () => {
  it("names five stages in flow order with done last", () => {
    expect([...HUB_FIRST_RUN_STAGES_V1]).toEqual(["welcome", "signIn", "space", "invite", "done"]);
  });

  it("opens on the first thing the person has not done", () => {
    expect(hubFirstRunStageV1(SIGNED_OUT)).toBe("signIn");
    expect(hubFirstRunStageV1(NO_SPACE)).toBe("space");
    expect(hubFirstRunStageV1(IN_SPACE)).toBe("invite");
  });

  it("treats a space the person is a member of but has not opened as the space stage", () => {
    expect(hubFirstRunStageV1({ signedIn: true, spaceCount: 3, openSpaceId: null, canInvite: true })).toBe("space");
  });

  it("never teaches inviting to someone whose role cannot invite", () => {
    expect(hubFirstRunStageV1(VIEWER_IN_SPACE)).toBe("done");
  });

  it("maps every stage to its own step index and back", () => {
    expect(hubFirstRunStepIndexV1(SIGNED_OUT)).toBe(1);
    expect(hubFirstRunStepIndexV1(NO_SPACE)).toBe(2);
    expect(hubFirstRunStepIndexV1(IN_SPACE)).toBe(3);
    expect(hubFirstRunStepIndexV1(VIEWER_IN_SPACE)).toBe(4);
    for (const [index, stage] of HUB_FIRST_RUN_STAGES_V1.entries()) expect(hubFirstRunStageAtIndex(index)).toBe(stage);
  });

  it("answers done for an index past the end instead of throwing", () => {
    expect(hubFirstRunStageAtIndex(99)).toBe("done");
    expect(hubFirstRunStageAtIndex(-1)).toBe("done");
  });

  it("builds one step per stage, in stage order, for both locales", () => {
    for (const locale of ["en", "de"] as const) {
      const introduction = hubFirstRunIntroductionV1(locale);
      expect(introduction.steps.map((entry) => entry.id)).toEqual(HUB_FIRST_RUN_STAGES_V1.map((stage) => `hub.firstRun.${stage}`));
    }
  });

  it("gives the two screen steps no anchor and centers them, so they survive a phone viewport", () => {
    const introduction = hubFirstRunIntroductionV1("en");
    for (const id of ["hub.firstRun.welcome", "hub.firstRun.done"]) {
      const entry = introduction.steps.find((candidate) => candidate.id === id);
      expect(entry?.introduce).toBeNull();
      expect(entry?.placement).toBe("center");
      expect(entry?.show).toEqual([]);
    }
  });

  it("gives every task step an anchor, a checklist and no ordering requirement", () => {
    const introduction = hubFirstRunIntroductionV1("en");
    for (const id of ["hub.firstRun.signIn", "hub.firstRun.space", "hub.firstRun.invite"]) {
      const entry = introduction.steps.find((candidate) => candidate.id === id);
      expect(typeof entry?.introduce).toBe("string");
      expect(entry?.interactions.length).toBeGreaterThan(0);
      expect(entry?.ordered).toBe(false);
    }
  });

  it("offers both ways into a space — create and redeem — as unordered alternatives", () => {
    const entry = hubFirstRunIntroductionV1("en").steps.find((candidate) => candidate.id === "hub.firstRun.space");
    expect(entry?.interactions).toHaveLength(2);
    expect(entry?.ordered).toBe(false);
  });

  it("uses only ids the shared element-id grammar accepts", () => {
    const introduction = hubFirstRunIntroductionV1("en");
    const ids = introduction.steps.flatMap((entry) => [
      ...(entry.introduce === null ? [] : [entry.introduce]),
      ...entry.show,
      ...entry.interactions.flatMap((interaction) => ("id" in interaction.on ? [String(interaction.on.id)] : [])),
      ...entry.interactions.flatMap((interaction) => (interaction.celebrate ? [interaction.celebrate] : [])),
    ]);
    expect(ids.length).toBeGreaterThan(0);
    for (const id of ids) expect(ELEMENT_ID_PATTERN.test(id)).toBe(true);
  });

  it("anchors only under the os.hub namespace the two hub panes own", () => {
    for (const anchor of [HUB_FIRST_RUN_ANCHORS_V1.signIn, HUB_FIRST_RUN_ANCHORS_V1.space, HUB_FIRST_RUN_ANCHORS_V1.invite]) {
      expect(anchor.startsWith("os.hub.")).toBe(true);
    }
  });
});
//#endregion 🎓️Contract

//#region 🌐️Locale
describe("hub first-run locale", () => {
  it("publishes exactly the same keys in English and German", () => {
    expect(Object.keys(HUB_FIRST_RUN_TEXT_V1.en).sort()).toEqual(Object.keys(HUB_FIRST_RUN_TEXT_V1.de).sort());
  });

  it("leaves no English string sitting in the German bundle", () => {
    for (const key of Object.keys(HUB_FIRST_RUN_TEXT_V1.en) as (keyof typeof HUB_FIRST_RUN_TEXT_V1.en)[]) {
      expect(HUB_FIRST_RUN_TEXT_V1.de[key]).not.toBe(HUB_FIRST_RUN_TEXT_V1.en[key]);
      expect(HUB_FIRST_RUN_TEXT_V1.de[key].length).toBeGreaterThan(0);
    }
  });

  it("refuses an unowned locale instead of silently falling back to English", () => {
    expect(() => hubFirstRunTextV1("fr")).toThrow("hub.first-run.locale-unsupported");
    expect(() => hubFirstRunIntroductionV1("")).toThrow("hub.first-run.locale-unsupported");
  });

  it("renders the German tour with no English leaking through", () => {
    const introduction = hubFirstRunIntroductionV1("de");
    expect(introduction.title).toBe(HUB_FIRST_RUN_TEXT_V1.de.title);
    const bodies = introduction.steps.map((entry) => String(entry.body));
    for (const body of Object.values(HUB_FIRST_RUN_TEXT_V1.en)) expect(bodies).not.toContain(body);
  });
});
//#endregion 🌐️Locale

//#region 🔖️Seen
describe("hub first-run seen flag", () => {
  it("is unseen on a fresh profile and seen after it is marked", () => {
    const storage = memoryStorage();
    expect(hubFirstRunSeenV1(storage)).toBe(false);
    markHubFirstRunSeenV1(storage);
    expect(storage.entries.get(HUB_FIRST_RUN_STORAGE_KEY_V1)).toBe(HUB_FIRST_RUN_SEEN_VALUE_V1);
    expect(hubFirstRunSeenV1(storage)).toBe(true);
  });

  it("reads unseen from a foreign value rather than trusting any non-empty string", () => {
    expect(hubFirstRunSeenV1(memoryStorage("yes"))).toBe(false);
  });

  it("survives a store with no handle and a store that throws", () => {
    expect(hubFirstRunSeenV1(null)).toBe(false);
    expect(hubFirstRunSeenV1(throwingStorage)).toBe(false);
    expect(() => markHubFirstRunSeenV1(null)).not.toThrow();
    expect(() => markHubFirstRunSeenV1(throwingStorage)).not.toThrow();
  });

  it("never auto-starts while the hub surface is closed, even on a fresh profile", () => {
    expect(hubFirstRunShouldStartV1(memoryStorage(), false)).toBe(false);
    expect(hubFirstRunShouldStartV1(memoryStorage(), true)).toBe(true);
    expect(hubFirstRunShouldStartV1(memoryStorage(HUB_FIRST_RUN_SEEN_VALUE_V1), true)).toBe(false);
  });

  // 🎓️ The whole point of restating the key instead of importing the React helpers: these four laws
  // are what stop the restatement drifting. They compare this pure module against the *actual*
  // `ui.introduction.seen.` machinery every other tour uses, in both directions.
  it("writes into the same key space every other introduction uses", () => {
    expect(HUB_FIRST_RUN_STORAGE_KEY_V1).toBe(`${UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX}${HUB_FIRST_RUN_INTRODUCTION_ID_V1}`);
  });

  it("is seen by the shared reader after this module marks it", () => {
    const entries = new Map<string, string>();
    markHubFirstRunSeenV1({ getItem: (key) => entries.get(key) ?? null, setItem: (key, value) => void entries.set(key, value) });
    const sharedPort = { get: (key: string) => entries.get(key) ?? null, set: (key: string, value: string) => void entries.set(key, value) };
    expect(readStoredIntroductionSeen(sharedPort as never, HUB_FIRST_RUN_INTRODUCTION_ID_V1)).toBe(true);
  });

  it("sees a flag the shared writer wrote", () => {
    const entries = new Map<string, string>();
    const sharedPort = { get: (key: string) => entries.get(key) ?? null, set: (key: string, value: string) => void entries.set(key, value) };
    writeStoredIntroductionSeen(sharedPort as never, HUB_FIRST_RUN_INTRODUCTION_ID_V1);
    expect(hubFirstRunSeenV1({ getItem: (key) => entries.get(key) ?? null, setItem: () => {} })).toBe(true);
  });

  it("does not collide with another app's introduction flag", () => {
    const entries = new Map<string, string>();
    const sharedPort = { get: (key: string) => entries.get(key) ?? null, set: (key: string, value: string) => void entries.set(key, value) };
    writeStoredIntroductionSeen(sharedPort as never, "note");
    expect(hubFirstRunSeenV1({ getItem: (key) => entries.get(key) ?? null, setItem: () => {} })).toBe(false);
  });
});
//#endregion 🔖️Seen

//#region 🧭️Projection
describe("hub first-run state projection", () => {
  const rows = [
    { id: "spc_owned", access: "author" },
    { id: "spc_guest", access: "member" },
  ];

  it("reads signed-in from the session phase and nothing else", () => {
    expect(hubFirstRunStateFromV1("signed-in", [], null).signedIn).toBe(true);
    for (const phase of ["signed-out", "signing-in", "expired", "signing-out"]) expect(hubFirstRunStateFromV1(phase, [], null).signedIn).toBe(false);
  });

  it("counts every space the person can see", () => {
    expect(hubFirstRunStateFromV1("signed-in", rows, null).spaceCount).toBe(2);
    expect(hubFirstRunStateFromV1("signed-in", [], null).spaceCount).toBe(0);
  });

  it("reports no open space when the active id is not among the rows", () => {
    expect(hubFirstRunStateFromV1("signed-in", rows, "spc_missing").openSpaceId).toBeNull();
  });

  it("scopes invite capability to the OPEN space once one is open", () => {
    expect(hubFirstRunStateFromV1("signed-in", rows, "spc_owned").canInvite).toBe(true);
    expect(hubFirstRunStateFromV1("signed-in", rows, "spc_guest").canInvite).toBe(false);
  });

  it("falls back to any authored space when none is open, so an owner still reaches the invite step", () => {
    expect(hubFirstRunStateFromV1("signed-in", rows, null).canInvite).toBe(true);
    expect(hubFirstRunStateFromV1("signed-in", [{ id: "spc_guest", access: "member" }], null).canInvite).toBe(false);
  });

  it("drives the stage a live workspace would open on", () => {
    expect(hubFirstRunStageV1(hubFirstRunStateFromV1("signed-out", [], null))).toBe("signIn");
    expect(hubFirstRunStageV1(hubFirstRunStateFromV1("signed-in", [], null))).toBe("space");
    expect(hubFirstRunStageV1(hubFirstRunStateFromV1("signed-in", rows, "spc_owned"))).toBe("invite");
    expect(hubFirstRunStageV1(hubFirstRunStateFromV1("signed-in", rows, "spc_guest"))).toBe("done");
  });
});
//#endregion 🧭️Projection

//#region 🖥️Pane
describe("hub first-run pane", () => {
  it("opens on the sign-in step for a signed-out person", () => {
    render(<HubFirstRun locale="en" state={SIGNED_OUT} onDismiss={() => {}} />);
    expect(screen.getByText(HUB_FIRST_RUN_TEXT_V1.en.signInTitle)).toBeTruthy();
  });

  it("opens on the invite step for someone already in a space", () => {
    render(<HubFirstRun locale="en" state={IN_SPACE} onDismiss={() => {}} />);
    expect(screen.getByText(HUB_FIRST_RUN_TEXT_V1.en.inviteTitle)).toBeTruthy();
  });

  it("renders German when asked for German", () => {
    render(<HubFirstRun locale="de" state={SIGNED_OUT} onDismiss={() => {}} />);
    expect(screen.getByText(HUB_FIRST_RUN_TEXT_V1.de.signInTitle)).toBeTruthy();
    expect(screen.queryByText(HUB_FIRST_RUN_TEXT_V1.en.signInTitle)).toBeNull();
  });

  it("honours an explicit entry step over the derived one, so a replay can restart at the beginning", () => {
    render(<HubFirstRun locale="en" state={IN_SPACE} onDismiss={() => {}} initialStepIndex={0} />);
    expect(screen.getByText(HUB_FIRST_RUN_TEXT_V1.en.welcomeTitle)).toBeTruthy();
  });

  it("reports a skip as not completed", () => {
    const seen: boolean[] = [];
    render(<HubFirstRun locale="en" state={SIGNED_OUT} onDismiss={(completed) => seen.push(completed)} />);
    const skip = document.querySelector('[id="ui.introduction.skip"]');
    expect(skip).toBeTruthy();
    fireEvent.click(skip as Element);
    expect(seen).toEqual([false]);
  });

  it("advances an informational step with the button", () => {
    render(<HubFirstRun locale="en" state={SIGNED_OUT} onDismiss={() => {}} initialStepIndex={0} />);
    expect(screen.getByText(HUB_FIRST_RUN_TEXT_V1.en.welcomeTitle)).toBeTruthy();
    fireEvent.click(document.querySelector('[id="ui.introduction.next"]') as Element);
    expect(screen.getByText(HUB_FIRST_RUN_TEXT_V1.en.signInTitle)).toBeTruthy();
  });

  it("reports the terminal step as completed", () => {
    const seen: boolean[] = [];
    render(<HubFirstRun locale="en" state={SIGNED_OUT} onDismiss={(completed) => seen.push(completed)} initialStepIndex={HUB_FIRST_RUN_STAGES_V1.length - 1} />);
    expect(screen.getByText(HUB_FIRST_RUN_TEXT_V1.en.doneTitle)).toBeTruthy();
    fireEvent.click(document.querySelector('[id="ui.introduction.next"]') as Element);
    expect(seen).toEqual([true]);
  });

  it("keeps the person on their step when the hub state changes underneath them", () => {
    const { rerender } = render(<HubFirstRun locale="en" state={SIGNED_OUT} onDismiss={() => {}} />);
    expect(screen.getByText(HUB_FIRST_RUN_TEXT_V1.en.signInTitle)).toBeTruthy();
    rerender(<HubFirstRun locale="en" state={IN_SPACE} onDismiss={() => {}} />);
    expect(screen.getByText(HUB_FIRST_RUN_TEXT_V1.en.signInTitle)).toBeTruthy();
  });
});
//#endregion 🖥️Pane

//#region 🏛️Mount
/** 🏛️ The tour as it actually reaches a person: mounted inside the real `HubWorkspace`, not rendered
 * standalone. These are the laws that would have stayed green while the feature shipped disabled. */
describe("hub first-run inside the hub workspace", () => {
  const idleTransport: HubSignInTransportV1 = {
    mint: async () => ({ status: 503, body: "", retryAfterHeader: null }),
    read: async () => ({ status: 401, body: "", retryAfterHeader: null }),
    end: async () => ({ status: 204, body: "", retryAfterHeader: null }),
  };

  function workspacePort(storage: HubConnectionStorageV1 | null): HubConnectionPortV1 {
    return {
      signIn: idleTransport,
      storage,
      bootstrapOrigin: "http://127.0.0.1:7777",
      deviceInstanceId: "device-d2",
      clientClass: "browser",
      listSpaces: async () => [],
      readSpaceMembers: async () => [],
      submitCommand: async () => {
        throw new Error("unused");
      },
      redeemInvite: async () => ({ status: 200 }),
    } as HubConnectionPortV1;
  }

  function mount(storage: HubConnectionStorageV1 | null) {
    return render(<HubWorkspace port={workspacePort(storage)} locale="en" activeSpaceId={null} onlineUserIds={[]} onOpenSpace={() => {}} onClose={() => {}} />);
  }

  /** 🎯️ The tour's own step title, scoped to the introduction info box — the workspace also renders a
   * "Sign in" submit button, so a bare text query is ambiguous by construction. */
  function tourStepTitle(): string | null {
    return document.querySelector('[data-slot="introduction-info-box-title"]')?.textContent ?? null;
  }

  it("auto-starts on a fresh profile, at the sign-in step", () => {
    mount(memoryStorage());
    expect(tourStepTitle()).toBe(HUB_FIRST_RUN_TEXT_V1.en.signInTitle);
  });

  it("does not auto-start for someone who already saw it", () => {
    mount(memoryStorage(HUB_FIRST_RUN_SEEN_VALUE_V1));
    expect(tourStepTitle()).toBeNull();
  });

  it("does not persist a dismissal it cannot store, but still closes for this session", () => {
    mount(null);
    expect(tourStepTitle()).toBe(HUB_FIRST_RUN_TEXT_V1.en.signInTitle);
    fireEvent.click(document.querySelector('[id="ui.introduction.skip"]') as Element);
    expect(tourStepTitle()).toBeNull();
  });

  it("persists dismissal into the shared introduction key, so it stays closed next time", () => {
    const storage = memoryStorage();
    mount(storage);
    fireEvent.click(document.querySelector('[id="ui.introduction.skip"]') as Element);
    expect(tourStepTitle()).toBeNull();
    expect(storage.entries.get(HUB_FIRST_RUN_STORAGE_KEY_V1)).toBe(HUB_FIRST_RUN_SEEN_VALUE_V1);
    cleanup();
    mount(storage);
    expect(tourStepTitle()).toBeNull();
  });

  it("offers a help affordance that re-opens the tour from the beginning", () => {
    const storage = memoryStorage(HUB_FIRST_RUN_SEEN_VALUE_V1);
    mount(storage);
    const replay = document.querySelector('[id="os.hub.firstRun.replay"]');
    expect(replay).toBeTruthy();
    fireEvent.click(replay as Element);
    expect(tourStepTitle()).toBe(HUB_FIRST_RUN_TEXT_V1.en.welcomeTitle);
  });

  it("gives the help affordance an accessible name in both locales", async () => {
    mount(memoryStorage(HUB_FIRST_RUN_SEEN_VALUE_V1));
    expect((document.querySelector('[id="os.hub.firstRun.replay"]') as HTMLElement).getAttribute("aria-label")).toBe("How this works");
    cleanup();
    await setUiLocale("de");
    mount(memoryStorage(HUB_FIRST_RUN_SEEN_VALUE_V1));
    expect((document.querySelector('[id="os.hub.firstRun.replay"]') as HTMLElement).getAttribute("aria-label")).toBe("So funktioniert das");
    await setUiLocale("en");
  });

  it("reaches the help affordance as a real button, so keyboard and screen readers get it", () => {
    mount(memoryStorage(HUB_FIRST_RUN_SEEN_VALUE_V1));
    const replay = document.querySelector('[id="os.hub.firstRun.replay"]') as HTMLElement;
    expect(replay.tagName).toBe("BUTTON");
    expect(replay.getAttribute("disabled")).toBeNull();
  });
});
//#endregion 🏛️Mount
