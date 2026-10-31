"""🔗 Declared wall junctions preserve source stock while assigning every shared layer once."""
import importlib,json,copy,math
from pathlib import Path
A=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
CASE=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'))

def fixture(row,declared=True):
    """🧱 Build a language-neutral independent two-wall model without unrelated physical cutters."""
    m=A.load_model();m['elements']=[];m['spaces']=[];m['relations']=[]
    for key in ('A','B'):
        tag='NEUTRAL-JUNCTION-'+key;material='NEUTRAL-LAYERS-'+key;layers=CASE['layers'][key]
        m['materials'].append({'tag':material,'guid':A.global_id(material),'class':'IfcMaterialLayerSet','name':{'en':material,'de':material},'layers':[{'id':part['id'],'material':part['material'],'name':{'en':part['id'],'de':part['id']},'thickness':{'value':part['thickness'],'unit':'m','basis':'assumed'}} for part in layers]})
        m['elements'].append({'tag':tag,'guid':A.global_id(tag),'class':'IfcWall','predefined_type':'STANDARD','name':{'en':'Neutral wall '+key,'de':'Neutrale Wand '+key},'storey':'ground','material':material,'geometry':{'path':copy.deepcopy(row['paths'][key]),'base_level':0.,'top_level':CASE['height'],'exterior_side':row['sides'][key],'path_role':'partition'},'attributes':{'thickness':{'value':sum(part['thickness'] for part in layers),'unit':'m','basis':'assumed'}},'psets':{},'metadata':{'author':'Atelier (architecture)','revision':1,'domains':['architecture']}})
    if declared:
        tag='NEUTRAL-PATH-JUNCTION';m['relations'].append({'tag':tag,'guid':A.global_id(tag),'class':'IfcRelConnectsPathElements','relating':'NEUTRAL-JUNCTION-A','related':['NEUTRAL-JUNCTION-B'],'name':{'en':'Declared layered junction','de':'Erklärter Schichtanschluss'},'path_connection':{'junction':'NEUTRAL-NODE','bounds':copy.deepcopy(row['bounds']),'relating_priorities':copy.deepcopy(CASE['priorities']['A']),'related_priorities':copy.deepcopy(CASE['priorities']['B']),'relating_connection_type':row['connection_types']['A'],'related_connection_type':row['connection_types']['B'],'basis':'inferred','source':'Neutral-layered-junction-contract'}})
    return m

def shape(parts):
    """📐 Independently combine actual emitted occupied planar profiles with GEOS."""
    from shapely.geometry import Polygon
    from shapely.ops import unary_union
    return unary_union([Polygon(A.part_footprint(part)) for part in parts])

def test_declared_t_l_rotated_and_reversed_nodes_have_one_material_owner():
    """⚖️ Compare native layer ownership and union quantities against independent GEOS differences."""
    from jsonschema import Draft202012Validator
    Draft202012Validator(json.loads(Path(__file__).with_suffix('.schema.json').read_text(encoding='utf8'))).validate(CASE)
    for row in CASE['cases']:
        m=fixture(row);raw=fixture(row,False);original=[A._wall_geometry(raw,wall)['parts'] for wall in raw['elements']];owned=[A._wall_geometry(m,wall)['parts'] for wall in m['elements']]
        first,second=map(shape,owned);baseline=shape(original[0]+original[1])
        assert first.intersection(second).area==0,(row['id'],'positive joined material overlap',first.intersection(second).area)
        assert first.union(second).symmetric_difference(baseline).area<1e-12,row['id']
        assert math.isclose(first.union(second).area*CASE['height'],row['expected_union'],abs_tol=1e-9)
        for index,key in enumerate(('A','B')):
            native=sum(A._polyhedron_volume(part) for part in owned[index]);assert math.isclose(native,row['expected_volume'][key],abs_tol=1e-9),(row['id'],key,native)
            assert math.isclose(native,shape(owned[index]).area*CASE['height'],abs_tol=1e-12)
            for layer in CASE['layers'][key]:
                source=shape([part for part in original[index] if part['layer']==layer['id']]);priority=CASE['priorities'][key][[part['id'] for part in CASE['layers'][key]].index(layer['id'])];other=1-index;otherkey=('A','B')[other];cut=shape([part for part in original[other] if CASE['priorities'][otherkey][[entry['id'] for entry in CASE['layers'][otherkey]].index(part['layer'])]>priority]);actual=shape([part for part in owned[index] if part['layer']==layer['id']]);assert actual.symmetric_difference(source.difference(cut)).area<1e-12,(row['id'],key,layer['id'])

