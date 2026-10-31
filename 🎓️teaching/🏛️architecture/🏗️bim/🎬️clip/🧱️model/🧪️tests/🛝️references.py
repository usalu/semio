"""🛝️ Original owned ramp stock stays exact through independent projections and isolated native queries."""
import importlib,json,copy
from pathlib import Path
from fractions import Fraction
import numpy as np
from scipy.spatial import ConvexHull
from shapely.geometry import MultiPoint
from jsonschema import Draft202012Validator
a=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')

def _case():
    """📜 Read the portable source dimensions before creating a neutral model."""
    case=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'));Draft202012Validator(json.loads(Path(__file__).with_suffix('.schema.json').read_text(encoding='utf8'))).validate(case);return case

def _model(case):
    """🧱 Create only independent original source stock, with no production-building dependency."""
    ident=lambda tag:{'tag':tag,'guid':a.global_id(tag)}
    m={key:[] for key in ('elements','types','spaces','storeys','materials')}
    m.update({key:ident(key) for key in ('project','site','building')});m['storeys']=[{**ident('low'),'id':'low','elevation':{'value':0}},{**ident('high'),'id':'high','elevation':{'value':1}}];m['materials']=[ident('MAT')]
    f,l=case['flight'],case['landing'];m['elements']=[{**ident('R'),'class':'IfcRamp','material':'MAT','geometry':{'from_level':'low','to_level':'high','finish_thickness':{'value':f['finish']},'interface_owners':[],'landings':[{'tag':'L','owner':'R','ownership':'ramp_component','outline':l['outline'],'portion':l['portion'],'thickness':{'value':l['thickness']}}]}},{**ident('F'),'class':'IfcRampFlight','material':'MAT','geometry':{'ramp':'R','path':f['path'],'from_level':'low','to_level':'high','portion':f['portion'],'finish_thickness':{'value':f['finish']},'finish_direction':'vertical','thickness':{'value':f['thickness']},'structural_offsets':[{'value':v} for v in f['offsets']],'flight_joint_owners':[],'interface_owners':[]}}];return m

def _volume(parts):
    """📐 Independently condition each native convex stock before measuring its positive material."""
    total=0.
    for p in parts:
        vertices=np.asarray(a.part_vertices(p),float);span=np.ptp(vertices,axis=0);total+=ConvexHull((vertices-vertices.min(axis=0))/span).volume*np.prod(span)
    return total

def _exact_volume(parts):
    """🧮 Keep the positive2nm removed stock exact before comparing independent floating oracles."""
    total=Fraction(0)
    for part in parts:
        vertices=a.part_vertices(part);origin=vertices[0];signed=Fraction(0)
        for face in a.part_faces(part):
            for i in range(1,len(face)-1):
                p,q,r=[[vertices[index][k]-origin[k] for k in range(3)] for index in (face[0],face[i],face[i+1])]
                signed+=p[0]*(q[1]*r[2]-q[2]*r[1])+p[1]*(q[2]*r[0]-q[0]*r[2])+p[2]*(q[0]*r[1]-q[1]*r[0])
        total+=abs(signed)/6
    return total


def test_neutral_owned_reference_is_exact_isolated_source_and_producer_bound():
    """🧪 Neutral stock/landing, positive2nm cut, source mutation, output mutation and producer replacement retain their independent oracle."""
    case=_case();m=_model(case);original=copy.deepcopy(m)
    for tag,expected in [('F',case['flight']['volume']),('R',case['landing']['volume'])]:
        parts=a._ramp_owned_reference_parts(m,a._find(m,tag));assert all(isinstance(v,Fraction) for p in parts for xyz in a.part_vertices(p) for v in xyz);assert abs(_volume(parts)-expected)<1e-15
    parts=a._ramp_owned_reference_parts(m,a._find(m,'F'));frozen=copy.deepcopy(parts);parts[0]['mesh']['vertices'][0][0]=999;fresh=a._ramp_owned_reference_parts(m,a._find(m,'F'));assert fresh==frozen and abs(_volume(fresh)-case['flight']['volume'])<1e-15;assert m==original
    w=case['positive_cut_width'];m['elements'].append({'tag':'CUT','guid':a.global_id('CUT'),'class':'IfcBeam','material':'MAT','geometry':{'base_level':0,'parts':[{'outline':[[-.5,0],[-.5+w,0],[-.5+w,2],[-.5,2]],'z0':-.1,'z1':2}]}});a._find(m,'F')['geometry']['interface_owners']=['CUT'];cut_parts=a._ramp_owned_reference_parts(m,a._find(m,'F'));reduced=_volume(cut_parts);removed=MultiPoint([[-.5,0],[-.5+w,0],[-.5+w,2],[-.5,2]]).convex_hull.area*.06;exact_removed=Fraction(str(case['flight']['volume']))-_exact_volume(cut_parts);assert exact_removed==Fraction(str(w))*2*Fraction(str(case['flight']['thickness'])) and exact_removed>0;assert abs(float(exact_removed)-removed)<1e-18 and abs(reduced-float(_exact_volume(cut_parts)))<1e-15
    a._find(m,'F')['geometry']['thickness']['value']=.08;assert abs(_volume(a._ramp_owned_reference_parts(m,a._find(m,'F')))-(.16-removed/.06*.08))<1e-15
    old=a._ramp_owned_reference_parts_uncached;calls=[]
    try:
        def first(*args,**kwargs):
            calls.append(1);return old(*args,**kwargs)
        a._ramp_owned_reference_parts_uncached=first;one=a._ramp_owned_reference_parts(original,a._find(original,'F'));a._ramp_owned_reference_parts(original,a._find(original,'F'));assert calls==[1]
        def second(*args,**kwargs):
            calls.append(2);return old(*args,**kwargs)
        a._ramp_owned_reference_parts_uncached=second;a._ramp_owned_reference_parts(original,a._find(original,'F'));assert calls==[1,2]
        try:a._ramp_owned_reference_parts(original,a._find(original,'F'),cancel=lambda:True)
        except InterruptedError:pass
        else:raise AssertionError('Cancellation must not return or publish geometry')
        cancelled=copy.deepcopy(original);cancelled['test_event']='cancelled-unpublished';checks=[];before_cancel=len(calls)
        def cancel_during():
            checks.append(True);return len(checks)>1
        try:a._ramp_owned_reference_parts(cancelled,a._find(cancelled,'F'),cancel=cancel_during)
        except InterruptedError:pass
        else:raise AssertionError('A cancelled cold construction must not publish')
        assert len(calls)==before_cancel+1
        events=[];a._ramp_owned_reference_parts(cancelled,a._find(cancelled,'F'),progress=events.append);assert len(calls)==before_cancel+2 and events
    finally:a._ramp_owned_reference_parts_uncached=old

def test_actual_owned_reference_matches_every_current_published_ramp_cell_exactly():
    """🧬 Every owned flight and middle landing publishes the existing native vertex multisets without a float-to-rational recast."""
    assert hasattr(a,'_ramp_owned_reference_parts');case=_case();m=a.load_model()
    signature=lambda parts:sorted(sorted(tuple(float(v) for v in point) for point in a.part_vertices(p)) for p in parts)
    for tag in case['actual_tags']:
        obj=a._find(m,tag);actual=a._ramp_owned_reference_parts(m,obj);native=a._ramp_geometry(m,obj)['parts'];assert signature(actual)==signature(native),tag
