"""🎨 Source-qualified face treatments preserve existing substrate geometry and native identity."""
from copy import deepcopy
import importlib,json,math
from pathlib import Path
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
CASE=json.loads(Path(__file__).with_name('🔣️.json').read_text())

def example():
    """🧱 Construct a source-neutral layered wall with one real aperture."""
    q=lambda value:{'value':value,'unit':'m','basis':'assumed'}
    m=api.load_model()
    template=deepcopy(api._find(m,'W-KITCHEN'))
    wall={k:deepcopy(template[k]) for k in ('class','predefined_type','name','metadata','historical_evidence')}
    wall.update(tag=CASE['wall']['tag'],guid=api.global_id(CASE['wall']['tag']),storey='ground',material='TEST-SURFACE-LAYERS',attributes={'thickness':q(CASE['wall']['thickness'])},psets={})
    wall['geometry']={'path':CASE['wall']['path'],'base_level':CASE['wall']['base'],'top_level':CASE['wall']['top'],'path_role':'partition','exterior_side':1,'surface_regions':[deepcopy(CASE['region'])]}
    layers={'tag':'TEST-SURFACE-LAYERS','guid':api.global_id('TEST-SURFACE-LAYERS'),'class':'IfcMaterialLayerSet','name':{'en':'Neutral wall layers','de':'Neutrale Wandschichten'},'layers':[{**row,'name':{'en':row['id'],'de':row['id']},'thickness':q(row['thickness'])} for row in CASE['layers']]}
    finish=deepcopy(api._find(m,CASE['material']['surface_finish']['substrate']))
    finish.update(deepcopy(CASE['material']));finish['guid']=api.global_id(finish['tag'])
    opening={'tag':CASE['opening']['tag'],'guid':api.global_id(CASE['opening']['tag']),'class':'IfcOpeningElement','predefined_type':'OPENING','name':{'en':'Neutral opening','de':'Neutrale Öffnung'},'storey':'ground','geometry':{'wall':wall['tag']},'attributes':{key:q(CASE['opening'][key]) for key in ('start','end','sill','head')},'psets':{},'metadata':deepcopy(wall['metadata'])}
    m['materials']+=[layers,finish];m['elements']=[wall,opening];m['spaces']=[]
    for model in m['models']:model['element_tags']=[wall['tag'],opening['tag']] if model['id']=='architecture' else []
    m['relations']=[]
    return m,wall

def test_wall_surface_regions_preserve_material_volume_aperture_and_exact_face_area():
    """📐 GEOS checks conserved substrate solids and the selected face without inventing paint-film volume."""
    from jsonschema import Draft202012Validator
    from shapely.geometry import Polygon,box
    from shapely.ops import unary_union
    import numpy as np
    Draft202012Validator(json.loads(Path(__file__).with_name('🔣️.schema.json').read_text())).validate(CASE)
    m,wall=example();g=api._wall_geometry(m,wall);expected=CASE['expected']
    painted=[p for p in g['parts'] if p['material']==CASE['material']['tag']]
    volume=lambda parts:math.fsum(Polygon(p['outline']).area*(p['z1']-p['z0']) for p in parts)
    assert abs(volume(painted)-expected['substrate_volume'])<1e-12
    assert abs(volume(g['parts'])-expected['volume'])<1e-12
    baseline=deepcopy(wall);baseline['geometry'].pop('surface_regions')
    original=api._wall_geometry(m,baseline);levels=sorted({p[k] for p in g['parts']+original['parts'] for k in ('z0','z1')})
    for lo,hi in zip(levels,levels[1:]):
        cut=(lo+hi)/2
        before=unary_union([Polygon(p['outline']) for p in original['parts'] if p['z0']<cut<p['z1']])
        after=unary_union([Polygon(p['outline']) for p in g['parts'] if p['z0']<cut<p['z1']])
        assert before.symmetric_difference(after).area<1e-13
    faces=[]
    for part in painted:
        vertices=np.asarray(api.part_vertices(part))
        for face in api.part_faces(part):
            points=vertices[face]
            if np.allclose(points@np.asarray(expected['normal']),expected['plane_offset'],atol=1e-12,rtol=0):
                faces.append(Polygon(points[:,[0,2]]))
    projected=unary_union(faces)
    region=box(CASE['region']['start']['value'],CASE['region']['from']['offset']['value'],CASE['region']['end']['value'],CASE['region']['to']['offset']['value'])
    aperture=box(CASE['opening']['start'],CASE['opening']['sill'],CASE['opening']['end'],CASE['opening']['head'])
    assert projected.symmetric_difference(region.difference(aperture)).area<1e-12
    assert abs(projected.area-expected['face_area'])<1e-12
    assert abs(g['surface_finishes'][0]['area']-projected.area)<1e-12
    quantities=api.quantities(wall['tag'],m)
    assert quantities['surface_finishes'][0]['quantity_scope']=='existing_substrate_envelope'
    assert abs(quantities['surface_finishes'][0]['substrate_volume']-volume(painted))<1e-12
    schema=json.loads((Path(api.__file__).parent/'🧬️schema/🔣️.schema.json').read_text())
    for obj,kind in [(wall,'element'),(api._find(m,CASE['material']['tag']),'material')]:
        Draft202012Validator({'$schema':schema['$schema'],'$defs':schema['$defs'],'$ref':'#/$defs/'+kind}).validate(obj)

