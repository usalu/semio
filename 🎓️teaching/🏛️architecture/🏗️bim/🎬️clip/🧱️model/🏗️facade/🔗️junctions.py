"""🔗 Declared material-layer junction ownership; https://standards.buildingsmart.org/IFC/DEV/IFC4_3/HTML/lexical/IfcRelConnectsPathElements.html."""
from copy import deepcopy
from fractions import Fraction
import math


def _layers(api,model,wall):
    """🧱 Keep priority order identical to the source material layer set."""
    material=api._material(model,wall)
    return material.get('layers') or [{'id':'body','material':material['tag']}]


def _on_path(point,path):
    """📐 Test exact source-coordinate incidence without snapping a registered axis."""
    p=tuple(Fraction(v) for v in point)
    for first,second in zip(path,path[1:]):
        a,b=(tuple(Fraction(v) for v in row) for row in (first,second));u=(b[0]-a[0],b[1]-a[1]);v=(p[0]-a[0],p[1]-a[1])
        if u[0]*v[1]-u[1]*v[0]==0 and all(min(a[i],b[i])<=p[i]<=max(a[i],b[i]) for i in (0,1)):return True
    return False


def path_material_layers(api,model,wall):
    """📚 Expose ordered native layer authority only for one unambiguous source layer set."""
    material=api._material(model,wall) or {};geometry=wall['geometry']
    return material.get('layers',[]) if not geometry.get('surface_regions') and not geometry.get('layer_regions') else []


def context(api,model):
    """📜 Validate explicit local authority and own one query's unjoined wall snapshots."""
    records=[];index={};groups={};ranks={};pairs=set()
    for relation in model.get('relations',()):
        if relation['class']=='IfcRelConnectsPathElements':data=relation.get('path_connection')
        elif relation['class']=='IfcRelConnectsElements' and relation.get('connection',{}).get('kind')=='material_junction':data=relation['connection']
        else:continue
        if not data or len(relation.get('related',()))!=1:raise ValueError('A wall junction needs exactly two declared participants')
        tags=(relation['relating'],relation['related'][0])
        if tags[0]==tags[1]:raise ValueError('A wall cannot connect to itself')
        bounds=data.get('bounds',())
        if len(bounds)!=4 or any(type(value) not in (int,float) or not math.isfinite(value) for value in bounds) or not(bounds[0]<bounds[2] and bounds[1]<bounds[3]):raise ValueError('Wall junction bounds need finite positive model-metre extents')
        group=data['junction'];scope=tuple(bounds)
        if group in groups and groups[group]!=scope:raise ValueError('One junction must use one declared bounded scope')
        groups[group]=scope;maps={};voids={};paths=[]
        for tag,field in zip(tags,('relating_priorities','related_priorities')):
            wall=api._find(model,tag)
            if wall.get('class')!='IfcWall' or 'path' not in wall.get('geometry',{}):raise ValueError('Layered junctions require actual source path walls')
            layers=_layers(api,model,wall);values=data.get(field,())
            if len(values)!=len(layers) or any(type(value) is not int or not 0<=value<=100 for value in values):raise ValueError('A junction needs one normalized integer priority per source layer')
            key=group,tag;priorities=tuple(values)
            if key in ranks and ranks[key]!=priorities:raise ValueError('One wall must have consistent priorities within one junction')
            ranks[key]=priorities;maps[tag]={layer['id']:value for layer,value in zip(layers,values)};voids[tag]={layer['id'] for layer in layers if layer.get('is_void')};paths.append(api._wall_path(model,wall))
        active=[{value for key,value in maps[tag].items() if key not in voids[tag]} for tag in tags]
        if active[0]&active[1]:raise ValueError('Equal joined material priorities require an explicit unambiguous cap rule')
        if relation['class']=='IfcRelConnectsPathElements':
            types=[data.get(field) for field in ('relating_connection_type','related_connection_type')]
            if any(value not in ('ATSTART','ATEND','ATPATH') for value in types) or types==['ATPATH','ATPATH']:raise ValueError('Native path junctions require declared L or T connection positions')
            endpoints=[path[0] if kind=='ATSTART' else path[-1] if kind=='ATEND' else None for path,kind in zip(paths,types)]
            point=next(value for value in endpoints if value is not None)
            if any(value is not None and value!=point for value in endpoints) or not all(_on_path(point,path) for path in paths):raise ValueError('Native path connection axes must actually connect; use a bounded general material interface for offset references')
            if not(bounds[0]<=point[0]<=bounds[2] and bounds[1]<=point[1]<=bounds[3]):raise ValueError('The path connection point lies outside its declared junction scope')
        pair=group,frozenset(tags)
        if pair in pairs:raise ValueError('A junction pair must be declared only once')
        pairs.add(pair);record={'tag':relation['tag'],'group':group,'tags':tags,'bounds':scope,'maps':maps};records.append(record)
        for tag in tags:index.setdefault(tag,[]).append(record)
    for first,rows in index.items():
        for i,a in enumerate(rows):
            for b in rows[i+1:]:
                if a['group']!=b['group'] and all(max(a['bounds'][k],b['bounds'][k])<min(a['bounds'][k+2],b['bounds'][k+2]) for k in (0,1)):raise ValueError(f'Overlapping authority at {first}: {a["group"]} / {b["group"]} must be combined into one consistent declared junction')
    return {'index':index,'raw':{},'scoped':{}}


