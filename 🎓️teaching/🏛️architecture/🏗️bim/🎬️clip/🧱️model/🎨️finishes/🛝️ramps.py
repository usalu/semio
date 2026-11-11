"""🛝️ Original affine wearing construction shared by physical floors and receiving stock."""
from copy import deepcopy
import math


def reference_stocks(api,model,obj):
    """📐 Retain source walking axes and declared layer depths before any float publication."""
    spec=obj['geometry']['ramp_finish'];ramp=api._find(model,spec['ramp']);q=api._source_fraction
    if obj['geometry']['base_level']!=0:raise ValueError('Ramp finish construction uses world source datums')
    if ramp['class']!='IfcRamp':raise ValueError('A ramp finish requires an authored ramp')
    layers=spec['layers'];roles=[layer['role'] for layer in layers]
    if roles[0]!='wearing' or len(set(roles))!=len(roles):raise ValueError('Ramp finish layers require unique roles in wearing-to-bedding order')
    result=[]
    for flight in [e for e in model['elements'] if e['class']=='IfcRampFlight' and e['geometry']['ramp']==ramp['tag']]:
        g=flight['geometry'];run=math.dist(*g['path']);rise=(api._z(model,g['to_level'])-api._z(model,g['from_level']))*(g['portion'][1]-g['portion'][0])
        factor=q(math.hypot(1.,api._round(rise/run))) if g['finish_direction']=='normal' else q(1)
        top=q(0)
        for layer in layers:
            depth=q(layer['depth']['value']);source=deepcopy(flight)
            if depth<=0:raise ValueError('Ramp finish layer depth must be positive')
            source['geometry']['structural_offsets']=deepcopy(g['walking_offsets'])
            source['geometry']['finish_thickness']['value']=top
            source['geometry']['thickness']['value']=depth*factor
            part=api._ramp_reference_stock(model,source)
            part.update(material=layer['material'],role=layer['role']);result.append(part);top+=depth
    for landing in ramp['geometry']['landings']:
        if landing['ownership']!='ramp_component' or landing['owner']!=ramp['tag']:continue
        z=q(api._z(model,ramp['geometry']['from_level']))+(q(api._z(model,ramp['geometry']['to_level']))-q(api._z(model,ramp['geometry']['from_level'])))*q(landing['portion']);top=q(0)
        for layer in layers:
            depth=q(layer['depth']['value'])
            result.append({'outline':[[q(v) for v in p] for p in landing['walking_outline']],'z0':z-top-depth,'z1':z-top,'material':layer['material'],'role':layer['role']});top+=depth
    return result


def reference_parts(api,model,obj):
    """🧩 Reserve declared actual owners while retaining the complete original rational continuation."""
    spec=obj['geometry']['ramp_finish'];refs=spec['interface_owners']+spec['wall_owners']
    if len(refs)!=len(set(refs)) or obj['tag'] in refs:raise ValueError('Ramp finish owners must be distinct independent products')
    cutters=api._declared_structural_interfaces(model,obj,reference=True)
    for key in spec['wall_owners']:
        wall=api._find(model,key)
        if wall['class']!='IfcWall':raise ValueError('A ramp finish wall owner must be a physical wall')
        cutters.extend(api._wall_geometry(model,wall)['parts'])
    return [part for stock in reference_stocks(api,model,obj) for part in api._reference_material_parts(stock,cutters)]


def geometry(api,model,obj):
    """📤 Publish native physical material once from the shared affine construction."""
    parts=api._native_public_value(api._ramp_finish_reference_parts(model,obj))
    for part in parts:
        part.update(api._polyhedron_part(api.part_vertices(part),api.part_faces(part)))
    api.surface_finishes().apply_material_appearance(parts,lambda tag:api._find(model,tag),obj.get('material'))
    return {'tag':obj['tag'],'guid':obj['guid'],'class':obj['class'],'storey':obj['storey'],'kind':obj['geometry']['kind'],'predefined_type':obj.get('predefined_type'),'name':deepcopy(obj['name']),'domains':deepcopy(obj['metadata']['domains']),'parts':parts,'shared_parts':[]}


def partitioned(api,model,obj):
    """🛡️ Prove original component separation before summing their constructed material cells."""
    stocks=reference_stocks(api,model,obj)
    for i,left in enumerate(stocks):
        for right in stocks[:i]:
            a,b=[api.part_vertices(part) for part in (left,right)];separate=False
            for part,vertices in ((left,a),(right,b)):
                for face in api.part_faces(part):
                    origin=vertices[face[0]]
                    for j in range(1,len(face)-1):
                        u,v=[[vertices[k][axis]-origin[axis] for axis in range(3)] for k in (face[j],face[j+1])]
                        normal=[u[1]*v[2]-u[2]*v[1],u[2]*v[0]-u[0]*v[2],u[0]*v[1]-u[1]*v[0]]
                        if any(normal):break
                    else:continue
                    x,y=[[sum(n*v for n,v in zip(normal,point)) for point in points] for points in (a,b)]
                    if max(x)<=min(y) or max(y)<=min(x):separate=True;break
                if separate:break
            if not separate:return False
    return True
_BIM_LOADED_MODULE_CODE = __import__("sys")._getframe().f_code
