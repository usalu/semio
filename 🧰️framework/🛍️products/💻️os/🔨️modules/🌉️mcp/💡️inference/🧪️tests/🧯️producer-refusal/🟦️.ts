import {expect, test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "../../🧬️schema/producer-refusal.json" with {type:"json"};
import cases from "../🧫️fixtures/producer-refusal.json" with {type:"json"};

test("only standalone original value refusals receive a resource or cancellation code", () => {
  const validate = new Ajv().compile(schema);
  expect(validate(cases), JSON.stringify(validate.errors)).toBe(true);
  using database = new Database(":memory:");
  database.exec(`create table outcomes(kind text primary key, code text not null);
    insert into outcomes values ('work','BUDGET_EXCEEDED'), ('ownership','BUDGET_EXCEEDED'),
      ('allocation','BUDGET_EXCEEDED'), ('depth','BUDGET_EXCEEDED'), ('cancel','CANCELLED')`);
  const project = database.query<{code:string;message:string}, [string|null,boolean,string]>(
    "select coalesce((select code from outcomes where kind = ?1 and not ?2), 'SIDE_EFFECT_REJECTED') code, ?3 message",
  );
  for (const {kind, compound, code, message} of cases) {
    expect(project.get(kind, compound, message)).toEqual({code, message});
  }
});
