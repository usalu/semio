/** 🗂️ Canvas2dHost JSON layers session: the class that interprets a `layersJson` scene into the retained canvas, its hit testing, selection and drawing. */
import { drawSceneNode, paintCompositedLayers, isDecodedImage } from "../🎨️paint/🟦️.ts";
import { type WasmCanvasSession, type CanvasInputModifiers } from "@semio-tech/framework/canvas/react-renderer";
import { fitCanvasFrame, type Canvas2dFraming, type MergeMode, type PinchStep } from "@semio-tech/framework";
import { type CanvasCamera, canvasPinchCamera, screenToWorldLogical, wheelCameraAtScreen } from "../📷️camera-math/🟦️.ts";
import { canvasLayerDisplayLabel, type CanvasLayerRecord, drawBoundsLayer, drawInfiniteCanvasGrid, layerBounds, layerColorCss, readCanvas2dSurfaceColors } from "../🖍️layer-paint/🟦️.ts";
import { type CanvasPointerDispatch, type CanvasPointerExtra, type CanvasPointerGestureLane, type CanvasPointerSample, createCanvasPointerGestureLane, modifiersForSelectionMode, pointerSample } from "../👆️pointer-lane/🟦️.ts";

//#region 🔖️Canvas2dHost
//#region JsonLayersCanvasSession
const sameCamera = (left: CanvasCamera, right: CanvasCamera): boolean => left.x === right.x && left.y === right.y && left.zoom === right.zoom;

/** 🖱️ Pixels a primary press may move before it becomes a pan rather than a pick. */
const CANVAS_PAN_DRAG_THRESHOLD_PX = 4;

export class JsonLayersCanvasSession implements WasmCanvasSession {
  private canvas: HTMLCanvasElement | null = null;
  private ctx: CanvasRenderingContext2D | null = null;
  private logicalWidth = 0;
  private logicalHeight = 0;
  /** 🖼️ The last framing this session applied; it follows the canvas size until the camera moves off it. */
  private framed: { readonly request: Canvas2dFraming; readonly camera: CanvasCamera } | null = null;
  private dpr = 1;
  private readonly imageCache = new Map<string, HTMLImageElement>();
  private readonly compositeSurfaces:CanvasRenderingContext2D[]=[];
  private panning = false;
  private panStart = { x: 0, y: 0 };
  private panCameraStart = { x: 0, y: 0 };
  private activeUtility = "selectDirect";
  private panButton = -1;
  private leftDragPans = false;
  private rightDragPans = false;
  private pendingLeft: { readonly x: number; readonly y: number; readonly modifiers?: CanvasInputModifiers } | null = null;
  private rightDragged = false;
  /** 🖱️ Per session (= per mounted host and per `dispatch` identity, since `sessionFactory` is memoised
   * on it): the gesture lane every non-pan pointer event goes through. */
  private readonly lane: CanvasPointerGestureLane | null;
  private gestureCounter = 0;
  private lastSample: CanvasPointerSample | null = null;
  private cancelledGestureAwaitingDown = false;

  private readLayersJson: () => string;
  private camera: CanvasCamera;
  private readonly onCameraChange: (camera: CanvasCamera) => void;
  private readonly onPointer?: CanvasPointerDispatch;

  constructor(
    readLayersJson: () => string,
    camera: CanvasCamera,
    onCameraChange: (camera: CanvasCamera) => void,
    onPointer?: CanvasPointerDispatch,
    private readonly readFraming?: () => Canvas2dFraming | null | undefined,
    private readonly readSelectionMode?: () => MergeMode | undefined,
  ) {
    this.readLayersJson = readLayersJson;
    this.camera = camera;
    this.onCameraChange = onCameraChange;
    this.onPointer = onPointer;
    this.lane = onPointer ? createCanvasPointerGestureLane(onPointer, () => ({ width: this.logicalWidth, height: this.logicalHeight })) : null;
  }

  /** 🖱️ The lane's own census, for laws and probes. */
  gestureLane(): CanvasPointerGestureLane | null {
    return this.lane;
  }

  syncLayersJson(): void {
    this.syncFraming();
    void this.preloadImages().then(() => this.renderFrame());
  }

  private layersJson(): string {
    return this.readLayersJson();
  }

