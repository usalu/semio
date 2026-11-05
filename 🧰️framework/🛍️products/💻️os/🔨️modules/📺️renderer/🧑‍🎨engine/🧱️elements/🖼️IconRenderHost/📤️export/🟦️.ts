/** 📤️ Event-sourced host batches for rendered image downloads. */
export type IconExportOutcome = "done" | "cancelled" | "failed";
export type IconExportRow = Readonly<{job:number;owner:string;label:string;total:number;completed:number;atMs:number;cancelling:boolean}>;
export type IconExportEvent = Readonly<{kind:"started";job:number;owner:string;label:string;total:number;atMs:number}> | Readonly<{kind:"progressed";job:number;completed:number}> | Readonly<{kind:"cancelRequested";job:number}> | Readonly<{kind:"finished";job:number;outcome:IconExportOutcome}>;

/** 📜️ Folds admitted job facts into the live task rows. */
export class IconExportLedger {
  private rows: readonly IconExportRow[] = [];
  private events: IconExportEvent[] = [];
  private lastJob = 0;
  apply(event: IconExportEvent): void {
    const row = this.rows.find(row => row.job === event.job);
    if (event.kind === "started") {
      if (!Number.isSafeInteger(event.job) || event.job <= this.lastJob || !Number.isSafeInteger(event.total) || event.total < 0 || !Number.isFinite(event.atMs)) throw new Error("Invalid image export start");
      this.lastJob = event.job;
      this.rows = [...this.rows, {job:event.job,owner:event.owner,label:event.label,total:event.total,atMs:event.atMs,completed:0,cancelling:false}];
    } else {
      if (!row) throw new Error("Image export job is not running");
      if (event.kind === "progressed") {
        if (!Number.isSafeInteger(event.completed) || event.completed <= row.completed || event.completed > row.total || row.cancelling) throw new Error("Invalid image export progress");
        this.rows = this.rows.map(current => current === row ? {...row, completed:event.completed} : current);
      } else if (event.kind === "cancelRequested") {
        if (row.cancelling) throw new Error("Image export is already cancelling");
        this.rows = this.rows.map(current => current === row ? {...row, cancelling:true} : current);
      } else {
        if (event.outcome === "done" && (row.completed !== row.total || row.cancelling)) throw new Error("Image export is incomplete");
        this.rows = this.rows.filter(current => current !== row);
      }
    }
    this.events.push(event);
    if (this.events.length > 256) this.events.splice(0, this.events.length - 256);
  }
  snapshot(): readonly IconExportRow[] { return this.rows; }
  log(): readonly IconExportEvent[] { return this.events.slice(); }
}
const ledger = new IconExportLedger();
const listeners = new Set<() => void>();
let nextJob = 0;
function record(event: IconExportEvent): void {
  ledger.apply(event);
  for (const listener of [...listeners]) { try { listener(); } catch(error) { console.error("Image export observer failed",error); } }
}
/** 🧵️ Reads the stable active image export task rows. */
export function iconExportJobsSnapshot(): readonly IconExportRow[] { return ledger.snapshot(); }
/** 🔔️ Subscribes to image export facts. */
export function subscribeIconExportJobs(listener: () => void): () => void { listeners.add(listener); return () => listeners.delete(listener); }
/** 🏷️ Identifies one image export in Task Manager. */
export function iconExportTaskId(job: number): string { return `export:icon#${job}`; }
/** 📖️ Reads the bounded tail of image export facts. */
export function iconExportJobLog(): readonly IconExportEvent[] { return ledger.log(); }
/** 🛑️ Requests cancellation through the job's event log. */
export function cancelIconExport(taskId: string): boolean {
  if (!taskId.startsWith("export:icon#")) return false;
  const job = Number(taskId.slice("export:icon#".length)), row = ledger.snapshot().find(row => row.job === job);
  if (!row || row.cancelling) return false;
  record({kind:"cancelRequested",job});
  return true;
}
const labels = {
  started:{en:"Rendering images — progress and cancel in Task Manager.",de:"Bilder werden gerendert — Fortschritt und Abbruch im Task-Manager."},
  done:{en:"Images exported.",de:"Bilder exportiert."},
  cancelled:{en:"Image export cancelled.",de:"Bildexport abgebrochen."},
  failed:{en:"Image export failed",de:"Bildexport fehlgeschlagen"},
} as const;
/** 🌐️ Resolves image export notices in the shell's explicit language. */
export function iconExportText(kind:keyof typeof labels,locale:string):string {
  if(locale!=="en"&&locale!=="de")throw new Error("Unsupported image export language");
  return labels[kind][locale];
}
/** 📏️ Describes delivered images in the shell's explicit language. */
export function iconExportProgressText(completed:number,total:number,locale:string):string {
  iconExportText("started",locale);
  return locale==="de" ? `Bild ${completed} von ${total}` : `Image ${completed} of ${total}`;
}
/** 🖼️ Renders a batch in order, yielding between images and withholding a cancelled image's download. */
export async function runIconExportBatch<Request, Result>(
  items:readonly Readonly<{filename:string;request:Request}>[],
  host:Readonly<{owner:string;locale:()=>string;render:(request:Request)=>Promise<Result>;deliver:(filename:string,result:Result)=>void;yieldToHost:()=>Promise<void>;announce?:(text:string,outcome?:IconExportOutcome)=>void}>
):Promise<Readonly<{outcome:IconExportOutcome;completed:number;reason?:string}>> {
  const locale=host.locale(),job=++nextJob;
  iconExportText("started",locale);
  record({kind:"started",job,owner:host.owner,label:items[0]?.filename??"",total:items.length,atMs:Date.now()});
  let completed=0;
  const announce=(text:string,outcome?:IconExportOutcome)=>{try{host.announce?.(text,outcome);}catch(error){console.error("Image export announcement failed",error);}};
  const cancelled=()=>ledger.snapshot().find(row=>row.job===job)?.cancelling===true;
  const finish=(outcome:IconExportOutcome,reason?:string)=>{
    record({kind:"finished",job,outcome});
    announce(iconExportText(outcome,locale)+(reason?": "+reason:""),outcome);
    return {outcome,completed,...(reason?{reason}:{})};
  };
  try {
    announce(iconExportText("started",locale));
    for(const item of items){
      await host.yieldToHost();
      if(cancelled())return finish("cancelled");
      const result=await new Promise<Result>((resolve,reject)=>{
        const unsubscribe=subscribeIconExportJobs(()=>{
          if(cancelled()){unsubscribe();reject(new Error("Image export cancelled"));}
        });
        Promise.resolve().then(()=>{if(cancelled())throw new Error("Image export cancelled");return host.render(item.request);}).then(value=>{unsubscribe();resolve(value);},error=>{unsubscribe();reject(error);});
      });
      if(cancelled())return finish("cancelled");
      host.deliver(item.filename,result);
      completed++;
      record({kind:"progressed",job,completed});
    }
    return finish(cancelled()?"cancelled":"done");
  } catch(error) {
    return cancelled()?finish("cancelled"):finish("failed",error instanceof Error?error.message:String(error));
  }
}
