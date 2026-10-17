/**
 * 🌊️ The flow session surface the renderer drives, declared by the renderer: only the members its flow surfaces call. The wasm package that implements it is supplied by the composition root through the wasm bindings port, which is type-checked there against this shape.
 * @see ../🟦️.tsx
 */

/** 📨️ One progress event of a running flow task. */
export interface FlowTaskEvent { readonly tag: 3; readonly requestId: bigint; readonly generation: number; readonly sequence: number; readonly event: number; readonly status: number; readonly body: Uint8Array; }

/** ⏳️ A cancellable flow task: its result and its event stream. */
export interface FlowTask<T = unknown> { readonly result: Promise<T>; cancel(): boolean; subscribe(observer: (event: FlowTaskEvent) => void): () => void; }

/** 🕸️ The members of one flow session the renderer calls. */
export interface FlowWasmSession {
  alignSelection(mode: string): FlowTask<unknown>;
  alignSelection(args: { readonly mode: string }): FlowTask<unknown>;
  applyEvalOutputsJson(json: string): FlowTask<unknown>;
  applyEvalOutputsJson(args: { readonly json: string }): FlowTask<unknown>;
  attachCanvas(canvas: HTMLCanvasElement, width: number, height: number, dpr: number): FlowTask<unknown>;
  catalogueJson(): FlowTask<unknown>;
  clearGhostWidget(): FlowTask<unknown>;
  free(): Promise<void>;
  hoveredChannelJson(): FlowTask<unknown>;
  hoveredWidgetId(): FlowTask<unknown>;
  labelOverlayPaintStateJson(): FlowTask<unknown>;
  pickTargetsAtScreenJson(sx: number, sy: number): FlowTask<unknown>;
  pickTargetsAtScreenJson(args: { readonly sx: number; readonly sy: number }): FlowTask<unknown>;
  pointerCancelScreen(): FlowTask<unknown>;
  pointerDownScreen(sx: number, sy: number, button: number, shift: boolean, ctrlOrMeta: boolean, alt: boolean, pan: boolean): FlowTask<unknown>;
  pointerDownScreen(args: { readonly sx: number; readonly sy: number; readonly button: number; readonly shift: boolean; readonly ctrlOrMeta: boolean; readonly alt: boolean; readonly pan: boolean }): FlowTask<unknown>;
  pointerMoveScreen(sx: number, sy: number, shift: boolean, ctrlOrMeta: boolean, alt: boolean): FlowTask<unknown>;
  pointerMoveScreen(args: { readonly sx: number; readonly sy: number; readonly shift: boolean; readonly ctrlOrMeta: boolean; readonly alt: boolean }): FlowTask<unknown>;
  pointerUpScreen(sx: number, sy: number, shift: boolean, ctrlOrMeta: boolean, alt: boolean): FlowTask<unknown>;
  pointerUpScreen(args: { readonly sx: number; readonly sy: number; readonly shift: boolean; readonly ctrlOrMeta: boolean; readonly alt: boolean }): FlowTask<unknown>;
  preselectWidgetIdsJson(): FlowTask<unknown>;
  previewOffWidgetIds(): FlowTask<unknown>;
  renderCanvas(canvas: HTMLCanvasElement): FlowTask<unknown>;
  selectedWidgetIds(): FlowTask<unknown>;
  selectionDomainsJson(): FlowTask<unknown>;
  selectionPreviewCrossing(): FlowTask<unknown>;
  selectionPreviewMethod(): FlowTask<unknown>;
  selectionPreviewPointsJson(): FlowTask<unknown>;
  selectionUnionBoundsScreenJson(): FlowTask<unknown>;
  setAutomaticLod(enabled: boolean): FlowTask<unknown>;
  setAutomaticLod(args: { readonly enabled: boolean }): FlowTask<unknown>;
  setCamera(x: number, y: number, zoom: number): FlowTask<unknown>;
  setCamera(args: { readonly x: number; readonly y: number; readonly zoom: number }): FlowTask<unknown>;
  setCanvasThemeJson(json: string): FlowTask<unknown>;
  setCanvasThemeJson(args: { readonly json: string }): FlowTask<unknown>;
  setCatalogueJson(json: string): FlowTask<unknown>;
  setCatalogueJson(args: { readonly json: string }): FlowTask<unknown>;
  setComputingProgress(json: string): FlowTask<unknown>;
  setComputingProgress(args: { readonly json: string }): FlowTask<unknown>;
  setForcedDrawLodLabel(label: string): FlowTask<unknown>;
  setForcedDrawLodLabel(args: { readonly label: string }): FlowTask<unknown>;
  setGhostWidget(descriptorJson: string, worldX: number, worldY: number): FlowTask<unknown>;
  setGhostWidget(args: { readonly descriptorJson: string; readonly worldX: number; readonly worldY: number }): FlowTask<unknown>;
  setHover(widgetId?: string | null): FlowTask<unknown>;
  setHover(args: { readonly widgetId?: string | null }): FlowTask<unknown>;
  setHoverChannel(widgetId?: string | null, port?: string | null): FlowTask<unknown>;
  setHoverChannel(args: { readonly widgetId?: string | null; readonly port?: string | null }): FlowTask<unknown>;
  setNeuronKindInfosJson(json: string): FlowTask<unknown>;
  setNeuronKindInfosJson(args: { readonly json: string }): FlowTask<unknown>;
  setNodeStatuses(json: string): FlowTask<unknown>;
  setNodeStatuses(args: { readonly json: string }): FlowTask<unknown>;
  setPreviewOff(json: string): FlowTask<unknown>;
  setPreviewOff(args: { readonly json: string }): FlowTask<unknown>;
  setSelection(json: string): FlowTask<unknown>;
  setSelection(args: { readonly json: string }): FlowTask<unknown>;
  setSize(width: number, height: number, dpr: number): FlowTask<unknown>;
  setSize(args: { readonly width: number; readonly height: number; readonly dpr: number }): FlowTask<unknown>;
  setSliderValue(widgetId: string, value: number): FlowTask<unknown>;
  setSliderValue(args: { readonly widgetId: string; readonly value: number }): FlowTask<unknown>;
  sliderOverlayStateJson(): FlowTask<unknown>;
  surfaceStatus(surface: number, surfaceGeneration: number, status: string): FlowTask<unknown>;
  surfaceStatus(args: { readonly surface: number; readonly surfaceGeneration: number; readonly status: string }): FlowTask<unknown>;
  synchronizeSnapshotJson(json: string): FlowTask<unknown>;
  synchronizeSnapshotJson(args: { readonly json: string }): FlowTask<unknown>;
  viewport(): FlowTask<unknown>;
  wheelScreen(sx: number, sy: number, deltaX: number, deltaY: number, zoomGesture: boolean): FlowTask<unknown>;
  wheelScreen(args: { readonly sx: number; readonly sy: number; readonly deltaX: number; readonly deltaY: number; readonly zoomGesture: boolean }): FlowTask<unknown>;
  worldFromScreen(sx: number, sy: number): FlowTask<unknown>;
  worldFromScreen(args: { readonly sx: number; readonly sy: number }): FlowTask<unknown>;
}

/** 🏃️ A started flow browser runtime: opens sessions. */
export interface FlowBrowserRuntimeV1 { openSession(): FlowWasmSession; }
