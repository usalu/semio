/** 🪟️ How the screens show the others (design §17): card activity, cursors and focus frames on their anchors, name labels and colours, the learner card, the quiz cards and the leaderboard, inside the pages behind the cards too; the activity vectors are shared JSON.
 *
 * @see ../🧰️presence-client-kit/🟦️.tsx — the sockets and fixtures
 * @see ../🧫️fixtures/📡️presence-client/🔣️.json
 * @see ../../🎯️targets/⚛️react/🔨️modules/👥️presence/🟦️.tsx
 */

import { fireEvent, render, screen, within } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { cursorProblem, learnerTag, type CursorState, type Place, type ThinkingState } from "@semio-tech/course";
import { LeaderboardCard, CardPresence, LeaderboardPage, LearnerCard, PreferencesPanel, PresenceOverlay, PresenceProvider, QuizCardView, PEER_LABEL_CLEARANCE_PX, placePeers, presenceView, courseText, readPreferences, challengeOf, localStore, memoryStorageOrigin, type PresenceView, type CourseSession } from "@semio-tech/course-react";
import { defaultCursorLightSvg as semioCursor } from "@semio-tech/assets/cursor/default/light";
import { quizStylesheet as stylesheet } from "@semio-tech/course-react/stylesheet";
import vectors from "../🧫️fixtures/📡️presence-client/🔣️.json";
import { BEN, COURSE, HOME_ROSTER, ME, MIRA, ROSTER, STATE, box, pointer, presenceOf, snapshot, stubSession, text } from "../🧰️presence-client-kit/🟦️.tsx";

