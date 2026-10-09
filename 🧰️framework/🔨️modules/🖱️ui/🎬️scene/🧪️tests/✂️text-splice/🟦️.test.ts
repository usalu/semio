/** 🧪️ Laws of the range-text operation (`semio.ui.scene.text-splice.v1`): the schema admits the language-neutral fixture, the TS
 * reference replays every vector exactly (edit → splice, application + inverse, concurrent folds in hub order, host rebase,
 * UTF-8 ↔ scalar offsets), jsdiff (third party) derives the same changed run for every edit, no concurrent insert-only
 * workload loses a typed scalar, and a reincarnated window never makes a host duplicate one. */
import { describe, expect, test } from "bun:test";

import { diffChars } from "diff";
import fixture from "../../🧫️fixtures/✂️text-splice/🔣️.json";
import fc from "fast-check";
import typingLaw from "../../../../🛠️tool-machine/🧫️fixtures/🧫️typing-law/🔣️.json";
import { applyTextSpliceV1, composeTextSplicesV1, createTextEditorTypingRunV1, locateTextSpliceV1, TEXT_EDITOR_TYPING_BUFFER_ARG, TEXT_EDITOR_TYPING_COMMIT_ARG, TEXT_EDITOR_TYPING_HOST_SIGNALS, TEXT_EDITOR_TYPING_IDLE_MS, rebaseTextEditsV1, receiveTextEditorSceneV1, refuseTextEditorSpliceV1, scalarOfUtf8OffsetV1, sendTextEditorSpliceV1, settleTextEditorSpliceV1, TEXT_SPLICE_CONTEXT_SCALARS, TEXT_SPLICE_MIN_TWO_SIDED_SCALARS, textEditorAppliedSpliceV1, textEditorSpliceHostV1, textEditorTypingV1, textSpliceFromEditV1, utf8OffsetOfScalarV1, type TextEditorSpliceHostV1, type TextEditorSpliceViewV1, type TextSpliceV1 } from "../../✂️text-splice/🟦️.ts";