def test_wall_surface_regions_reject_ambiguous_or_incompatible_identity():
    """🚫 Reject unsupported layers, substrate substitutions, bounds and overlapping face records."""
    for invalid in CASE['invalid']:
        m,wall=example();region=wall['geometry']['surface_regions'][0]
        if invalid=='overlap':wall['geometry']['surface_regions'].append({**deepcopy(region),'tag':'OTHER-FACE'})
        elif invalid=='wrong_layer':region['layer']='absent-layer'
        elif invalid=='wrong_substrate':region['substrate']='MAT-METAL'
        elif invalid=='unknown_material':region['material']='ABSENT-MATERIAL'
        elif invalid=='outside_station':region['end']['value']=CASE['wall']['path'][1][0]+1
        elif invalid=='inverted_height':region['to']['offset']['value']=0
        try:api._wall_geometry(m,wall)
        except ValueError:continue
        raise AssertionError(invalid+' surface region was silently accepted')

def test_native_wall_face_treatments_keep_closed_bodies_materials_and_qualified_styles():
    """🏗️ Native face styles bind illustrative RGB to one real face and retain the substrate quantity scope."""
    oracle=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧪️tests.🐍️')
    m,wall=example()
    for language in ('en','de'):
        fragment=api.step_excerpt(wall['tag'],m,language=language)
        entities=oracle.step_parse(fragment['lines'])['entities']
        entity,values=entities[int(fragment['ids']['element'][1:])]
        assert entity=='IFCWALL' and len(values)==9
        names=[oracle.step_decode(values[0]) for entity,values in entities.values() if entity=='IFCMATERIAL']
        assert CASE['material']['name'][language] in names
        material_properties=[values for entity,values in entities.values() if entity=='IFCMATERIALPROPERTIES']
        assert material_properties and all(len(values)==4 for values in material_properties)
        for values in material_properties:
            assert entities[values[3][1]][0]=='IFCMATERIAL'
            properties={oracle.step_decode(entities[index][1][0]):entities[index][1][2] for index in oracle.step_refs(values[2])}
            assert 'unmeasured' in str(properties['FilmThickness'])
            assert 'existing_substrate_envelope' in str(properties['QuantityScope'])
            assert 'inferred' in str(properties['DisplayRGBBasis'])
        for entity,values in entities.values():
            if entity=='IFCSHAPEREPRESENTATION' and any(entities[index][0]=='IFCFACETEDBREP' for index in oracle.step_refs(values[3])):
                assert oracle.step_decode(values[2])=='SolidModel'
        styles=[values for entity,values in entities.values() if entity=='IFCSTYLEDITEM']
        assert styles
        for values in styles:
            target=entities[values[0][1]]
            assert target[0]=='IFCFACE'
        colours=[values for entity,values in entities.values() if entity=='IFCCOLOURRGB']
        assert any(all(abs(float(a)-b)<1e-12 for a,b in zip(row[1:],CASE['material']['surface_finish']['display_rgb']['value'])) for row in colours)
        assert any('RGB' in str(values) for entity,values in entities.values() if entity=='IFCSURFACESTYLE')


