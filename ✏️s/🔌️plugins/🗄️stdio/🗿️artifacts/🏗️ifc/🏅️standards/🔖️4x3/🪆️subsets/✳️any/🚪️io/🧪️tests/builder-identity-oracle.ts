/** 🧮️ A test-only independent database observes monotonic source identity after row retirement. */
import{Database}from'bun:sqlite';
export interface BuilderIdentityStep{operation:string;id:number}
export interface BuilderIdentityObserver{observe(initial:number,steps:readonly BuilderIdentityStep[]):number[]}
export const sqliteBuilderIdentity:BuilderIdentityObserver={observe(initial,steps){const database=new Database(':memory:');try{database.exec('CREATE TABLE identities (id INTEGER PRIMARY KEY AUTOINCREMENT)');const insert=database.prepare('INSERT INTO identities DEFAULT VALUES'),remove=database.prepare('DELETE FROM identities WHERE id=?'),result:number[]=[];for(let at=0;at<initial;at++)insert.run();for(const step of steps)if(step.operation==='add')result.push(Number(insert.run().lastInsertRowid));else if(step.operation==='remove')remove.run(step.id);return result;}finally{database.close();}}};
