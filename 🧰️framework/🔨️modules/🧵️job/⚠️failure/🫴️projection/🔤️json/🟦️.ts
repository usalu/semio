import {test,expect} from 'bun:test';
import {Database} from 'bun:sqlite';
import Ajv from 'ajv/dist/2020';
import corpus from './🧪️fixtures/🔣️.json';
import schema from './🧬️schema/🔣️.json';

test('complete JSON original fields agree with independent ordered SQL paths',()=>{
    expect(new Ajv({strict:true}).compile(schema)(corpus)).toBe(true);
    const db=new Database(':memory:');
    try {
        db.run('CREATE TABLE fields(kind TEXT, ordinal INTEGER, field TEXT, PRIMARY KEY(kind,ordinal))');
        for(const [kind,...fields] of corpus.variants)
            for(const [index,field] of fields.entries())
                db.run('INSERT INTO fields VALUES(?1,?2,?3)',[kind,index,field]);
        expect(corpus.variants.length).toBe(12);
        for(const [kind,...fields] of corpus.variants){
            expect(db.query('SELECT field FROM fields WHERE kind=?1 ORDER BY ordinal').all(kind).map((r:any)=>r.field)).toEqual(fields);
            expect(new Set(fields).size).toBe(fields.length);
        }
        const original={kind:'DuplicateMember',name:corpus.name,offset:corpus.offset};
        expect(JSON.parse(JSON.stringify(original))).toEqual(original);
    } finally {
        db.close();
    }
});
