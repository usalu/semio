/** 🔶️ The places observed card cursors tests of the 👁️ what the screens show suite. */
import { render, screen, within } from "@testing-library/react";
import { expect, it } from "vitest";
import { learnerTag, type CursorState, type Place } from "@semio-tech/course";
import { CardPresence, PresenceOverlay, PresenceProvider, placePeers, presenceView, courseText } from "@semio-tech/course-react";
import { defaultCursorLightSvg as semioCursor } from "@semio-tech/assets/cursor/default/light";
import vectors from "../../🧫️fixtures/📡️presence-client/🔣️.json";
import { BEN, HOME_ROSTER, ME, MIRA, ROSTER, box, presenceOf, snapshot, text } from "../../🧰️presence-client-kit/🟦️.tsx";

/** 🔷 Registers the places observed card cursors tests of the 👁️ what the screens show suite. */
export function registerPlacesObservedCardCursorsTests(): void {
  for (const vector of vectors.cardActivity) {
    it(`shows card activity: ${vector.id}`, () => {
      const roster = [{ session: "s-me", colour: 0, state: presenceOf(ME, "Ada", { screen: "home" }) }, { session: "s-mira", colour: 3, state: presenceOf(MIRA, "Mira K.", vector.place as Place, vector.active) }];
      const member = { tag: learnerTag(MIRA), ...("cursor" in vector ? { cursor: vector.cursor } : {}), ...("focus" in vector ? { focus: vector.focus } : {}) } as CursorState;
      const view = presenceView(snapshot(roster, [ { session: "r-mira", colour: 3, state: member } ]), courseText("en"));
      render(<PresenceProvider view={view} setTask={() => undefined}><CardPresence anchor={vector.anchor} accepts={(place) => place.quiz === "heating"} text={courseText("en")} /></PresenceProvider>);
      expect(document.querySelector(`[data-card-presence] [data-tag="${learnerTag(MIRA)}"]`) !== null).toBe(vector.expected);
    });
  }
  it("places observed card cursors and focus on the visible grid without using hidden owner or preview anchors", () => {
    const { container } = render(<>
      <div hidden inert><div data-presence-anchor="home:board">Held owner</div></div>
      <div data-follow-screen="home">
        <div data-course-grid="root" data-grid-active="">
          <div data-layered-pane="preview" inert><div data-presence-anchor="home:board">Preview</div></div>
          <div data-layered-overlay="" inert><div data-presence-anchor="home:board">Observed card</div></div>
        </div>
      </div>
      <div data-presence-layer="">
        <div data-peer="cursor" data-anchor="home:board" data-x="0.5" data-y="0.5" hidden />
        <div data-peer="focus" data-anchor="home:board" hidden />
      </div>
    </>);
    const anchors = [...container.querySelectorAll<HTMLElement>("[data-presence-anchor]")];
    box(anchors[0]!, { left: 0, top: 0, width: 400, height: 400 });
    box(anchors[1]!, { left: 900, top: 900, width: 400, height: 400 });
    box(anchors[2]!, { left: 50, top: 250, width: 200, height: 100 });
    const layer = container.querySelector<HTMLElement>("[data-presence-layer]")!;
    const [cursor, focus] = [...layer.children] as HTMLElement[];
    placePeers(layer);
    expect([cursor!.hidden, cursor!.style.transform]).toEqual([false, "translate(150px, 300px)"]);
    expect([focus!.hidden, focus!.style.transform, focus!.style.width, focus!.style.height]).toEqual([false, "translate(50px, 250px)", "200px", "100px"]);
    container.querySelector("[data-grid-active]")!.removeAttribute("data-grid-active");
    placePeers(layer);
    expect([cursor!.hidden, focus!.hidden]).toEqual([true, true]);
  });
  it("maps viewport cursors over gaps above cards and scales them inside preview pages", () => {
    const layer = document.createElement("div");
    layer.innerHTML = '<div data-peer="cursor" data-anchor="viewport" data-x="0.25" data-y="0.75" hidden></div>';
    document.body.append(layer);
    const mark = layer.firstElementChild as HTMLElement;
    placePeers(layer);
    expect(mark.hidden).toBe(false);
    expect(mark.style.transform).toBe(`translate(${window.innerWidth / 4}px, ${window.innerHeight * 0.75}px)`);
    const page = document.createElement("div");
    Object.defineProperties(page, { offsetWidth: { value: 800 }, offsetHeight: { value: 600 } });
    page.append(layer);
    document.body.append(page);
    placePeers(layer, page);
    expect(mark.style.transform).toBe("translate(200px, 450px)");
    page.remove();
  });
  it("shows named colour badges for card occupants, hover and keyboard focus, excluding away learners", () => {
    const roster = ROSTER.map((member) => member.state.tag === learnerTag(BEN) ? { ...member, state: { ...member.state, active: true, place: { screen: "home" as const } } } : member);
    const view = presenceView(snapshot(roster, [{ session: "r-ben", colour: 5, state: { tag: learnerTag(BEN), focus: "home:quiz:heating" } }]), courseText("en"));
    const { rerender } = render(<PresenceProvider view={view} setTask={() => undefined}><CardPresence anchor="home:quiz:heating" accepts={(place) => place.quiz === "heating"} text={courseText("en")} /></PresenceProvider>);
    const list = document.querySelector('[data-card-presence="home:quiz:heating"]')!;
    expect(screen.getByRole("list", { name: "Online: 2" })).toBe(list);
    expect(list.getAttribute("role")).toBe("list");
    expect(within(list as HTMLElement).getAllByRole("listitem").map((item) => [item.getAttribute("data-tag"), item.textContent])).toEqual([[learnerTag(BEN), "Ben"], [learnerTag(MIRA), "Mira K."]]);
    expect((list.querySelector(`[data-tag="${learnerTag(MIRA)}"]`) as HTMLElement).style.getPropertyValue("--course-peer")).toBe("var(--presence-3)");
    const away = presenceView(snapshot(roster.map((member) => ({ ...member, state: { ...member.state, active: false } }))), courseText("en"));
    rerender(<PresenceProvider view={away} setTask={() => undefined}><CardPresence anchor="home:quiz:heating" accepts={(place) => place.quiz === "heating"} text={courseText("en")} /></PresenceProvider>);
    expect(document.querySelector("[data-card-presence]")).toBeNull();
  });
  it("keeps root and nested overview viewport cursors on their own public home route", () => {
    const roster = HOME_ROSTER.map((member) => member.state.tag === learnerTag(MIRA) ? { ...member, state: { ...member.state, view: { home: "quizzes" } } } : member);
    const captured = snapshot(roster, [{ session: "r-mira", colour: 3, state: { tag: learnerTag(MIRA), place: { screen: "home" }, home: "quizzes", cursor: { anchor: "viewport", x: 0.5, y: 0.5 } } }]);
    expect(presenceView({ ...captured, place: { screen: "home" } }, courseText("en")).peers).toEqual([]);
    expect(presenceView({ ...captured, place: { screen: "home" }, home: "quizzes" }, courseText("en")).peers.map((peer) => peer.session)).toEqual(["r-mira"]);
    const stale = { ...captured, room: { ...captured.room!, members: captured.room!.members.map((member) => ({ ...member, state: { ...member.state, home: undefined } })) } };
    expect(presenceView({ ...stale, place: { screen: "home" }, home: "quizzes" }, courseText("en")).peers).toEqual([]);
    render(<PresenceProvider view={presenceView(captured, courseText("en"))} setTask={() => undefined}><CardPresence anchor="home:quizzes" accepts={(place, view) => place.screen === "home" && view?.home === "quizzes"} text={courseText("en")} /></PresenceProvider>);
    expect(document.querySelector('[data-card-presence="home:quizzes"]')?.textContent).toBe("Mira K.");
  });
  it("filters stale, away, missing-roster and mixed-screen pointers within a shared quiz room", () => {
    const at: Place = { screen: "run", quiz: "heating", task: "heat" };
    const roster = [
      { session: "s-me", colour: 0, state: presenceOf(ME, "Ada", at) },
      { session: "s-mira", colour: 3, state: presenceOf(MIRA, "Mira K.", at) },
      { session: "s-ben", colour: 5, state: presenceOf(BEN, "Ben", { screen: "results", quiz: "heating" }) },
    ];
    const cursor = { anchor: "viewport", x: 0.25, y: 0.5 };
    const captured = snapshot(roster, [
      { session: "r-mira", colour: 3, state: { tag: learnerTag(MIRA), place: at, cursor } },
      { session: "r-stale", colour: 3, state: { tag: learnerTag(MIRA), place: { screen: "quiz", quiz: "heating" }, cursor } },
      { session: "r-ben", colour: 5, state: { tag: learnerTag(BEN), place: { screen: "results", quiz: "heating" }, cursor } },
      { session: "r-unknown", colour: 7, state: { tag: "0badc0de", place: at, cursor } },
    ]);
    expect(presenceView({ ...captured, scopes: { ...captured.scopes, room: "arch/quiz/heating" } }, courseText("en")).peers.map((peer) => peer.session)).toEqual(["r-mira"]);
    const hidden = { ...captured, roster: { ...captured.roster!, members: captured.roster!.members.map((member) => ({ ...member, state: { ...member.state, active: false } })) }, scopes: { ...captured.scopes, room: "arch/quiz/heating" } };
    expect(presenceView(hidden, courseText("en")).peers).toEqual([]);
  });
  it("labels the others by name, colours them by their roster slot everywhere and leaves the own sessions out of the cursors", () => {
    const view = presenceView(
      snapshot(HOME_ROSTER, [
        { session: "r-me", colour: 0, state: { tag: learnerTag(ME), cursor: { anchor: "home:board", x: 0.1, y: 0.1 } } },
        { session: "r-me-other-tab", colour: 1, state: { tag: learnerTag(ME) } },
        { session: "r-mira", colour: 9, state: { tag: learnerTag(MIRA), cursor: { anchor: "home:board", x: 0.5, y: 0.5 } } },
        { session: "r-anon", colour: 7, state: { tag: "0badc0de", focus: "home:learner" } },
      ]),
      courseText("en"),
    );
    expect(view.roster).toMatchObject({ online: 3, active: 3, quizzes: {} });
    expect(view.colours.get(learnerTag(MIRA))).toBe(3);
    expect(view.peers.map((peer) => [peer.label, peer.colour, peer.cursor, peer.focus])).toEqual([
      ["Mira K.", 3, { anchor: "home:board", x: 0.5, y: 0.5 }, undefined],
    ]);
  });
  it("draws the others' pointers and focus frames on their anchors, decoratively, and nothing when switched off", () => {
    const view = presenceView(
      snapshot(HOME_ROSTER, [
        { session: "r-mira", colour: 3, state: { tag: learnerTag(MIRA), cursor: { anchor: "home:board", x: 0.5, y: 0.25 } } },
        { session: "r-ben", colour: 5, state: { tag: learnerTag(BEN), focus: "home:learner" } },
      ]),
      courseText("en"),
    );
    const { rerender } = render(
      <>
        <section data-presence-anchor="home:board">Board</section>
        <section data-presence-anchor="home:learner">Learner</section>
        <PresenceOverlay view={view} show={false} />
      </>,
    );
    expect(document.querySelector("[data-presence-layer]")).toBeNull();
    box(document.querySelector('[data-presence-anchor="home:board"]')!, { left: 400, top: 100, width: 300, height: 200 });
    box(document.querySelector('[data-presence-anchor="home:learner"]')!, { left: 10, top: 20, width: 100, height: 50 });
    rerender(
      <>
        <section data-presence-anchor="home:board">Board</section>
        <section data-presence-anchor="home:learner">Learner</section>
        <PresenceOverlay view={view} show />
      </>,
    );
    const layer = document.querySelector<HTMLElement>("[data-presence-layer]")!;
    expect(layer.getAttribute("aria-hidden")).toBe("true");
    expect(layer.className).toContain("pointer-events-none");
    const cursor = layer.querySelector<HTMLElement>('[data-peer="cursor"]')!;
    expect(cursor.hidden).toBe(false);
    expect(cursor.style.transform).toBe("translate(550px, 150px)");
    expect(cursor.textContent).toBe("Mira K.");
    expect([...cursor.querySelectorAll(".course-peer-arrow path")].map((path) => path.getAttribute("d"))).toEqual([...semioCursor.matchAll(/\sd="([^"]+)"/gu)].map((match) => match[1]));
    expect((cursor.parentElement as HTMLElement).style.getPropertyValue("--course-peer")).toBe("var(--presence-3)");
    const focus = layer.querySelector<HTMLElement>('[data-peer="focus"]')!;
    expect([focus.style.transform, focus.style.width, focus.style.height]).toEqual(["translate(10px, 20px)", "100px", "50px"]);
    expect(focus.textContent).toBe("Ben");
  });
}
