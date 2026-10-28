/** 🌳 Owned canonical EXPRESS syntax; literals remain decimal text across implementations. */
export interface ExpressExpression {
  readonly kind: "integer"|"real"|"string"|"logical"|"unknown"|"identifier"|"unary"|"binary"|"attribute"|"qualify"|"call"|"index"|"slice"|"aggregate"|"query"|"interval";
  readonly offset: number;
  readonly text?: string;
  readonly operator?: string;
  readonly name?: string;
  readonly operands?: readonly ExpressExpression[];
  readonly entries?: readonly {readonly value:ExpressExpression;readonly repeat?:ExpressExpression}[];
}
/** 🪑 Formal and local variable declarations retain their EXPRESS type source. */
export interface ExpressVariable {readonly names:readonly string[];readonly type:string;readonly initial?:ExpressExpression}
/** 🎬 Owned imperative EXPRESS statement variants. */
export interface ExpressStatement {
  readonly kind:"assign"|"return"|"if"|"case"|"repeat"|"block"|"escape"|"skip"|"call"|"null";
  readonly offset:number;
  readonly name?:string;
  readonly expressions?:readonly ExpressExpression[];
  readonly body?:readonly ExpressStatement[];
  readonly otherwise?:readonly ExpressStatement[];
  readonly cases?:readonly {readonly labels:readonly ExpressExpression[];readonly body:readonly ExpressStatement[]}[];
  readonly while?:ExpressExpression;
  readonly until?:ExpressExpression;
}
/** 🏭 Callable EXPRESS function declaration. */
export interface ExpressFunction {readonly name:string;readonly parameters:readonly ExpressVariable[];readonly resultType:string;readonly locals:readonly ExpressVariable[];readonly body:readonly ExpressStatement[]}
/** ⚖ Named entity/type expression with its owning declaration. */
export interface ExpressRule {readonly owner:string;readonly label:string;readonly expression:ExpressExpression}
/** 🌐 Population rule evaluated after its LOCAL and statement body. */
export interface ExpressGlobalRule {readonly name:string;readonly populations:readonly string[];readonly locals:readonly ExpressVariable[];readonly body:readonly ExpressStatement[];readonly rules:readonly ExpressRule[]}
/** 🔗 A schema inverse relationship and declared cardinality. */
export interface ExpressInverse {readonly owner:string;readonly name:string;readonly type:string;readonly attribute:string;readonly aggregate?:boolean;readonly lower?:ExpressExpression;readonly upper?:ExpressExpression}
/** 📚 Canonical executable schema program shared by language implementations. */
export interface ExpressProgram {readonly schema:"IFC4X3_ADD2";readonly functions:readonly ExpressFunction[];readonly globalRules:readonly ExpressGlobalRule[];readonly rules:readonly ExpressRule[];readonly derived:readonly ExpressRule[];readonly unique:readonly ExpressRule[];readonly bounds:readonly ExpressRule[];readonly inverse:readonly ExpressInverse[]}
/** 🚦 Original caller controls compilation and execution limits. */
export interface ExpressControl {signal?:AbortSignal;batchSize?:number;maximumWork?:number;maximumDepth?:number;maximumIterations?:number;progress?:(progress:{phase:string;completed:number;total:number})=>void}
/** 📍 A failed or incomplete construct never becomes a semantic success. */
export interface ExpressDiagnostic {readonly code:string;readonly message:string;readonly owner?:string;readonly rule?:string;readonly offset?:number;readonly entityId?:bigint}
