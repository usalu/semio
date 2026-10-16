/** 🎯️ Owns node, edge and handle selection facts. */
export interface DagSelectionDomains {readonly nodes:readonly string[];readonly edges:readonly string[];readonly handles:readonly string[]}
/** 🔌️ Identifies one semantic port on its widget. */
export interface DagChannelRef {readonly widgetId:string;readonly port:string;readonly direction:DagChannelDirection}
/** 🚦️ Declares complete evaluation status facts before host publication. */
export type DagNodeEvaluationStatus={readonly status:"ok"|"queued"|"computing"}|{readonly status:"error";readonly message:string}|{readonly status:"blocked";readonly ports:readonly string[]};
/** 🗂️ Associates admitted status facts with semantic widget identities. */
export type DagNodeStatuses=Readonly<Record<string,DagNodeEvaluationStatus>>;

/** 🔱️ Identifies a semantic graph selection relation. */
export type DagSelectionDomain="nodes"|"edges";
/** 👁️ Borrows candidate identities independently of physical output. */
export interface DagSelectionSource {selectionCandidateCount(domain:DagSelectionDomain):number;selectionCandidateId(domain:DagSelectionDomain,index:number):string|undefined}

/** 🧭️ Declares complete semantic port direction. */
export type DagChannelDirection="in"|"out";

/** 🚫️ Identifies a refused port pair and its declared semantic value types. */
export interface DagWireTypeRefusal {readonly source:string;readonly sourceTypes:readonly string[];readonly target:string;readonly targetTypes:readonly string[]}
/** 🖱️ Projects hover and wire refusal facts from the same accepted graph state. */
export interface DagHoverFacts {readonly channel:DagChannelRef|null;readonly refusal:DagWireTypeRefusal|null}
