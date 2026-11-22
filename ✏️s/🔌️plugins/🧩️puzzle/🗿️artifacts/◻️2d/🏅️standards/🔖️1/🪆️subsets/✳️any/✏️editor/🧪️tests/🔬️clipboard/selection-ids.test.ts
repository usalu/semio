import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import law from"../🧫️fixtures/selection-ids.json";
import schema from"../🧬️schema/selection-ids.json";
import topology from "../🧫️fixtures/clipboard-topology.json";
test("selection identity corpus is closed",()=>{expect(new Ajv().compile(schema)(law)).toBe(true)});
for(const row of law.cases)test(row.name,()=>{
 const selected=row.groups.flat(),ids=new Set(selected);
 expect(law.nodes.filter(node=>ids.has(node.id)||node.handles.some(handle=>ids.has(handle.id))).map(node=>node.id)).toEqual(row.expected);
 const db=new Database(":memory:");
 try{
  const rows=db.query("select json_extract(n.value,'$.id') as id from json_each(?) n where json_extract(n.value,'$.id') in (select value from json_each(?)) or exists(select 1 from json_each(json_extract(n.value,'$.handles')) h where json_extract(h.value,'$.id') in (select value from json_each(?))) order by cast(n.key as integer)").all(JSON.stringify(law.nodes),JSON.stringify(selected),JSON.stringify(selected)) as {id:string}[];
  expect(rows.map(row=>row.id)).toEqual(row.expected);
 }finally{db.close()}
});

test("clipboard topology is preserved by full handle identity", () => {
  const db = new Database(":memory:");
  try {
    const nodes = JSON.stringify(topology.nodes);
    const edges = JSON.stringify(topology.edges);
    const ports = `
      WITH ports AS (
        SELECT CAST(node.key AS INTEGER) AS node, CAST(handle.key AS INTEGER) AS port,
          node.value ->> '$.id' AS owner, handle.value ->> '$.id' AS id
        FROM json_each(?) AS node, json_each(node.value -> '$.handles') AS handle
      )`;
    const mapped = db.query(`${ports}
      SELECT json_array(json_array(source.node, source.port), json_array(target.node, target.port)) AS endpoints
      FROM json_each(?) AS edge
      JOIN ports AS source ON source.id = edge.value ->> '$.source'
      JOIN ports AS target ON target.id = edge.value ->> '$.target'
      ORDER BY CAST(edge.key AS INTEGER)`).all(nodes, edges) as { endpoints: string }[];
    expect(mapped.map(({ endpoints }) => JSON.parse(endpoints))).toEqual(topology.endpoints);

    const refused = db.query(`${ports}
      SELECT DISTINCT ports.owner AS id FROM ports JOIN json_each(?) AS edge
      ON ports.id IN (edge.value ->> '$.source', edge.value ->> '$.target')
      WHERE edge.value ->> '$.locked' = 1 ORDER BY id`).all(nodes, edges) as { id: string }[];
    expect(refused.map(({ id }) => id)).toEqual(topology.cutRefused);
    expect(topology.nodes.map(({ id }) => id).filter((id) => !refused.some((row) => row.id === id))).toEqual(topology.cutAllowed);
  } finally {
    db.close();
  }
});

test("clipboard paste skips consecutive occupied node and edge serials", () => {
  const db = new Database(":memory:");
  try {
    const nextTwo = db.query(`
      WITH RECURSIVE serials(n) AS (
        SELECT ? UNION ALL SELECT n + 1 FROM serials WHERE n < ?
      )
      SELECT n FROM serials WHERE n NOT IN (SELECT value FROM json_each(?))
      ORDER BY n LIMIT 2`);
    const available = (first: number, occupied: number[]) =>
      (nextTwo.all(first, Math.max(first, ...occupied) + 2, JSON.stringify(occupied)) as { n: number }[]).map(({ n }) => n);
    const nodes = available(1, topology.occupiedSerials.nodes);
    const edges = available(nodes.at(-1)! + 1, topology.occupiedSerials.edges);
    expect(nodes.map((serial) => `node-${serial}`)).toEqual(topology.pastedIds.nodes);
    expect(edges.map((serial) => `edge-${serial}`)).toEqual(topology.pastedIds.edges);
  } finally {
    db.close();
  }
});

test("clipboard paste skips endpoint names owned by other destination nodes", () => {
  const db = new Database(":memory:");
  try {
    const namespace = topology.handleNamespace;
    const destination = [
      ...namespace.nodes,
      ...topology.occupiedSerials.nodes.map(serial => ({ id: `node-${serial}`, handles: [] })),
    ];
    const nextNode = db.query(`
      WITH RECURSIVE serials(n) AS (
        SELECT ?1 UNION ALL SELECT n + 1 FROM serials WHERE n < 64
      ), endpoints(id) AS (
        SELECT node.value ->> '$.id' FROM json_each(?2) AS node
        UNION ALL
        SELECT handle.value ->> '$.id'
        FROM json_each(?2) AS node, json_each(node.value -> '$.handles') AS handle
      )
      SELECT n FROM serials
      WHERE 'node-' || n NOT IN (SELECT id FROM endpoints)
        AND NOT EXISTS (
          SELECT 1 FROM json_each(?3) AS port
          WHERE 'node-' || n || ':v' || port.key IN (SELECT id FROM endpoints)
        )
      ORDER BY n LIMIT 1`);
    let serial = 1;
    const pasted = topology.selected.map(id => {
      const source = topology.nodes.find(node => node.id === id)!;
      const { n } = nextNode.get(serial, JSON.stringify(destination), JSON.stringify(source.handles)) as { n: number };
      serial = n + 1;
      return `node-${n}`;
    });
    expect(pasted).toEqual(namespace.pastedIds.nodes);
  } finally {
    db.close();
  }
});

test("clipboard labels follow the first authored root across accepted clones", () => {
  const db = new Database(":memory:");
  try {
    const { catalog, peers, empty, occupied } = topology.labeling;
    const split = (label: string) => /^(.*) (\d+)$/.exec(label);
    const nextTwo = db.query(`
      WITH labels(value) AS (SELECT value FROM json_each(?1)),
      highest(n) AS (
        SELECT MAX(COALESCE(MAX(
          CASE WHEN value = ?2 THEN 1
               WHEN substr(value, 1, length(?2) + 1) = ?2 || ' '
                 THEN CAST(substr(value, length(?2) + 2) AS INTEGER)
               ELSE 0 END
        ), 0), COUNT(*) - COUNT(value)) FROM labels
      )
      SELECT ?2 || ' ' || (n + 1) AS label FROM highest
      UNION ALL SELECT ?2 || ' ' || (n + 2) FROM highest`);
    const afterPeers = (nextTwo.all(JSON.stringify(peers), split(peers[0]!)![1]) as { label: string }[]).map(({ label }) => label);
    const afterCatalog = (nextTwo.get(JSON.stringify([catalog]), split(catalog)![1]) as { label: string }).label;
    expect(afterPeers).toEqual(occupied);
    expect([catalog, afterCatalog]).toEqual(empty);
  } finally {
    db.close();
  }
});
