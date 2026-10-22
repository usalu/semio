export type FormatStampRowV1=Readonly<{id:string;own:string;live:string}>;
export type FormatStampEntryV1=Readonly<{id:string;schema:string;pointer:string;member?:string}>;
export type FormatStampTableV1=Readonly<{exitCode:number;maximumReexecutions:number;reexecutionEnvironment:string;admissionGuardRegistry:string;maximumSchemaBytes:number;readAttempts:number;readRetryMilliseconds:number;settleMilliseconds:number;maximumReportedStamps:number;languages:readonly string[];stamps:readonly FormatStampEntryV1[];resources:readonly Readonly<{prefix:string;stamps:readonly string[]}>[];texts:Readonly<Record<string,Readonly<Record<string,string>>>>}>;
export type FormatStampAdmissionV1=Readonly<{directory?:string;resource:string;mode:"shared"|"exclusive";phase?:"admitted"|"waiting"}>;
export type FormatStampKindV1="yield"|"refuse";
export interface FormatStampsV1{declare(id:string,document:unknown):void;owned():Map<string,unknown>;live(id:string):unknown;moved(ids:Iterable<string>):FormatStampRowV1[];ensureCurrent(ids?:Iterable<string>,context?:string,kind?:FormatStampKindV1):void;idsOf(resource:string):string[];guard(admission:FormatStampAdmissionV1):void}
export function formatStampProtocolV1():FormatStampTableV1;
export function formatStampLanguagesV1(environment?:Readonly<Record<string,string|undefined>>):string[];
export function formatStampTextV1(language:string,key:string,values?:Readonly<Record<string,string|number>>):string;
export function renderStaleImplementationV1(languages:readonly string[],moved:readonly FormatStampRowV1[],context?:string,kind?:FormatStampKindV1):string;
export class StaleImplementationErrorV1 extends Error{constructor(moved:readonly FormatStampRowV1[],options?:{context?:string;kind?:FormatStampKindV1;languages?:readonly string[]});readonly code:"SEMIO_STALE_IMPLEMENTATION";readonly moved:readonly FormatStampRowV1[];readonly context:string;readonly kind:FormatStampKindV1}
export function createFormatStampsV1(options?:{base:string;table?:FormatStampTableV1;environment?:Readonly<Record<string,string|undefined>>;read?:(path:string)=>unknown}):FormatStampsV1;
export function registerFormatStampGuardV1(registry:Pick<FormatStampsV1,"guard">,target?:object):()=>boolean;
export function staleImplementationOfV1(error:unknown):{origin:"error"|"exit";moved:readonly FormatStampRowV1[];message:string}|undefined;
export function exitIfStaleImplementationV1(error:unknown,options?:{write?:(text:string)=>void;exit?:(code:number)=>never|void}):void;
export const formatStampsV1:FormatStampsV1;
export function declareFormatStampV1(id:string,document:unknown):void;
export function requireCurrentFormatStampsV1(ids?:Iterable<string>,context?:string,kind?:FormatStampKindV1):void;
