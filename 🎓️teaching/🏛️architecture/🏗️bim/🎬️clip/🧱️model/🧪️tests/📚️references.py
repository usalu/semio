"""📚 Preserve qualified citations while rejecting unsupported and unanchored evidence sources."""
import importlib,json
from pathlib import Path

def test_qualified_citation_grammar_matches_language_neutral_and_independent_schema_controls():
    """🔬 Cross-check the actual core source validator against independent JSON Schema evaluation."""
    from jsonschema import Draft202012Validator
    path=Path(__file__);case=json.loads(path.with_suffix('.json').read_text(encoding='utf8'));Draft202012Validator(json.loads(path.with_suffix('.schema.json').read_text(encoding='utf8'))).validate(case);main=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧪️tests.🐍️')
    for row in case['cases']:assert bool(main.SOURCE_ID.fullmatch(row['source']))==row['accepted'],row['id']
    schema=json.loads(path.parent.parent.joinpath('🧬️schema/🔣️.schema.json').read_text(encoding='utf8'));validator=Draft202012Validator(schema['$defs']['evidence_source'])
    for row in case['cases']:assert validator.is_valid(row['source'])==row['accepted'],row['id']
    for definition in ('quantity','property'):
        reference=Draft202012Validator({'$ref':'#/$defs/'+definition,'$defs':schema['$defs']})
        value={'value':1,'basis':'sourced',**({'unit':'m'}if definition=='quantity'else{'ifc_type':'IfcReal'})}
        for row in case['cases']:assert reference.is_valid({**value,'source':row['source']})==row['accepted'],(definition,row['id'])

    subject=main.BimModelStructureContract("test_every_quantity_is_sourced_or_assumed_with_a_unit");subject.model=main.model_api().load_model();subject.test_every_quantity_is_sourced_or_assumed_with_a_unit()
