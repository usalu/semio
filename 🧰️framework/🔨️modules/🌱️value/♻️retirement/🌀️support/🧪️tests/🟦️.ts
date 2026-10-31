import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv2020 from "ajv";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};

/** 🕸️ Independent finite graph reachability proves support even when edges cycle. */
export function conjunctiveRetirementSupportVectorLawV1(): void {
    const validate=new Ajv2020({strict:true}).compile(schema);
    expect(validate(fixture),JSON.stringify(validate.errors)).toBe(true);
    const database=new Database(":memory:");
    try {
        const query=database.query("WITH RECURSIVE reachable(n) AS (SELECT CAST(?2 AS INTEGER) UNION SELECT CAST(edge.value AS INTEGER) FROM reachable AS current,json_each(json_extract(?1,'$['||current.n||'].children')) AS edge) SELECT NOT EXISTS(SELECT 1 FROM reachable WHERE json_extract(?1,'$['||n||'].supported')=0) AS supported");
        for(const row of fixture.cases) {
            expect(row.root).toBeLessThan(row.nodes.length);
            for(const node of row.nodes)for(const child of node.children)expect(child).toBeLessThan(row.nodes.length);
            expect(query.get(JSON.stringify(row.nodes),row.root)).toEqual({supported:Number(row.expected)});
        }
    } finally { database.close(); }
}

test("recursive retirement support matches independent SQLite reachability",conjunctiveRetirementSupportVectorLawV1);