def test_rounded_skirting_stock_preserves_section_and_closed_native_material():
    """🪵 Independent GEOS/SciPy verifies stock sections, inclined length, volume and closed native boards."""
    import numpy as np
    from scipy.spatial import ConvexHull
    from shapely.geometry import Polygon
    from jsonschema import Draft202012Validator
    Draft202012Validator(json.loads(Path(__file__).with_name("🔣️.schema.json").read_text())).validate(CASE)
    backend=api.surface_finishes();spec=CASE["skirting"]["section"]
    t,h,r,n=[spec[k] for k in ("thickness","height","round_radius","round_segments")]
    profile=[[0,0],[t,0]]+[[t-r+r*math.cos(i*math.pi/(2*n)),h-r+r*math.sin(i*math.pi/(2*n))] for i in range(n+1)]+[[0,h]]
    area=Polygon(profile).area
    oracle=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧪️tests.🐍️")
    for case in CASE["skirting"]["cases"]:
        part=backend.skirting_stock(case["start"],case["end"],case["normal"],spec,"MAT-PAINTED-WOOD")
        from collections import Counter
        incidence=Counter(tuple(sorted((a,b))) for face in part["mesh"]["faces"] for a,b in zip(face,face[1:]+face[:1]))
        assert set(incidence.values())=={2}
        vertices=np.asarray(part["mesh"]["vertices"]);length=np.linalg.norm(np.asarray(case["end"])-case["start"])
        assert abs(ConvexHull(vertices).volume-area*length)<1e-12
        assert abs(api._polyhedron_volume(part)-area*length)<1e-12
        d=(np.asarray(case["end"])-case["start"])/length;normal=np.asarray(case["normal"]);up=np.array([0.,0.,1.])-d*d[2];up/=np.linalg.norm(up)
        local=vertices-np.asarray(case["start"]);section=np.column_stack((local@normal,local@up))
        assert Polygon(section[:len(profile)]).symmetric_difference(Polygon(profile)).area<1e-12
        for axis,expected in ((local@normal,t),(local@up,h)):
            assert abs(axis.min())<1e-12 and abs(axis.max()-expected)<1e-12
        m=api.load_model();obj=deepcopy(api._find(m,"RAIN-KITCHEN-PATIO-9-COVER"))
        obj.update(tag="TEST-SKIRTING",guid=api.global_id("TEST-SKIRTING"),predefined_type="SKIRTINGBOARD",geometry={"kind":"skirting","base_level":"ground","parts":[part]},attributes={},psets={})
        obj.pop("system",None);obj["metadata"]["domains"]=["architecture"];m["elements"].append(obj)
        for language in ("en","de"):
            fragment=api.step_excerpt(obj["tag"],m,language=language);parsed=oracle.step_parse(fragment["lines"])["entities"]
            entity,attributes=parsed[int(fragment["ids"]["element"][1:])]
            assert entity=="IFCCOVERING" and len(attributes)==9 and attributes[8]==("enum","SKIRTINGBOARD")
            assert any(kind=="IFCCLOSEDSHELL" for kind,args in parsed.values())
    for invalid in CASE["skirting"]["invalid"]:
        case=deepcopy(CASE["skirting"]["cases"][0]);bad=deepcopy(spec)
        if invalid=="zero_length":case["end"]=case["start"]
        elif invalid=="nonorthogonal_normal":case["normal"]=[1,0,0]
        elif invalid=="radius_exceeds_section":bad["round_radius"]=.02
        else:bad["height"]=0
        try:backend.skirting_stock(case["start"],case["end"],case["normal"],bad,"MAT-PAINTED-WOOD")
        except ValueError:pass
        else:raise AssertionError(invalid)


