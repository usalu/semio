/** 🚪️ The presence room in the web client (design §15): joining with the presence subprotocol, rendering its batches, leaving, throttling the frames it sends — with `lodash`'s `throttle` (leading and trailing edge) as the third-party oracle — rejoining after a lost socket and watching other rooms read-only.
 *
 * @see ../🧰️presence-client-kit/🟦️.tsx — the sockets and fixtures
 * @see ../🧫️fixtures/📡️presence-client/🔣️.json
 * @see ../../🎯️targets/⚛️react/🔨️modules/👥️presence/🟦️.tsx
 */

import throttle from "lodash/throttle";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { PRESENCE_HEARTBEAT_MS, PRESENCE_PROTOCOL, PRESENCE_SILENCE_FACTOR } from "@semio-tech/framework-server";
import { cursorProblem, learnerTag, type CursorState } from "@semio-tech/course";
import { MAX_WATCHED_ROOMS, PRESENCE_FRAME_HZ, PRESENCE_FRAME_INTERVAL_MS, PRESENCE_JOIN_MS, PRESENCE_STALL_GRACE_MS, PresenceRoom, WATCH_INTERVAL_MS } from "@semio-tech/course-react";
import vectors from "../🧫️fixtures/📡️presence-client/🔣️.json";
import { BEN, FakeProctorSockets, ME, MIRA, TIMING, snapshot } from "../🧰️presence-client-kit/🟦️.tsx";

