"""🪵 Original timber stock stays native, bilingual, source-qualified and physically positive."""
import importlib,json
from pathlib import Path
import numpy as np
from scipy.spatial import ConvexHull
from jsonschema import Draft202012Validator
P='🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.'
a=importlib.import_module(P+'🐍️')
CASE=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'))


def test_original_skirting_has_source_qualified_material_and_canonical_membership():
    """📜 Preserve secure timber/gray identity while exposing unmeasured pigment, species and fit."""
    Draft202012Validator(json.loads(Path(__file__).with_suffix('.schema.json').read_text(encoding='utf8'))).validate(CASE)
    m=a.load_model();objects=[e for e in m['elements'] if e.get('predefined_type')==CASE['predefined_type']]
    assert objects and set(CASE['levels'])=={obj['storey'] for obj in objects}
    material=a._find(m,CASE['material']);assert material['density']['basis']=='assumed'
    finish=material['surface_finish'];assert finish['binder']=='unmeasured_paint' and finish['film_thickness']=='unmeasured' and finish['display_rgb']['basis']=='inferred'
    np.testing.assert_array_equal(finish['display_rgb']['value'],CASE['display_rgb'])
    architecture=next(model for model in m['models'] if model['id']=='architecture');services=next(model for model in m['models'] if model['id']=='services')
    for obj in objects:
        assert obj['class']==CASE['class'] and obj['material']==CASE['material'] and obj['guid']==a.global_id(obj['tag'])
        assert set(obj['name'])==set(m['languages']) and obj['metadata']['domains']==['architecture'] and 'system' not in obj
        assert CASE['forbidden_host'] not in obj['tag'] and obj['historical_evidence']['status']=='inferred_reconstruction'
        assert obj['tag'] in architecture['element_tags'] and obj['tag'] not in services['element_tags']
        contains=[r for r in m['relations'] if r['class']=='IfcRelContainedInSpatialStructure' and obj['tag'] in r['related']]
        assert len(contains)==1 and contains[0]['relating']==obj['storey']
        materials=[r for r in m['relations'] if r['class']=='IfcRelAssociatesMaterial' and obj['tag'] in r['related']]
        assert len(materials)==1 and materials[0]['relating']==CASE['material']


def test_all_original_skirting_cells_have_strict_native_and_independent_positive_volume():
    """📐 Validate all actual cells using affine-conditioned SciPy rather than an envelope substitute."""
    m=a.load_model();tags={e['tag'] for e in m['elements'] if e.get('predefined_type')==CASE['predefined_type']};assert tags
    g=a.building_geometry(m);objects=[obj for obj in g['details'] if obj['tag'] in tags];assert {obj['tag'] for obj in objects}==tags
    for obj in objects:
        assert obj['parts']
        for part in obj['parts']:
            assert part['material']==CASE['material'];a._polyhedron_part(a.part_vertices(part),a.part_faces(part))
            points=np.asarray(a.part_vertices(part),float);span=np.ptp(points,axis=0);assert np.all(np.isfinite(points)) and np.all(span>0)
            independent=ConvexHull((points-np.min(points,axis=0))/span).volume*float(np.prod(span));native=a._polyhedron_volume(part)
            assert independent>0 and native>0 and abs(native-independent)<=max(1e-24,independent*1e-8),(obj['tag'],native,independent)