def test_original_paint_faces_keep_source_registration_and_occupied_substrate():
    """🏛️ Current source-painted walls preserve independent material volume, openings and unknown-film provenance."""
    from shapely.geometry import Polygon
    from shapely.ops import unary_union
    m=api.load_model();plain=deepcopy(m)
    for obj in plain["elements"]:obj.get("geometry",{}).pop("surface_regions",None)
    for row in CASE["original_faces"]:
        wall=api._find(m,row["host"]);regions=[r for r in wall["geometry"].get("surface_regions",[]) if r["face"]==row["face"] and r["layer"]==row["layer"] and api._find(m,r["material"])["surface_finish"]["colour_family"]==row["family"]]
        assert regions,row
        for r in regions:
            material=api._find(m,r["material"]);finish=material["surface_finish"]
            assert finish["binder"]==row["binder"] and row["source"] in finish["source"]
            assert finish["film_thickness"]=="unmeasured" and finish["quantity_scope"]=="existing_substrate_envelope"
            assert material["density"]==api._find(m,r["substrate"])["density"]
        g=api._wall_geometry(m,wall)
        baseline=api._wall_geometry(plain,api._find(plain,row["host"]))
        volume=lambda parts:math.fsum(Polygon(p["outline"]).area*(p["z1"]-p["z0"]) for p in parts)
        assert abs(volume(g["parts"])-volume(baseline["parts"]))<1e-9
        summaries={r["tag"]:r for r in g["surface_finishes"]}
        for r in regions:assert summaries[r["tag"]]["area"]>0
        heights=sorted({p[k] for p in g["parts"]+baseline["parts"] for k in ("z0","z1")})
        for lo,hi in zip(heights,heights[1:]):
            z=(lo+hi)/2;section=lambda parts:unary_union([Polygon(p["outline"]) for p in parts if p["z0"]<z<p["z1"]])
            assert section(g["parts"]).symmetric_difference(section(baseline["parts"])).area<1e-9


def test_skirting_ownership_cuts_keep_closed_positive_stock_and_independent_volume():
    """✂️ Diagonal joints, short ends and partial-height ownership retain exact closed timber instead of overlap."""
    import numpy as np
    from scipy.spatial import ConvexHull
    from shapely.geometry import Polygon,box
    from collections import Counter
    backend=api.surface_finishes();spec=CASE["skirting"]["section"]
    stock=backend.skirting_stock([0,0,0],[2,0,0],[0,1,0],spec,"MAT-PAINTED-WOOD")
    n=len(stock["mesh"]["vertices"])//2;profile=Polygon([[v[1],v[2]] for v in stock["mesh"]["vertices"][:n]])
    expected={"diagonal_end":profile.area*(1-profile.centroid.x),"short_end":profile.area*.005,"half_height":profile.intersection(box(0,0,spec["thickness"],.045)).area*2}
    for case in CASE["skirting"]["clips"]:
        part=backend.clip_stock(stock,case["planes"])
        if case["mode"]=="empty":assert part is None;continue
        vertices=part["mesh"]["vertices"];faces=part["mesh"]["faces"]
        api._polyhedron_part(vertices,faces)
        incidence=Counter(tuple(sorted((a,b))) for f in faces for a,b in zip(f,f[1:]+f[:1]))
        assert set(incidence.values())=={2}
        assert abs(api._polyhedron_volume(part)-expected[case["mode"]])<1e-12
        assert abs(ConvexHull(np.asarray(vertices)).volume-expected[case["mode"]])<1e-12
        assert part["material"]==stock["material"] and part["role"]==stock["role"]
        assert all(sum(a*b for a,b in zip(p,plane["normal"]))<=plane["offset"]+1e-12 for p in vertices for plane in case["planes"])


