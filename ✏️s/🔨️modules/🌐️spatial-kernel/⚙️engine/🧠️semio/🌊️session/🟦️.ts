/** 🌐️ One CAD engine owns one concrete Rust geometry session and its bounded retirement cursor. */
export interface BrowserGeometrySession {
  brep_invoke(method: string, argumentsJson: string): string;
  begin_close(): void;
  close_step(maximumItems:number,maximumCopyBytes:number,maximumCapacityBytes:number,maximumReleaseBytes:number,maximumDepth:number):string;
  next_close_copy_byte_demand():number;
  next_close_capacity_byte_demand(copyBytes:number):number;
  next_close_release_byte_demand():number;
  next_close_depth_demand():number;
  cancel_close(): void;
  resume_close(): void;
  terminal_is_empty(): boolean;
  free(): void;
}
export interface GeometryCloseReceipt { phase:"blocked" | "pending" | "complete"; items:number; copyBytes:number; capacityBytes:number; releaseBytes:number; }
export interface GeometryCloseOptions { maximumItems?:number; maximumCopyBytes?:number; maximumCapacityBytes?:number; maximumReleaseBytes?:number; maximumDepth?:number; cancelled?:() => boolean; onProgress?:(receipt:GeometryCloseReceipt) => void; }
async function initializeBindings() {
  const bindings = await import("./📦️packages/🦀️rust/🕸️bindings/semio_session.js");
  const source = new URL("./📦️packages/🦀️rust/🕸️bindings/semio_session_bg.wasm", import.meta.url);
  const module_or_path = source.protocol === "file:" ? await (await import("node:fs/promises")).readFile(source) : source;
  await bindings.default({ module_or_path });
  return bindings;
}
let initializedBindings:ReturnType<typeof initializeBindings> | undefined;
async function openSession(): Promise<BrowserGeometrySession> {
  const bindings = await (initializedBindings ??= initializeBindings());
  return new bindings.BrowserSession();
}
export class SemioGeometrySession {
  private readonly session:Promise<BrowserGeometrySession>;
  constructor(factory:() => Promise<BrowserGeometrySession> = openSession) { this.session=factory(); }
  private readonly lifecycle: { closed:boolean; terminal:boolean; closing:Promise<void> | null } = { closed:false,terminal:false,closing:null };
  async invoke<T = unknown>(method: string, args: Record<string, unknown>): Promise<T> {
    const session = await this.session;
    if (this.lifecycle.closed) throw new Error("geometry.session-closed");
    const raw = JSON.parse(session.brep_invoke(method, JSON.stringify(args))) as { error?: string };
    if (raw.error) throw new Error(`semio.geometry.${method}: ${raw.error}`);
    return raw as T;
  }
  close(options:GeometryCloseOptions = {}): Promise<void> {
    if (this.lifecycle.terminal) return Promise.resolve();
    if (this.lifecycle.closing) return this.lifecycle.closing;
    const maximumItems = options.maximumItems ?? 1;
    const explicit=[options.maximumCopyBytes,options.maximumCapacityBytes,options.maximumReleaseBytes,options.maximumDepth].filter((value):value is number=>value!==undefined);
    if (!Number.isSafeInteger(maximumItems) || maximumItems<1 || maximumItems>0xffff_ffff || !explicit.every(value=>Number.isSafeInteger(value) && value>=0 && value<=0xffff_ffff)) return Promise.reject(new Error("geometry.close-positive-grants-required"));
    this.lifecycle.closed = true;
    const operation = this.retire(maximumItems,options);
    this.lifecycle.closing = operation;
    void operation.then(() => { this.lifecycle.closing = null; },() => { this.lifecycle.closing = null; });
    return operation;
  }
  private async retire(maximumItems:number,options:GeometryCloseOptions): Promise<void> {
    const session = await this.session;
    session.begin_close(); session.resume_close();
    for (;;) {
      if (options.cancelled?.()) { session.cancel_close(); throw new Error("geometry.close-cancelled"); }
      const copyDemand=session.next_close_copy_byte_demand();
      const capacityDemand=session.next_close_capacity_byte_demand(copyDemand);
      const releaseDemand=session.next_close_release_byte_demand();
      const depthDemand=session.next_close_depth_demand();
      const copy=options.maximumCopyBytes ?? copyDemand,capacity=options.maximumCapacityBytes ?? capacityDemand,release=options.maximumReleaseBytes ?? releaseDemand,depth=options.maximumDepth ?? depthDemand;
      if (copy<copyDemand || capacity<capacityDemand || release<releaseDemand || depth<depthDemand) throw new Error("geometry.close-grant-refused");
      const receipt = JSON.parse(session.close_step(maximumItems,copy,capacity,release,depth)) as GeometryCloseReceipt & { error?:string };
      if (receipt.error) throw new Error(receipt.error);
      options.onProgress?.(receipt);
      if (receipt.phase === "complete") {
        if (!session.terminal_is_empty()) throw new Error("geometry.close-terminal-owner-retained");
        session.free(); this.lifecycle.terminal = true; return;
      }
      await new Promise<void>(resolve => setTimeout(resolve,0));
    }
  }
}
