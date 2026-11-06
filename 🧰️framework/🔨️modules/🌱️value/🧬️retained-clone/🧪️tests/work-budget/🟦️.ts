import {test,expect} from 'bun:test';
import {Database} from 'bun:sqlite';
import Ajv from 'ajv/dist/2020';
import law from './🔣️.json';
import schema from './🧬️.json';
test('bounded clone work admission agrees with independent SQL arithmetic',()=>{expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);const db=new Database(':memory:');try{const query=db.query('SELECT CASE WHEN ?1 > 0 AND ?1 <= ?2 THEN ?1 ELSE 0 END AS paid');for(const action of law.actions){if('childWork' in action)expect(action.work).toBe(action.childWork+action.parentWork);for(const budget of law.maximumWork){const expected=action.work<=budget?action.work:0;expect((query.get(action.work,budget)as{paid:number}).paid).toBe(expected);if(budget===0)expect(expected).toBe(0)}}}finally{db.close()}});