def test_original_surface_treatments_keep_unmeasured_mass_and_source_scopes():
    """📜 Source-backed binder/colour identities remain native product facts without fabricated coating mass or unlocated colours."""
    from jsonschema import Draft202012Validator
    m=api.load_model()
    Draft202012Validator(json.loads((Path(api.__file__).parent/"🧬️schema/🔣️.schema.json").read_text())).validate(m)
    oracle=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧪️tests.🐍️")
    for row in CASE["original_treatments"]:
        obj=api._find(m,row["host"]);facts=obj["psets"]["Pset_OriginalSurfaceTreatment"]
        assert facts["Binder"]["value"]==row["binder"]
        assert facts["Scope"]["value"]==row["scope"]
        assert all(facts[key]["value"]=="unmeasured" for key in ("FilmThickness","CoatingMass","OriginalRGB"))
        assert all(p["basis"] in ("sourced","inferred") and p["source"] for p in facts.values())
        if obj["class"]=="IfcColumn":assert facts["ColourFamily"]["value"]=="unlocated"
    for language in ("en","de"):
        fragment=api.step_excerpt("GARAGE-SLIDING",m,language=language,property_set=CASE["native_property_set"],properties=tuple(facts));parsed=oracle.step_parse(fragment["lines"])["entities"]
        assert any(kind=="IFCPROPERTYSET" and args[2]==CASE["native_property_set"] for kind,args in parsed.values())
    try:api.step_excerpt("GARAGE-SLIDING",m,property_set="UNAUTHORED-SET")
    except ValueError:pass
    else:raise AssertionError("An unauthored selected native property set must be rejected")


def test_uniform_material_appearance_keeps_native_whole_item_styles_and_panel_geometry():
    """🚪 Fully painted panel material styles its actual whole items while keeping substrate geometry and unknown film mass."""
    from shapely.geometry import Polygon
    from types import SimpleNamespace
    from PIL import ImageColor
    case=CASE["uniform_appearance"];m=api.load_model();obj=api._find(m,case["host"]);baseline=api._filling_geometry(m,obj)
    material=deepcopy(api._find(m,case["substrate"]));material.update(tag=case["tag"],guid=api.global_id(case["tag"]))
    material["surface_finish"]={"binder":case["binder"],"colour_family":case["family"],"substrate":case["substrate"],"quantity_scope":"existing_substrate_envelope","film_thickness":"unmeasured","basis":"assumed","source":"Synthetic uniform painted panel","display_rgb":{"value":case["rgb"],"basis":"inferred","source":"Synthetic illustrative RGB"}}
    m["materials"].append(material);api._find(m,obj["type"])["filling"]["panel_material"]=material["tag"]
    resolved=api._filling_geometry(m,obj);assert all("surface_appearance" in p for p in resolved["parts"])
    assert [(p["outline"],p["z0"],p["z1"]) for p in resolved["parts"]]==[(p["outline"],p["z0"],p["z1"]) for p in baseline["parts"]]
    core=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧱️core.🐍️")
    for part in resolved["parts"]:assert ImageColor.getrgb(core.surface_color({"kind":"panel"},part))==tuple(round(v*255) for v in case["rgb"])
    primitives=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧊️axon.🧩️primitives")
    axon=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧊️axon.🐍️")
    faces=primitives._window_faces({}, {"fill_tag":obj["tag"]},{"fillings":[resolved]})
    assert faces and all(isinstance(face[3],dict) for face in faces)
    painter=axon._Painter(None,None,False)
    for face in faces:assert ImageColor.getrgb(painter.base(SimpleNamespace(base=None),face[3],"architecture"))==tuple(round(v*255) for v in case["rgb"])
    service=primitives._service_prims({"tag":"NEUTRAL-APPLIANCE","kind":"appliance","parts":resolved["parts"]},{},-1,3.17)
    service_faces=[face for piece in service for face in piece.faces]
    assert service_faces and all(isinstance(face[3],dict) for face in service_faces)
    for face in service_faces:assert ImageColor.getrgb(painter.base(SimpleNamespace(base=None),face[3],"services"))==tuple(round(v*255) for v in case["rgb"])
    oracle=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧪️tests.🐍️")
    for language in ("en","de"):
        fragment=api.step_excerpt(obj["tag"],m,language=language);entities=oracle.step_parse(fragment["lines"])["entities"]
        styled=[args for kind,args in entities.values() if kind=="IFCSTYLEDITEM"]
        assert len(styled)==len(resolved["parts"])
        assert all(entities[args[0][1]][0]=="IFCEXTRUDEDAREASOLID" for args in styled)


