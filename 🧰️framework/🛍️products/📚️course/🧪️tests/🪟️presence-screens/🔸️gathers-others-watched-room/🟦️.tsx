/** 🔸️ The gathers others watched room tests of the 👁️ what the screens show suite. */
import { fireEvent, render, screen, within } from "@testing-library/react";
import { expect, it, vi } from "vitest";
import { cursorProblem, learnerTag, type ThinkingState } from "@semio-tech/course";
import { LeaderboardCard, LeaderboardPage, LearnerCard, PreferencesPanel, PresenceOverlay, PresenceProvider, QuizCardView, PEER_LABEL_CLEARANCE_PX, placePeers, presenceView, courseText, readPreferences, challengeOf, localStore, memoryStorageOrigin, type PresenceView, type CourseSession } from "@semio-tech/course-react";
import { quizStylesheet as stylesheet } from "@semio-tech/course-react/stylesheet";
import { BEN, COURSE, HOME_ROSTER, ME, MIRA, ROSTER, STATE, box, snapshot, stubSession, text } from "../../🧰️presence-client-kit/🟦️.tsx";

/** 🔹 Registers the gathers others watched room tests of the 👁️ what the screens show suite. */
export function registerGathersOthersWatchedRoomTests(): void {
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
}
