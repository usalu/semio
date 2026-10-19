//#region 🔖️Contract
/** 🧭️ The journeys this harness runs, each one acceptance check. */
export type HubCollaborationJourneyV1 = "wasm32" | "wasm32-react" | "wasm32-native" | "native-react" | "native-react-cursors" | "cursors" | "agent-pixels";

/** 🧾️ The acceptance check each journey files its record under (the gate appends `-<locale>`). */
export const HUB_COLLABORATION_CHECKS: Readonly<Record<HubCollaborationJourneyV1, string>> = {
  wasm32: "wgpu-collaboration-wasm32",
  "wasm32-react": "wgpu-collaboration-wasm32-react",
  "wasm32-native": "wgpu-collaboration-wasm32-native",
  "native-react": "wgpu-collaboration-native-react",
  "native-react-cursors": "wgpu-peer-cursors-native-react",
  cursors: "wgpu-peer-cursors",
  "agent-pixels": "wgpu-agent-reply-pixels",
};

/** 🧩️ The hub kind (creation-catalog schema) each journey's document is, the wasm32 playground variant that edits it, the
 * browser serves the journey drives (`wgpu` release serve of that variant, the React `s` dev serve) and whether the native wgpu
 * shell (the renderer's live law, which creates its own space and document) is one of the users. */
export const HUB_COLLABORATION_KINDS: Readonly<Record<HubCollaborationJourneyV1, Readonly<{ schema: string; variant: string; wgpu: boolean; react: boolean; native: boolean }>>> = {
  wasm32: { schema: "note.document", variant: "note", wgpu: true, react: false, native: false },
  "wasm32-react": { schema: "block.2d", variant: "block2d", wgpu: true, react: true, native: false },
  "wasm32-native": { schema: "block.2d", variant: "block2d", wgpu: true, react: false, native: true },
  "native-react": { schema: "block.2d", variant: "block2d", wgpu: false, react: true, native: true },
  "native-react-cursors": { schema: "block.2d", variant: "block2d", wgpu: false, react: true, native: true },
  cursors: { schema: "board.ports.directed.v1", variant: "puzzle2d", wgpu: true, react: false, native: false },
  "agent-pixels": { schema: "note.document", variant: "note", wgpu: true, react: false, native: false },
};

/** ⏱️ Bounds every judgement uses (one place, so a slower machine changes numbers, never code paths). */
export const HUB_COLLABORATION_BOUNDS = {
  bootMs: 180_000,
  liveMs: 120_000,
  presenceMs: 45_000,
  editSeenMs: 60_000,
  shortCutMs: 15_000,
  mediumCutMs: 20_000,
  expiryMs: 120_000,
  relinkMs: 60_000,
  frozenBeatMs: 2_000,
  keptLineMs: 8_000,
  cursorSettleMs: 3_500,
  agentSeenMs: 60_000,
  agentEditMinPixels: 256,
  agentSameMaxPixels: 64,
  nativeStepMs: 900_000,
} as const;

/** 👤️ One human: a hub credential and the tongue their browser boots in. */
export type HubCollaborationHuman = Readonly<{ label: string; email: string; password: string }>;

/** 🔑️ The two humans, from `SEMIO_TWO_HUMAN_USER{1,2}_{EMAIL,PASSWORD}` only (the goal gate's variables, shared with the React
 * two-human matrix). Absent → the run is `blocked`. */
export function hubCollaborationHumans(env: NodeJS.ProcessEnv = process.env): readonly [HubCollaborationHuman, HubCollaborationHuman] {
  const humans = [1, 2].map((index) => ({ label: index === 1 ? "A" : "B", email: env[`SEMIO_TWO_HUMAN_USER${index}_EMAIL`] ?? "", password: env[`SEMIO_TWO_HUMAN_USER${index}_PASSWORD`] ?? "" }));
  if (!humans.every((human) => human.email && human.password)) throw new HubCollaborationBlocked("SEMIO_TWO_HUMAN_USER{1,2}_{EMAIL,PASSWORD} are not set");
  return humans as unknown as readonly [HubCollaborationHuman, HubCollaborationHuman];
}

/** 🚧️ A missing precondition (credential, hub, serve, native runtime): the record is `blocked`, not `fail`. */
export class HubCollaborationBlocked extends Error {}

/** 🎛️ One run. `wgpuServe` / `reactServe` name the serves to drive; `null` → the run starts its own through `ensureDevServe`
 * (see {@link provisionServe}); `nativeBinary` runs a prebuilt renderer test binary instead of `cargo test`. */
export type HubCollaborationOptionsV1 = Readonly<{
  repoRoot: string;
  journey: HubCollaborationJourneyV1;
  hub: string;
  wgpuServe: string | null;
  reactServe: string | null;
  locale: "en" | "de";
  humans: readonly [HubCollaborationHuman, HubCollaborationHuman];
  spaceId: string | null;
  documentId: string | null;
  nativeBinary: string | null;
  nativeModules: string | null;
  outDir: string;
  tag: string;
  signal: AbortSignal;
}>;

/** 📒️ One judged step. */
export type HubCollaborationStepV1 = Readonly<{ name: string; pass: boolean; atMs: number; detail: unknown }>;

/** 📜️ One run's report (`report.json`). */
export type HubCollaborationReportV1 = {
  schema: "semio.wgpu.hub-collaboration-report/v1";
  journey: HubCollaborationJourneyV1;
  locale: "en" | "de";
  hub: string;
  spaceId: string | null;
  documentId: string | null;
  startedAt: string;
  finishedAt: string | null;
  steps: HubCollaborationStepV1[];
  measured: Record<string, number | string | boolean>;
  fatal: string | null;
};

//#endregion 🔖️Contract

/** ✂️️ A severable TCP relay in front of the hub: a session that signs in through `origin` reaches the hub until `sever()`
 * destroys every open connection (the document socket included — a browser's offline switch leaves an open WebSocket
 * alive, ticket 26/09/23 run s14a) and refuses new ones until `heal()`. A real connection shortage, not an emulated one. */
export type SeverableRelayV1 = Readonly<{ origin: string; sever: () => number; heal: () => void; close: () => Promise<void> }>;