describe("text splice", () => {
  test("the schema admits the fixture and pins the constants", () => {
    expect(fixture.contextScalars).toBe(TEXT_SPLICE_CONTEXT_SCALARS);
    expect(fixture.minTwoSidedScalars).toBe(TEXT_SPLICE_MIN_TWO_SIDED_SCALARS);
  });

  for (const row of fixture.edits) {
    test(`edit ${row.id}: the splice and jsdiff agree on the one changed run`, () => {
      const splice = textSpliceFromEditV1(row.previous, row.next);
      expect(splice).toEqual(row.splice);
      if (splice === null) return expect(row.previous).toBe(row.next);
      const parts = diffChars(row.previous, row.next);
      expect(parts.filter((part) => part.removed).map((part) => part.value).join("")).toBe(splice.deleted);
      expect(parts.filter((part) => part.added).map((part) => part.value).join("")).toBe(splice.insert);
      expect(applyTextSpliceV1(row.previous, splice).text).toBe(row.next);
    });
  }

  for (const row of fixture.applications) {
    test(`apply ${row.id}: lands, results and inverts exactly`, () => {
      const applied = applyTextSpliceV1(row.text, row.splice);
      expect(locateTextSpliceV1(row.text, row.splice)).toEqual(row.located);
      expect(applied.text).toBe(row.result);
      expect(applied.inverse).toEqual(row.inverse);
      if (!row.located.clamped) expect(applyTextSpliceV1(applied.text, applied.inverse).text).toBe(row.text);
    });
  }

  for (const row of fixture.compositions) {
    test(`compose ${row.id}: ${row.note}`, () => {
      const composition = composeTextSplicesV1(row.net, row.next);
      expect(composition).toEqual(row.composition as typeof composition);
      const typed = applyTextSpliceV1(applyTextSpliceV1(row.text, row.net).text, row.next).text;
      if (composition.kind === "composed") {
        expect(applyTextSpliceV1(row.text, composition.splice).text).toBe(typed);
        const canonical = textSpliceFromEditV1(row.text, typed)!;
        expect(trimmedTextSplice(composition.splice)).toEqual({ start: canonical.start, deleted: canonical.deleted, insert: canonical.insert });
        const parts = diffChars(row.text, typed);
        const changed = parts.flatMap((part, index) => (part.added || part.removed ? [index] : []));
        if (parts.slice(changed[0], changed.at(-1)! + 1).every((part) => part.added || part.removed)) {
          expect(parts.filter((part) => part.removed).map((part) => part.value).join("")).toBe(canonical.deleted);
          expect(parts.filter((part) => part.added).map((part) => part.value).join("")).toBe(canonical.insert);
        } else expect(parts.filter((part) => part.added || part.removed).reduce((cost, part) => cost + Array.from(part.value).length, 0)).toBeLessThanOrEqual(Array.from(canonical.deleted).length + Array.from(canonical.insert).length);
      }
      if (composition.kind === "cancelled") expect(typed).toBe(row.text);
    });
  }

  /** ✂️ A run splice without the scalars both its sides share at their ends (the canonical change it makes). */
  const trimmedTextSplice = (splice: TextSpliceV1) => {
    const trimmed = textSpliceFromEditV1([...Array.from(splice.before), ...Array.from(splice.deleted), ...Array.from(splice.after)].join(""), [...Array.from(splice.before), ...Array.from(splice.insert), ...Array.from(splice.after)].join(""), 0);
    return trimmed === null ? null : { start: splice.start - Array.from(splice.before).length + trimmed.start, deleted: trimmed.deleted, insert: trimmed.insert };
  };

  /** 🔗️ The fold every typing run performs: compose while the edits touch the run, start a new run on a disjoint edit, drop a cancelled one. */
  const foldTypingRuns = (texts: readonly string[]): TextSpliceV1[] => {
    const runs: TextSpliceV1[] = [];
    let net: TextSpliceV1 | null = null;
    for (let index = 1; index < texts.length; index += 1) {
      const next = textSpliceFromEditV1(texts[index - 1]!, texts[index]!);
      if (next === null) continue;
      const composition: ReturnType<typeof composeTextSplicesV1> = net === null ? { kind: "composed", splice: next } : composeTextSplicesV1(net, next);
      if (composition.kind === "disjoint") {
        runs.push(net!);
        net = next;
      } else net = composition.kind === "composed" ? composition.splice : null;
    }
    return net === null ? runs : [...runs, net];
  };

  for (const row of fixture.typingRuns) {
    test(`typing run ${row.id}: ${row.note}`, () => {
      const runs = foldTypingRuns(row.texts);
      expect(runs).toEqual(row.runs);
      expect(runs.reduce((text, run) => applyTextSpliceV1(text, run).text, row.texts[0]!)).toBe(row.texts.at(-1)!);
    });
  }

  test("every random typing session folds to runs that each make their span's one canonical change, and only caret jumps split runs (fast-check)", () => {
    const alphabet = ["a", "b", " ", "\n", "😀", "é"];
    const edit = fc.record({ kind: fc.constantFrom("type", "type", "type", "erase", "jump"), at: fc.nat(), scalar: fc.constantFrom(...alphabet) });
    fc.assert(
      fc.property(fc.array(fc.constantFrom(...alphabet), { maxLength: 80 }), fc.array(edit, { minLength: 1, maxLength: 60 }), (initial, edits) => {
        let scalarsNow = [...initial];
        let caret = scalarsNow.length;
        const texts = [scalarsNow.join("")];
        for (const step of edits) {
          if (step.kind === "jump") caret = step.at % (scalarsNow.length + 1);
          else if (step.kind === "erase") {
            if (caret === 0) continue;
            scalarsNow = [...scalarsNow.slice(0, caret - 1), ...scalarsNow.slice(caret)];
            caret -= 1;
          } else {
            scalarsNow = [...scalarsNow.slice(0, caret), step.scalar, ...scalarsNow.slice(caret)];
            caret += 1;
          }
          texts.push(scalarsNow.join(""));
        }
        const runs = foldTypingRuns(texts);
        expect(runs.reduce((text, run) => applyTextSpliceV1(text, run).text, texts[0]!)).toBe(texts.at(-1)!);
        let text = texts[0]!;
        for (const run of runs) {
          const next = applyTextSpliceV1(text, run).text;
          const canonical = textSpliceFromEditV1(text, next);
          expect(trimmedTextSplice(run)).toEqual(canonical === null ? null : { start: canonical.start, deleted: canonical.deleted, insert: canonical.insert });
          text = next;
        }
        expect(runs.length, "only a caret jump ends a run").toBeLessThanOrEqual(edits.filter((step) => step.kind === "jump").length + 1);
      }),
      { numRuns: 400 },
    );
  });

  for (const row of fixture.concurrent) {
    test(`concurrent ${row.id}: the hub-order fold is exact and every author's surviving run stays`, () => {
      let text = row.base;
      const clamped: number[] = [];
      for (const [index, entry] of row.order.entries()) {
        const applied = applyTextSpliceV1(text, entry.splice as TextSpliceV1);
        if (applied.located.clamped) clamped.push(index);
        const undone = applyTextSpliceV1(applied.text, applied.inverse);
        if (!applied.located.clamped) expect(undone.text, `${row.id} #${index} inverse`).toBe(text);
        text = applied.text;
      }
      expect(text).toBe(row.expected.text);
      expect(clamped).toEqual(row.expected.clamped);
      for (const author of new Set(row.order.map((entry) => entry.author))) {
        const inserted = row.order.filter((entry) => entry.author === author).map((entry) => entry.splice.insert).join("");
        for (const scalar of new Set(Array.from(inserted))) expect(Array.from(text).filter((char) => char === scalar).length, `${author}'s ${JSON.stringify(scalar)}`).toBeGreaterThanOrEqual(Array.from(inserted).filter((char) => char === scalar).length - row.order.map((entry) => Array.from(entry.splice.deleted).filter((char) => char === scalar).length).reduce((sum, count) => sum + count, 0));
      }
    });
  }

  for (const row of fixture.rebases) {
    test(`rebase ${row.id}: the host view folds its unapplied splices onto the guest's text`, () => {
      expect(rebaseTextEditsV1(row.remote, row.unapplied, row.local, row.selection)).toEqual(row.expected);
    });
  }

  test("a scene declares splice typing and echoes the applied splice seq", () => {
    expect(textEditorTypingV1(JSON.stringify({ fontPx: 13, typing: { mode: "splice" } }))).toEqual({ mode: "splice" });
    expect(textEditorTypingV1(JSON.stringify({ readOnly: true }))).toBeNull();
    expect(textEditorTypingV1("not json")).toBeNull();
    expect(textEditorAppliedSpliceV1(JSON.stringify({ start: 1, end: 2, splice: 7 }))).toBe(7);
    expect(textEditorAppliedSpliceV1(JSON.stringify({ start: 1, end: 2 }))).toBe(0);
  });

  test("UTF-8 offsets and scalar indices convert both ways", () => {
    for (const row of fixture.offsets) {
      expect(scalarOfUtf8OffsetV1(row.text, row.utf8)).toBe(row.scalar);
      if (new TextEncoder().encode(Array.from(row.text).slice(0, row.scalar).join("")).length === row.utf8) expect(utf8OffsetOfScalarV1(row.text, row.scalar)).toBe(row.utf8);
    }
  });

  test("no concurrent insert-only typing loses a scalar (seeded, 400 workloads)", () => {
    let seed = 0x5eed_c12;
    const random = () => ((seed = (Math.imul(seed ^ (seed >>> 15), 0x2c1b3c6d) + 0x6d2b79f5) >>> 0) / 2 ** 32);
    const alphabet = Array.from("ab ab😀\nxyz");
    for (let workload = 0; workload < 400; workload += 1) {
      const base = Array.from({ length: Math.floor(random() * 24) }, () => alphabet[Math.floor(random() * alphabet.length)]).join("");
      const views = [{ text: base, caret: Math.floor(random() * (Array.from(base).length + 1)) }, { text: base, caret: Math.floor(random() * (Array.from(base).length + 1)) }];
      const committed: TextSpliceV1[] = [];
      const typed = [0, 0];
      for (let key = 0; key < 24; key += 1) {
        const author = random() < 0.5 ? 0 : 1;
        const view = views[author]!;
        const chars = Array.from(view.text);
        const next = [...chars.slice(0, view.caret), author === 0 ? "A" : "B", ...chars.slice(view.caret)].join("");
        committed.push(textSpliceFromEditV1(view.text, next)!);
        views[author] = { text: next, caret: view.caret + 1 };
        typed[author]! += 1;
        if (random() < 0.15) {
          const synced = committed.reduce((text, splice) => applyTextSpliceV1(text, splice).text, base);
          views[author] = { text: synced, caret: locateTextSpliceV1(synced, { start: views[author]!.caret, deleted: "", insert: "", before: Array.from(next).slice(Math.max(0, views[author]!.caret - 32), views[author]!.caret).join(""), after: Array.from(next).slice(views[author]!.caret, views[author]!.caret + 32).join("") }).start };
        }
      }
      const folded = Array.from(committed.reduce((text, splice) => applyTextSpliceV1(text, splice).text, base));
      expect(folded.filter((char) => char === "A").length, `workload ${workload}`).toBe(typed[0]);
      expect(folded.filter((char) => char === "B").length, `workload ${workload}`).toBe(typed[1]);
      expect(folded.length).toBe(Array.from(base).length + typed[0]! + typed[1]!);
    }
  });

  for (const row of fixture.hostTyping) {
    test(`host ${row.id}: sends one numbered splice per run, never reverts its own typing, drops a refused run`, () => {
      let host = textEditorSpliceHostV1(row.init.buffer, row.init.applied);
      let local = row.init.buffer, caret = Array.from(local).length;
      for (const [index, event] of (row.events as readonly Record<string, unknown>[]).entries()) {
        if (typeof event.local === "string") {
          local = event.local;
          caret = event.caret as number;
        } else if (event.send !== undefined) {
          const sent = sendTextEditorSpliceV1(host, local);
          expect(sent && { seq: sent.seq, splice: sent.splice }, `${row.id} #${index}`).toEqual(event.send as { seq: number; splice: TextSpliceV1 });
          host = sent!.host;
        } else if (event.settle !== undefined) {
          host = settleTextEditorSpliceV1(host, event.settle as number);
          expect(host.unapplied.map((entry) => entry.seq), `${row.id} #${index} unapplied`).toEqual(event.unapplied as number[]);
        } else {
          const received = typeof event.scene === "string" ? receiveTextEditorSceneV1(host, event.scene, event.applied as number, local, { anchor: caret, caret }) : refuseTextEditorSpliceV1(host, event.refuse as number, local, { anchor: caret, caret });
          expect(received.show, `${row.id} #${index} show`).toEqual(event.show as never);
          host = received.host;
          expect(host.unapplied.map((entry) => entry.seq), `${row.id} #${index} unapplied`).toEqual(event.unapplied as number[]);
          if (received.show !== null) ({ text: local, caret } = received.show);
        }
      }
    });
  }

  test("two splice-typing hosts over two replicas converge on the hub order and never lose or hide a typed scalar (seeded, 300 sessions)", () => {
    let seed = 0xc12_14c;
    const random = () => ((seed = (Math.imul(seed ^ (seed >>> 15), 0x2c1b3c6d) + 0x6d2b79f5) >>> 0) / 2 ** 32);
    const pick = <T,>(items: readonly T[]): T => items[Math.floor(random() * items.length)]!;
    type Op = { readonly author: number; readonly seq: number; readonly splice: TextSpliceV1 };
    for (let session = 0; session < 300; session += 1) {
      const initial = Array.from({ length: Math.floor(random() * 16) }, () => pick(Array.from("ab c\n😀"))).join("");
      const hub: Op[] = [];
      const replicas = [0, 1].map(() => ({ known: 0, inbox: [] as Op[], pending: [] as Op[], outbox: [] as Op[] }));
      const hosts = [0, 1].map(() => ({ host: textEditorSpliceHostV1(initial, 0) as TextEditorSpliceHostV1, local: initial, caret: Math.floor(random() * (Array.from(initial).length + 1)), typed: "" }));
      const letters = ["A", "B"];
      const replicaText = (author: number) => [...hub.slice(0, replicas[author]!.known), ...replicas[author]!.pending].reduce((text, op) => applyTextSpliceV1(text, op.splice).text, initial);
      const applied = (author: number) => Math.max(0, ...hub.slice(0, replicas[author]!.known).filter((op) => op.author === author).map((op) => op.seq), ...replicas[author]!.pending.map((op) => op.seq));
      const publish = (author: number) => {
        const view = hosts[author]!;
        const received = receiveTextEditorSceneV1(view.host, replicaText(author), applied(author), view.local, { anchor: view.caret, caret: view.caret });
        view.host = received.host;
        if (received.show !== null) ({ text: view.local, caret: view.caret } = received.show);
        for (const letter of Array.from(view.typed)) expect(Array.from(view.local).filter((char) => char === letter).length, `session ${session}: host ${author} shows every ${letter} it typed`).toBe(view.typed.length);
      };
      const step = (action: number) => {
        const author = action % 2, view = hosts[author]!, replica = replicas[author]!;
        switch (Math.floor(action / 2)) {
          case 0: {
            const chars = Array.from(view.local);
            view.local = [...chars.slice(0, view.caret), letters[author]!, ...chars.slice(view.caret)].join("");
            view.caret += 1;
            view.typed += letters[author]!;
            return;
          }
          case 1: {
            const sent = sendTextEditorSpliceV1(view.host, view.local);
            if (sent === null) return;
            view.host = sent.host;
            replica.inbox.push({ author, seq: sent.seq, splice: sent.splice });
            return;
          }
          case 2: {
            const op = replica.inbox.shift();
            if (op === undefined) return;
            replica.pending.push(op);
            replica.outbox.push(op);
            return publish(author);
          }
          case 3: {
            const op = replica.outbox.shift();
            if (op !== undefined) hub.push(op);
            return;
          }
          default: {
            if (replica.known === hub.length) return;
            const op = hub[replica.known]!;
            replica.known += 1;
            if (op.author === author) replica.pending = replica.pending.filter((entry) => entry.seq !== op.seq);
            return publish(author);
          }
        }
      };
      for (let turn = 0; turn < 60; turn += 1) step(Math.floor(random() * 10));
      for (let drain = 0; drain < 400 && (replicas.some((replica) => replica.inbox.length > 0 || replica.outbox.length > 0 || replica.known < hub.length) || hosts.some((view) => textSpliceFromEditV1(view.host.base, view.local) !== null)); drain += 1) step(2 + (drain % 8));
      const final = hub.reduce((text, op) => applyTextSpliceV1(text, op.splice).text, initial);
      for (const author of [0, 1]) {
        expect(hosts[author]!.local, `session ${session}: host ${author} converged`).toBe(final);
        expect(Array.from(final).filter((char) => char === letters[author]).length, `session ${session}: every ${letters[author]} survives`).toBe(hosts[author]!.typed.length);
      }
    }
  });

  test("a window that reincarnates (its document reopened) while both hosts type: the hosts still converge on the hub order, no scalar is ever duplicated, and only runs no window applied are lost (seeded, 300 sessions)", () => {
    let seed = 0xc12_29a;
    const random = () => ((seed = (Math.imul(seed ^ (seed >>> 15), 0x2c1b3c6d) + 0x6d2b79f5) >>> 0) / 2 ** 32);
    const pick = <T,>(items: readonly T[]): T => items[Math.floor(random() * items.length)]!;
    type Op = { readonly author: number; readonly seq: number; readonly splice: TextSpliceV1 };
    const count = (text: string, letter: string) => Array.from(text).filter((char) => char === letter).length;
    let reincarnations = 0, lostAcknowledgements = 0;
    for (let session = 0; session < 300; session += 1) {
      const initial = Array.from({ length: Math.floor(random() * 16) }, () => pick(Array.from("ab c\n😀"))).join("");
      const hub: Op[] = [];
      const replicas = [0, 1].map(() => ({ known: 0, applied: 0, unpublished: false, inbox: [] as Op[], pending: [] as Op[], outbox: [] as Op[], settles: [] as Op[] }));
      const hosts = [0, 1].map(() => ({ host: textEditorSpliceHostV1(initial, 0) as TextEditorSpliceHostV1, local: initial, caret: Math.floor(random() * (Array.from(initial).length + 1)), typed: 0, lost: 0 }));
      const letters = ["A", "B"];
      const replicaText = (author: number) => [...hub.slice(0, replicas[author]!.known), ...replicas[author]!.pending].reduce((text, op) => applyTextSpliceV1(text, op.splice).text, initial);
      const show = (author: number, update: { readonly host: TextEditorSpliceHostV1; readonly show: TextEditorSpliceViewV1 }) => {
        const view = hosts[author]!;
        view.host = update.host;
        if (update.show !== null) ({ text: view.local, caret: view.caret } = update.show);
      };
      const publish = (author: number) => {
        replicas[author]!.unpublished = false;
        show(author, receiveTextEditorSceneV1(hosts[author]!.host, replicaText(author), replicas[author]!.applied, hosts[author]!.local, { anchor: hosts[author]!.caret, caret: hosts[author]!.caret }));
      };
      const refuse = (author: number, op: Op) => {
        const view = hosts[author]!;
        if (view.host.unapplied.some((entry) => entry.seq === op.seq)) view.lost += count(textSpliceFromEditV1(view.host.base, view.local)?.insert ?? "", letters[author]!);
        show(author, refuseTextEditorSpliceV1(view.host, op.seq, view.local, { anchor: view.caret, caret: view.caret }));
      };
      const send = (author: number) => {
        const view = hosts[author]!, sent = sendTextEditorSpliceV1(view.host, view.local);
        if (sent === null) return;
        view.host = sent.host;
        replicas[author]!.inbox.push({ author, seq: sent.seq, splice: sent.splice });
      };
      const process = (author: number, publishNow: boolean) => {
        const replica = replicas[author]!, op = replica.inbox.shift();
        if (op === undefined) return;
        replica.pending.push(op);
        replica.outbox.push(op);
        replica.applied = op.seq;
        replica.settles.push(op);
        if (publishNow) publish(author);
        else replica.unpublished = true;
      };
      const settle = (author: number) => {
        const op = replicas[author]!.settles.shift();
        if (op !== undefined) hosts[author]!.host = settleTextEditorSpliceV1(hosts[author]!.host, op.seq);
      };
      const reopen = (author: number) => {
        const view = hosts[author]!, replica = replicas[author]!, refused = replica.inbox;
        reincarnations += 1;
        if (replica.unpublished) lostAcknowledgements += 1;
        view.lost += refused.reduce((lost, op) => lost + count(op.splice.insert, letters[author]!), 0);
        const owed = replica.settles.map((op) => ({ op, applied: random() < 0.5 }));
        replica.inbox = [];
        replica.pending = [];
        replica.settles = [];
        replica.applied = 0;
        const answer = () => {
          for (const { op, applied } of owed) {
            if (applied) hosts[author]!.host = settleTextEditorSpliceV1(hosts[author]!.host, op.seq);
            else refuse(author, op);
          }
          for (const op of refused) refuse(author, op);
        };
        const answersFirst = random() < 0.5;
        if (answersFirst) answer();
        publish(author);
        if (!answersFirst) answer();
      };
      const step = (action: number) => {
        const author = action % 2, view = hosts[author]!, replica = replicas[author]!;
        if (replica.unpublished && random() < 0.5) return publish(author);
        if (replica.settles.length > 0 && random() < 0.4) return settle(author);
        switch (Math.floor(action / 2)) {
          case 0: {
            const chars = Array.from(view.local);
            view.local = [...chars.slice(0, view.caret), letters[author]!, ...chars.slice(view.caret)].join("");
            view.caret += 1;
            view.typed += 1;
            return;
          }
          case 1:
            return send(author);
          case 2:
            return process(author, random() < 0.7);
          case 3: {
            const op = replica.outbox.shift();
            if (op !== undefined) hub.push(op);
            return;
          }
          case 4: {
            if (replica.known === hub.length) return;
            const op = hub[replica.known]!;
            replica.known += 1;
            if (op.author === author) replica.pending = replica.pending.filter((entry) => entry.seq !== op.seq);
            return publish(author);
          }
          default:
            return random() < 0.3 ? reopen(author) : undefined;
        }
      };
      for (let turn = 0; turn < 80; turn += 1) step(Math.floor(random() * 12));
      const lastReopen = session % 3;
      if (lastReopen < 2) {
        send(lastReopen);
        process(lastReopen, false);
        reopen(lastReopen);
      }
      for (let drain = 0; drain < 600 && (replicas.some((replica) => replica.unpublished || replica.settles.length > 0 || replica.inbox.length > 0 || replica.outbox.length > 0 || replica.known < hub.length) || hosts.some((view) => textSpliceFromEditV1(view.host.base, view.local) !== null)); drain += 1) step(2 + (drain % 8));
      const final = hub.reduce((text, op) => applyTextSpliceV1(text, op.splice).text, initial);
      for (const author of [0, 1]) {
        expect(hosts[author]!.local, `session ${session}: host ${author} converged`).toBe(final);
        expect(count(final, letters[author]!), `session ${session}: no ${letters[author]} duplicated, only never-applied runs lost`).toBe(hosts[author]!.typed - hosts[author]!.lost);
      }
    }
    expect(reincarnations, "the seeded sessions reincarnate windows").toBeGreaterThan(300);
    expect(lostAcknowledgements, "some windows reincarnate before acknowledging an applied run").toBeGreaterThan(50);
  });
});