def test_undeclared_wall_penetration_remains_visible():
    """🚧 A coincident footprint does not authorize clipping unrelated source walls."""
    row=CASE['cases'][0];m=fixture(row,False);parts=[A._wall_geometry(m,wall)['parts'] for wall in m['elements']];assert math.isclose(shape(parts[0]).intersection(shape(parts[1])).area*CASE['height'],row['expected_raw_overlap'],abs_tol=1e-12)

def test_path_connection_schema_requires_explicit_priorities_and_positions():
    """📜 Validate native path relationship authority before any construction inference is executed."""
    from jsonschema import Draft202012Validator
    relation=fixture(CASE['cases'][0])['relations'][0];schema=json.loads((Path(A.__file__).parent/'🧬️schema/🔣️.schema.json').read_text(encoding='utf8'));validator=Draft202012Validator({'$schema':schema['$schema'],'$defs':schema['$defs'],**schema['properties']['relations']['items']});validator.validate(relation)
    for field in ('relating_priorities','related_priorities','relating_connection_type','related_connection_type','bounds','junction'):
        invalid=copy.deepcopy(relation);del invalid['path_connection'][field];assert list(validator.iter_errors(invalid)),field
    for value in (-1,101,True):
        invalid=copy.deepcopy(relation);invalid['path_connection']['relating_priorities'][0]=value;assert list(validator.iter_errors(invalid)),value

def test_ambiguous_layer_priority_ties_are_rejected():
    """🛑 No tag order supplies missing material continuity authority."""
    m=fixture(CASE['cases'][0]);m['relations'][0]['path_connection']['related_priorities'][1]=100
    try:A._wall_geometry(m,m['elements'][0])
    except ValueError:return
    raise AssertionError('An ambiguous equal core priority was accepted')

def test_scope_leaves_undeclared_material_overlap_visible():
    """🪟 A bounded connection owns only its explicit local window, never the whole intersecting pair."""
    m=fixture(CASE['cases'][0]);m['relations'][0]['path_connection']['bounds']=[1.98,-.2,2.02,.2]
    parts=[A._wall_geometry(m,wall)['parts'] for wall in m['elements']]
    overlap=shape(parts[0]).intersection(shape(parts[1]))
    assert overlap.area>0
    from shapely.geometry import box
    assert overlap.intersection(box(1.98,-.2,2.02,.2)).area==0
    raw=fixture(CASE['cases'][0],False)
    assert shape(parts[0]+parts[1]).symmetric_difference(shape([part for wall in raw['elements'] for part in A._wall_geometry(raw,wall)['parts']])).area<1e-12


def small_interface_fixture():
    """🧬 Declare the same positive two-nanometre interface for native and independent implementations."""
    row=copy.deepcopy(CASE['cases'][0]);row['paths']['B'][-1][1]=-.1+CASE['positive_overlap_width'];m=fixture(row)
    relation=m['relations'][0];data=relation.pop('path_connection');relation['class']='IfcRelConnectsElements'
    for key in ('relating_connection_type','related_connection_type'):data.pop(key)
    relation['connection']={'kind':'material_junction',**data}
    return m


def test_two_nanometre_positive_interface_is_resolved_without_thresholds():
    """🔬 Preserve a real two-nanometre witness and independently integrate every native material cell."""
    from scipy.spatial import ConvexHull
    import numpy as np
    m=small_interface_fixture()
    raw=copy.deepcopy(m);raw['relations']=[]
    originals=[A._wall_geometry(raw,wall)['parts'] for wall in raw['elements']];parts=[A._wall_geometry(m,wall)['parts'] for wall in m['elements']]
    assert shape(originals[0]).intersection(shape(originals[1])).area>0
    assert shape(parts[0]).intersection(shape(parts[1])).area==0
    baseline=shape(originals[0]+originals[1]);assert shape(parts[0]+parts[1]).symmetric_difference(baseline).area<1e-14
    for collection in parts:
        for part in collection:
            points=np.asarray(A.part_vertices(part),dtype=float);points-=points.mean(axis=0)
            assert math.isclose(ConvexHull(points).volume,A._polyhedron_volume(part),rel_tol=1e-12,abs_tol=1e-18)