describe("👁️ what the screens show", () => {
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

  it("gathers the others of every watched room and the drafts of every thinking room, never the own", () => {
    const mira: ThinkingState = { tag: learnerTag(MIRA), answers: { heat: { kind: "classification", assignments: { kettle: "power" } } } };
    const ben: ThinkingState = { tag: learnerTag(BEN), answers: {} };
    const mine: ThinkingState = { tag: learnerTag(ME), answers: {} };
    const view = presenceView(
      snapshot(
        ROSTER.map((member) => member.state.tag === learnerTag(MIRA) ? { ...member, state: { ...member.state, place: { screen: "leaderboard" as const } } } : member),
        [{ session: "r-mira", colour: 3, state: { tag: learnerTag(MIRA), focus: "home:board" } }],
        {
          "arch/leaderboard": [
            { session: "w-mira", colour: 9, state: { tag: learnerTag(MIRA), cursor: { anchor: "leaderboard", x: 0.5, y: 0.5 } } },
            { session: "w-me", colour: 0, state: { tag: learnerTag(ME), cursor: { anchor: "leaderboard", x: 0.1, y: 0.1 } } },
          ],
          "arch/quiz/heating/thinking": [
            { session: "t-mira", colour: 9, state: mira },
            { session: "t-me-other-tab", colour: 0, state: mine },
          ],
        },
        {
          scope: "arch/quiz/cooling/thinking",
          members: [
            { session: "t-ben", colour: 5, state: ben },
            { session: "t-me", colour: 0, state: mine },
          ],
        },
      ),
      courseText("en"),
    );
    expect(view.rooms.get("arch/home")?.map((peer) => peer.label)).toEqual([]);
    expect(view.rooms.get("arch/leaderboard")?.map((peer) => [peer.label, peer.colour, peer.cursor])).toEqual([["Mira K.", 3, { anchor: "leaderboard", x: 0.5, y: 0.5 }]]);
    expect(view.rooms.has("arch/quiz/heating/thinking")).toBe(false);
    expect(view.thinking.get("arch/quiz/heating/thinking")).toEqual([mira]);
    expect(view.thinking.get("arch/quiz/cooling/thinking")).toEqual([ben]);
  });

  it("names the item a learner drags beside the pointer", () => {
    const view = presenceView(snapshot(HOME_ROSTER, [{ session: "r-mira", colour: 3, state: { tag: learnerTag(MIRA), cursor: { anchor: "item:candle", x: 0.5, y: 0.5 }, drag: { item: "candle" } } }]), courseText("en"));
    render(
      <>
        <li data-presence-anchor="item:candle">Tea light</li>
        <PresenceOverlay view={view} show itemLabel={(item) => (item === "candle" ? "Tea light" : undefined)} />
      </>,
    );
    const cursor = document.querySelector<HTMLElement>('[data-presence-layer] [data-peer="cursor"]')!;
    expect(cursor.textContent).toBe("Mira K. ▸ Tea light");
    expect(cursor.dataset.drag).toBe("candle");
  });

  it("places the others inside a page scaled on the overview in the page's own pixels, on its own anchors only", () => {
    const page = document.createElement("div");
    Object.defineProperty(page, "offsetWidth", { value: 800 });
    box(page, { left: 100, top: 50, width: 400, height: 300 });
    const anchor = document.createElement("section");
    anchor.dataset.presenceAnchor = "leaderboard";
    box(anchor, { left: 150, top: 100, width: 100, height: 50 });
    const layer = document.createElement("div");
    layer.innerHTML = '<div data-peer="cursor" data-anchor="leaderboard" data-x="0.5" data-y="0.5" hidden></div><div data-peer="focus" data-anchor="leaderboard" hidden></div><div data-peer="cursor" data-anchor="badges" data-x="0" data-y="0"></div>';
    page.append(anchor, layer);
    const elsewhere = document.createElement("section");
    elsewhere.dataset.presenceAnchor = "badges";
    document.body.append(page, elsewhere);
    placePeers(layer, page);
    const [cursor, focus, outside] = [...layer.querySelectorAll<HTMLElement>("[data-peer]")];
    expect([cursor!.hidden, cursor!.style.transform]).toEqual([false, "translate(200px, 150px)"]);
    expect([focus!.style.transform, focus!.style.width, focus!.style.height]).toEqual(["translate(100px, 100px)", "200px", "100px"]);
    expect(outside!.hidden).toBe(true);
    page.remove();
    elsewhere.remove();
  });

  it("hides a name label that would cover the focused control or come closer than its clearance, and shows it again elsewhere", () => {
    const anchor = document.createElement("section");
    anchor.dataset.presenceAnchor = "task:masses";
    box(anchor, { left: 0, top: 0, width: 1000, height: 600 });
    const control = document.createElement("button");
    box(control, { left: 400, top: 300, width: 120, height: 24 });
    const layer = document.createElement("div");
    layer.innerHTML = '<div data-peer="cursor" data-anchor="task:masses" data-x="0.41" data-y="0.5" hidden><span class="course-peer-label">Mira K.</span></div><div data-peer="cursor" data-anchor="task:masses" data-x="0.9" data-y="0.9" hidden><span class="course-peer-label">Ben</span></div>';
    for (const label of layer.querySelectorAll(".course-peer-label")) {
      Object.defineProperty(label, "offsetWidth", { value: 60 });
      Object.defineProperty(label, "offsetHeight", { value: 18 });
    }
    document.body.append(anchor, control, layer);
    const [near, far] = [...layer.querySelectorAll<HTMLElement>("[data-peer]")];
    placePeers(layer);
    expect([near!.hasAttribute("data-shy"), far!.hasAttribute("data-shy")]).toEqual([false, false]);
    control.focus();
    placePeers(layer);
    expect([near!.hidden, near!.hasAttribute("data-shy"), far!.hasAttribute("data-shy")]).toEqual([false, true, false]);
    near!.dataset.x = String((400 - 16 - 60 - PEER_LABEL_CLEARANCE_PX - 2) / 1000);
    placePeers(layer);
    expect(near!.hasAttribute("data-shy")).toBe(false);
    near!.dataset.x = String((400 - 16 - 60 - PEER_LABEL_CLEARANCE_PX + 2) / 1000);
    placePeers(layer);
    expect(near!.hasAttribute("data-shy")).toBe(true);
    control.blur();
    placePeers(layer);
    expect(near!.hasAttribute("data-shy")).toBe(false);
    expect(stylesheet).toMatch(/\[data-shy\] > \.course-peer-label \{\s*visibility: hidden;/u);
    for (const element of [anchor, control, layer]) element.remove();
  });

  it("shows who is where on the learner card, learners in a quiz on its card and online marks in the leaderboard", () => {
    const view: PresenceView = presenceView(snapshot(ROSTER), courseText("en"));
    const common = { session: stubSession() as unknown as CourseSession, state: STATE, text: courseText("en"), locale: "en" as const };
    const card = { revealed: false, onOpen: () => undefined };
    render(
      <PresenceProvider view={view} setTask={() => undefined}>
        <LearnerCard {...common} {...card} />
        {COURSE.quizzes.map((quiz) => (
          <QuizCardView key={quiz.id} {...common} {...card} quiz={quiz} busy={false} act={() => undefined} challenge="medium" />
        ))}
        <LeaderboardCard {...common} {...card} />
        <LeaderboardPage {...common} view={{ opened: true, revealed: false }} />
      </PresenceProvider>,
    );
    const learner = screen.getByRole("region", { name: "Ada" });
    expect(learner.getAttribute("data-presence-anchor")).toBe("home:learner");
    const list = within(learner).getByRole("group");
    expect(within(list).getByText("Online: 3 · Who is where")).toBeTruthy();
    expect(
      within(list)
        .getAllByRole("listitem")
        .map((item) => item.textContent),
    ).toEqual(["onlineAda (you)– on the overview", "onlineBen– on the leaderboard (away)", "onlineMira K.– working on Quiz heating"]);
    const heating = screen.getByRole("region", { name: "Quiz heating" });
    expect(heating.getAttribute("data-presence-anchor")).toBe("home:quiz:heating");
    expect(within(heating).getByText("Learning now: 1").getAttribute("aria-hidden")).toBeNull();
    const nobody = within(screen.getByRole("region", { name: "Quiz cooling" })).getByText("Learning now: 0");
    expect([nobody.getAttribute("aria-hidden"), nobody.classList.contains("invisible")]).toEqual(["true", true]);
    const table = screen.getAllByRole("table")[1]!;
    const names = within(table)
      .getAllByRole("rowheader")
      .map((cell) => [cell.textContent, cell.querySelector(".course-online") === null ? "offline" : "online"]);
    expect(names).toEqual([
      ["Mira K.online", "online"],
      ["Benonline", "online"],
      ["Ada (you)online", "online"],
    ]);
    for (const anchor of document.querySelectorAll<HTMLElement>("[data-presence-anchor]")) expect(cursorProblem({ tag: learnerTag(ME), focus: anchor.dataset.presenceAnchor })).toBeUndefined();
  });

  it("keeps the cursor preference on the device, shown unless switched off", () => {
    const area = memoryStorageOrigin().tab();
    const store = localStore(area, "arch");
    expect(readPreferences(store).showCursors).toBe(true);
    store.write("preferences", { theme: "dark", textSize: "large", showCursors: false });
    expect(readPreferences(store)).toMatchObject({ theme: "dark", textSize: "large", showCursors: false });
    const onChange = vi.fn();
    render(<PreferencesPanel preferences={readPreferences(store)} locale="de" text={courseText("de")} onChange={onChange} />);
    const box = screen.getByRole("checkbox", { name: "Cursor der anderen anzeigen" }) as HTMLInputElement;
    expect(box.checked).toBe(false);
    fireEvent.click(box);
    expect(onChange).toHaveBeenLastCalledWith({ theme: "dark", textSize: "large", showCursors: true, others: "submitted", animateIcons: true, iconsChosen: false, pets: "lively", petsLiveliness: "lively", petsChosen: false, petsPlay: true, petsMischief: true, challenges: {}, locale: undefined });
    store.write("preferences", { challenges: { physics: "expert", heating: "easy" } });
    expect(readPreferences(store).challenges).toEqual({ physics: "expert", heating: "easy" });
    expect(["physics", "heating", "cooling"].map((quiz) => challengeOf(readPreferences(store), quiz))).toEqual(["expert", "easy", "medium"]);
    store.write("preferences", { challenges: { physics: "lenient", "Not A Slug": "hard", "../heating": "hard", cooling: 3, demand: "hard" } });
    expect(readPreferences(store).challenges).toEqual({ demand: "hard" });
    for (const stored of [{}, { challenge: "expert" }, { challenges: "expert" }, { challenges: 3 }, { challenges: ["hard"] }]) {
      store.write("preferences", stored);
      expect(readPreferences(store).challenges, JSON.stringify(stored)).toEqual({});
    }
    expect(screen.queryByRole("group", { name: "Herausforderung" })).toBeNull();
  });
});
