"""🧪 Original-Q ramp finish construction and exact bearing provenance."""
import importlib,json,copy
from pathlib import Path
from types import SimpleNamespace
from fractions import Fraction
import numpy as np
from scipy.spatial import ConvexHull
from shapely.geometry import MultiPoint,Polygon
from jsonschema import Draft202012Validator
P='🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.'
a=importlib.import_module(P+'🐍️')


def test_neutral_original_wearing_stock_matches_independent_geometry():
    """📐 Source dimensions determine exact stock, material mass and unchanged walking endpoints."""
    f=importlib.import_module(P+'🎨️finishes.🛝️ramps')
    case=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'))
    m=a.load_model();flight=copy.deepcopy(a._find(m,'RF-1A'));g=flight['geometry'];g.update(finish_direction=case['flight']['finish_direction'],path=case['flight']['path'],portion=[0,1],from_level='ground',to_level='first',walking_offsets=[{'value':v,'unit':'m'} for v in case['flight']['walking_offsets']]);next(x for x in m['storeys'] if x['tag']=='first')['elevation']['value']=1
    m['elements']=[flight];flight['geometry']['ramp']='RM-TEST';m['elements'].append({'tag':'RM-TEST','guid':a.global_id('RM-TEST'),'class':'IfcRamp','geometry':{'from_level':'ground','to_level':'first','landings':[]}})
    obj=copy.deepcopy(a._find(a.load_model(),'FIN-RAMP-INTERIOR'));obj.update(tag='FIN-TEST',guid=a.global_id('FIN-TEST'));obj['geometry']={'kind':'ramp_finish','base_level':0,'ramp_finish':{'ramp':'RM-TEST','layers':case['layers'],'interface_owners':[],'wall_owners':[]}}
    before=copy.deepcopy(m);parts=f.reference_parts(SimpleNamespace(**vars(a)),m,obj)
    assert m==before and all(isinstance(v,Fraction) for p in parts for point in a.part_vertices(p) for v in point)
    actual=sum(ConvexHull(np.asarray(a.part_vertices(p),float)).volume for p in parts)
    assert abs(actual-case['expected']['volume'])<1e-14,(actual,case['expected']['volume'],g)
    wear=[p for p in parts if p['role']=='wearing'];assert len(wear)==1
    vertices=a.part_vertices(wear[0]);assert all(z==y/2 for x,y,z in vertices[4:])
    area=MultiPoint([[float(x),float(y)] for x,y,z in vertices]).convex_hull.area
    assert area==case['expected']['footprint_area']
    width=case['positive_cut_width'];material=a._material(m,flight)['tag']
    cutter={'tag':'CUT-2NM','guid':a.global_id('CUT-2NM'),'class':'IfcBeam','material':material,'geometry':{'base_level':0,'parts':[{'outline':[[-.5,0],[-.5+width,0],[-.5+width,2],[-.5,2]],'z0':-.1,'z1':2}]}}
    m['elements'].append(cutter);obj['geometry']['ramp_finish']['interface_owners']=[cutter['tag']]
    cut=f.reference_parts(SimpleNamespace(**vars(a)),m,obj)
    reduced=sum(ConvexHull(np.asarray(a.part_vertices(p),float)).volume for p in cut)
    independent_removed=Polygon(cutter['geometry']['parts'][0]['outline']).area*.008*np.sqrt(1.25)
    assert actual-reduced>0 and abs((actual-reduced)-independent_removed)<1e-16
    m['elements'].append(obj)
    quantity=a.quantities(obj['tag'],m);assert abs(quantity['volume']-reduced)<=5.00001e-10
    for kind in case['representations']:
        representation=a.represent(obj['tag'],kind,m)
        assert representation['primitives'] and abs(representation['derived']['volume']-reduced)<=5.00001e-10
    assert f.partitioned(SimpleNamespace(**vars(a)),m,obj)
    twin=copy.deepcopy(flight);twin.update(tag='OVERLAP-FLIGHT',guid=a.global_id('OVERLAP-FLIGHT'));m['elements'].append(twin)
    assert not f.partitioned(SimpleNamespace(**vars(a)),m,obj)
    assert abs(a.quantities(obj['tag'],m)['volume']-reduced)<=5.00001e-10