def test_progress_cancellation_preserves_source_and_complete_results():
    """🛑 Cancel during declared ownership without mutating source or publishing a partial body."""
    m=fixture(CASE['cases'][0]);before=copy.deepcopy(m);events=[];stopped=[False]
    def progress(event):
        """📣 Deliver observable per-material work before cooperative cancellation."""
        events.append(event);stopped[0]=True
    try:A._wall_geometry(m,m['elements'][0],progress=progress,cancel=lambda:stopped[0])
    except InterruptedError:pass
    else:raise AssertionError('Declared wall work ignored cancellation')
    assert events and events[0]['phase']=='wall-junctions' and m==before
    completed=A._wall_geometry(m,m['elements'][0]);assert math.isclose(completed['solid_volume'],CASE['cases'][0]['expected_volume']['A'],abs_tol=1e-12)
    assert sum(A._polyhedron_volume(part) for part in completed['interface_reference_parts'])>completed['solid_volume']


def test_native_path_relationship_keeps_ifc_attribute_order():
    """📄 Native IFC keeps both actual products and source-layer priority arrays in standard attribute order."""
    m=fixture(CASE['cases'][0]);relation=m['relations'][0]
    for language in CASE['languages']:
        fragment=A.wall_junction_excerpt(relation['guid'] if language=='de' else relation['tag'],m,language=language)
        text='\n'.join(fragment['lines']);line=next(line for line in fragment['lines'] if '=IFCRELCONNECTSPATHELEMENTS(' in line)
        assert line.endswith('(30,100,30),(20,80,20),.ATEND.,.ATPATH.);'),line
        assert text.count('=IFCWALL(')==2 and text.count("'Reference'")>=2
        assert relation['guid'] in line

def test_public_wall_queries_are_revision_bound_and_isolated():
    """🗂️ Public wall bodies cache only complete private source snapshots and return isolated values."""
    m=fixture(CASE['cases'][0]);tag=m['elements'][0]['tag'];before=copy.deepcopy(m);events=[]
    first=A.wall_geometry(tag,m,progress=events.append);assert events
    first['parts'].clear();second=A.wall_geometry(tag,m)
    assert second['parts'] and m==before and math.isclose(second['solid_volume'],CASE['cases'][0]['expected_volume']['A'],abs_tol=1e-12)
    m['relations']=[];unjoined=A.wall_geometry(tag,m);assert unjoined['solid_volume']>second['solid_volume']


def test_public_wall_semantic_values_are_json_native():
    """📤 Publish exact private construction only once as isolated language-neutral JSON numbers."""
    from fractions import Fraction
    m=fixture(CASE['cases'][0]);tag=m['elements'][0]['tag'];original=A._wall_geometry
    stock={'kind':'prism','outline':[[0,0],[1,0],[1,1],[0,1]],'holes':[],'z0':0,'z1':Fraction(1,3)};part={'material':'MAT-BRICK','role':'body','csg':{'operator':'difference','stock':stock,'cutters':[]}}
    def constructed(*args,**kwargs):
        """🧱 Supply private semantic construction at the public query boundary."""
        return {'tag':tag,'parts':[copy.deepcopy(part)]}
    try:
        A._wall_geometry=constructed;published=A.wall_geometry(tag,m);json.dumps(published);assert published['parts'][0]['csg']['stock']['z1']==float(stock['z1'])
        published['parts'].clear();assert A.wall_geometry(tag,m)['parts'] and stock['z1']==Fraction(1,3)
    finally:A._wall_geometry=original


def coated_fixture():
    """🎨 Retain an existing substrate coating identity across a declared neutral material interface."""
    m=fixture(CASE['cases'][0]);quantity=lambda value:{'value':value,'unit':'m','basis':'assumed'}
    m['elements'][0]['geometry']['surface_regions']=[{'tag':'NEUTRAL-COATING','layer':'skin-a','face':'positive','substrate':'MAT-PLASTER','material':CASE['coating_material'],'start':quantity(0),'end':quantity(4),'from':{'datum':'base','offset':quantity(0)},'to':{'datum':'top','offset':quantity(0)},'basis':'assumed','source':'Neutral-coating-interface-contract'}]
    return m


