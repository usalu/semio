/** 🧪️ Source-order records and scalar folds remain cooperative without reassociating binary64 addition. */
import {test,expect} from 'bun:test';
import Ajv from 'ajv/dist/2020';
import {Database} from 'bun:sqlite';
import cases from './source-rows-cases.json';
import schema from './source-rows.schema.json';
import {femConnectivityFixture} from './fem-isolated-fixtures.ts';
import {ifcBimAxisFixture} from './bim-axis-fixtures.ts';
import {fem3dToIfc4x3} from '../fem.ts';
import {bimSpatialToIfc4x3} from '../bim.ts';
import type {IfcIoControl} from '../control.ts';
const load=()=>import('../source-rows.ts'),drain=<T>(g:Generator<void,T,void>):T=>{for(;;){const next=g.next();if(next.done)return next.value;}};
test('neutral source-order laws have a closed schema',()=>expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true));
for(const [i,row]of cases.entries.entries())test('record entry order '+i,async()=>{const {ifcSourceEntriesSteps}=await load(),actual=drain(ifcSourceEntriesSteps(row.source));expect(actual as unknown).toEqual(row.expected);const db=new Database(':memory:');try{expect(db.query('SELECT key,value FROM json_each(?)').all(JSON.stringify(row.source)).map(row=>Object.values(row as object))).toEqual(actual);}finally{db.close();}});
for(const [i,row]of cases.levels.entries())test('cumulative source arithmetic '+i,async()=>{const {ifcLevelElevationSteps}=await load(),actual=drain(ifcLevelElevationSteps(row.rows,row.query));expect(actual).toBe(row.expected);const db=new Database(':memory:');try{db.exec('CREATE TABLE height(ordinal INTEGER,level REAL,value REAL)');for(const [ordinal,item]of row.rows.entries())db.run('INSERT INTO height VALUES(?,?,?)',[ordinal,item.level,item.height]);const found=db.query(row.query>=0?'SELECT value FROM height WHERE level>=0 AND level<? ORDER BY ordinal':'SELECT value FROM height WHERE level<0 AND level>=? ORDER BY ordinal').all(row.query)as {value:number}[];let total=0;for(const value of found)total+=value.value;expect(actual).toBe(row.query>=0?total:-total);}finally{db.close();}});
test('large source entry and level rosters yield individually',async()=>{const {ifcSourceEntriesSteps,ifcLevelElevationSteps}=await load(),rows=Array.from({length:1024},(_,level)=>({level,height:3}));let entries=0,levels=0;for(const _ of ifcSourceEntriesSteps(Object.fromEntries(rows.map(row=>[String(row.level),row]))))entries++;for(const _ of ifcLevelElevationSteps(rows,1024))levels++;expect(entries).toBeGreaterThanOrEqual(rows.length);expect(levels).toBeGreaterThanOrEqual(rows.length);});
for(const kind of ['bim','fem'])for(const fault of ['none','abort','undefined'])test(kind+' export original semantic roster '+fault,async()=>{
 const input=kind==='bim'?{...ifcBimAxisFixture(.3),storeys:Object.fromEntries(Array.from({length:32},(_,level)=>['s'+level,{name:'Storey'+level,building:'building',level,height:3}]))}:femConnectivityFixture({nodes:['a','b'],members:[{start:'a',end:'b'}],supports:['a'],loads:Array.from({length:32},()=> 'b')});
 if('loadCases' in input)input.loadCases[0]!.loads.forEach((load,index)=>{load.id='load'+index;});
 const abort=new AbortController(),failure=undefined;let turns=0,caught=false,result:unknown;const phase=kind==='bim'?'bim-source-records':'fem-source-index',control:IfcIoControl={signal:abort.signal,batchSize:1,progress(p){if(p.phase!==phase)return;turns++;if(turns===3&&fault!=='none'){control.signal=new AbortController().signal;control.progress=()=>{};if(fault==='abort')abort.abort();else throw failure;}}};
 try{result=kind==='bim'?await bimSpatialToIfc4x3(input as Parameters<typeof bimSpatialToIfc4x3>[0],control):await fem3dToIfc4x3(input as Parameters<typeof fem3dToIfc4x3>[0],control);}catch(error){caught=true;if(fault==='abort')expect(error).toMatchObject({code:'io.cancelled'});else expect(error).toBe(failure);}
 expect(turns).toBeGreaterThanOrEqual(3);if(fault==='none')expect((result as {artifact?:unknown})?.artifact).toBeDefined();else{expect(caught).toBe(true);expect(result).toBeUndefined();}
});
test('prepared ordered level prefixes stay linear and preserve source arithmetic',async()=>{const {ifcPrepareLevelsSteps}=await load(),rows=Array.from({length:1024},(_,level)=>({level,height:3}));let turns=0;const prepare=ifcPrepareLevelsSteps(rows);let group:ReturnType<typeof prepare.next>['value'];for(;;){const step=prepare.next();turns++;if(step.done){group=step.value;break;}}let lookupTurns=0;for(const row of rows){const g=group!.elevationSteps(row.level);for(;;){const next=g.next();lookupTurns++;if(next.done){expect(next.value).toBe(row.level*3);break;}}}expect(turns+lookupTurns).toBeLessThan(rows.length*10);});
for(const [i,row]of cases.levels.entries())test('prepared source level sum '+i,async()=>{const {ifcPrepareLevelsSteps,ifcLevelElevationSteps}=await load(),group=drain(ifcPrepareLevelsSteps(row.rows));expect(drain(group.elevationSteps(row.query))).toBe(drain(ifcLevelElevationSteps(row.rows,row.query)));});