def _analytic(part):
    """⭕ Keep original curved stock and existing Boolean authority out of display-cell construction."""
    return 'csg' in part or bool(part.get('profile'))


def _operand(api,part):
    """🧊 Retain the exact finite topology of affine native stocks in semantic operations."""
    backend=api._analytic_solids_backend()
    return backend.operand(part,api.part_vertices(part),api.part_faces(part)) if 'section' in part or 'top_outline' in part else backend.operand(part)


def _volume(api,part,progress=None,cancel=None):
    """⚖️ Measure semantic material with cooperative cancellation and finite native cells exactly."""
    return api._analytic_solids_backend().measure(part,progress=progress,cancel=cancel)['volume'] if 'csg' in part else api.part_volume(part)


def _scope(api,part,bounds,progress=None,cancel=None):
    """✂️ Restrict a real material cutter to its exact declared junction window."""
    backend=api._analytic_solids_backend();box=backend.bounds(part) if _analytic(part) else api._material_bounds(part);x0,y0,x1,y1=bounds
    if box[0][1]<=x0 or box[0][0]>=x1 or box[1][1]<=y0 or box[1][0]>=y1:return []
    if x0<=box[0][0] and box[0][1]<=x1 and y0<=box[1][0] and box[1][1]<=y1:return [part]
    if _analytic(part):
        boundary={'outline':[[x0,y0],[x1,y0],[x1,y1],[x0,y1]],'holes':[],'z0':box[2][0],'z1':box[2][1]}
        return backend.reference_material_parts(part,[],bounds=[boundary],progress=progress,cancel=cancel)
    finish=api.surface_finishes();planes=[{'normal':[-1,0,0],'point':[x0,0,0],'offset':-x0},{'normal':[1,0,0],'point':[x1,0,0],'offset':x1},{'normal':[0,-1,0],'point':[0,y0,0],'offset':-y0},{'normal':[0,1,0],'point':[0,y1,0],'offset':y1}];result=[]
    for cell in api._native_material_cells(part):
        stock=finish.clip_stock({**cell,'role':cell.get('role','body')},planes,exact=True)
        if stock is not None:result.append(finish.publish_stock(stock))
    return result


def partition(api,model,wall,parts,state,air=False,progress=None,cancel=None):
    """⚖️ Assign only actual common material to explicitly stronger connected layers."""
    links=state['index'].get(wall['tag'],())
    if not links:return deepcopy(parts),0.
    def check():
        """🛑 Never publish partially resolved expensive material ownership."""
        if cancel and cancel():raise InterruptedError('Wall junction material construction cancelled')
    check();cutters=[]
    for link in links:
        other=next(tag for tag in link['tags'] if tag!=wall['tag']);check()
        if other not in state['raw']:state['raw'][other]=api._wall_geometry(model,api._find(model,other),resolve_junctions=False,progress=progress,cancel=cancel)
        for i,part in enumerate(state['raw'][other]['parts']):
            layer=part.get('layer','body')
            if layer not in link['maps'][other]:raise ValueError('A joined material cell has no declared source layer priority')
            key=link['tag'],other,i
            if key not in state['scoped']:state['scoped'][key]=_scope(api,part,link['bounds'],progress=progress,cancel=cancel)
            cutters.extend((link,link['maps'][other][layer],cell,other) for cell in state['scoped'][key])
    result=[]
    for index,part in enumerate(parts):
        check();layer=part.get('layer','body');active=[]
        for link,priority,cutter,other in cutters:
            if layer not in link['maps'][wall['tag']]:raise ValueError('A joined material cell has no declared source layer priority')
            if air or priority>link['maps'][wall['tag']][layer]:active.append((cutter,other))
        if active and (_analytic(part) or any(_analytic(cutter) for cutter,other in active)):
            wrappers=[{'operand':_operand(api,cutter),'owner':other,'owner_guid':api._find(model,other)['guid'],'role':'material'} for cutter,other in active]
            stock={**part,'mesh':{'vertices':api.part_vertices(part),'faces':api.part_faces(part)}} if 'section' in part or 'top_outline' in part else part
            owned=[api._analytic_solids_backend().difference(stock,wrappers)]
        else:owned,_=api._subtract_native_material([part],[cutter for cutter,other in active])
        result.extend(owned)
        if progress:progress({'phase':'wall-junctions','wall':wall['tag'],'completed':index+1,'total':len(parts)})
    check();before=sum(_volume(api,part,progress,cancel) for part in parts);after=sum(_volume(api,part,progress,cancel) for part in result)
    return result,max(0.,before-after)


def producers():
    """🧬 Include every first-party junction producer in source and private memo identity."""
    return (_layers,_on_path,path_material_layers,context,_analytic,_operand,_volume,_scope,partition)
_BIM_LOADED_MODULE_CODE = __import__("sys")._getframe().f_code
