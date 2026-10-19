/** 🕸️ Wasm session contract between the framework's board hosts and the plugin-owned engines they drive: the loader every wasm module uses, the board session an engine implements and the factory a plugin registers for one of its apps. */

/** 📦️ Deduplicates in-flight module initialization and retires only the exact failed attempt. */
export function createWasmModuleLoader<T>(load: () => Promise<T>): () => Promise<T> {
  let pending: Promise<T> | undefined;
  return () => {
    if (pending) return pending;
    const attempt = Promise.resolve().then(load);
    pending = attempt;
    void attempt.catch(() => { if (pending === attempt) pending = undefined; });
    return attempt;
  };
}


/** 🧩️ The board engine surface a plugin-owned wasm module implements; optional members are capabilities a host feature-detects. */
export type Board2dWasmSession = {
  attach_canvas(canvas: HTMLCanvasElement, logicalW: number, logicalH: number, dpr: number): Promise<unknown>;
  setSize(width: number, height: number, dpr: number): void;
  renderFrame(): void;
  loadBoardSnapshotJson(json: string): boolean;
  syncDescriptorJson(json: string): void;
  setKindCatalogsJson(json: string): void;
  setCamera(x: number, y: number, zoom: number): void;
  setSelectionIdsJson(json: string): void;
  setCanvasThemeJson(json: string): void;
  pointerDownScreen(sx: number, sy: number, button: number, shift: boolean, ctrlOrMeta: boolean): void;
  pointerMoveScreen(sx: number, sy: number, shift: boolean, ctrlOrMeta: boolean, alt: boolean): void;
  pointerUpScreen(sx: number, sy: number, shift: boolean, ctrlOrMeta: boolean, alt: boolean): void;
  pointerCancelScreen(): void;
  wheelScreen(sx: number, sy: number, deltaY: number): void;
  drainEventsJson(): string;
  cameraJson(): string;
  gpuReady(): boolean;
  setHoveredIdSilent?(id?: string | null): void;
  /** 🔗️ Paints the ids a time-travel draft references highlighted, as a JSON id array (`[]` clears them). */
  setHighlightedIdsJson?(json: string): void;
  setActiveUtility?(label: string): void;
  setSelectionOptions?(method: string, mode: string, selectNodes: boolean, selectEdges: boolean, selectHandles: boolean): void;
  setGridVisible?(visible: boolean): void;
  setGridSnapEnabled?(enabled: boolean): void;
  setGridFactor?(v: number): void;
  setSuggestionOffset?(distance: number): void;
  setBrushKindWeights?(json: string): void;
  setHandleLinkCompatJson?(json: string): void;
  setAutomaticLod?(enabled: boolean): void;
  setForcedDrawLodLabel?(label: string): void;
  setSelectionIdsJsonSilent?(json: string): void;
  setCameraSilent?(x: number, y: number, zoom: number): void;
  pointerLeaveScreen?(alt: boolean): void;
  pickTargetsAtScreenJson?(sx: number, sy: number): string;
  deleteSelection?(): void;
  cancelAreaSelect?(): boolean;
  brushCycleCandidate?(forward: boolean): void;
  /** 💡️ The brush slot the handle-suggestions popup drives locally: opening it paints the first
   * candidate provisionally, `brushSetCandidateIndex` previews another, `brushCancelSlot` discards. The
   * COMMIT never runs here — accepting goes through the guest so the placement is one document edit. */
  brushOpenSlot?(handleId: string): void;
  brushSetCandidateIndex?(index: number): void;
  brushCancelSlot?(): void;
  /** 🕹️ Composes the select utility's gumball handles; scale is deliberately absent on boards. */
  setTransformFlags?(moveEnabled: boolean, rotateEnabled: boolean): void;
  /** 🩺️ Gumball vitals (`{move,rotate,ringVisible,dragging,radians,pivot,radius}`). */
  transformGumballJson?(): string;
  /** 🩺️ Live gesture vitals (`{mode,utility,hoveredId,selectionCount,preselectCount,revision}`). */
  interactionJson?(): string;
  /** 🖍️ World extent one area-brush CLICK paints; a click-drag states its own rectangle. */
  setAreaBrushExtent?(width: number, height: number): void;
  /** 🏷️ Installs (a string) or clears (`null`) the scene's brush placement prefixes in the board's wire form (`Board2dScene.placementNaming`). */
  setPlacementNamingJson?(json: string | null): void;
  /** 🩺️ Every target region the board holds (`[{id,x,y,width,height,label?,hidden,locked,selected}]`). */
  targetRegionsJson?(): string;
  /** 🩺️ On-screen handle vitals (`{total,onScreen,published,capped,rows:[[id,x,y,nodeId,handleKind,open]]}`) — the only DOM channel that names a handle. */
  handlePositionsJson?(): string;
  /** 🖱️ Transitive same-kind hover: every element of `kindId` in `domain` paints hovered, with no board event and no guest round trip. Both `null` clears it. */
  setHoveredKindSilent?(domain: string | null, kindId: string | null): void;
  setDropPreviewJson?(json: string): void;
  clearDropPreview?(): void;
  defersDescriptorSyncFromJs?(): boolean;
  isDraggingAreaSelect?(): boolean;
  /** 🐢️ Silent cross-pane mirror setters (WS-live-sync round 4) — move nodes/set preselect/set the marquee outline without emitting board events or a fixture reset, so a peer pane can mirror another pane's live gesture without round-tripping through the program. */
  setNodePositionsJson?(json: string): void;
  setPreselectStateJsonSilent?(json: string): void;
  setSelectionScreenPreview?(flatXy: Float64Array): void;
  clearSelectionScreenPreview?(): void;
  free(): void;
};

/** 🪪️ Registers one plugin-owned board engine constructor for the app that opens it. */
export type AppSurfaceSessionFactory = {
  readonly kind: "board-2d";
  readonly pluginId: string;
  readonly appId: string;
  readonly create: () => Promise<Board2dWasmSession>;
};