  async attachCanvas(canvas: HTMLCanvasElement, logicalW: number, logicalH: number, dpr: number): Promise<unknown> {
    this.canvas = canvas;
    this.ctx = canvas.getContext("2d");
    this.logicalWidth = logicalW;
    this.logicalHeight = logicalH;
    this.dpr = dpr;
    this.syncFraming();
    await this.preloadImages();
    this.renderFrame();
    return undefined;
  }

  setSize(width: number, height: number, dpr: number): void {
    this.logicalWidth = width;
    this.logicalHeight = height;
    this.dpr = dpr;
    this.syncFraming();
  }

  private syncFraming(): void {
    const live = this.readFraming?.();
    const fresh = live && live.revision !== this.framed?.request.revision ? live : null;
    const request = fresh ?? (this.framed && sameCamera(this.camera, this.framed.camera) ? this.framed.request : null);
    const camera = request && fitCanvasFrame(request, this.logicalWidth, this.logicalHeight);
    if (!request || !camera) return;
    const publish = this.framed === null || !sameCamera(camera, this.camera);
    this.framed = { request, camera };
    if (!publish) return;
    this.camera = camera;
    this.onCameraChange(camera);
  }

  updateCamera(camera: CanvasCamera): void {
    this.camera = camera;
  }

  private parseLayers(): CanvasLayerRecord[] {
    try {
      return JSON.parse(this.layersJson()) as CanvasLayerRecord[];
    } catch {
      return [];
    }
  }

  private async preloadImages(): Promise<void> {
    const layers = this.parseLayers();
    const urls = new Set<string>();
    for (const layer of layers) {
      if (layer.kind === "image" && layer.dataUrl) urls.add(layer.dataUrl);
      if (layer.image?.src) urls.add(layer.image.src);
    }
    await Promise.all(
      [...urls].map(async (key) => {
        if (this.imageCache.has(key)) return;
        const image = new Image();
        image.decoding = "async";
        image.src = key;
        await image.decode().catch(() => undefined);
        this.imageCache.set(key, image);
      }),
    );
  }

