/**
 * 🧫️ Law of `semio.io.stream-mux/v1` over its language-agnostic fixture (`../../🧫️fixtures/🔣️.json`): every codec vector
 * decodes to its frame and re-encodes byte-exact, every hostile is refused with its code and field, every server and channel
 * scenario emits exactly its frames. Third-party oracle: AJV judges every vector, hostile and emitted frame against the
 * contract's own JSON schema (`../../🧬️schema/🔣️.json`), and Python's `json` encoder wrote the canonical texts.
 */
import {
  StreamMuxChannelV1,
  StreamMuxPortEndpointV1,
  StreamMuxServerV1,
  STREAM_MUX_BOUNDS_V1,
  decodeStreamMuxClientFrameV1,
  decodeStreamMuxServerFrameV1,
  encodeStreamMuxFrameV1,
  type StreamMuxClientFrameV1,
  type StreamMuxConnectionV1,
  type StreamMuxJobV1,
  type StreamMuxJsonV1,
  type StreamMuxRouteV1,
  type StreamMuxServerFrameV1,
  type StreamMuxTimersV1,
} from "../../🟦️.ts";

type Json = StreamMuxJsonV1;
type Step = Readonly<Record<string, unknown>>;
type Fixture = {
  readonly epoch: string;
  readonly vectors: readonly { readonly side: "client" | "server"; readonly text: string; readonly frame: Json }[];
  readonly hostiles: readonly { readonly side: "client" | "server"; readonly text?: string; readonly fill?: { readonly prefix: string; readonly char: string; readonly count: number; readonly suffix: string }; readonly refusal: string; readonly field: string | null }[];
  readonly routes: Readonly<Record<string, { readonly snapshot?: Json; readonly coalesceBy?: string; readonly coalesceAll?: boolean; readonly admitKey?: string; readonly source?: boolean; readonly job?: boolean }>>;
  readonly serverScenarios: readonly { readonly name: string; readonly steps: readonly Step[] }[];
  readonly channelScenarios: readonly { readonly name: string; readonly random: number; readonly steps: readonly Step[] }[];
};

/** 🕰️ A deterministic clock: `advance` runs every due timer in due order, flushing the event loop after each. */
class FakeClock implements StreamMuxTimersV1 {
  time = 0;
  private nextId = 1;
  private tasks: { readonly id: number; readonly at: number; readonly run: () => void }[] = [];
  now(): number {
    return this.time;
  }
  setTimeout(run: () => void, ms: number): unknown {
    const id = this.nextId++;
    this.tasks.push({ id, at: this.time + Math.max(0, ms), run });
    return id;
  }
  clearTimeout(handle: unknown): void {
    this.tasks = this.tasks.filter((task) => task.id !== handle);
  }
  async advance(ms: number): Promise<void> {
    const target = this.time + ms;
    for (;;) {
      const due = this.tasks.filter((task) => task.at <= target).sort((left, right) => left.at - right.at || left.id - right.id)[0];
      if (due === undefined) break;
      this.tasks = this.tasks.filter((task) => task !== due);
      this.time = due.at;
      due.run();
      await settle();
    }
    this.time = target;
  }
}

const settle = async (): Promise<void> => {
  for (let turn = 0; turn < 3; turn += 1) await new Promise<void>((resolve) => setImmediate(resolve));
};

const textOf = (hostile: Fixture["hostiles"][number]): string => hostile.text ?? `${hostile.fill!.prefix}${hostile.fill!.char.repeat(hostile.fill!.count)}${hostile.fill!.suffix}`;