def test_scoped_junction_preserves_real_analytic_owner_and_bore_air():
    """⭕ Compare a declared bounded CSG wall intersection against independent analytic integration."""
    from types import SimpleNamespace
    from jsonschema import Draft202012Validator
    row=json.loads(Path(__file__).with_name('🔗️wall-csg.json').read_text(encoding='utf8'));m=fixture(CASE['cases'][0]);m['relations'][0]['path_connection']['bounds']=row['bounds'];api=SimpleNamespace(**vars(A));backend=A._wall_junction_backend();state=backend.context(api,m)
    schema=json.loads(Path(__file__).with_name('🔗️wall-csg.schema.json').read_text(encoding='utf8'));schema['$defs']=json.loads((Path(A.__file__).parent/'🧊️solids/➖️difference/🧬️.schema.json').read_text(encoding='utf8'))['$defs'];Draft202012Validator(schema).validate(row)
    part={'material':'MAT-BRICK','layer':'core','role':'body','csg':{'operator':'difference','stock':row['stock'],'cutters':[{'operand':row['bore'],'role':'construction_boundary'}]}}
    state['raw'][m['elements'][0]['tag']]={'parts':[part]};target={'material':'MAT-BRICK','layer':'core','role':'body',**copy.deepcopy(row['stock'])};target.pop('kind');before=copy.deepcopy((m,part,target))
    owned,removed=backend.partition(api,m,m['elements'][1],[target],state)
    assert len(owned)==1 and 'csg' in owned[0] and not any(key in owned[0] for key in ('outline','mesh','profile'))
    oracle=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧊️solids.➖️difference.🧪️tests.🔬️scipy');volume,error=oracle.volume(A._native_public_value(owned[0]['csg']));expected=1-row['expected_removed_rectangle_volume']+math.pi*row['bore']['profile']['radius']**2
    assert abs(volume-expected)<=8*error+64*math.ulp(expected),(volume,expected,error)
    assert math.isclose(removed,1-expected,abs_tol=1e-12) and (m,part,target)==before
    cutter=owned[0]['csg']['cutters'][0];assert cutter['owner']==m['elements'][0]['tag'] and cutter['owner_guid']==m['elements'][0]['guid'] and cutter['role']=='material'
    try:backend.partition(api,m,m['elements'][1],[target],state,cancel=lambda:True)
    except InterruptedError:pass
    else:raise AssertionError('Analytic wall junction ignored cancellation')


def test_native_path_priorities_reference_real_ordered_layer_sets():
    """📚 Curved-path cells retain their actual ordered source layers instead of replacing them with constituents."""
    parser=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧪️tests.🐍️');m=fixture(CASE['cases'][0]);m['elements'][0]['geometry']['path']=[[0,0],[2,0],[4,0]]
    fragment=A.wall_junction_excerpt(m['relations'][0]['tag'],m,language='en');entities=parser.step_parse(fragment['lines'])['entities']
    for prefix in ('','related.'):
        _,relation=entities[int(fragment['ids'][prefix+'rel_material'][1:])];name,attributes=entities[relation[-1][1]]
        assert name=='IFCMATERIALLAYERSET' and len(attributes[0])==3,(name,attributes)


def test_coated_material_interfaces_keep_constituents_without_false_native_path_priorities():
    """🖌️ Complex coating ownership uses a bounded general connection while preserving native constituent identity."""
    m=coated_fixture();relation=m['relations'][0]
    try:A.wall_junction_excerpt(relation['tag'],m,language='en')
    except ValueError:pass
    else:raise AssertionError('A native Path priority list was emitted without an actual native layered material association')
    data=relation.pop('path_connection');relation['class']='IfcRelConnectsElements'
    for key in ('relating_connection_type','related_connection_type'):data.pop(key)
    relation['connection']={'kind':'material_junction',**data};fragment=A.wall_junction_excerpt(relation['tag'],m,language='en')
    assert any('=IFCRELCONNECTSELEMENTS(' in line for line in fragment['lines'])
    assert any('=IFCMATERIALCONSTITUENTSET(' in line for line in fragment['lines'])