describe("👥️ presence room", () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  it("joins with the presence subprotocol, keeps the admitted members of the welcome and sends its state after joining", async () => {
    const proctor = new FakeProctorSockets();
    const room = new PresenceRoom<CursorState>({
      url: "ws://proctor.test/scopes/arch%2Fhome/presence/ws?surface=home",
      connect: proctor.connect,
      parse: (state) => (cursorProblem(state) === undefined ? (state as CursorState) : undefined),
      timing: TIMING,
    });
    room.start();
    const socket = proctor.sockets[0]!;
    expect(socket.protocol).toBe(PRESENCE_PROTOCOL);
    expect(room.getSnapshot().status).toBe("connecting");
    room.publish({ tag: learnerTag(ME) });
    expect(socket.sent).toEqual([]);
    socket.welcome("r-me", 2, [
      { session: "r-mira", colour: 3, surface: "home", state: { cursor: { anchor: "home:board", x: 0.5, y: 0.25 }, tag: learnerTag(MIRA) } },
      { session: "r-new", colour: 4, surface: "home", state: null },
      { session: "r-bad", colour: 5, surface: "home", state: { tag: learnerTag(BEN), cursor: { anchor: "Home Board", x: 2, y: 0 } } },
    ]);
    expect(room.getSnapshot()).toMatchObject({ status: "open", self: "r-me", colour: 2 });
    expect(room.getSnapshot().members).toEqual([{ session: "r-mira", colour: 3, surface: "home", state: { cursor: { anchor: "home:board", x: 0.5, y: 0.25 }, tag: learnerTag(MIRA) } }]);
    expect(socket.sent).toEqual([{ type: "state", state: { tag: learnerTag(ME) } }]);
    room.stop();
    expect(socket.closed).toBe(true);
    expect(room.getSnapshot()).toMatchObject({ status: "stopped", members: [] });
  });

  it("leaves a socket that still connects only once it is open, so no connection is reported failed, and ignores what it says meanwhile", async () => {
    const proctor = new FakeProctorSockets();
    const room = new PresenceRoom<CursorState>({ url: "ws://proctor.test/x", connect: proctor.connect, parse: (state) => (cursorProblem(state) === undefined ? (state as CursorState) : undefined), timing: TIMING });
    room.start();
    const socket = proctor.sockets[0]!;
    socket.readyState = 0;
    room.stop();
    expect(socket.closed).toBe(false);
    expect(room.getSnapshot().status).toBe("stopped");
    socket.open();
    expect(socket.closed).toBe(true);
    socket.welcome("r-late", 1, [{ session: "r-mira", colour: 3, surface: "home", state: { tag: learnerTag(MIRA) } }]);
    expect(room.getSnapshot()).toMatchObject({ status: "stopped", members: [] });
    await vi.advanceTimersByTimeAsync(TIMING.maxMs * 4);
    expect(proctor.sockets).toHaveLength(1);
  });

  it("renders every batch — joins, moves and leaves — and keeps a refusal without closing", async () => {
    const proctor = new FakeProctorSockets();
    const room = new PresenceRoom<CursorState>({ url: "ws://proctor.test/x", connect: proctor.connect, parse: (state) => (cursorProblem(state) === undefined ? (state as CursorState) : undefined) });
    room.start();
    const socket = proctor.sockets[0]!;
    socket.welcome("r-me", 0);
    socket.deliver({ type: "batch", entries: [{ session: "r-mira", colour: 3, surface: "home", state: { tag: learnerTag(MIRA), cursor: { anchor: "home:board", x: 0.1, y: 0.2 } } }], left: [] });
    socket.deliver({
      type: "batch",
      entries: [
        { session: "r-mira", colour: 3, surface: "home", state: { tag: learnerTag(MIRA), cursor: { anchor: "home:learner", x: 0.3, y: 0.4 } } },
        { session: "r-ben", colour: 5, surface: "home", state: { tag: learnerTag(BEN), focus: "home:board" } },
      ],
      left: [],
    });
    expect(room.getSnapshot().members.map((member) => [member.session, member.state])).toEqual([
      ["r-mira", { tag: learnerTag(MIRA), cursor: { anchor: "home:learner", x: 0.3, y: 0.4 } }],
      ["r-ben", { tag: learnerTag(BEN), focus: "home:board" }],
    ]);
    socket.deliver({ type: "batch", entries: [], left: ["r-mira"] });
    expect(room.getSnapshot().members.map((member) => member.session)).toEqual(["r-ben"]);
    socket.deliver({ type: "refused", reason: "tag-invalid /tag" });
    expect(room.getSnapshot()).toMatchObject({ status: "open", refused: "tag-invalid /tag" });
    expect(socket.closed).toBe(false);
    socket.deliver("not json");
    socket.deliver({ type: "surprise" });
    expect(room.getSnapshot().members.map((member) => member.session)).toEqual(["r-ben"]);
  });

  for (const vector of vectors.throttle) {
    it(`sends at most ${PRESENCE_FRAME_HZ} frames a second, latest state first: ${vector.id}`, async () => {
      const proctor = new FakeProctorSockets();
      const room = new PresenceRoom<CursorState>({ url: "ws://proctor.test/x", connect: proctor.connect, parse: () => undefined });
      room.start();
      const socket = proctor.sockets[0]!;
      socket.welcome("r-me", 0);
      const start = Date.now();
      const sent: [number, number][] = [];
      const send = socket.send.bind(socket);
      socket.send = (data) => {
        sent.push([Date.now() - start, (JSON.parse(data) as { readonly state: { readonly cursor: { readonly x: number } } }).state.cursor.x * 100]);
        send(data);
      };
      const oracle: [number, number][] = [];
      const reference = throttle((value: number) => oracle.push([Date.now() - start, value]), PRESENCE_FRAME_INTERVAL_MS);
      for (const [at, value] of vector.calls as [number, number][]) {
        await vi.advanceTimersByTimeAsync(at - (Date.now() - start));
        room.publish({ tag: learnerTag(ME), cursor: { anchor: "home:board", x: value / 100, y: 0 } });
        reference(value);
      }
      await vi.advanceTimersByTimeAsync(1_000);
      expect(sent).toEqual(vector.sent);
      const values = vector.calls.map((call) => call[1]);
      if (new Set(values).size === values.length) expect(sent).toEqual(oracle);
    });
  }

  it("keeps a continuous stream under the rate and delivers the last position", async () => {
    const proctor = new FakeProctorSockets();
    const room = new PresenceRoom<CursorState>({ url: "ws://proctor.test/x", connect: proctor.connect, parse: () => undefined });
    room.start();
    const socket = proctor.sockets[0]!;
    socket.welcome("r-me", 0);
    const times: number[] = [];
    const send = socket.send.bind(socket);
    socket.send = (data) => {
      times.push(Date.now());
      send(data);
    };
    for (let step = 1; step <= 1_000; step += 1) {
      room.publish({ tag: learnerTag(ME), cursor: { anchor: "home:board", x: step / 1_000, y: 0.5 } });
      await vi.advanceTimersByTimeAsync(1);
    }
    await vi.advanceTimersByTimeAsync(1_000);
    expect(times.length).toBeLessThanOrEqual(PRESENCE_FRAME_HZ + 1);
    for (const [index, time] of times.entries()) if (index > 0) expect(time - times[index - 1]!).toBeGreaterThanOrEqual(PRESENCE_FRAME_INTERVAL_MS);
    expect(socket.states().at(-1)).toEqual({ tag: learnerTag(ME), cursor: { anchor: "home:board", x: 1, y: 0.5 } });
  });

  it("rejoins a lost socket with jittered backoff, forgets the members meanwhile and sends the latest state again", async () => {
    vi.spyOn(Math, "random").mockReturnValue(0.5);
    const proctor = new FakeProctorSockets();
    const room = new PresenceRoom<CursorState>({ url: "ws://proctor.test/x", connect: proctor.connect, parse: (state) => (cursorProblem(state) === undefined ? (state as CursorState) : undefined), timing: TIMING, random: () => 0.5 });
    room.start();
    const first = proctor.sockets[0]!;
    first.welcome("r-1", 0, [{ session: "r-mira", colour: 3, surface: "home", state: { tag: learnerTag(MIRA) } }]);
    room.publish({ tag: learnerTag(ME), focus: "home:board" });
    expect(first.states()).toEqual([{ tag: learnerTag(ME), focus: "home:board" }]);
    first.drop();
    expect(room.getSnapshot()).toMatchObject({ status: "waiting", members: [] });
    await vi.advanceTimersByTimeAsync(49);
    expect(proctor.sockets).toHaveLength(1);
    await vi.advanceTimersByTimeAsync(1);
    expect(proctor.sockets).toHaveLength(2);
    proctor.sockets[1]!.drop();
    await vi.advanceTimersByTimeAsync(149);
    expect(proctor.sockets).toHaveLength(2);
    await vi.advanceTimersByTimeAsync(1);
    expect(proctor.sockets).toHaveLength(3);
    const third = proctor.sockets[2]!;
    expect(third.sent).toEqual([]);
    third.welcome("r-3", 1, [{ session: "r-mira", colour: 3, surface: "home", state: { tag: learnerTag(MIRA) } }]);
    expect(third.states()).toEqual([{ tag: learnerTag(ME), focus: "home:board" }]);
    expect(room.getSnapshot()).toMatchObject({ status: "open", self: "r-3" });
    expect(room.getSnapshot().members.map((member) => member.session)).toEqual(["r-mira"]);
    room.stop();
    await vi.advanceTimersByTimeAsync(10_000);
    expect(proctor.sockets).toHaveLength(3);
  });

  it("closes a socket that neither opens, closes nor welcomes within the join time and joins again, however often that happens", async () => {
    vi.spyOn(Math, "random").mockReturnValue(0.5);
    const proctor = new FakeProctorSockets();
    const room = new PresenceRoom<CursorState>({ url: "ws://proctor.test/x", connect: proctor.connect, parse: () => undefined, timing: TIMING, random: () => 0.5, joinMs: 500 });
    room.start();
    const stalled = proctor.sockets[0]!;
    stalled.readyState = 0;
    await vi.advanceTimersByTimeAsync(499);
    expect(stalled.closed).toBe(false);
    await vi.advanceTimersByTimeAsync(1);
    expect(stalled.closed).toBe(true);
    expect(room.getSnapshot().status).toBe("waiting");
    for (let round = 0; round < 5; round += 1) await vi.advanceTimersByTimeAsync(500 + TIMING.maxMs);
    expect(proctor.sockets.length).toBeGreaterThanOrEqual(6);
    expect(proctor.sockets.slice(0, 5).every((socket) => socket.closed)).toBe(true);
    const joined = proctor.sockets.at(-1)!;
    const count = proctor.sockets.length;
    joined.welcome("r-ok", 1);
    await vi.advanceTimersByTimeAsync(PRESENCE_JOIN_MS * 2);
    expect(joined.closed).toBe(false);
    expect(room.getSnapshot()).toMatchObject({ status: "open", self: "r-ok" });
    expect(proctor.sockets).toHaveLength(count);
    room.stop();
  });

  it("gives a socket the whole join time by default and stops asking once the room is left while a socket still connects", async () => {
    const proctor = new FakeProctorSockets();
    const room = new PresenceRoom<CursorState>({ url: "ws://proctor.test/x", connect: proctor.connect, parse: () => undefined, timing: TIMING, random: () => 0.5 });
    room.start();
    proctor.sockets[0]!.readyState = 0;
    await vi.advanceTimersByTimeAsync(PRESENCE_JOIN_MS - 1);
    expect(proctor.sockets[0]!.closed).toBe(false);
    room.stop();
    await vi.advanceTimersByTimeAsync(PRESENCE_JOIN_MS * 3);
    expect(proctor.sockets).toHaveLength(1);
  });

  it("closes a welcomed socket that is silent for 2.5 heartbeats and joins again, while heartbeats and any other frame keep it alive", async () => {
    const proctor = new FakeProctorSockets();
    const room = new PresenceRoom<CursorState>({ url: "ws://proctor.test/x", connect: proctor.connect, parse: () => undefined, timing: TIMING, random: () => 0.5 });
    const silence = PRESENCE_HEARTBEAT_MS * PRESENCE_SILENCE_FACTOR;
    expect(silence).toBe(25_000);
    room.start();
    const first = proctor.sockets[0]!;
    first.welcome("r-1", 0, [{ session: "r-mira", colour: 3, surface: "home", state: { tag: learnerTag(MIRA) } }]);
    for (let beat = 0; beat < 6; beat += 1) {
      await vi.advanceTimersByTimeAsync(PRESENCE_HEARTBEAT_MS);
      first.deliver({ type: "heartbeat" });
    }
    expect(first.closed).toBe(false);
    expect(room.getSnapshot().status).toBe("open");
    await vi.advanceTimersByTimeAsync(silence - 1);
    expect(first.closed).toBe(false);
    await vi.advanceTimersByTimeAsync(1);
    expect(first.closed).toBe(true);
    expect(room.getSnapshot()).toMatchObject({ status: "waiting", members: [] });
    await vi.advanceTimersByTimeAsync(TIMING.maxMs);
    expect(proctor.sockets).toHaveLength(2);
    const second = proctor.sockets[1]!;
    second.welcome("r-2", 1);
    await vi.advanceTimersByTimeAsync(silence - 1);
    second.deliver({ type: "batch", entries: [], left: [] });
    await vi.advanceTimersByTimeAsync(silence - 1);
    expect(second.closed).toBe(false);
    room.stop();
    await vi.advanceTimersByTimeAsync(silence * 3);
    expect(proctor.sockets).toHaveLength(2);
  });

  it("takes a watchdog that fires late for a page that was frozen, not for a dead socket: the frames that waited for the page keep it, one still silent once the page runs is closed", async () => {
    const proctor = new FakeProctorSockets();
    const room = new PresenceRoom<CursorState>({ url: "ws://proctor.test/x", connect: proctor.connect, parse: () => undefined, timing: TIMING, random: () => 0.5 });
    const silence = PRESENCE_HEARTBEAT_MS * PRESENCE_SILENCE_FACTOR;
    room.start();
    const socket = proctor.sockets[0]!;
    socket.welcome("r-1", 0);
    for (let frozen = 0; frozen < 3; frozen += 1) {
      vi.setSystemTime(Date.now() + 120_000);
      await vi.advanceTimersByTimeAsync(silence);
      expect(socket.closed, "the watchdog of a frozen page ran before the frames that waited").toBe(false);
      socket.deliver({ type: "heartbeat" });
    }
    await vi.advanceTimersByTimeAsync(silence - 1);
    expect(socket.closed).toBe(false);
    vi.setSystemTime(Date.now() + 60_000);
    await vi.advanceTimersByTimeAsync(1);
    expect(socket.closed).toBe(false);
    await vi.advanceTimersByTimeAsync(PRESENCE_STALL_GRACE_MS);
    expect(socket.closed, "silent through the grace of a page that runs again").toBe(true);
    expect(proctor.sockets).toHaveLength(1);
    room.stop();
  });

  it("takes a join timer that fires late for a page that was frozen: the welcome that waited for the page joins, one that never comes ends the join once the page runs", async () => {
    const proctor = new FakeProctorSockets();
    const room = new PresenceRoom<CursorState>({ url: "ws://proctor.test/x", connect: proctor.connect, parse: () => undefined, timing: TIMING, random: () => 0.5 });
    room.start();
    const waited = proctor.sockets[0]!;
    vi.setSystemTime(Date.now() + 120_000);
    await vi.advanceTimersByTimeAsync(PRESENCE_JOIN_MS);
    expect(waited.closed, "the join timer of a frozen page ran before the welcome that waited").toBe(false);
    waited.welcome("r-1", 0);
    expect(room.getSnapshot()).toMatchObject({ status: "open", self: "r-1" });
    room.stop();
    const late = new FakeProctorSockets();
    const again = new PresenceRoom<CursorState>({ url: "ws://proctor.test/x", connect: late.connect, parse: () => undefined, timing: TIMING, random: () => 0.5 });
    again.start();
    vi.setSystemTime(Date.now() + 120_000);
    await vi.advanceTimersByTimeAsync(PRESENCE_JOIN_MS);
    expect(late.sockets[0]!.closed).toBe(false);
    await vi.advanceTimersByTimeAsync(PRESENCE_STALL_GRACE_MS);
    expect(late.sockets[0]!.closed).toBe(true);
    again.stop();
  });

  it("does not time a socket out while it is joining (the join time rules there) and again after a silent one was replaced, however often", async () => {
    const proctor = new FakeProctorSockets();
    const room = new PresenceRoom<CursorState>({ url: "ws://proctor.test/x", connect: proctor.connect, parse: () => undefined, timing: TIMING, random: () => 0.5 });
    const silence = PRESENCE_HEARTBEAT_MS * PRESENCE_SILENCE_FACTOR;
    room.start();
    for (let round = 0; round < 4; round += 1) {
      const socket = proctor.sockets.at(-1)!;
      socket.welcome(`r-${round}`, 0);
      await vi.advanceTimersByTimeAsync(silence);
      expect(socket.closed).toBe(true);
      await vi.advanceTimersByTimeAsync(TIMING.maxMs);
      expect(proctor.sockets).toHaveLength(round + 2);
    }
    room.stop();
  });

  it("leaves no abort listener behind on its lifetime for any socket that ended", async () => {
    const proctor = new FakeProctorSockets();
    const signal = Object.getPrototypeOf(new AbortController().signal) as AbortSignal;
    let held = 0;
    const addOriginal = signal.addEventListener;
    const removeOriginal = signal.removeEventListener;
    const add = vi.spyOn(signal, "addEventListener").mockImplementation(function (this: AbortSignal, ...args: Parameters<AbortSignal["addEventListener"]>) {
      if (args[0] === "abort") held += 1;
      return addOriginal.apply(this, args);
    });
    const remove = vi.spyOn(signal, "removeEventListener").mockImplementation(function (this: AbortSignal, ...args: Parameters<AbortSignal["removeEventListener"]>) {
      if (args[0] === "abort") held -= 1;
      return removeOriginal.apply(this, args);
    });
    try {
      const room = new PresenceRoom<CursorState>({ url: "ws://proctor.test/x", connect: proctor.connect, parse: (state) => (cursorProblem(state) === undefined ? (state as CursorState) : undefined), timing: TIMING, random: () => 0.5 });
      room.start();
      proctor.sockets[0]!.welcome("r-1", 0);
      proctor.sockets[0]!.drop();
      await vi.advanceTimersByTimeAsync(TIMING.maxMs * 2);
      proctor.sockets[1]!.welcome("r-2", 0);
      await vi.advanceTimersByTimeAsync(0);
      const settled = held;
      expect(settled).toBeGreaterThan(0);
      for (let round = 1; round <= 4; round += 1) {
        proctor.sockets[round]!.drop();
        await vi.advanceTimersByTimeAsync(TIMING.maxMs * 2);
        proctor.sockets[round + 1]!.welcome(`r-${round + 2}`, 0);
        await vi.advanceTimersByTimeAsync(0);
        expect(held).toBe(settled);
      }
      room.stop();
    } finally {
      add.mockRestore();
      remove.mockRestore();
    }
  });

  it("watches rooms once joined, replaces a room by its snapshot, applies changes, forgets unwatched rooms and watches again after a rejoin", async () => {
    const proctor = new FakeProctorSockets();
    const cursor = (state: unknown): CursorState | undefined => (cursorProblem(state) === undefined ? (state as CursorState) : undefined);
    const room = new PresenceRoom<CursorState, CursorState>({ url: "ws://proctor.test/x", connect: proctor.connect, parse: cursor, parseWatched: (_, state) => cursor(state), timing: TIMING, random: () => 0.5 });
    room.start();
    room.watch([]);
    room.watch(["arch/leaderboard", "arch/badges", "arch/leaderboard"]);
    const first = proctor.sockets[0]!;
    expect(first.sent).toEqual([]);
    first.welcome("r-me", 0);
    expect(first.sent).toEqual([{ type: "watch", scopes: ["arch/leaderboard", "arch/badges"], intervalMs: WATCH_INTERVAL_MS }]);
    room.watch(["arch/leaderboard", "arch/badges"]);
    expect(first.sent).toHaveLength(1);
    const mira = { tag: learnerTag(MIRA), cursor: { anchor: "leaderboard", x: 0.5, y: 0.5 } };
    first.deliver({ type: "watched", scope: "arch/leaderboard", entries: [{ session: "w-mira", colour: 3, surface: "leaderboard", state: mira }], left: [], snapshot: true });
    first.deliver({ type: "watched", scope: "arch/badges", entries: [{ session: "w-ben", colour: 5, surface: "badges", state: { tag: learnerTag(BEN) } }], left: [] });
    first.deliver({ type: "watched", scope: "arch/secret", entries: [{ session: "w-eve", colour: 1, surface: "x", state: { tag: learnerTag(BEN) } }], left: [] });
    first.deliver({ type: "watched", scope: "arch/badges", entries: [{ session: "w-bad", colour: 1, surface: "badges", state: { tag: "Not A Tag" } }], left: [] });
    const watched = () => Object.fromEntries([...room.getSnapshot().watched].map(([scope, members]) => [scope, members.map((member) => member.session)]));
    expect(watched()).toEqual({ "arch/leaderboard": ["w-mira"], "arch/badges": ["w-ben"] });
    expect(room.getSnapshot().watched.get("arch/leaderboard")?.[0]?.state).toEqual(mira);
    first.deliver({ type: "watched", scope: "arch/leaderboard", entries: [{ session: "w-ann", colour: 2, surface: "leaderboard", state: { tag: learnerTag(ME) } }], left: ["w-mira"] });
    first.deliver({ type: "watched", scope: "arch/badges", entries: [{ session: "w-ann", colour: 2, surface: "badges", state: { tag: learnerTag(ME) } }], left: [], snapshot: true });
    expect(watched()).toEqual({ "arch/leaderboard": ["w-ann"], "arch/badges": ["w-ann"] });
    room.watch(["arch/badges"]);
    expect(first.sent.at(-1)).toEqual({ type: "watch", scopes: ["arch/badges"], intervalMs: WATCH_INTERVAL_MS });
    expect(watched()).toEqual({ "arch/badges": ["w-ann"] });
    room.watch(Array.from({ length: MAX_WATCHED_ROOMS + 4 }, (_, index) => `arch/quiz/q${index}`));
    expect((first.sent.at(-1) as { readonly scopes: readonly string[] }).scopes).toHaveLength(MAX_WATCHED_ROOMS);
    first.drop();
    expect(watched()).toEqual({});
    await vi.advanceTimersByTimeAsync(TIMING.minMs);
    const second = proctor.sockets.at(-1)!;
    expect(second).not.toBe(first);
    second.welcome("r-me-2", 0);
    expect(second.sent).toEqual([first.sent.at(-1)]);
    room.stop();
  });
});