if (import.meta.vitest) {
  const { describe, expect, it, vi } = import.meta.vitest;
  const fixture = (await import("../../🧫️fixtures/🔣️.json", { with: { type: "json" } })).default as unknown as Fixture;
  const schema = (await import("../../🧬️schema/🔣️.json", { with: { type: "json" } })).default;
  const { default: Ajv } = await import("ajv/dist/2020.js");
  const ajv = new Ajv({ strict: false });
  ajv.addSchema(schema);
  const judge = {
    client: ajv.getSchema(`${schema.$id}#/$defs/ClientFrame`)!,
    server: ajv.getSchema(`${schema.$id}#/$defs/ServerFrame`)!,
  };

  describe("stream-mux frame codec (semio.io.stream-mux/v1)", () => {
    it("reads its bounds from the contract", () => {
      expect(STREAM_MUX_BOUNDS_V1.subprotocol).toBe("semio.stream-mux.v1");
      expect(Object.keys(STREAM_MUX_BOUNDS_V1).sort()).toEqual([...schema.$defs.Bounds.required].sort());
    });

    it.each(fixture.vectors.map((vector) => [vector.side, vector.text.slice(0, 80), vector] as const))("%s vector %s decodes, re-encodes byte-exact and satisfies the schema (AJV)", (_side, _label, vector) => {
      const decoded = vector.side === "client" ? decodeStreamMuxClientFrameV1(vector.text) : decodeStreamMuxServerFrameV1(vector.text);
      expect(decoded.ok).toBe(true);
      if (!decoded.ok) return;
      expect(decoded.frame).toEqual(vector.frame);
      expect(encodeStreamMuxFrameV1(decoded.frame)).toBe(vector.text);
      expect(judge[vector.side](JSON.parse(vector.text))).toBe(true);
    });

    it.each(fixture.hostiles.map((hostile) => [hostile.side, hostile.refusal, hostile.field ?? "-", hostile] as const))("%s hostile is refused %s (%s) and AJV agrees", (_side, _refusal, _field, hostile) => {
      const text = textOf(hostile);
      const decoded = hostile.side === "client" ? decodeStreamMuxClientFrameV1(text) : decodeStreamMuxServerFrameV1(text);
      expect(decoded).toEqual({ ok: false, refusal: hostile.refusal, field: hostile.field });
      if (hostile.refusal === "too-large") {
        expect(new TextEncoder().encode(text).byteLength).toBeGreaterThan(STREAM_MUX_BOUNDS_V1.maxFrameBytes);
        return;
      }
      let parsed: unknown;
      try {
        parsed = JSON.parse(text);
      } catch {
        expect(hostile.refusal).toBe("malformed-json");
        return;
      }
      expect(judge[hostile.side](parsed)).toBe(false);
    });
  });

  describe("stream-mux server scenarios", () => {
    it.each(fixture.serverScenarios.map((scenario) => [scenario.name, scenario] as const))("%s", async (_name, scenario) => {
      const clock = new FakeClock();
      const server = new StreamMuxServerV1({ epoch: fixture.epoch, timers: clock });
      const runs: Record<string, number> = {};
      const aborted: Record<string, boolean> = {};
      const jobs = new Map<string, { readonly job: StreamMuxJobV1; readonly resolve: () => void; readonly reject: (error: Error) => void }>();
      const sources = new Map<string, { state: "live" | "stopped"; readonly emit: (data: Json) => void }>();
      for (const [name, policy] of Object.entries(fixture.routes)) {
        const route: StreamMuxRouteV1 = {
          ...(policy.snapshot !== undefined ? { snapshot: () => policy.snapshot! } : {}),
          ...(policy.coalesceBy !== undefined ? { coalesce: (data: Json) => String((data as Record<string, Json>)[policy.coalesceBy!]) } : {}),
          ...(policy.coalesceAll === true ? { coalesce: () => "all" } : {}),
          ...(policy.source === true
            ? {
                source: (key: string, emit: (data: Json) => void) => {
                  const source = { state: "live" as "live" | "stopped", emit };
                  sources.set(key, source);
                  return () => {
                    source.state = "stopped";
                  };
                },
              }
            : {}),
          ...(policy.admitKey !== undefined ? { admit: (key: string) => new RegExp(policy.admitKey!, "u").test(key) } : {}),
          ...(policy.job === true
            ? {
                run: (key: string, job: StreamMuxJobV1) =>
                  new Promise<void>((resolve, reject) => {
                    runs[key] = (runs[key] ?? 0) + 1;
                    aborted[key] = false;
                    job.signal.addEventListener("abort", () => {
                      aborted[key] = true;
                      reject(new Error("aborted"));
                    });
                    jobs.set(key, { job, resolve, reject });
                  }),
              }
            : {}),
        };
        server.route(name, route);
      }
      const sockets = new Map<string, { sent: string[]; cursor: number; buffered: number; closed: readonly [number, string] | null; connection: StreamMuxConnectionV1 | null }>();
      for (const step of scenario.steps) {
        if ("connect" in step) {
          const socket = { sent: [] as string[], cursor: 0, buffered: 0, closed: null as readonly [number, string] | null, connection: null as StreamMuxConnectionV1 | null };
          sockets.set(step.connect as string, socket);
          socket.connection = server.connect({ send: (frame) => socket.sent.push(frame), bufferedAmount: () => socket.buffered, close: (code, reason) => (socket.closed = [code, reason]) });
        } else if ("send" in step) sockets.get(step.send as string)!.connection!.receive(JSON.stringify(step.frame));
        else if ("sendText" in step) sockets.get(step.sendText as string)!.connection!.receive(step.text as string);
        else if ("drop" in step) sockets.get(step.drop as string)!.connection!.closed();
        else if ("publish" in step) {
          const publish = step.publish as { route: string; key: string; data: Json };
          server.publish(publish.route, publish.key, publish.data);
        } else if ("publishMany" in step) {
          const publish = step.publishMany as { route: string; key: string; count: number };
          for (let index = 1; index <= publish.count; index += 1) server.publish(publish.route, publish.key, { n: index });
        } else if ("advance" in step) await clock.advance(step.advance as number);
        else if ("buffered" in step) for (const [name, bytes] of Object.entries(step.buffered as Record<string, number>)) sockets.get(name)!.buffered = bytes;
        else if ("jobProgress" in step) {
          const progress = step.jobProgress as { key: string; done: number; total: number | null; note: string };
          jobs.get(progress.key)!.job.progress(progress.done, progress.total, progress.note);
        } else if ("jobSettle" in step) {
          const outcome = step.jobSettle as { key: string; outcome: "done" | "failed"; detail?: string };
          if (outcome.outcome === "done") jobs.get(outcome.key)!.resolve();
          else jobs.get(outcome.key)!.reject(new Error(outcome.detail ?? ""));
        } else if ("expect" in step) {
          for (const [name, frames] of Object.entries(step.expect as Record<string, readonly Json[]>)) {
            const socket = sockets.get(name)!;
            const sent = socket.sent.slice(socket.cursor);
            socket.cursor = socket.sent.length;
            expect(sent).toEqual(frames.map((frame) => JSON.stringify(frame)));
            for (const frame of sent) expect(judge.server(JSON.parse(frame))).toBe(true);
          }
        } else if ("closed" in step) for (const [name, closed] of Object.entries(step.closed as Record<string, readonly [number, string]>)) expect(sockets.get(name)!.closed).toEqual(closed);
        else if ("census" in step) expect(server.census()).toEqual(step.census);
        else if ("jobRuns" in step) for (const [key, count] of Object.entries(step.jobRuns as Record<string, number>)) expect(runs[key] ?? 0).toBe(count);
        else if ("sourceEmit" in step) {
          const emit = step.sourceEmit as { key: string; data: Json };
          sources.get(emit.key)!.emit(emit.data);
        } else if ("sources" in step) for (const [key, state] of Object.entries(step.sources as Record<string, string>)) expect(sources.get(key)?.state).toBe(state);
        else if ("jobAborted" in step) for (const [key, value] of Object.entries(step.jobAborted as Record<string, boolean>)) expect(aborted[key]).toBe(value);
        else throw new Error(`unknown server step ${JSON.stringify(step)}`);
        await settle();
      }
      server.close();
    });
  });

  describe("stream-mux channel scenarios", () => {
    it.each(fixture.channelScenarios.map((scenario) => [scenario.name, scenario] as const))("%s", async (_name, scenario) => {
      const random = vi.spyOn(Math, "random").mockReturnValue(scenario.random);
      const quiet = vi.spyOn(console, "error").mockImplementation(() => undefined);
      try {
        const clock = new FakeClock();
        const sent: string[] = [];
        let sentCursor = 0;
        const links: { readonly events: { readonly open: () => void; readonly message: (frame: string) => void; readonly closed: () => void }; closed: boolean }[] = [];
        const channel = new StreamMuxChannelV1(
          {
            connect(events) {
              const link = { events, closed: false };
              links.push(link);
              return {
                send: (frame) => sent.push(frame),
                close: () => {
                  if (link.closed) return;
                  link.closed = true;
                  events.closed();
                },
              };
            },
          },
          clock,
        );
        const subs = new Map<string, { readonly events: unknown[][]; cursor: number; readonly pending: (() => void)[]; close: () => void }>();
        for (const step of scenario.steps) {
          const link = links.at(-1);
          if ("open" in step) {
            const open = step.open as { sub: string; route: string; key: string; credit: number; deferData?: boolean };
            const sub = { events: [] as unknown[][], cursor: 0, pending: [] as (() => void)[], close: () => undefined as void };
            subs.set(open.sub, sub);
            sub.close = channel.open(
              open.route,
              open.key,
              {
                opened: (mode) => sub.events.push(["opened", mode]),
                data: (data) => {
                  sub.events.push(["data", data]);
                  if (open.deferData === true) return new Promise<void>((resolve) => sub.pending.push(resolve));
                },
                progress: (done, total, note) => sub.events.push(["progress", done, total, note]),
                end: (reason, detail) => sub.events.push(["end", reason, detail]),
              },
              { credit: open.credit },
            ).close;
          } else if ("linkOpen" in step) link!.events.open();
          else if ("linkClosed" in step) {
            link!.closed = true;
            link!.events.closed();
          } else if ("server" in step) link!.events.message(JSON.stringify(step.server));
          else if ("serverText" in step) link!.events.message(step.serverText as string);
          else if ("advance" in step) await clock.advance(step.advance as number);
          else if ("close" in step) subs.get(step.close as string)!.close();
          else if ("release" in step) for (const resolve of subs.get(step.release as string)!.pending.splice(0)) resolve();
          else if ("expect" in step) {
            const expectation = step.expect as { links?: number; linkClosed?: boolean; sent?: readonly Json[]; events?: Record<string, readonly unknown[]> };
            if (expectation.links !== undefined) expect(links.length).toBe(expectation.links);
            if (expectation.linkClosed !== undefined) expect(links.at(-1)!.closed).toBe(expectation.linkClosed);
            if (expectation.sent !== undefined) {
              const frames = sent.slice(sentCursor);
              sentCursor = sent.length;
              expect(frames).toEqual(expectation.sent.map((frame) => JSON.stringify(frame)));
              for (const frame of frames) expect(judge.client(JSON.parse(frame))).toBe(true);
            }
            for (const [name, events] of Object.entries(expectation.events ?? {})) {
              const sub = subs.get(name)!;
              expect(sub.events.slice(sub.cursor)).toEqual(events);
              sub.cursor = sub.events.length;
            }
          } else throw new Error(`unknown channel step ${JSON.stringify(step)}`);
          await settle();
        }
        channel.close();
      } finally {
        random.mockRestore();
        quiet.mockRestore();
      }
    });

    it("bridges a worker port onto the ONE page link: local stream ids, credit and cancel cross the port", async () => {
      const clock = new FakeClock();
      const sent: string[] = [];
      let events: { readonly open: () => void; readonly message: (frame: string) => void; readonly closed: () => void } | null = null;
      let connects = 0;
      const channel = new StreamMuxChannelV1(
        {
          connect(handlers) {
            connects += 1;
            events = handlers;
            return { send: (frame) => sent.push(frame), close: () => undefined };
          },
        },
        clock,
      );
      const pipe = new MessageChannel();
      const detach = channel.attachPort(pipe.port1);
      const worker = new StreamMuxPortEndpointV1(pipe.port2);
      const seen: unknown[][] = [];
      channel.open("plugin-modules.watch", "", { data: (data) => void seen.push(["page", data]) }, { credit: 4 });
      const folder = worker.open("backbone.folder", "folder:///a", { opened: (mode) => void seen.push(["worker-opened", mode]), data: (data) => void seen.push(["worker", data]) }, { credit: 2 });
      await settle();
      events!.open();
      await settle();
      expect(connects).toBe(1);
      const opens = sent.map((frame) => decodeStreamMuxClientFrameV1(frame)).filter((decoded) => decoded.ok).map((decoded) => (decoded as { frame: StreamMuxClientFrameV1 }).frame);
      expect(opens).toEqual([
        { kind: "open", stream: 1, route: "plugin-modules.watch", key: "", resume: null, credit: 4 },
        { kind: "open", stream: 2, route: "backbone.folder", key: "folder:///a", resume: null, credit: 2 },
      ]);
      const serve = (frame: StreamMuxServerFrameV1): void => events!.message(encodeStreamMuxFrameV1(frame));
      serve({ kind: "opened", stream: 2, mode: "fresh", epoch: fixture.epoch, seq: 3 });
      serve({ kind: "data", stream: 2, seq: 4, data: { changed: true } });
      serve({ kind: "data", stream: 1, seq: 5, data: { kind: "built" } });
      await settle();
      expect(seen).toHaveLength(3);
      expect(seen).toContainEqual(["worker-opened", "fresh"]);
      expect(seen).toContainEqual(["worker", { changed: true }]);
      expect(seen).toContainEqual(["page", { kind: "built" }]);
      const before = sent.length;
      serve({ kind: "data", stream: 2, seq: 6, data: { changed: true } });
      await settle();
      expect(sent.slice(before)).toEqual([encodeStreamMuxFrameV1({ kind: "grant", stream: 2, credit: 1 })]);
      folder.close();
      await settle();
      expect(sent.at(-1)).toBe(encodeStreamMuxFrameV1({ kind: "cancel", stream: 2 }));
      worker.open("backbone.folder", "folder:///b", {}, { credit: 1 });
      await settle();
      detach();
      expect(sent.at(-1)).toBe(encodeStreamMuxFrameV1({ kind: "cancel", stream: 3 }));
      expect(channel.census()).toEqual({ open: true, streams: 1, links: 1 });
      pipe.port1.close();
      pipe.port2.close();
      channel.close();
    });
  });
}