def test_ramp_finish_schema_discriminates_explicit_stock_from_construction():
    """📜 One authoritative construction excludes contradictory materialized parts."""
    m=a.load_model();schema=json.loads((Path(a.__file__).parent/'🧬️schema/🔣️.schema.json').read_text(encoding='utf8'))
    obj=copy.deepcopy(a._find(m,'FIN-RAMP-INTERIOR'));obj['geometry']={'kind':'ramp_finish','base_level':0,'ramp_finish':{'ramp':'RM-1','layers':[{'role':'wearing','material':'MAT-RAMP-RESILIENT','depth':{'value':.005,'unit':'m','basis':'sourced','source':'SAB2024 ramp finish'}}],'interface_owners':[],'wall_owners':[]}}
    validator=Draft202012Validator({'$ref':'#/$defs/element','$defs':schema['$defs']});validator.validate(obj)
    obj['geometry']['parts']=[];assert not validator.is_valid(obj)


def test_exact_finish_memo_isolation_and_source_mutation():
    """🧊 Exact private floor material retains full-source and producer-sensitive isolation."""
    f=importlib.import_module(P+'🎨️finishes.🛝️ramps');m=a.load_model();obj=copy.deepcopy(a._find(m,'FIN-RAMP-INTERIOR'))
    obj['geometry']={'kind':'ramp_finish','base_level':0,'ramp_finish':{'ramp':'RM-1','layers':[{'role':'wearing','material':'MAT-RAMP-RESILIENT','depth':{'value':.005,'unit':'m','basis':'sourced','source':'SAB2024 ramp finish'}}],'interface_owners':[],'wall_owners':[]}}
    next(e for e in m['elements'] if e['tag']==obj['tag'])['geometry']=copy.deepcopy(obj['geometry'])
    original=f.reference_parts;calls=[]
    def producer(api,model,element):
        """🧪 Observe actual source-sensitive construction calls behind the model interface."""
        calls.append(1);return original(api,model,element)
    f.reference_parts=producer
    try:
        first=a._ramp_finish_reference_parts(m,obj);expected=copy.deepcopy(first)
        first[0]['mesh']['vertices'][0][0]+=1
        assert a._ramp_finish_reference_parts(m,obj)==expected and len(calls)==1
        obj=a._find(m,obj['tag']);obj['geometry']['ramp_finish']['layers'][0]['depth']['value']=.006
        second=a._ramp_finish_reference_parts(m,obj);assert second!=expected and len(calls)==2
        def replacement(api,model,element):
            """🧪 A replaced producer must not reuse prior original material."""
            result=original(api,model,element);result[0]['role']='replacement';return result
        f.reference_parts=replacement
        assert a._ramp_finish_reference_parts(m,obj)[0]['role']=='replacement'
    finally:f.reference_parts=original


def test_actual_ramp_floors_preserve_mass_native_cells_and_walking_datums():
    """🏠 Independent materialized-evidence volume and source walking geometry remain intact."""
    f=importlib.import_module(P+'🎨️finishes.🛝️ramps');m=a.load_model();cases=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'))
    for case in cases['actual']:
        obj=a._find(m,case['tag']);assert obj['geometry']['kind']=='ramp_finish'
        assert f.partitioned(SimpleNamespace(**vars(a)),m,obj)
        body=a.solid_geometry(obj['tag'],m);volumes={role:sum(ConvexHull(np.asarray(a.part_vertices(p),float)).volume for p in body['parts'] if p['role']==role) for role in case['layers']}
        for role,volume in case['layers'].items():assert abs(volumes[role]-volume)<1e-10,(case['tag'],role,volumes[role],volume)
        assert abs(a.quantities(obj['tag'],m)['volume']-sum(volumes.values()))<1e-9
        for part in body['parts']:
            a._polyhedron_part(a.part_vertices(part),a.part_faces(part));assert ConvexHull(np.asarray(a.part_vertices(part),float)).volume>0
        stocks=f.reference_stocks(SimpleNamespace(**vars(a)),m,obj)
        for part in stocks:
            if part['role']!='wearing' or 'owner' not in part:continue
            flight=a._find(m,part['owner']);assert flight['geometry']['finish_direction']==case['direction']
            elevations=[a._source_fraction(a._z(m,flight['geometry'][key])) for key in ('from_level','to_level')]
            expected=[elevations[0]+(elevations[1]-elevations[0])*a._source_fraction(v) for v in flight['geometry']['portion']]
            top=a.part_vertices(part)[4:];assert sorted({p[2] for p in top})==sorted(set(expected))
