/** 🎯️ Owns node, edge and handle selection facts. */
export interface DagSelectionDomains {readonly nodes:readonly string[];readonly edges:readonly string[];readonly handles:readonly string[]}
/** 🔌️ Identifies one semantic port on its widget. */
export interface DagChannelRef {readonly widgetId:string;readonly port:string;readonly direction:"in"|"out"}
/** 🚦️ Declares complete evaluation status facts before host publication. */
export type DagNodeEvaluationStatus={readonly status:"ok"|"queued"|"computing"}|{readonly status:"error";readonly message:string}|{readonly status:"blocked";readonly ports:readonly string[]};
/** 🗂️ Associates admitted status facts with semantic widget identities. */
export type DagNodeStatuses=Readonly<Record<string,DagNodeEvaluationStatus>>;
