"""🗺️ Exact native coordinate conditioning preserves source geometry and every positive facet."""
import importlib,json,struct,copy,re
from pathlib import Path
import numpy as np
from scipy.spatial import ConvexHull
from shapely.geometry import Polygon
from jsonschema import Draft202012Validator
P='🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.'

def _cases():
    """📜 Read original language-neutral geometry and expected conditioning decisions."""
    cases=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'));Draft202012Validator(json.loads(Path(__file__).with_suffix('.schema.json').read_text(encoding='utf8'))).validate(cases);return cases

def _mesh(case):
    """🧱 Extrude an original positive stock without reducing its native face cycles."""
    o=case['origin'];profile=case['profile'];n=len(profile);vertices=[[o[0]+x,o[1]+y,o[2]+z] for y in (0,case['sweep']) for x,z in profile];faces=[list(reversed(range(n))),list(range(n,2*n))]+[[i,(i+1)%n,(i+1)%n+n,i+n] for i in range(n)];return {'vertices':vertices,'faces':faces}

def test_coordinate_frame_keeps_every_binary64_vertex_and_independent_material():
    """📐 Inverse powers of two preserve each original coordinate, topology and independent SciPy/GEOS material volume."""
    f=importlib.import_module(P+'🔌️exchange.📐️frames.🐍️');cases=_cases()
    for case in cases['cases']:
        mesh=_mesh(case);before=copy.deepcopy(mesh);frame=f.coordinate_frame(mesh['vertices'],mesh['faces'],minimum_basis=cases['minimum_basis']);assert frame['conditioned']==case['conditioned'] and mesh==before
        restored=[[frame['origin'][i]+value/frame['scale'] for i,value in enumerate(p)] for p in frame['vertices']]
        assert [[struct.pack('!d',v) for v in p] for p in restored]==[[struct.pack('!d',v) for v in p] for p in mesh['vertices']]
        local=np.asarray(frame['vertices']);span=np.ptp(local,axis=0);volume=ConvexHull((local-local.min(axis=0))/span).volume*np.prod(span)/frame['scale']**3;expected=Polygon(case['profile']).area*(mesh['vertices'][len(case['profile'])][1]-mesh['vertices'][0][1]);assert volume>0 and abs(volume-expected)<=expected*1e-12
    try:f.coordinate_frame([[float('nan'),0,0]]*4,[[0,1,2]])
    except ValueError:pass
    else:raise AssertionError('Nonfinite native points cannot be conditioned')

def test_conditioned_native_representation_binds_exact_inverse_and_material_aspects():
    """📦 All mapped body/aspect items keep their original physical source and bilingual identity."""
    a=importlib.import_module(P+'🐍️');ex=importlib.import_module(P+'🔌️exchange.🐍️');case=_cases()['cases'][1];mesh=_mesh(case);m=a.load_model();obj=copy.deepcopy(a._find(m,'FIN-RAMP-INTERIOR'));obj.update(tag='FRAME-TEST',guid=a.global_id('FRAME-TEST'),predefined_type='SKIRTINGBOARD',material='MAT-PAINTED-WOOD');obj['geometry']={'kind':'skirting','base_level':0,'parts':[{'mesh':mesh,'material':'MAT-PAINTED-WOOD','role':'skirting'},{'outline':[[10,12],[10.1,12],[10.1,13],[10,13]],'z0':2,'z1':2.1,'material':'MAT-PAINTED-WOOD','role':'skirting'}]};m['elements'].append(obj);before=copy.deepcopy(obj)
    for language in ('en','de'):
        fragment=ex.step_excerpt(obj['tag'],m,language=language);ids=fragment['ids'];lines=fragment['lines'];line=lambda key:next(x for x in lines if x.startswith(ids[key]+'='));assert "'MappedRepresentation'" in line('representation');assert '=IFCMAPPEDITEM(' in line('mapped0');assert all(all(ids['mapped'+str(i)] in line(key) for i in (0,1)) for key in ids if key.startswith('aspect_representation'));assert '=IFCMAPPEDITEM(' in line('mapped1')
        origin=[float(v) for v in re.search(r'\(\((.*?)\)\)',line('mapping_point0')).group(1).split(',')];factor=float(line('mapping_target0').split(',')[-2]);actual=[]
        for j in range(len(mesh['vertices'])):actual.append([origin[i]+float(value)*factor for i,value in enumerate(re.search(r'\(\((.*?)\)\)',line('brep_vertex0_'+str(j))).group(1).split(','))])
        assert actual==mesh['vertices'] and obj==before
