/** 📦 PluginRuntime documentPack and transaction wire adapter. */
import { encodeAppFrame } from "@semio-tech/framework-os";
import { loadChannelMediaExportWireFixture } from "../../../../📡️spr/🧪️tests/🧫️fixtures/🟦️.ts";
import type { TurnOutcome } from "@semio-tech/framework-kernel";
import { type PluginRuntimeVitest, type PluginRuntimeDependencies, type PluginRuntimeSource } from "../🛠️plugin-runtime-helpers/🟦️.tsx";

export async function registerDocumentPackWireAdapterTests(vitest: PluginRuntimeVitest, dependencies: PluginRuntimeDependencies, source: PluginRuntimeSource): Promise<void> {
  const { describe, expect, it } = vitest;
  const { adaptPluginHandle, createTurnOutcomeBroadcast, isPluginInstanceRetiredV1 } = dependencies;
    describe("PluginRuntime documentPack/transaction wire adapter", () => {
      it("keeps the exact channel subscribed through refused close and releases only that channel after retry", async () => {
        const { default: fixture } = await import("../../🧱️elements/🔌️PluginRuntime/🧪️tests/🧫️fixtures/🔒️channel-close.json");
        const { produce } = await import("immer");
        const returned: number[] = [];
        let subscriptions = 0;
        let rejectClose!: (reason: unknown) => void;
        let resolveClose!: () => void;
        const lease = {
          handle: {
            manifest: { pluginId: "close-fixture", apps: [] } as unknown as import("@semio-tech/framework").PluginManifest, createApp: async () => fixture.instance,
            destroyApp: () => new Promise<void>((resolve, reject) => { resolveClose = resolve; rejectClose = reject; }),
            enqueue: () => {}, takeSegmentedDownloadChunk: async () => undefined,
            outcomes: { [Symbol.asyncIterator](): AsyncIterator<TurnOutcome> { const id = subscriptions++; return { next: () => new Promise(() => {}), return: async () => { returned.push(id); return { done: true, value: undefined }; } }; } },
            dispose: async () => {},
          },
          release: async () => {},
        };
        const handle = await adaptPluginHandle("close-fixture", lease); const instance = await handle.createApp("fixture");
        expect(returned).toEqual(fixture.refusal.before);
        const first = handle.destroyApp(instance); const refused = expect(first).rejects.toThrow("native close refused");
        expect(returned).toEqual(fixture.refusal.before);
        rejectClose(new Error("native close refused")); await refused;
        expect(returned).toEqual(fixture.refusal.afterFailure); expect(handle.documentPack(instance)).toBeNull();
        const retry = handle.destroyApp(instance); resolveClose(); await retry;
        expect(returned).toEqual(produce(fixture.refusal.afterFailure as number[], state => { state.push(0); }));
        expect(returned).toEqual(fixture.refusal.afterRetry);
      });
  
      it("settles an old channel close without removing a replacement using the same numeric instance", async () => {
        const { default: fixture } = await import("../../🧱️elements/🔌️PluginRuntime/🧪️tests/🧫️fixtures/🔒️channel-close.json");
        const returned: number[] = [];
        let subscriptions = 0;
        const closes: Array<() => void> = [];
        const lease = {
          handle: {
            manifest: { pluginId: "close-fixture", apps: [] } as unknown as import("@semio-tech/framework").PluginManifest, createApp: async () => fixture.instance,
            destroyApp: () => new Promise<void>(resolve => { closes.push(resolve); }),
            enqueue: () => {}, takeSegmentedDownloadChunk: async () => undefined,
            outcomes: { [Symbol.asyncIterator](): AsyncIterator<TurnOutcome> { const id = subscriptions++; return { next: () => new Promise(() => {}), return: async () => { returned.push(id); return { done: true, value: undefined }; } }; } },
            dispose: async () => {},
          },
          release: async () => {},
        };
        const handle = await adaptPluginHandle("close-fixture", lease); const instance = await handle.createApp("old");
        const first = handle.destroyApp(instance); await handle.createApp("replacement");
        expect(returned).toEqual(fixture.replacement.whileClosing);
        closes[0]!(); await first;
        expect(returned).toEqual(fixture.replacement.afterOldClose); expect(handle.documentPack(instance)).toBeNull();
        const replacement = handle.destroyApp(instance); closes[1]!(); await replacement;
        expect(returned).toEqual(fixture.replacement.afterReplacementClose);
      });
  
      /** 🪦️ The teardown race, as a law. A gesture that arrives while an instance is being destroyed —
       * a canvas host remounting and republishing its selection reset, a chord already in flight when a
       * role switch or a hot-swap tore the instance down — used to be ADMITTED by the channel (whose
       * queue stayed open for the whole close ladder) and refused one layer lower by `requireActorId`,
       * as `no actor for instance N` plus a stack. Both halves are pinned here: the actor never sees the
       * late command, and what the caller gets back is a typed retirement it can drop. */
      it("retires an instance's dispatch queue before its actor, so a gesture arriving during teardown never reaches the actor", async () => {
        const enqueued: number[] = [];
        let resolveClose!: () => void;
        const lease = {
          handle: {
            manifest: { pluginId: "retire-fixture", apps: [] } as unknown as import("@semio-tech/framework").PluginManifest,
            createApp: async () => 7,
            destroyApp: () => new Promise<void>((resolve) => { resolveClose = resolve; }),
            enqueue: (instanceId: number) => { enqueued.push(instanceId); },
            takeSegmentedDownloadChunk: async () => undefined,
            outcomes: { [Symbol.asyncIterator](): AsyncIterator<TurnOutcome> { return { next: () => new Promise(() => {}), return: async () => ({ done: true, value: undefined }) }; } },
            dispose: async () => {},
          },
          release: async () => {},
        };
        const handle = await adaptPluginHandle("retire-fixture", lease);
        const instance = await handle.createApp("fixture");
        const attempt = async (): Promise<unknown> => { try { return await handle.readHistory(instance); } catch (error) { return error; } };

        const inFlight = attempt();
        expect(enqueued, "a live instance still crosses to its actor").toEqual([instance]);

        const closing = handle.destroyApp(instance);
        expect(isPluginInstanceRetiredV1(await inFlight), "the gesture already in the queue is settled by the retirement").toBe(true);
        expect(isPluginInstanceRetiredV1(await attempt()), "a gesture dispatched DURING the close ladder is a typed retirement").toBe(true);
        expect(enqueued, "and never reached the actor").toEqual([instance]);

        resolveClose();
        await closing;
        expect(isPluginInstanceRetiredV1(await attempt()), "a gesture dispatched after the close ladder is the same typed retirement").toBe(true);
        expect(enqueued).toEqual([instance]);
      });

      it("cancels a turn whose actor activation was revoked under it with that activation — a typed retirement, not a failure", () => {
        expect(isPluginInstanceRetiredV1(new Error("actor-activation.revoked")), "the shard client's revoked-activation terminal").toBe(true);
        expect(isPluginInstanceRetiredV1(new Error("[os-shell] refresh-ui#4 settled: actor-activation.revoked")), "however a lane words the rethrow").toBe(true);
        expect(isPluginInstanceRetiredV1(new Error("actor-activation.budget-exhausted")), "any other activation fault stays loud").toBe(false);
      });

      it("keeps a never-created instance a loud failure rather than a retirement", async () => {
        const lease = {
          handle: {
            manifest: { pluginId: "retire-fixture", apps: [] } as unknown as import("@semio-tech/framework").PluginManifest,
            createApp: async () => 7,
            destroyApp: async () => {},
            enqueue: () => {},
            takeSegmentedDownloadChunk: async () => undefined,
            outcomes: { [Symbol.asyncIterator](): AsyncIterator<TurnOutcome> { return { next: () => new Promise(() => {}), return: async () => ({ done: true, value: undefined }) }; } },
            dispose: async () => {},
          },
          release: async () => {},
        };
        const handle = await adaptPluginHandle("retire-fixture", lease);
        let raised: unknown = null;
        try { await handle.readHistory(99); } catch (error) { raised = error; }
        expect(String(raised)).toContain("no channel for instance 99");
        expect(isPluginInstanceRetiredV1(raised), "nothing ever created instance 99 — that is a bug, not a teardown race").toBe(false);
      });

      it("resumable media export forwarding preserves exact ownership and rejects foreign receipts", async () => {
        const { decodeAppCommand } = await import("@semio-tech/framework-os");
        const fixture = await loadChannelMediaExportWireFixture();
        const authority = { app_instance_id: fixture.handle.appInstanceId, parent_document_id: fixture.handle.parentDocumentId, operation_id: BigInt(fixture.handle.operationId), base_revision: BigInt(fixture.handle.baseRevision), generation: BigInt(fixture.handle.generation) };
        const status = { handle: authority, state: "running" as const, applied_progress: 2n, checkpoint_available: true, mime_type: "audio/mpeg", total_bytes: 3n, detail: "" };
        const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
        const commands: import("@semio-tech/framework-os").AppCommandValue[] = [];
        let replyHandle = authority;
        let replyKind: "normal" | "missing" | "fault" = "normal";
        const lease = {
          handle: {
            manifest: { pluginId: "media-fixture", apps: [] } as unknown as import("@semio-tech/framework").PluginManifest,
            createApp: async () => authority.app_instance_id,
            destroyApp: async () => {},
            takeSegmentedDownloadChunk: async () => undefined,
            enqueue: (instanceId: number, events: readonly Uint8Array[]) => {
              const frames = events.map((bytes) => {
                const command = decodeAppCommand(bytes);
                commands.push(command);
                const seq = BigInt(Object.values(command)[0]!.seq);
                if (replyKind === "missing") return encodeAppFrame({ Done: { in_reply_to: Number(seq) } });
                if (replyKind === "fault") return encodeAppFrame({ Error: { in_reply_to: Number(seq), fault: [99], report: [] } });
                if ("SubmitMediaExport" in command) return encodeAppFrame({ MediaExportSubmitted: { in_reply_to: seq, handle: replyHandle } });
                if ("CancelMediaExport" in command) return encodeAppFrame({ Done: { in_reply_to: Number(seq) } });
                if ("TakeMediaExportChunk" in command) return encodeAppFrame({ MediaExportChunk: { in_reply_to: seq, handle: replyHandle, data: [0, 128, 255], terminal: true } });
                return encodeAppFrame({ MediaExportStatus: { in_reply_to: seq, ...status, handle: replyHandle } });
              });
              broadcast.push({ instanceId, frames });
            },
            outcomes: broadcast.stream,
            dispose: async () => {},
          },
          release: async () => {},
        };
        const handle = await adaptPluginHandle("media-fixture", lease);
        const instanceId = await handle.createApp("media-editor");
        try {
          expect(await handle.submitMediaExport(instanceId, "playback:out", authority.parent_document_id, authority.base_revision)).toEqual(authority);
          expect(await handle.pollMediaExport(instanceId, authority)).toEqual(status);
          expect(await handle.cancelMediaExport(instanceId, authority)).toBeUndefined();
          expect(await handle.takeMediaExportChunk(instanceId, authority)).toEqual({ handle: authority, data: new Uint8Array([0, 128, 255]), terminal: true });
          expect(commands).toEqual([
            { SubmitMediaExport: { seq: 1n, port: "playback:out", expected_parent_document_id: authority.parent_document_id, expected_base_revision: authority.base_revision } },
            { PollMediaExport: { seq: 2n, handle: authority } }, { CancelMediaExport: { seq: 3n, handle: authority } }, { TakeMediaExportChunk: { seq: 4n, handle: authority } },
          ]);
          for (const field of ["app_instance_id", "parent_document_id", "operation_id", "base_revision", "generation"] as const) {
            const value = authority[field];
            replyHandle = { ...authority, [field]: typeof value === "string" ? "foreign-document" : typeof value === "bigint" ? value - 1n : value - 1 };
            for (const operation of [handle.pollMediaExport, handle.takeMediaExportChunk]) await expect(operation(instanceId, authority)).rejects.toThrow("media-export.authority-mismatch");
            if (field === "app_instance_id" || field === "parent_document_id" || field === "base_revision") await expect(handle.submitMediaExport(instanceId, "playback:out", authority.parent_document_id, authority.base_revision)).rejects.toThrow("media-export.authority-mismatch");
          }
          replyHandle = authority;
          const count = commands.length;
          for (const operation of [handle.pollMediaExport, handle.cancelMediaExport, handle.takeMediaExportChunk]) await expect(operation(instanceId, { ...authority, app_instance_id: instanceId + 1 })).rejects.toThrow("media-export.authority-mismatch");
          expect(commands).toHaveLength(count);
          for (const kind of ["missing", "fault"] as const) {
            replyKind = kind;
            await expect(handle.submitMediaExport(instanceId, "playback:out", authority.parent_document_id, authority.base_revision)).rejects.toThrow(kind === "missing" ? "media-export.missing-reply" : "media-export.failed");
          }
        } finally {
          await handle.dispose();
        }
      });

      it("adaptPluginHandle's documentPack/transactionPrepare/transactionCommit/transactionRollback/transactionUndo/transactionRedo frame through AppChannelClient", async () => {
        const { decodeAppCommand, encodeAppFrame } = await import("@semio-tech/framework-os");
        const seenCommands: unknown[] = [];
        const turnBroadcast = createTurnOutcomeBroadcast<TurnOutcome>();
        const fakeLease = {
          handle: {
            manifest: { pluginId: "b-plugin", label: "B", version: "1.0.0", apps: [], workflows: [], examples: [] } as unknown as import("@semio-tech/framework").PluginManifest,
            createApp: async () => 20,
            destroyApp: async () => {},
            takeSegmentedDownloadChunk: async () => undefined,
            enqueue: (instanceId: number, events: readonly Uint8Array[]) => {
              const commands = events.map((frame) => decodeAppCommand(frame));
              seenCommands.push(...commands);
              const frames = commands.flatMap((command) => {
                if ("transactionPrepare" in command) {
                  return [encodeAppFrame({ transactionPrepared: { txn_id: command.transactionPrepare.txn_id, foreign: [], rejection: [] } }), encodeAppFrame({ Done: { in_reply_to: command.transactionPrepare.seq } })];
                }
                if ("transactionCommit" in command) {
                  return [encodeAppFrame({ transactionCommitted: { txn_id: command.transactionCommit.txn_id, edit_id: "edit-1" } }), encodeAppFrame({ Done: { in_reply_to: command.transactionCommit.seq } })];
                }
                if ("transactionRollback" in command) return [encodeAppFrame({ transactionRolledBack: { txn_id: command.transactionRollback.txn_id } }), encodeAppFrame({ Done: { in_reply_to: command.transactionRollback.seq } })];
                if ("transactionUndo" in command) return [encodeAppFrame({ Done: { in_reply_to: command.transactionUndo.seq } })];
                if ("transactionRedo" in command) return [encodeAppFrame({ Done: { in_reply_to: command.transactionRedo.seq } })];
                if ("ReadDocument" in command) {
                  return [encodeAppFrame({ Document: { in_reply_to: command.ReadDocument.seq, pack: [5, 5], spr: [6], ops: "" } })];
                }
                if ("ReadWindowConfigs" in command) {
                  return [encodeAppFrame({ WindowConfigs: { in_reply_to: command.ReadWindowConfigs.seq, entries: [{ window_id: "graph-a", window_kind_id: "graph", envelope_pack: [7] }] } })];
                }
                if ("LoadWindowConfig" in command) return [encodeAppFrame({ Done: { in_reply_to: command.LoadWindowConfig.seq } })];
                throw new Error(`unexpected command ${JSON.stringify(command)}`);
              });
              turnBroadcast.push({ instanceId, frames });
            },
            outcomes: turnBroadcast.stream,
            dispose: async () => {},
          },
          release: async () => {},
        };
        const handle = await adaptPluginHandle("b-plugin", fakeLease);
        const instanceId = await handle.createApp("app-b");
  
        // 📦️ documentPack() is null until the underlying AppChannelClient has observed a document —
        // this adapter only ever surfaces the CACHE, it never issues a document round trip itself.
        expect(handle.documentPack(instanceId)).toBeNull();
  
        const prepareOutcome = await handle.transactionPrepare(instanceId, "txn-1", { form: "owner", mutationId: "s.b#mutate", payload: new Uint8Array([1]) });
        expect(prepareOutcome).toEqual({ foreign: [], rejection: null });
  
        const commitOutcome = await handle.transactionCommit(instanceId, "txn-1");
        expect(commitOutcome).toEqual({ editId: "edit-1" });
  
        await handle.transactionRollback(instanceId, "txn-2");
        await handle.transactionUndo(instanceId, "grp-1");
        await handle.transactionRedo(instanceId, "grp-1");
        expect(await handle.readWindowConfigPacks(instanceId)).toEqual([{ window_id: "graph-a", window_kind_id: "graph", envelope_pack: [7] }]);
        await handle.loadWindowConfigPack(instanceId, { window_id: "graph-b", window_kind_id: "graph", envelope_pack: [8] });
  
        expect(seenCommands).toEqual([
          { transactionPrepare: { seq: 1, txn_id: "txn-1", mutation_id: "s.b#mutate", payload: [1], prepared_ops: [], origin: [], prepared_child_ops: [] } },
          { transactionCommit: { seq: 2, txn_id: "txn-1" } },
          { transactionRollback: { seq: 3, txn_id: "txn-2" } },
          { transactionUndo: { seq: 4, group_id: "grp-1" } },
          { transactionRedo: { seq: 5, group_id: "grp-1" } },
          { ReadWindowConfigs: { seq: 6 } },
          { LoadWindowConfig: { seq: 7, entry: { window_id: "graph-b", window_kind_id: "graph", envelope_pack: [8] } } },
        ]);
      });
  
      it("loads a pack+spr document through the stepped archive load — admit, polled progress, acknowledge, never `LoadDocument` — and documentPack() reads the channel it loaded through", async () => {
        const { decodeAppCommand, encodeAppFrame } = await import("@semio-tech/framework-os");
        const turnBroadcast = createTurnOutcomeBroadcast<TurnOutcome>();
        const sent: string[] = [];
        let polls = 0;
        const fakeLease = {
          handle: {
            manifest: { pluginId: "b-plugin", label: "B", version: "1.0.0", apps: [], workflows: [], examples: [] } as unknown as import("@semio-tech/framework").PluginManifest,
            createApp: async () => 20,
            destroyApp: async () => {},
            takeSegmentedDownloadChunk: async () => undefined,
            enqueue: (instanceId: number, events: readonly Uint8Array[]) => {
              const frames = events.map((frame) => decodeAppCommand(frame)).map((command) => {
                sent.push(Object.keys(command)[0]!);
                if ("LoadDocumentArchive" in command) return encodeAppFrame({ Done: { in_reply_to: command.LoadDocumentArchive.seq } });
                if ("PollDocumentArchiveLoad" in command) {
                  polls += 1;
                  const { seq, operation } = command.PollDocumentArchiveLoad;
                  return encodeAppFrame({ DocumentArchiveLoad: { in_reply_to: seq, status: { operation, state: polls < 3 ? "running" : "ready", completed: polls, total: 3, ahead: 0, fault: [] } } });
                }
                if ("AcknowledgeDocumentArchiveLoad" in command) return encodeAppFrame({ Done: { in_reply_to: command.AcknowledgeDocumentArchiveLoad.seq } });
                throw new Error(`unexpected command ${JSON.stringify(command)}`);
              });
              turnBroadcast.push({ instanceId, frames });
            },
            outcomes: turnBroadcast.stream,
            dispose: async () => {},
          },
          release: async () => {},
        };
        const handle = await adaptPluginHandle("b-plugin", fakeLease);
        const instanceId = await handle.createApp("app-b");
        expect([handle.documentPack(instanceId), "loadAppDocumentPack" in handle]).toEqual([null, false]);
        const progress: (readonly [number, number])[] = [];
        await handle.loadAppDocumentArchive?.(instanceId, { parent_pack: [1, 2], parent_spr: [3], members: [] }, undefined, (status) => progress.push([status.completed, status.total]));
        expect(handle.documentPack(instanceId)).toEqual({ pack: new Uint8Array([1, 2]), spr: new Uint8Array([3]) });
        expect(progress).toEqual([[1, 3], [2, 3], [3, 3]]);
        expect(sent).toEqual(["LoadDocumentArchive", "PollDocumentArchiveLoad", "PollDocumentArchiveLoad", "PollDocumentArchiveLoad", "AcknowledgeDocumentArchiveLoad"]);
      });
  
      /** 📜️ `ops` is the third half of the frame, not decoration: a reader of the live document gets the
       * pack, the spr AND the op text, so an adapter that drops it silently loses the document's ops.
       * This law therefore pins the text crossing verbatim, not merely that a document was read. */
      it("readAppDocumentPack() returns the AppFrame::Document pack/spr/ops, and null when the reply carries no document frame", async () => {
        const { decodeAppCommand, encodeAppFrame } = await import("@semio-tech/framework-os");
        const turnBroadcast = createTurnOutcomeBroadcast<TurnOutcome>();
        let replyWithDocument = true;
        const fakeLease = {
          handle: {
            manifest: { pluginId: "c-plugin", label: "C", version: "1.0.0", apps: [], workflows: [], examples: [] } as unknown as import("@semio-tech/framework").PluginManifest,
            createApp: async () => 30,
            destroyApp: async () => {},
            takeSegmentedDownloadChunk: async () => undefined,
            enqueue: (instanceId: number, events: readonly Uint8Array[]) => {
              const commands = events.map((frame) => decodeAppCommand(frame));
              const frames = commands.map((command) => {
                if (!("ReadDocument" in command)) throw new Error(`unexpected command ${JSON.stringify(command)}`);
                const seq = command.ReadDocument.seq;
                return replyWithDocument ? encodeAppFrame({ Document: { in_reply_to: seq, pack: [7, 8], spr: [9], ops: "(extrude (polygon 6))" } }) : encodeAppFrame({ Done: { in_reply_to: seq } });
              });
              turnBroadcast.push({ instanceId, frames });
            },
            outcomes: turnBroadcast.stream,
            dispose: async () => {},
          },
          release: async () => {},
        };
        const handle = await adaptPluginHandle("c-plugin", fakeLease);
        const instanceId = await handle.createApp("app-c");
        expect(await handle.readAppDocumentPack?.(instanceId)).toEqual({ pack: new Uint8Array([7, 8]), spr: new Uint8Array([9]), ops: "(extrude (polygon 6))" });
        replyWithDocument = false;
        expect(await handle.readAppDocumentPack?.(instanceId)).toBeNull();
      });
    });
}
