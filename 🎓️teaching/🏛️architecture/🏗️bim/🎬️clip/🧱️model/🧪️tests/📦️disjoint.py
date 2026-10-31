"""📦️ Add native covering cells only after conservative world-space separation is proven."""
import importlib,json,math
from copy import deepcopy
from pathlib import Path
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
CASE=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'))

def parts(row):
    """🧱 Construct neutral native prisms and true affine top/bottom cells without source-specific tags."""
    result=[]
    for spec in row['parts']:
        if spec['kind']=='prism':p={'outline':deepcopy(spec['outline']),'z0':spec['z'][0],'z1':spec['z'][1],**({'holes':deepcopy(spec['holes'])}if spec.get('holes')else{})}
        else:
            top=[[*point,sum(a*b for a,b in zip(spec['plane'],[*point,1]))]for point in spec['outline']];vertices=[[x,y,z-spec['depth']]for x,y,z in top]+top;p=api._polyhedron_part(vertices)
        result.append({**p,'material':'MAT-TEST','role':'wearing'})
    return result

def reference(row,cells):
    """🔬 Compute identical physical mass with independent GEOS slice unions or SciPy convex hulls."""
    import numpy as np
    from shapely.geometry import Polygon
    from shapely.ops import unary_union
    from scipy.spatial import ConvexHull
    if all(p['kind']=='prism'for p in row['parts']):
        levels=sorted({v for p in row['parts']for v in p['z']});return math.fsum((hi-lo)*unary_union([Polygon(p['outline'],p.get('holes',[]))for p in row['parts']if p['z'][0]<(lo+hi)/2<p['z'][1]]).area for lo,hi in zip(levels,levels[1:]))
    for i,a in enumerate(row['parts']):
        for b in row['parts'][i+1:]:
            if Polygon(a['outline']).intersection(Polygon(b['outline'])).area>0:assert a['plane'][:2]==b['plane'][:2]and abs(a['plane'][2]-b['plane'][2])>=max(a['depth'],b['depth'])
    return math.fsum(ConvexHull(np.asarray(api.part_vertices(p),float)).volume for p in cells)

def model(row,cells):
    """🌐 Build a small complete language-explicit model for the actual public quantity interface."""
    obj={'tag':'COVERING-TEST','guid':api.global_id('COVERING-TEST'),'class':'IfcCovering','predefined_type':'FLOORING','name':{'en':row['id'],'de':row['id']},'storey':'ground','material':'MAT-TEST','geometry':{'kind':'flooring','base_level':0,'parts':deepcopy(cells)},'attributes':{},'psets':{},'metadata':{'author':'Neutral','revision':1,'domains':['architecture']}}
    return {'version':1,'languages':['en','de'],'revision':1,'schema_id':'IFC4X3_ADD2','elements':[obj],'types':[],'spaces':[],'storeys':[],'materials':[{'tag':'MAT-TEST','guid':api.global_id('MAT-TEST'),'class':'IfcMaterial','name':{'en':'Test material','de':'Testmaterial'},'density':{'value':24,'unit':'kN/m³','basis':'assumed','source':'Neutral physical control'}}],**{key:{'tag':key.upper(),'guid':api.global_id(key.upper()),'class':kind}for key,kind in (('project','IfcProject'),('site','IfcSite'),('building','IfcBuilding'))},'relations':[],'models':[]}

def test_native_partition_proof_matches_independent_positive_overlap_and_conservative_controls():
    """🧬 Preserve two-nanometre overlaps and leave unproven hollow/sloped envelopes on true union integration."""
    from jsonschema import Draft202012Validator
    Draft202012Validator(json.loads(Path(__file__).with_suffix('.schema.json').read_text(encoding='utf8'))).validate(CASE)
    import numpy as np
    from shapely.geometry import Polygon
    for guard in CASE['analytic_guards']:
        assert np.max(np.linalg.norm(np.asarray(guard['box']),axis=1))<guard['radius'];assert Polygon(guard['outline']).intersection(Polygon(guard['box'])).area==0
        analytic={'outline':guard['outline'],'z0':0,'z1':1,'profile':{'shape':'circle','diameter':{'value':2*guard['radius'],'unit':'m','basis':'assumed'}}};other={'outline':guard['box'],'z0':0,'z1':1};assert not api._parts_disjoint([analytic,other])
    for row in CASE['cases']:
        cells=parts(row);expected=reference(row,cells);assert math.isclose(expected,row['volume'],abs_tol=1e-12);assert api._parts_disjoint(cells)==row['partitioned'],row['id']
        if row['partitioned']:assert math.isclose(api._partition_volume(cells),expected,abs_tol=1e-12)
        if row['id'].startswith('positive-'):assert api._partition_volume(cells)-expected>1e-9

def test_public_covering_quantity_preserves_union_mass_and_avoids_slice_union_only_after_proof():
    """⚖️ Check actual API and material quantities against GEOS/SciPy, including the algorithmic fast-path contract."""
    original=api._occupied_volume
    try:
        for row in CASE['cases']:
            calls=[]
            def observed(cells):
                """📋 Record actual generic occupied-union evaluations without replacing their physical result."""
                calls.append(len(cells));return original(cells)
            api._occupied_volume=observed;cells=parts(row);actual=api.quantities('COVERING-TEST',model(row,cells));expected=reference(row,cells);assert math.isclose(actual['volume'],expected,abs_tol=CASE['comparison']['volume']),row['id'];assert math.isclose(actual['material_quantities'][0]['volume'],expected,abs_tol=CASE['comparison']['volume']),row['id'];assert (not calls)==row['partitioned'],(row['id'],calls)
    finally:api._occupied_volume=original

def test_native_partition_proof_reports_progress_and_observes_explicit_cancellation():
    """🛑 Keep a potentially large component proof observable and interruptible."""
    cells=parts(CASE['cases'][0]);events=[];assert api._parts_disjoint(cells,progress=events.append);assert events[-1]=={'completed':len(cells),'total':len(cells)}
    try:api._parts_disjoint(cells,cancelled=lambda:True)
    except InterruptedError:pass
    else:raise AssertionError('Partition proof ignored explicit cancellation')

def test_native_partition_producer_replacement_invalidates_actual_cached_public_mass():
    """🔒 Never reuse a formerly proven additive quantity after its exact partition producer changes."""
    original=api._parts_disjoint;union=api._occupied_volume;row=CASE['cases'][0];cells=parts(row);m=model(row,cells);calls=[]
    try:
        def observed(values):
            """📋 Observe the real fallback integration after producer replacement."""
            calls.append(len(values));return union(values)
        api._occupied_volume=observed;first=api.quantities('COVERING-TEST',m);revision=api.geometry_revision(m);assert not calls;api._parts_disjoint=lambda *a,**k:False;second=api.quantities('COVERING-TEST',m);assert api.geometry_revision(m)!=revision and calls;assert first['part_count']==second['part_count'];assert math.isclose(first['volume'],second['volume'],abs_tol=CASE['comparison']['volume']);assert math.isclose(first['weight'],second['weight'],abs_tol=CASE['comparison']['volume']*24+1e-9);assert math.isclose(first['mass'],second['mass'],abs_tol=CASE['comparison']['volume']*24*1000/9.80665+1e-9)
    finally:api._parts_disjoint=original;api._occupied_volume=union
