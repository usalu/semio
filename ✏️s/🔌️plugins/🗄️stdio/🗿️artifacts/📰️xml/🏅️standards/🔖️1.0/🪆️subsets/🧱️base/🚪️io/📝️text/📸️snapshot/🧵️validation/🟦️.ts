/** 🧵 Validates borrowed UTF8 XML1.0 without DTDs or a retained DOM. */
export type XmlByteSource = Uint8Array | readonly number[];
export interface XmlSpan { start: number; end: number }
export type XmlValidationEvent = { kind: 'element'; depth: number; name: XmlSpan } | { kind: 'attribute'; depth: number; name: XmlSpan; value: XmlSpan };
export interface XmlValidationStep { work: number; offset: number; done: boolean; event?: XmlValidationEvent; error?: string }
const space = (c: number) => c === 32 || c === 9 || c === 10 || c === 13;
const character = (c: number) => c === 9 || c === 10 || c === 13 || c >= 32 && c <= 0xd7ff || c >= 0xe000 && c <= 0xfffd || c >= 0x10000 && c <= 0x10ffff;
const startName = (c: number) => c === 58 || c === 95 || c >= 65 && c <= 90 || c >= 97 && c <= 122 || c >= 0xc0 && c <= 0xd6 || c >= 0xd8 && c <= 0xf6 || c >= 0xf8 && c <= 0x2ff || c >= 0x370 && c <= 0x37d || c >= 0x37f && c <= 0x1fff || c >= 0x200c && c <= 0x200d || c >= 0x2070 && c <= 0x218f || c >= 0x2c00 && c <= 0x2fef || c >= 0x3001 && c <= 0xd7ff || c >= 0xf900 && c <= 0xfdcf || c >= 0xfdf0 && c <= 0xfffd || c >= 0x10000 && c <= 0xeffff;
const nameChar = (c: number) => startName(c) || c === 45 || c === 46 || c >= 48 && c <= 57 || c === 0xb7 || c >= 0x300 && c <= 0x36f || c >= 0x203f && c <= 0x2040;
const match = (s: XmlByteSource, a: XmlSpan, b: string) => a.end - a.start === b.length && Array.from(b).every((c,i) => s[a.start+i] === c.charCodeAt(0));
/** 🧭 Each unit reads one byte with at most16 fixed-token lookups, or compares one pair of original bytes. */
export class XmlValidationCursor {
  private offset = 0;
  private mode = 'text';
  private error?: string;
  private terminal = false;
  private cancelled = false;
  private source?: XmlByteSource;
  private scalar = 0;
  private continuation = 0;
  private scalarStart = 0;
  private minimum = 0;
  private depth = 0;
  private roots = 0;
  private ancestors = new Uint32Array(512);
  private attributes = new Uint32Array(2048);
  private count = 0;
  private nameStart = 0;
  private element: XmlSpan = {start:0,end:0};
  private attribute: XmlSpan = {start:0,end:0};
  private valueStart = 0;
  private quote = 0;
  private spaced = false;
  private declaration = false;
  private declarationStage = 0;
  private declarationField = 0;
  private declarationValue = '';
  private declarationLength = 0;
  private piStart = 0;
  private token = 0;
  private tail = 0;
  private returnMode = '';
  private entityMode = 0;
  private entityValue = 0;
  private entityDigits = 0;
  private entityName = '';
  private compare = '';
  private compareIndex = 0;
  private compareOffset = 0;
  private pendingEvent?: XmlValidationEvent;
  private root?: XmlSpan;
  /** 🔄 Reuses fixed metadata for a new immutable member without allocation. */
  restart(): void {
    this.offset=0;this.mode='text';this.error=undefined;this.terminal=false;this.cancelled=false;this.source=undefined;this.scalar=0;this.continuation=0;this.scalarStart=0;this.minimum=0;this.depth=0;this.roots=0;this.count=0;this.nameStart=0;this.valueStart=0;this.quote=0;this.spaced=false;this.declaration=false;this.declarationStage=0;this.declarationField=0;this.declarationValue='';this.declarationLength=0;this.piStart=0;this.token=0;this.tail=0;this.returnMode='';this.entityMode=0;this.entityValue=0;this.entityDigits=0;this.entityName='';this.compare='';this.compareIndex=0;this.compareOffset=0;this.pendingEvent=undefined;this.root=undefined;
  }
  rootName(): XmlSpan | undefined { return this.root && {...this.root}; }
  close(): void { this.cancelled=true; this.terminal=true; this.source=undefined; this.pendingEvent=undefined; }
  terminalIsEmpty(): boolean { return this.cancelled; }
  private fail(code: string): void { this.error='xml.validation.'+code; }
  private event(event: XmlValidationEvent): void { this.pendingEvent=event; }
  private finishElement(): void {
    this.count=0;
    if (this.depth===0) { if(this.roots!==0) {this.fail('root');return;} this.roots++;this.root={...this.element}; }
    if(this.depth===256){this.fail('depth');return;}
    this.ancestors[this.depth*2]=this.element.start;this.ancestors[this.depth*2+1]=this.element.end;this.depth++;
    this.event({kind:'element',depth:this.depth,name:{...this.element}});
  }
  private finishAttribute(s: XmlByteSource, end: number): void {
    if(this.declaration) {
      const v=this.declarationValue;
      if(this.declarationField===1 && v!=='1.0' || this.declarationField===2 && this.declarationLength===0 || this.declarationField===3 && v!=='yes' && v!=='no')this.fail('declaration');
      this.declarationStage=this.declarationField;this.mode='after-value';return;
    }
    this.event({kind:'attribute',depth:this.depth,name:{...this.attribute},value:{start:this.valueStart,end}});this.mode='after-value';
  }
  private accept(c: number, begin: number, end: number, s: XmlByteSource): void {
    let again=true;
    while(again && !this.error) { again=false;
      switch(this.mode) {
      case 'text':
        if(c===0xfeff && begin===0)break;
        if(c===60){this.mode='lt';this.tail=0;break;}
        if(this.depth===0){if(!space(c))this.fail('root');break;}
        if(c===38){this.returnMode='text';this.mode='entity';this.entityMode=0;this.entityName='';this.entityDigits=0;this.entityValue=0;break;}
        if(c===62 && this.tail===2){this.fail('character-data');break;}this.tail=c===93?Math.min(2,this.tail+1):0;break;
      case 'lt':
        if(c===47){this.mode='close-start';break;}
        if(c===33){this.mode='bang';this.token=0;break;}
        if(c===63){this.mode='pi-start';this.piStart=begin-1;break;}
        if(!startName(c)){this.fail('name');break;}this.nameStart=begin;this.mode='open-name';break;
      case 'open-name':
        if(nameChar(c))break;
        this.element={start:this.nameStart,end:begin};this.finishElement();this.mode='between';this.spaced=false;again=true;break;
      case 'between':
        if(space(c)){this.spaced=true;break;}
        if(c===62 && !this.declaration){this.mode='text';this.tail=0;break;}
        if(c===47 && !this.declaration){this.mode='slash';break;}
        if(c===63 && this.declaration){if(this.declarationStage===0)this.fail('declaration');else this.mode='declaration-end';break;}
        if(!this.spaced){this.fail('attribute-space');break;}
        if(!startName(c)){this.fail('name');break;}this.nameStart=begin;this.mode='attribute-name';break;
      case 'attribute-name':
        if(nameChar(c))break;
        this.attribute={start:this.nameStart,end:begin};
        if(this.declaration){
          this.declarationField=match(s,this.attribute,'version')?1:match(s,this.attribute,'encoding')?2:match(s,this.attribute,'standalone')?3:0;
          if(this.declarationField===0 || this.declarationField<=this.declarationStage || this.declarationStage===0 && this.declarationField!==1)this.fail('declaration');
        }else{
          if(this.count===1024){this.fail('attributes');break;}this.compare='attribute';this.compareIndex=0;this.compareOffset=0;
        }
        this.mode='before-equals';again=true;break;
      case 'before-equals':if(space(c))break;if(c!==61){this.fail('attribute-equals');break;}this.mode='before-value';break;
      case 'before-value':if(space(c))break;if(c!==34 && c!==39){this.fail('attribute-value');break;}this.quote=c;this.valueStart=end;this.declarationValue='';this.declarationLength=0;this.mode='value';break;
      case 'value':
        if(c===this.quote){this.finishAttribute(s,begin);break;}
        if(c===60){this.fail('attribute-value');break;}
        if(this.declaration){
          if(this.declarationField===2){if(!(c>=65 && c<=90 || c>=97 && c<=122 || this.declarationLength>0 && (c>=48 && c<=57 || c===46 || c===95 || c===45)))this.fail('declaration');}
          else if(c>127 || this.declarationLength===3)this.fail('declaration');else this.declarationValue+=String.fromCharCode(c);
          this.declarationLength++;break;}
        if(c===38){this.returnMode='value';this.mode='entity';this.entityMode=0;this.entityName='';this.entityDigits=0;this.entityValue=0;}break;
      case 'after-value':this.mode='between';this.spaced=false;again=true;break;
      case 'slash':if(c!==62){this.fail('close');break;}this.depth--;this.mode='text';this.tail=0;break;
      case 'close-start':if(!startName(c)){this.fail('name');break;}this.nameStart=begin;this.mode='close-name';break;
      case 'close-name':
        if(nameChar(c))break;
        this.element={start:this.nameStart,end:begin};if(this.depth===0){this.fail('close-name');break;}
        this.compare='close';this.compareOffset=0;this.mode='close-tail';again=true;break;
      case 'close-tail':if(space(c))break;if(c!==62){this.fail('close');break;}this.mode='text';this.tail=0;break;
      case 'bang':
        if(this.token===0 && c===45){this.mode='comment-open';break;}
        if(this.token===0 && c===91){if(this.depth===0){this.fail('cdata');break;}this.mode='cdata-open';this.token=0;break;}
        this.fail(c===68?'doctype':'markup');break;
      case 'comment-open':if(c!==45)this.fail('comment');else{this.mode='comment';this.tail=0;}break;
      case 'comment':
        if(this.tail===2){if(c!==62)this.fail('comment');else{this.mode='text';this.tail=0;}break;}
        this.tail=c===45?this.tail+1:0;break;
      case 'cdata-open':if(c!=='CDATA['.charCodeAt(this.token++))this.fail('cdata');else if(this.token===6){this.mode='cdata';this.tail=0;}break;
      case 'cdata':if(c===62 && this.tail===2){this.mode='text';this.tail=0;}else this.tail=c===93?Math.min(2,this.tail+1):0;break;
      case 'pi-start':if(!startName(c)){this.fail('processing-instruction');break;}this.nameStart=begin;this.mode='pi-name';break;
      case 'pi-name':
        if(nameChar(c))break;
        this.element={start:this.nameStart,end:begin};
        if(this.element.end-this.element.start===3 && [88,120].includes(s[this.nameStart]) && [77,109].includes(s[this.nameStart+1]) && [76,108].includes(s[this.nameStart+2])){
          if(!match(s,this.element,'xml') || this.piStart!==0 && !(this.piStart===3 && s[0]===239 && s[1]===187 && s[2]===191) || !space(c)){this.fail('processing-instruction');break;}
          this.declaration=true;this.declarationStage=0;this.mode='between';this.spaced=true;break;
        }
        if(c===63){this.mode='pi-end';break;}if(!space(c)){this.fail('processing-instruction');break;}this.mode='pi-body';break;
      case 'pi-body':if(c===63)this.mode='pi-end';break;
      case 'pi-end':if(c===62){this.mode='text';this.tail=0;}else if(c!==63)this.mode='pi-body';break;
      case 'declaration-end':if(c!==62)this.fail('declaration');else{this.declaration=false;this.mode='text';}break;
      case 'entity':
        if(c===59){
          if(this.entityMode===0){if(!['amp','lt','gt','apos','quot'].includes(this.entityName))this.fail('entity');}
          else if(!this.entityDigits)this.fail('entity');else if(!character(this.entityValue))this.fail('character');
          this.mode=this.returnMode;this.tail=0;break;
        }
        if(this.entityMode===0 && this.entityName==='' && c===35){this.entityMode=10;break;}
        if(this.entityMode===10 && !this.entityDigits && c===120){this.entityMode=16;break;}
        if(this.entityMode===0){if(c>127 || this.entityName.length===5){this.fail('entity');break;}this.entityName+=String.fromCharCode(c);break;}
        const d=c>=48 && c<=57?c-48:this.entityMode===16 && c>=65 && c<=70?c-55:this.entityMode===16 && c>=97 && c<=102?c-87:-1;
        if(d<0 || d>=this.entityMode || this.entityValue>0x10ffff){this.fail('entity');break;}this.entityDigits++;this.entityValue=this.entityValue*this.entityMode+d;break;
      default:this.fail('state');
      }
    }
  }
  private metadata(s: XmlByteSource): void {
    if(this.compare==='close') {
      const a=this.ancestors[(this.depth-1)*2],b=this.ancestors[(this.depth-1)*2+1];
      if(b-a!==this.element.end-this.element.start){this.fail('close-name');this.compare='';return;}
      if(this.compareOffset===b-a){this.depth--;this.compare='';return;}
      if(s[a+this.compareOffset]!==s[this.element.start+this.compareOffset])this.fail('close-name');this.compareOffset++;return;
    }
    if(this.compareIndex===this.count){this.attributes[this.count*2]=this.attribute.start;this.attributes[this.count*2+1]=this.attribute.end;this.count++;this.compare='';return;}
    const a=this.attributes[this.compareIndex*2],b=this.attributes[this.compareIndex*2+1];
    if(b-a!==this.attribute.end-this.attribute.start){this.compareIndex++;this.compareOffset=0;return;}
    if(this.compareOffset===b-a){this.fail('duplicate-attribute');return;}
    if(s[a+this.compareOffset]!==s[this.attribute.start+this.compareOffset]){this.compareIndex++;this.compareOffset=0;}else this.compareOffset++;
  }
  advance(source: XmlByteSource, maximumWork: number): XmlValidationStep {
    if(!Number.isSafeInteger(maximumWork) || maximumWork<0)throw new Error('xml.validation.grant');
    if(this.cancelled)return {work:0,offset:this.offset,done:true,error:'xml.validation.closed'};
    if(!this.source)this.source=source;else if(this.source!==source)this.fail('source');
    if(source.length>8388608)this.fail('source-limit');
    let work=0;
    while(work<maximumWork && !this.error && !this.terminal && !this.pendingEvent){
      if(this.compare){this.metadata(source);work++;continue;}
      if(this.offset===source.length){
        if(this.continuation)this.fail('utf8');else if(this.mode!=='text' || this.depth)this.fail('unclosed');else if(this.roots!==1)this.fail('root');else this.terminal=true;work++;break;
      }
      const b=source[this.offset++];work++;
      if(!Number.isInteger(b) || b<0 || b>255){this.fail('utf8');continue;}
      if(!this.continuation){this.scalarStart=this.offset-1;
        if(b<128){if(!character(b))this.fail('character');else this.accept(b,this.scalarStart,this.offset,source);continue;}
        if(b>=194 && b<=223){this.scalar=b&31;this.continuation=1;this.minimum=128;}
        else if(b>=224 && b<=239){this.scalar=b&15;this.continuation=2;this.minimum=2048;}
        else if(b>=240 && b<=244){this.scalar=b&7;this.continuation=3;this.minimum=65536;}else this.fail('utf8');
      }else{
        if((b&192)!==128){this.fail('utf8');continue;}this.scalar=this.scalar*64+(b&63);this.continuation--;
        if(!this.continuation){if(this.scalar<this.minimum || this.scalar>0x10ffff || this.scalar>=0xd800 && this.scalar<=0xdfff)this.fail('utf8');else if(!character(this.scalar))this.fail('character');else this.accept(this.scalar,this.scalarStart,this.offset,source);}
      }
    }
    const event=this.pendingEvent;this.pendingEvent=undefined;return {work,offset:this.offset,done:this.terminal,event,error:this.error};
  }
}