def test_skirting_source_joint_is_upright_and_keeps_positive_nanometre_stock():
    """📏 The real service-jamb fixture rejects zero-width arithmetic remnants while preserving a two-nanometre positive stock."""
    import numpy as np
    from scipy.spatial import ConvexHull
    from shapely.geometry import Polygon
    row=CASE["skirting"]["source_joint"];backend=api.surface_finishes()
    stock=backend.skirting_stock(row["start"],row["end"],row["normal"],CASE["skirting"]["section"],"MAT-PAINTED-WOOD")
    assert stock["z0"]==0 and abs(stock["z1"]-.09)<1e-15
    station=row["station"]
    assert backend.clip_stock(stock,[{"normal":[1,0,0],"offset":station},{"normal":[-1,0,0],"offset":-station}]) is None
    part=backend.clip_stock(stock,[{"normal":[1,0,0],"offset":station},{"normal":[-1,0,0],"offset":-(station-row["positive_width"])}])
    assert part is not None
    vertices=np.asarray(part["mesh"]["vertices"]);n=len(stock["mesh"]["vertices"])//2
    area=Polygon([[v[1],v[2]] for v in stock["mesh"]["vertices"][:n]]).area
    expected=area*(station-(station-row["positive_width"]))
    assert abs(api._polyhedron_volume(part)-expected)<1e-20
    assert abs(ConvexHull(vertices-vertices[0]).volume-expected)<1e-20
    api._polyhedron_part(vertices.tolist(),part["mesh"]["faces"])


