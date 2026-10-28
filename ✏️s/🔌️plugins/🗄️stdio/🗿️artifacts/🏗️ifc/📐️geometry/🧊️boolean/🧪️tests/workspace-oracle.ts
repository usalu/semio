/** 🪶️ SQLite constraints independently replay neutral logical-capacity ownership programs. */
import {Database} from 'bun:sqlite';
export interface WorkspaceProgram{id:string;limit:number;steps:readonly (readonly unknown[])[];}
export interface WorkspaceSnapshot{used:number;peak:number;denied:boolean;}
export function sqliteWorkspaceOracle(program:WorkspaceProgram):WorkspaceSnapshot[]{
 const db=new Database(':memory:');try{
  db.exec('CREATE TABLE policy(cap INTEGER NOT NULL,peak INTEGER NOT NULL);CREATE TABLE lease(name TEXT PRIMARY KEY,amount INTEGER NOT NULL CHECK(amount>=0));CREATE TRIGGER admit BEFORE INSERT ON lease BEGIN SELECT CASE WHEN NEW.amount>(SELECT cap FROM policy)-coalesce((SELECT sum(amount) FROM lease),0) THEN RAISE(ABORT,\'limit\') END;END;CREATE TRIGGER grow BEFORE UPDATE ON lease BEGIN SELECT CASE WHEN NEW.amount>(SELECT cap FROM policy)-coalesce((SELECT sum(amount) FROM lease WHERE name<>OLD.name),0) THEN RAISE(ABORT,\'limit\') END;END;');
  db.query('INSERT INTO policy VALUES(?,0)').run(program.limit);const states:WorkspaceSnapshot[]=[];
  for(const step of program.steps){const [operation,name,value,other]=step;let denied=false;try{
   if(operation==='reserve')db.query('INSERT INTO lease VALUES(?,?)').run(name as string,value as number);
   else if(operation==='grow'||operation==='denyGrow')db.query('UPDATE lease SET amount=amount+? WHERE name=?').run(value as number,name as string);
   else if(operation==='shrink')db.query('UPDATE lease SET amount=amount-? WHERE name=?').run(value as number,name as string);
   else if(operation==='release')db.query('DELETE FROM lease WHERE name=?').run(name as string);
   else if(operation==='split')db.transaction(()=>{db.query('UPDATE lease SET amount=amount-? WHERE name=?').run(value as number,name as string);db.query('INSERT INTO lease VALUES(?,?)').run(other as string,value as number);})();
   else if(operation==='merge')db.transaction(()=>{const row=db.query('SELECT amount FROM lease WHERE name=?').get(value as string) as {amount:number};db.query('DELETE FROM lease WHERE name=?').run(value as string);db.query('UPDATE lease SET amount=amount+? WHERE name=?').run(row.amount,name as string);})();
   else throw Error('unknown workspace oracle operation');
  }catch(error){if(operation!=='denyGrow')throw error;denied=true;}
  if(operation==='denyGrow'&&!denied)throw Error('SQLite failed to enforce original capacity');
  db.exec('UPDATE policy SET peak=max(peak,coalesce((SELECT sum(amount) FROM lease),0))');const state=db.query('SELECT coalesce((SELECT sum(amount) FROM lease),0) AS used,peak FROM policy').get() as {used:number;peak:number};states.push({...state,denied});
  }return states;
 }finally{db.close();}
}
