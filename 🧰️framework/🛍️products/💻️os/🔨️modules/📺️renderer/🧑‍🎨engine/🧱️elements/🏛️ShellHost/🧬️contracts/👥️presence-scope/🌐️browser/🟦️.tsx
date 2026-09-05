import React, { useCallback, useEffect, useRef, useState } from "react";
import { createRoot, type Root } from "react-dom/client";
import {
  decodeBackboneWorkerResponse,
  documentRuntimeKeyV1,
  encodeBackboneWorkerRequest,
  type BackboneWorkerRequest,
  type DocumentExecutionTargetLeaseFieldsV1,
  type DocumentScope,
} from "@semio-tech/framework-os";
import type { PresencePeer } from "@semio-tech/ui-react";
import { scopedPresencePeersV1 } from "../🟦️.ts";

export type ScopedPresenceBrowserCaseV1 = Readonly<{
  id: "a" | "b";
  scope: DocumentScope;
  schema: string;
  surfaceId: string;
  installedTarget: DocumentExecutionTargetLeaseFieldsV1;
}>;

export type ScopedPresenceBrowserConfigV1 = Readonly<{
  proof: string;
  hubOrigin: string;
  workerUrl: string;
  cases: readonly [ScopedPresenceBrowserCaseV1, ScopedPresenceBrowserCaseV1];
}>;

type RuntimeRow = Readonly<{ ready: boolean; closed: boolean; peers: readonly PresencePeer[]; presenceEvents: number }>;
type RuntimeRows = Readonly<Record<"a" | "b", RuntimeRow>>;

function initialRows(): RuntimeRows {
  return {
    a: { ready: false, closed: false, peers: [], presenceEvents: 0 },
    b: { ready: false, closed: false, peers: [], presenceEvents: 0 },
  };
}

function browserRequest(scopeCase: ScopedPresenceBrowserCaseV1, hubOrigin: string): BackboneWorkerRequest {
  return {
    kind: "open",
    documentId: scopeCase.scope.documentId,
    schema: scopeCase.schema,
    actor: `browser-shell-${scopeCase.id}`,
    packSchemaHash: Array.from({ length: 32 }, () => 17),
    bindings: [{ kind: "hub", baseUrl: hubOrigin, spaceId: scopeCase.scope.spaceId, installedTarget: scopeCase.installedTarget }],
  };
}

function ScopedPresenceBrowserShell({ config }: { readonly config: ScopedPresenceBrowserConfigV1 }) {
  const workerRef = useRef<Worker | null>(null);
  const [rows, setRows] = useState<RuntimeRows>(initialRows);
  const casesByKey = useRef(new Map(config.cases.map((row) => [documentRuntimeKeyV1({ kind: "hub", ...row.scope }), row])));

  const post = useCallback((request: BackboneWorkerRequest): void => {
    workerRef.current?.postMessage({ wire: encodeBackboneWorkerRequest(request) });
  }, []);

  const heartbeat = useCallback((row: ScopedPresenceBrowserCaseV1): void => {
    post({
      kind: "send",
      documentId: row.scope.documentId,
      spaceId: row.scope.spaceId,
      message: {
        kind: "presenceHeartbeat",
        peer: {
          actor: `browser-shell-${row.id}`,
          label: row.id === "a" ? "Ada" : "Berta",
          connectedAtMs: row.id === "a" ? 101 : 202,
          views: [],
        },
      },
    });
  }, [post]);

  useEffect(() => {
    const worker = new Worker(config.workerUrl, { type: "module" });
    workerRef.current = worker;
    const channel = new MessageChannel();
    worker.postMessage({ kind: "semio-browser-broker-port", port: channel.port2 }, [channel.port2]);
    channel.port1.onmessage = (event) => {
      if (event.data?.kind !== "initialized" || event.data.ok !== true) return;
      for (const row of config.cases) post(browserRequest(row, config.hubOrigin));
    };
    channel.port1.start();
    channel.port1.postMessage({ kind: "initialize", proof: config.proof });
    worker.onmessage = (event) => {
      if (!event.data?.wire) return;
      const message = decodeBackboneWorkerResponse(new Uint8Array(event.data.wire));
      if ((message.kind !== "socket-actor" && message.kind !== "event") || message.scope === undefined) return;
      const runtimeKey = documentRuntimeKeyV1({ kind: "hub", ...message.scope });
      const scopeCase = casesByKey.current.get(runtimeKey);
      if (!scopeCase) return;
      if (message.kind === "socket-actor") {
        setRows((current) => ({ ...current, [scopeCase.id]: { ...current[scopeCase.id], ready: true } }));
        console.log(`[DEBUG] scoped-presence socket-ready ${scopeCase.id} ${runtimeKey}`);
        heartbeat(scopeCase);
        return;
      }
      if (message.event.kind !== "presence") return;
      const peers = scopedPresencePeersV1(message, scopeCase.scope);
      setRows((current) => ({ ...current, [scopeCase.id]: { ...current[scopeCase.id], peers, presenceEvents: current[scopeCase.id].presenceEvents + 1 } }));
      console.log(`[DEBUG] scoped-presence roster ${scopeCase.id} peers=${peers.map((peer) => peer.actor).join(",") || "empty"}`);
    };
    worker.onerror = (event) => console.error(`[DEBUG] scoped-presence worker-error ${event.message}`);
    return () => {
      channel.port1.close();
      worker.terminate();
      workerRef.current = null;
    };
  }, [config, heartbeat, post]);

  useEffect(() => {
    (window as unknown as { __semioScopedPresence?: unknown }).__semioScopedPresence = {
      a: { ...rows.a, peers: rows.a.peers.map((peer) => ({ ...peer })) },
      b: { ...rows.b, peers: rows.b.peers.map((peer) => ({ ...peer })) },
    };
  }, [rows]);

  const closeA = useCallback(() => {
    const row = config.cases[0];
    post({ kind: "close", documentId: row.scope.documentId, spaceId: row.scope.spaceId });
    setRows((current) => ({ ...current, a: { ...current.a, closed: true } }));
    console.log("[DEBUG] scoped-presence close a");
  }, [config.cases, post]);

  return (
    <main data-shell-host="scoped-presence" aria-label="Scoped presence browser shell">
      <h1>Scoped presence browser shell</h1>
      {config.cases.map((row) => {
        const state = rows[row.id];
        return (
          <section key={row.id} id={`scope-${row.id}`} aria-label={`Scope ${row.id.toUpperCase()}`} data-ready={String(state.ready)} data-closed={String(state.closed)} data-presence-events={state.presenceEvents}>
            <h2>{row.scope.spaceId}/{row.scope.documentId}</h2>
            <output aria-label={`Scope ${row.id.toUpperCase()} roster`}>{state.peers.map((peer) => `${peer.label}:${peer.role ?? "unknown"}:${peer.color ?? -1}`).join("|") || "empty"}</output>
          </section>
        );
      })}
      <button type="button" id="close-a" onClick={closeA}>Close A</button>
      <button type="button" id="heartbeat-b" onClick={() => heartbeat(config.cases[1])}>Heartbeat B</button>
    </main>
  );
}

/** 🌐️ Mounts the production projection boundary around the real browser Worker. */
export function mountScopedPresenceBrowserShellV1(element: HTMLElement, config: ScopedPresenceBrowserConfigV1): Root {
  const root = createRoot(element);
  root.render(<ScopedPresenceBrowserShell config={config} />);
  return root;
}