def test_shared_skirting_mitre_planes_match_independent_convex_and_concave_offsets():
    """📐 Shared inferred joint planes conserve GEOS offset sections and disjoint material at both corner senses."""
    import numpy as np
    from shapely.geometry import LineString,Polygon
    from shapely.ops import unary_union
    backend=api.surface_finishes();spec=CASE["skirting"]["section"]
    for case in CASE["skirting"]["joints"]:
        a,p,b=map(np.asarray,case["path"]);joint=backend.skirting_joint(a.tolist(),p.tolist(),b.tolist(),spec["thickness"])
        d0=(p-a)/np.linalg.norm(p-a);d1=(b-p)/np.linalg.norm(b-p);normal=lambda d:[d[1],-d[0],0.]
        first=backend.skirting_stock(a.tolist(),(p+d0*joint["extension"]).tolist(),normal(d0),spec,"MAT-PAINTED-WOOD")
        second=backend.skirting_stock((p-d1*joint["extension"]).tolist(),b.tolist(),normal(d1),spec,"MAT-PAINTED-WOOD")
        first=backend.clip_stock(first,[joint["plane"]]);second=backend.clip_stock(second,[{"normal":[-n for n in joint["plane"]["normal"]],"offset":-joint["plane"]["offset"]}])
        n=len(first["mesh"]["vertices"]);raw=backend.skirting_stock([0,0,0],[1,0,0],[0,1,0],spec,"MAT-PAINTED-WOOD");profile=Polygon([[v[1],v[2]] for v in raw["mesh"]["vertices"][:len(raw["mesh"]["vertices"])//2]])
        for z in (.01,.0885,.0895):
            width=profile.intersection(LineString([[-1,z],[1,z]])).bounds[2]
            expected=LineString([a[:2],p[:2],b[:2]]).buffer(-width,single_sided=True,join_style=2)
            occupied=[Polygon(api.part_outline_at(part,z)) for part in (first,second)]
            assert occupied[0].intersection(occupied[1]).area<1e-12
            assert unary_union(occupied).symmetric_difference(expected).area<1e-12
        for part in (first,second):api._polyhedron_part(part["mesh"]["vertices"],part["mesh"]["faces"])


def test_native_polyhedron_retains_positive_interface_facets_without_area_cutoff():
    """🔺 Real stock/post interface facets remain positive closed geometry and match independent recentered SciPy volume."""
    import numpy as np
    from scipy.spatial import ConvexHull
    part=CASE["skirting"]["positive_interface_mesh"];vertices=np.asarray(part["mesh"]["vertices"])
    normalized=api._polyhedron_part(vertices.tolist(),part["mesh"]["faces"])
    assert sorted(map(tuple,normalized["mesh"]["vertices"]))==sorted(map(tuple,vertices.tolist()))
    assert len(normalized["mesh"]["faces"])==len(part["mesh"]["faces"])
    assert abs(api._polyhedron_volume(normalized)-ConvexHull(vertices-vertices[0]).volume)<1e-12


def test_nearly_coincident_stock_support_planes_replay_to_closed_material():
    """🧮 Actual source support cuts retain closed topology and independently integrated convex sections without an epsilon or hull repair."""
    import numpy as np
    from scipy.spatial import ConvexHull
    from scipy.integrate import quad
    from shapely.geometry import Polygon,MultiPoint
    from collections import Counter
    row=CASE["skirting"]["support_domain"];part=api.surface_finishes().clip_stock(row["stock"],row["planes"]);assert part is not None
    edges=Counter(tuple(sorted((a,b))) for face in part["mesh"]["faces"] for a,b in zip(face,face[1:]+face[:1]));assert set(edges.values())=={2}
    api._polyhedron_part(part["mesh"]["vertices"],part["mesh"]["faces"])
    points=np.asarray(row["stock"]["mesh"]["vertices"]);hull=ConvexHull(points-points[0]);links={tuple(sorted((int(a),int(b)))) for face in hull.simplices for a,b in zip(face,np.roll(face,-1))};domain=Polygon(row["cell"])
    def area(z):
        """📐 Independent SciPy hull-edge slices are intersected by GEOS with the actual support polygon."""
        found=[]
        for a,b in links:
            p,q=points[a],points[b]
            if p[2]==z:found.append(p[:2])
            if (p[2]-z)*(q[2]-z)<0:found.append((p+(q-p)*(z-p[2])/(q[2]-p[2]))[:2])
        return MultiPoint(found).convex_hull.intersection(domain).area if found else 0.
    levels=sorted(set(points[:,2]));expected=sum(quad(area,a,b,epsabs=1e-13)[0] for a,b in zip(levels,levels[1:]))
    assert abs(api._polyhedron_volume(part)-expected)<1e-12

def test_exact_stock_continuation_preserves_owned_rational_operands_and_json_publication():
    """🧮 Ordered and branched exact cuts agree with a single rational replay and independent SciPy volumes without inheriting preview rounding."""
    import json
    from fractions import Fraction
    from scipy.spatial import ConvexHull
    import numpy as np
    backend=api.surface_finishes();row=CASE["skirting"]["support_domain"];raw=deepcopy(row["stock"]);planes=deepcopy(row["planes"])
    state=backend.clip_stock(raw,[],exact=True)
    for plane in planes:state=backend.clip_stock(state,[plane])
    replay=backend.clip_stock(raw,planes,exact=True)
    assert backend.publish_stock(state)==backend.publish_stock(replay)
    assert all(isinstance(v,Fraction) for p in state._exact_mesh["vertices"] for v in p)
    assert raw==row["stock"] and planes==row["planes"]
    a=backend.clip_stock(state,[{"normal":[0,0,1],"offset":.04}]);b=backend.clip_stock(state,[{"normal":[0,0,-1],"offset":-.04}])
    assert a is not None and b is not None
    expected=api._polyhedron_volume(state)
    assert abs(api._polyhedron_volume(a)+api._polyhedron_volume(b)-expected)<1e-12
    for part in (state,a,b):
        output=backend.publish_stock(part);assert type(output) is dict
        json.dumps(output);vertices=np.asarray(output["mesh"]["vertices"])
        assert abs(api._polyhedron_volume(output)-ConvexHull(vertices-vertices[0]).volume)<1e-12
        output["mesh"]["vertices"][0][0]+=1
        assert output["mesh"]["vertices"]!=part["mesh"]["vertices"]

def test_stock_planes_and_overlength_retain_original_source_coordinate_relations():
    """📐 Original three-point planes and stock endpoints control exact construction instead of rounded display coefficients."""
    from shapely.geometry import Polygon
    from scipy.spatial import ConvexHull
    import numpy as np
    row=CASE["skirting"]["plane_provenance"];backend=api.surface_finishes()
    stock=backend.skirting_stock(row["start"],row["end"],row["normal"],CASE["skirting"]["section"],"MAT-PAINTED-WOOD",extensions=row["extensions"])
    state=backend.clip_stock(stock,[{"normal":[-1,0,0],"offset":0,"point":row["start"]},{"normal":[1,0,0],"offset":1,"point":row["end"]}],exact=True)
    part=backend.clip_stock(state,[row["plane"]]);assert part is not None
    n=len(stock["mesh"]["vertices"])//2;profile=Polygon([[v[1],v[2]] for v in stock["mesh"]["vertices"][:n]])
    expected=profile.area*(row["expected_intercept"]-profile.centroid.x)
    points=np.asarray(part["mesh"]["vertices"])
    assert abs(api._polyhedron_volume(part)-expected)<1e-14
    assert abs(ConvexHull(points-points[0]).volume-expected)<1e-14
    api._polyhedron_part(points.tolist(),part["mesh"]["faces"])

def test_inferred_back_fitting_clearance_preserves_full_timber_section_and_floor_bearing():
    """🪵 A declared installation fit translates the entire15×90mm stock without eroding material or moving its host/floor."""
    from shapely.geometry import Polygon
    from scipy.spatial import ConvexHull
    import numpy as np
    backend=api.surface_finishes();row=CASE["skirting"]["plane_provenance"];spec=CASE["skirting"]["section"]
    raw=backend.skirting_stock(row["start"],row["end"],row["normal"],spec,"MAT-PAINTED-WOOD")
    fitted=backend.skirting_stock(row["start"],row["end"],row["normal"],spec,"MAT-PAINTED-WOOD",back_clearance=row["back_clearance"])
    assert raw["z0"]==fitted["z0"]==0 and fitted["z1"]==raw["z1"]
    a=np.asarray(raw["mesh"]["vertices"]);b=np.asarray(fitted["mesh"]["vertices"])
    assert np.allclose(b-a,np.asarray(row["normal"])*row["back_clearance"],rtol=0,atol=1e-17)
    assert abs(api._polyhedron_volume(raw)-api._polyhedron_volume(fitted))<1e-15
    assert abs(ConvexHull(b).volume-api._polyhedron_volume(raw))<1e-15
    assert Polygon(fitted["outline"]).area>0

def test_authored_interface_fitting_planes_keep_exact_complements_and_independent_volume():
    """📏 Signed fitting offsets belong to newly authored stock placement and conserve both exact complementary branches."""
    from shapely.geometry import Polygon
    from scipy.spatial import ConvexHull
    import numpy as np
    backend=api.surface_finishes();row=CASE["skirting"]["plane_provenance"];spec=CASE["skirting"]["section"]
    raw=backend.skirting_stock(row["start"],row["end"],row["normal"],spec,"MAT-PAINTED-WOOD")
    plane={**row["plane"],"clearance":row["back_clearance"]}
    inside=backend.clip_stock(raw,[plane],exact=True)
    outside=backend.clip_stock(raw,[{**plane,"normal":[-v for v in plane["normal"]],"offset":-plane["offset"],"clearance":-plane["clearance"]}],exact=True)
    assert inside is not None and outside is not None
    n=len(raw["mesh"]["vertices"])//2;profile=Polygon([[v[1],v[2]] for v in raw["mesh"]["vertices"][:n]])
    expected=profile.area*(row["expected_intercept"]+row["back_clearance"]*2**.5-profile.centroid.x)
    assert abs(api._polyhedron_volume(inside)-expected)<1e-14
    assert abs(api._polyhedron_volume(inside)+api._polyhedron_volume(outside)-api._polyhedron_volume(raw))<1e-14
    for part in (inside,outside):
        p=np.asarray(part["mesh"]["vertices"])
        assert abs(ConvexHull(p-p[0]).volume-api._polyhedron_volume(part))<1e-14