  renderFrame(): void {
    const ctx = this.ctx;
    const canvas = this.canvas;
    if (!ctx || !canvas) return;
    const deviceWidth = canvas.width;
    const deviceHeight = canvas.height;
    const logicalWidth = this.logicalWidth;
    const logicalHeight = this.logicalHeight;
    const dpr = this.dpr;
    const zoom = this.camera.zoom || 1;
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, deviceWidth, deviceHeight);
    const surface = readCanvas2dSurfaceColors();
    ctx.fillStyle = surface.clear;
    ctx.fillRect(0, 0, deviceWidth, deviceHeight);
    const records = this.parseLayers();
    const meta = records.find((record) => record.role === "meta");
    if (meta?.utility) this.activeUtility = meta.utility;
    if (meta) {
      this.leftDragPans = meta.leftDragPans === true;
      this.rightDragPans = meta.rightDragPans === true;
    }
    const layers = records.filter((record) => record.role !== "meta");
    ctx.save();
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    ctx.translate(logicalWidth * 0.5 - this.camera.x * zoom, logicalHeight * 0.5 - this.camera.y * zoom);
    ctx.scale(zoom, zoom);
    drawInfiniteCanvasGrid(ctx, this.camera, logicalWidth, logicalHeight, zoom, surface.grid);
    paintCompositedLayers(ctx,layers,(ctx,layer,index)=>{
      if (layer.segments?.length || layer.text || layer.image?.src) {
        drawSceneNode(ctx, layer, this.imageCache);
        return;
      }
      if (layer.kind === "image" && layer.dataUrl) {
        const bounds = layerBounds(layer);
        const image = this.imageCache.get(layer.dataUrl);
        if (bounds && isDecodedImage(image)) {
          ctx.drawImage(image, bounds.x, bounds.y, bounds.width, bounds.height);
        }
        return;
      }
      if (layer.kind === "polyline" && layer.points?.length) {
        const seams = layer.seams ?? [];
        for (let segment = 0; segment + 1 < layer.points.length; segment += 2) {
          const [x0, y0] = layer.points[segment]!;
          const [x1, y1] = layer.points[segment + 1]!;
          const seamIndex = segment / 2;
          ctx.strokeStyle = layerColorCss(layer, (index * 47) % 360, 0.95);
          ctx.lineWidth = Math.max(1 / zoom, 1);
          ctx.setLineDash(seams[seamIndex] ? [6 / zoom, 4 / zoom] : []);
          ctx.beginPath();
          ctx.moveTo(x0, y0);
          ctx.lineTo(x1, y1);
          ctx.stroke();
        }
        ctx.setLineDash([]);
        return;
      }
      const bounds = layerBounds(layer);
      const label = canvasLayerDisplayLabel(layer);
      const hue = (index * 47) % 360;
      if (layer.kind === "line" || layer.x0 != null) {
        const x0 = layer.x0 ?? layer.x ?? 0;
        const y0 = layer.y0 ?? layer.y ?? 0;
        const x1 = layer.x1 ?? (layer.x ?? 0) + (layer.width ?? 0);
        const y1 = layer.y1 ?? (layer.y ?? 0) + (layer.height ?? 0);
        const isWire = layer.role === "wire";
        ctx.strokeStyle = layerColorCss(layer, hue, 0.9);
        ctx.lineWidth = Math.max((isWire ? 1.25 : 2) / zoom, 1 / zoom);
        ctx.setLineDash(isWire ? [6 / zoom, 4 / zoom] : []);
        ctx.beginPath();
        ctx.moveTo(x0, y0);
        ctx.lineTo(x1, y1);
        ctx.stroke();
        ctx.setLineDash([]);
        return;
      }
      if (bounds) {
        drawBoundsLayer(ctx, layer, bounds, label, hue, zoom);
      } else {
        ctx.fillStyle = "rgba(226, 232, 240, 0.75)";
        ctx.font = `${12 / zoom}px ui-monospace, monospace`;
        ctx.fillText(label, -logicalWidth / 2 + 16, -logicalHeight / 2 + 20 + index * 18);
      }
    },(depth,width,height)=>{
      let target:CanvasRenderingContext2D|undefined=this.compositeSurfaces[depth];
      if(!target) {
        const buffer=canvas.ownerDocument.createElement("canvas");
        target=buffer.getContext("2d")??undefined;
        if(!target) throw new Error("Canvas compositing surface is unavailable");
        this.compositeSurfaces[depth]=target;
      }
      if(target.canvas.width!==width) target.canvas.width=width;
      if(target.canvas.height!==height) target.canvas.height=height;
      return target;
    });
    if (layers.length === 0) {
      ctx.fillStyle = "rgba(148, 163, 184, 0.7)";
      ctx.font = `${12 / zoom}px ui-monospace, monospace`;
      ctx.fillText("Empty canvas", -36, 0);
    }
    ctx.restore();
  }

  /** 🧭️ One sample at `(x, y)`, its world position read under the camera the session holds now. */
  private sample(x: number, y: number, modifiers?: CanvasInputModifiers): CanvasPointerSample {
    return pointerSample(x,y,modifiersForSelectionMode(modifiers,this.readSelectionMode?.()),screenToWorldLogical(x,y,this.camera,this.logicalWidth,this.logicalHeight));
  }

  /** 🖱️ Whether a right drag panned since the last call — the context menu that trails it is swallowed, a plain right click still opens it. */
  consumeRightDragged(): boolean {
    const dragged = this.rightDragged;
    this.rightDragged = false;
    return dragged;
  }

  private beginPan(x: number, y: number, button: number): void {
    this.panning = true;
    this.panButton = button;
    this.panStart = { x, y };
    this.panCameraStart = { x: this.camera.x, y: this.camera.y };
  }

  private beginLane(x: number, y: number, button: number, modifiers?: CanvasInputModifiers): void {
    this.lastSample = this.sample(x,y,modifiers);
    this.gestureCounter += 1;
    this.lane?.begin(this.gestureCounter, this.lastSample, {
      button,
      shift: this.lastSample[2].shift ?? false,
      ctrl: this.lastSample[2].ctrl ?? false,
      meta: this.lastSample[2].meta ?? false,
      alt: this.lastSample[2].alt ?? false,
      width: this.logicalWidth,
      height: this.logicalHeight,
    });
  }

  pointerDown(x: number, y: number, button: number, _extend: boolean, modifiers?: CanvasInputModifiers): void {
    this.cancelledGestureAwaitingDown = false;
    this.rightDragged = false;
    this.pendingLeft = null;
    const rightPans = button === 2 && this.rightDragPans;
    if (this.activeUtility === "transform" && button !== 1 && !rightPans) {
      return;
    }
    if (button === 1 || rightPans || this.activeUtility === "transformMove") {
      this.beginPan(x, y, button);
      return;
    }
    const held = modifiersForSelectionMode(modifiers, this.readSelectionMode?.());
    if (button === 0 && this.leftDragPans && !(held.shift || held.ctrl || held.meta)) {
      this.pendingLeft = { x, y, modifiers };
      return;
    }
    this.beginLane(x, y, button, modifiers);
  }

  pointerMove(x: number, y: number, modifiers?: CanvasInputModifiers): void {
    if (this.pendingLeft) {
      if (Math.hypot(x - this.pendingLeft.x, y - this.pendingLeft.y) < CANVAS_PAN_DRAG_THRESHOLD_PX) return;
      this.beginPan(this.pendingLeft.x, this.pendingLeft.y, 0);
      this.pendingLeft = null;
    }
    if (this.panning) {
      if (this.panButton === 2 && Math.hypot(x - this.panStart.x, y - this.panStart.y) >= CANVAS_PAN_DRAG_THRESHOLD_PX) this.rightDragged = true;
      const zoom = this.camera.zoom || 1;
      const next = {
        ...this.camera,
        x: this.panCameraStart.x - (x - this.panStart.x) / zoom,
        y: this.panCameraStart.y - (y - this.panStart.y) / zoom,
      };
      this.camera = next;
      this.onCameraChange(next);
      this.renderFrame();
      return;
    }
    if (this.cancelledGestureAwaitingDown) return;
    this.lastSample = this.sample(x,y,modifiers);
    this.lane?.offer(this.lastSample);
  }

  pointerUp(x: number, y: number, modifiers?: CanvasInputModifiers): void {
    if (this.pendingLeft) {
      const press = this.pendingLeft;
      this.pendingLeft = null;
      this.beginLane(press.x, press.y, 0, press.modifiers);
    }
    if (this.panning) {
      this.panning = false;
      return;
    }
    if (this.cancelledGestureAwaitingDown) return;
    this.lastSample = this.sample(x,y,modifiers);
    const extra: CanvasPointerExtra = {
      shift: this.lastSample[2].shift ?? false,
      ctrl: this.lastSample[2].ctrl ?? false,
      meta: this.lastSample[2].meta ?? false,
      alt: this.lastSample[2].alt ?? false,
      width: this.logicalWidth,
      height: this.logicalHeight,
    };
    if (this.lane?.activeGesture() != null) {
      this.lane.end(this.lastSample, extra);
      return;
    }
    void this.onPointer?.("canvasPointerUp", { x, y, ...extra, cancelled: false });
  }

  /** 🚫️ Pointer left the canvas / capture lost / browser reclaimed the pointer: ends a pan locally and
   * turns an open gesture into `canvasPointerUp { cancelled: true }`. Outside a gesture this is a no-op —
   * the pre-lane `pointerleave → pointerUp` mapping forged a release here. */
  pointerCancel(): void {
    this.pendingLeft = null;
    if (this.panning) {
      this.panning = false;
      return;
    }
    const active = this.lane?.activeGesture() != null;
    this.lane?.cancel(this.lastSample);
    if (active) this.cancelledGestureAwaitingDown = true;
  }

  doubleClick(x: number, y: number): void {
    this.onPointer?.("canvasDoubleClick", { x, y, width: this.logicalWidth, height: this.logicalHeight });
  }

  pinch(step: PinchStep): void {
    const next = canvasPinchCamera(this.camera, step, this.logicalWidth, this.logicalHeight);
    this.camera = next;
    this.onCameraChange(next);
    this.renderFrame();
  }

  wheel(x: number, y: number, deltaY: number): void {
    const next = wheelCameraAtScreen(this.camera, x, y, deltaY, this.logicalWidth, this.logicalHeight);
    this.camera = next;
    this.onCameraChange(next);
    this.renderFrame();
  }
}
//#endregion JsonLayersCanvasSession
//#endregion 🔖️Canvas2dHost