describe("text editor typing run (host side)", () => {
  /** ⏱️ A fake host timer: `advance(ms)` fires every armed run whose deadline it reaches, in order. */
  const fakeTimers = () => {
    let now = 0;
    const armed: { at: number; run: () => void; live: boolean }[] = [];
    return {
      schedule: (run: () => void, ms: number) => {
        const entry = { at: now + ms, run, live: true };
        armed.push(entry);
        return () => void (entry.live = false);
      },
      advance: (ms: number) => {
        now += ms;
        for (const entry of armed.filter((entry) => entry.live && entry.at <= now)) {
          entry.live = false;
          entry.run();
        }
      },
    };
  };

  test("the protocol constants are the tool-machine owner's (typing-law fixture)", () => {
    expect([TEXT_EDITOR_TYPING_BUFFER_ARG, TEXT_EDITOR_TYPING_COMMIT_ARG, TEXT_EDITOR_TYPING_IDLE_MS]).toEqual([typingLaw.args.buffer, typingLaw.args.commit, typingLaw.idleMs]);
  });

  test("every host signal ends the run with the corpus reason, once, and every reason is the tool machine's", () => {
    const rows = fixture.hostSignals as readonly { readonly signal: keyof typeof TEXT_EDITOR_TYPING_HOST_SIGNALS; readonly commit: string }[];
    expect(Object.fromEntries(rows.map((row) => [row.signal, row.commit]))).toEqual(TEXT_EDITOR_TYPING_HOST_SIGNALS);
    for (const row of rows) {
      expect(typingLaw.reasons).toContain(row.commit);
      const timers = fakeTimers();
      const sent: [string, string][] = [];
      const run = createTextEditorTypingRunV1((verb, reason) => sent.push([verb, reason]), timers.schedule);
      run.typed("textSplice", "Hello");
      if (row.signal === "idle") timers.advance(TEXT_EDITOR_TYPING_IDLE_MS);
      else run.commit(TEXT_EDITOR_TYPING_HOST_SIGNALS[row.signal]);
      timers.advance(TEXT_EDITOR_TYPING_IDLE_MS);
      run.dispose();
      expect(sent).toEqual([["textSplice", row.commit]]);
    }
  });

  test("a run commits once on idle, a pause starts a new run, and the preview is the run's one splice", () => {
    const timers = fakeTimers();
    const sent: [string, string][] = [];
    const run = createTextEditorTypingRunV1((verb, reason) => sent.push([verb, reason]), timers.schedule);
    run.typed("textSplice", "Hello");
    timers.advance(TEXT_EDITOR_TYPING_IDLE_MS - 1);
    run.typed("textSplice", "Hello ");
    expect(run.preview("Hello wo")).toEqual(textSpliceFromEditV1("Hello", "Hello wo"));
    timers.advance(TEXT_EDITOR_TYPING_IDLE_MS - 1);
    expect(sent).toEqual([]);
    timers.advance(1);
    expect(sent).toEqual([["textSplice", "idle"]]);
    expect([run.open(), run.preview("Hello wo")]).toEqual([false, null]);
    run.typed("textSplice", "Hello wo");
    run.commit("blur");
    run.commit("hidden");
    timers.advance(TEXT_EDITOR_TYPING_IDLE_MS * 2);
    expect(sent).toEqual([["textSplice", "idle"], ["textSplice", "blur"]]);
  });

  test("a run the window closed itself (a caret move) sends nothing; dispose ends an open run like a blur", () => {
    const timers = fakeTimers();
    const sent: [string, string][] = [];
    const run = createTextEditorTypingRunV1((verb, reason) => sent.push([verb, reason]), timers.schedule);
    run.typed("textEdit", "q");
    run.closed();
    timers.advance(TEXT_EDITOR_TYPING_IDLE_MS);
    expect(sent).toEqual([]);
    run.typed("textEdit", "qu");
    run.dispose();
    run.dispose();
    expect(sent).toEqual([["textEdit", "blur"]]);
  });
});

test("draft JSON wire preserves literal ranges and canonical native text authority",()=>{
 for(const row of fixture.draftJson){expect(JSON.stringify(row.changes)).toBe(row.json);expect(JSON.parse(row.json)).toEqual(row.changes);console.log("[DEBUG] draft JSON neutral="+row.id+" bytes="+new TextEncoder().encode(row.json).length);}
 const source=require("node:fs").readFileSync(require("node:path").resolve(import.meta.dir,"../../✂️text-splice/🦀️.rs"),"utf8");const writer=source.slice(source.indexOf("pub fn write_draft_changes_json_into"),source.indexOf("pub fn apply_draft_changes"));expect(writer).toContain("write_json_source_into");expect(writer).not.toContain("serde_json");
});

import { draftChangesJsonV1 } from "../../✂️text-splice/🟦️.ts";

test("draft wire preserves original exact JSON vectors", () => {
  for(const row of fixture.draftJson){
    expect(draftChangesJsonV1(row.changes)).toBe(row.json);
    expect(JSON.parse(row.json)).toEqual(row.changes);
  }
});

