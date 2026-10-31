"""🧿 Native analytic profiles retain supplied axes and declared void offsets without facet averages."""
import copy,importlib,json,math
from pathlib import Path
A=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
E=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🐍️')
T=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧪️tests.🐍️')
CASE=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'))


def fixture(row):
    """🏺 Isolate one real native column product with a portable analytic section."""
    m=A.load_model();obj=copy.deepcopy(A._by_class(m,'IfcColumn')[0]);kind=copy.deepcopy(A._find(m,obj['type']))
    obj.update(tag='NEUTRAL-AXIS',guid=A.global_id('NEUTRAL-AXIS'),type='NEUTRAL-AXIS-TYPE',storey='ground',geometry={'axis':copy.deepcopy(row['axis']),'base_level':0.,'top_level':3.})
    kind.update(tag='NEUTRAL-AXIS-TYPE',guid=A.global_id('NEUTRAL-AXIS-TYPE'),profile=copy.deepcopy(row['profile']))
    m['elements']=[obj];m['spaces']=[];m['relations']=[];m['types'].append(kind)
    return m,obj


def expected_points(row,fragment):
    """📍 Address native placement points with exact source axes and rotated declared void offsets."""
    profile=row['profile'];shape=E._native_profile_kind({'profile':profile});points={}
    if shape=='profile_void' and profile['shape'] in ('circle','ellipse'):points[fragment['ids']['outer_point0']]=row['axis']
    elif shape not in ('polygon','profile_void'):points[fragment['ids']['profile_point0']]=row['axis']
    if shape=='profile_void':
        angle=math.radians(profile.get('rotation',0));c,s=math.cos(angle),math.sin(angle)
        for index,hole in enumerate(profile['voids']):
            x,y=hole['offset'];points[fragment['ids']['hole_point0_'+str(index)]]=[row['axis'][0]+c*x-s*y,row['axis'][1]+s*x+c*y]
    return points


def test_native_analytic_axes_and_void_centres_are_exact_source_values():
    """📐 Exact STEP values follow analytic source placement for translated, hollow and eccentric sections."""
    from jsonschema import Draft202012Validator
    Draft202012Validator(json.loads(Path(__file__).with_suffix('.schema.json').read_text(encoding='utf8'))).validate(CASE)
    for row in CASE['cases']:
        m,obj=fixture(row)
        for language in CASE['languages']:
            fragment=E.step_excerpt(obj['tag'],m,language=language,spatial=False);entities=T.step_parse(fragment['lines'])['entities']
            for ref,expected in expected_points(row,fragment).items():
                name,arguments=entities[int(ref[1:])];assert name=='IFCCARTESIANPOINT' and arguments[0]==expected,(row['id'],ref,arguments[0],expected)


def test_missing_analytic_axis_is_rejected_without_centroid_fallback():
    """🛑 A facet outline or shifted material centroid cannot replace missing analytic source placement."""
    part={'outline':[[0,0],[1,0],[1,1]],'centroid':[.25,.25],'profile':CASE['cases'][0]['profile']}
    try:E._profile_axis(part)
    except ValueError:return
    raise AssertionError('A missing analytic profile axis was accepted')


def test_uneven_vertices_and_mass_centroid_cannot_move_the_axis():
    """🎯 Source placement survives uneven facet density and an unrelated centroid without aliasing input."""
    part=copy.deepcopy(CASE['uneven_sample']);axis=E._profile_axis(part);assert axis==CASE['uneven_sample']['axis']
    axis[0]+=1;assert part==CASE['uneven_sample']
    for invalid in (None,[0],[True,0],[0,float('inf')]):
        bad={**part,'axis':invalid}
        try:E._profile_axis(bad)
        except ValueError:continue
        raise AssertionError('Invalid analytic source axis was accepted')
