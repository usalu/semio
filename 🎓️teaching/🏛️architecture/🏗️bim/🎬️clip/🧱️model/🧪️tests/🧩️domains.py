"""🧩 Preserve deliberately authored physical discipline sharing without missing architectural assets."""
import importlib,json
from pathlib import Path

def test_explicit_models_and_product_domains_match_reviewed_construction_ownership():
    """🗂️ Compare every explicit roster with product labels and independent NumPy membership matrices."""
    import numpy as np
    from jsonschema import Draft202012Validator
    path=Path(__file__);case=json.loads(path.with_suffix('.json').read_text(encoding='utf8'));Draft202012Validator(json.loads(path.with_suffix('.schema.json').read_text(encoding='utf8'))).validate(case)
    api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');source=api.load_model();domains=case['domains'];members={model['id']:set(model['element_tags'])for model in source['models']};items={e['tag']:e for e in source['elements']};assert set(members)==set(domains);assert all(tags<=set(items)|{space['tag']for space in source['spaces']}for tags in members.values())
    for row in case['cases']:
        expected=set(row['domains']);assert set(items[row['tag']]['metadata']['domains'])==expected,row['tag'];assert {name for name,tags in members.items()if row['tag']in tags}==expected,row['tag']
    labels=np.asarray([[domain in item['metadata']['domains']for domain in domains]for item in source['elements']],dtype=bool);explicit=np.asarray([[item['tag']in members[domain]for domain in domains]for item in source['elements']],dtype=bool);np.testing.assert_array_equal(labels,explicit);assert np.all(np.any(explicit,axis=1))
