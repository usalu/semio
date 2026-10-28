import type {ExpressExpression as Expr,ExpressFunction,ExpressGlobalRule,ExpressStatement as Stmt,ExpressVariable,ExpressRule} from "./ast.ts";

interface Token {text:string;kind:"word"|"number"|"string"|"symbol"|"end";offset:number}
/** 🚫 A source-positioned EXPRESS syntax refusal. */
export class ExpressSyntaxError extends Error {
  constructor(message:string,readonly offset:number){super(`${message} at ${offset}`);this.name="ExpressSyntaxError";}
}

function lex(source:string,maximum=2_000_000):Token[]{
  const tokens:Token[]=[];
  let at=0;
  while(at<source.length){
    if(tokens.length>=maximum)throw new ExpressSyntaxError("EXPRESS token budget exceeded",at);
    const start=at,c=source[at]!;
    if(/\s/.test(c)){at++;continue;}
    if(source.startsWith("--",at)){while(at<source.length&&source[at]!=="\n")at++;continue;}
    if(source.startsWith("(*",at)){
      at+=2;let depth=1;
      while(at<source.length&&depth){if(source.startsWith("(*",at)){depth++;at+=2;}else if(source.startsWith("*)",at)){depth--;at+=2;}else at++;}
      if(depth)throw new ExpressSyntaxError("Unterminated EXPRESS comment",start);
      continue;
    }
    if(c==="'"){
      at++;let text="",closed=false;
      while(at<source.length){const v=source[at++]!;if(v==="'"){if(source[at]==="'"){text+="'";at++;}else{closed=true;break;}}else text+=v;}
      if(!closed)throw new ExpressSyntaxError("Unterminated EXPRESS string",start);
      tokens.push({text,kind:"string",offset:start});continue;
    }
    if(/[A-Za-z_]/.test(c)){at++;while(at<source.length&&/[A-Za-z0-9_]/.test(source[at]!))at++;tokens.push({text:source.slice(start,at).toUpperCase(),kind:"word",offset:start});continue;}
    if(/[0-9]/.test(c)){
      at++;while(at<source.length&&/[0-9]/.test(source[at]!))at++;
      if(source[at]==="."){at++;while(at<source.length&&/[0-9]/.test(source[at]!))at++;}
      if(/[Ee]/.test(source[at]??"")){at++;if(/[+-]/.test(source[at]??""))at++;const exponent=at;while(at<source.length&&/[0-9]/.test(source[at]!))at++;if(exponent===at)throw new ExpressSyntaxError("Malformed exponent",start);}
      tokens.push({text:source.slice(start,at),kind:"number",offset:start});continue;
    }
    const operator=[":<>:",":=:","<*",":=","<=",">=","<>","**","||"].find(operator=>source.startsWith(operator,at));
    if(operator){at+=operator.length;tokens.push({text:operator,kind:"symbol",offset:start});continue;}
    if("()[]{}.,;:+-*/=<>?|\\".includes(c)){at++;tokens.push({text:c,kind:"symbol",offset:start});continue;}
    throw new ExpressSyntaxError(`Unsupported EXPRESS token ${c}`,at);
  }
  tokens.push({text:"",kind:"end",offset:source.length});return tokens;
}

const precedence:Record<string,number>={OR:4,XOR:4,AND:5,"=":3,"<>":3,":=:":3,":<>:":3,"<":3,"<=":3,">":3,">=":3,IN:3,LIKE:3,"+":4,"-":4,"||":5,"*":5,"/":5,DIV:5,MOD:5,"**":7};

/** 📖 Bounded Pratt and imperative parser over the retained EXPRESS source. */
export class ExpressParser {
  readonly tokens:Token[];
  at=0;
  depth=0;
  constructor(readonly source:string,readonly maximumDepth=256){this.tokens=lex(source);}
  peek():Token{return this.tokens[this.at]!;}
  take():Token{return this.tokens[this.at++]!;}
  accept(text:string):boolean{if(this.peek().text!==text)return false;this.at++;return true;}
  require(text:string):Token{const token=this.take();if(token.text!==text)throw new ExpressSyntaxError(`Expected ${text}, received ${token.text||"end"}`,token.offset);return token;}
  name():string{const token=this.take();if(token.kind!=="word")throw new ExpressSyntaxError("Identifier required",token.offset);return token.text;}
  done():void{if(this.peek().kind!=="end")throw new ExpressSyntaxError(`Unexpected ${this.peek().text}`,this.peek().offset);}
  expression(minimum=0):Expr{
    if(++this.depth>this.maximumDepth)throw new ExpressSyntaxError("EXPRESS expression depth exceeded",this.peek().offset);
    let value=this.prefix();
    while(true){
      const next=this.peek(),operator=next.text;
      if(minimum<=8&&operator==="("){
        this.take();const args:Expr[]=[];if(!this.accept(")")){do{args.push(this.expression());}while(this.accept(","));this.require(")");}value={kind:"call",offset:value.offset,operands:[value,...args]};continue;
      }
      if(minimum<=8&&(operator==="."||operator==="\\")){this.take();value={kind:operator==="."?"attribute":"qualify",offset:value.offset,name:this.name(),operands:[value]};continue;}
      if(minimum<=8&&operator==="["){
        this.take();const lo=this.expression();if(this.accept(":")){const hi=this.expression();this.require("]");value={kind:"slice",offset:value.offset,operands:[value,lo,hi]};}else{this.require("]");value={kind:"index",offset:value.offset,operands:[value,lo]};}continue;
      }
      const binding=precedence[operator];if(binding===undefined||binding<minimum)break;
      this.take();const right=this.expression(binding+(operator==="**"?0:1));value={kind:"binary",offset:value.offset,operator,operands:[value,right]};
    }
    this.depth--;return value;
  }
  private prefix():Expr{
    const token=this.take(),text=token.text,offset=token.offset;
    if(token.kind==="string")return{kind:"string",offset,text};
    if(token.kind==="number")return{kind:/[.Ee]/.test(text)?"real":"integer",offset,text};
    if(text==="?")return{kind:"unknown",offset};
    if(["TRUE","FALSE","UNKNOWN"].includes(text))return{kind:"logical",offset,text:text.toLowerCase()};
    if(["NOT","+","-"].includes(text))return{kind:"unary",offset,operator:text,operands:[this.expression(8)]};
    if(text==="("){const value=this.expression();this.require(")");return value;}
    if(text==="["){
      const entries:NonNullable<Expr["entries"]>[number][]=[];
      if(!this.accept("]")){do{const value=this.expression();entries.push(this.accept(":")?{value,repeat:this.expression()}:{value});}while(this.accept(","));this.require("]");}
      return{kind:"aggregate",offset,entries};
    }
    if(text==="{"){
      const lower=this.expression(4),lo=this.take().text;
      if(!["<","<="].includes(lo))throw new ExpressSyntaxError("Interval lower comparison required",offset);
      const value=this.expression(4),hi=this.take().text;
      if(!["<","<="].includes(hi))throw new ExpressSyntaxError("Interval upper comparison required",offset);
      const upper=this.expression(4);this.require("}");return{kind:"interval",offset,operator:`${lo},${hi}`,operands:[lower,value,upper]};
    }
    if(text==="QUERY"){
      this.require("(");const name=this.name();this.require("<*");const population=this.expression();this.require("|");const predicate=this.expression();this.require(")");return{kind:"query",offset,name,operands:[population,predicate]};
    }
    if(token.kind==="word")return{kind:"identifier",offset,name:text};
    throw new ExpressSyntaxError(`Expression required, received ${text||"end"}`,offset);
  }
  private rawType(stops:Set<string>):string{
    const start=this.peek().offset;let brackets=0,parens=0;
    while(this.peek().kind!=="end"){
      const text=this.peek().text;if(!brackets&&!parens&&stops.has(text))break;
      if(text==="[")brackets++;if(text==="]")brackets--;if(text==="(")parens++;if(text===")")parens--;
      this.take();
    }
    return this.source.slice(start,this.peek().offset).trim();
  }
  private declaration(stops:Set<string>):ExpressVariable{
    this.accept("VAR");const names=[this.name()];while(this.accept(","))names.push(this.name());this.require(":");
    const type=this.rawType(new Set([...stops,":="]));
    return this.accept(":=")?{names,type,initial:this.expression()}:{names,type};
  }
  locals():ExpressVariable[]{const locals:ExpressVariable[]=[];if(this.accept("LOCAL")){while(!this.accept("END_LOCAL")){locals.push(this.declaration(new Set([";"])));this.require(";");}this.require(";");}return locals;}
  statements(stops:Set<string>):Stmt[]{const body:Stmt[]=[];while(this.peek().kind!=="end"&&!stops.has(this.peek().text))body.push(this.statement());return body;}
  statement():Stmt{
    const token=this.peek(),offset=token.offset;
    if(this.accept(";"))return{kind:"null",offset};
    if(this.accept("RETURN")){const value=this.expression();this.require(";");return{kind:"return",offset,expressions:[value]};}
    if(this.accept("ESCAPE")||this.accept("SKIP")){this.require(";");return{kind:token.text==="ESCAPE"?"escape":"skip",offset};}
    if(this.accept("BEGIN")){const body=this.statements(new Set(["END"]));this.require("END");this.require(";");return{kind:"block",offset,body};}
    if(this.accept("IF")){
      const condition=this.expression();this.require("THEN");const body=this.statements(new Set(["ELSE","END_IF"]));const otherwise=this.accept("ELSE")?this.statements(new Set(["END_IF"])):[];this.require("END_IF");this.require(";");return{kind:"if",offset,expressions:[condition],body,otherwise};
    }
    if(this.accept("CASE")){
      const selector=this.expression();this.require("OF");const cases:NonNullable<Stmt["cases"]>[number][]=[];let otherwise:Stmt[]=[];
      while(!this.accept("END_CASE")){
        if(this.accept("OTHERWISE")){this.accept(":");otherwise=[this.statement()];continue;}
        const labels=[this.expression()];while(this.accept(","))labels.push(this.expression());this.require(":");cases.push({labels,body:[this.statement()]});
      }
      this.require(";");return{kind:"case",offset,expressions:[selector],cases,otherwise};
    }
    if(this.accept("REPEAT")){
      const result:{kind:'repeat';offset:number;name?:string;expressions:Expr[];body?:Stmt[];while?:Expr;until?:Expr}={kind:"repeat",offset,expressions:[]};
      if(this.tokens[this.at+1]?.text===":="){result.name=this.name();this.require(":=");result.expressions!.push(this.expression());this.require("TO");result.expressions!.push(this.expression());result.expressions!.push(this.accept("BY")?this.expression():{kind:"integer",offset,text:"1"});}
      if(this.accept("WHILE"))result.while=this.expression();if(this.accept("UNTIL"))result.until=this.expression();this.require(";");result.body=this.statements(new Set(["END_REPEAT"]));this.require("END_REPEAT");this.require(";");return result;
    }
    const target=this.expression();if(this.accept(":=")){const value=this.expression();this.require(";");return{kind:"assign",offset,expressions:[target,value]};}
    this.require(";");return{kind:"call",offset,expressions:[target]};
  }
  function(name:string):ExpressFunction{
    this.require("(");const parameters:ExpressVariable[]=[];
    if(!this.accept(")")){do{parameters.push(this.declaration(new Set([";",")"])));}while(this.accept(";"));this.require(")");}
    this.require(":");const resultType=this.rawType(new Set([";"]));this.require(";");const locals=this.locals();const body=this.statements(new Set());this.done();return{name,parameters,resultType,locals,body};
  }
  rule(owner:string):ExpressRule{const label=this.name();this.require(":");const expression=this.expression();this.accept(";");return{owner,label,expression};}
  global(name:string):ExpressGlobalRule{
    this.require("FOR");this.require("(");const populations=[this.name()];while(this.accept(","))populations.push(this.name());this.require(")");this.require(";");const locals=this.locals();const body=this.statements(new Set(["WHERE"]));this.require("WHERE");const rules:ExpressRule[]=[];while(this.peek().kind!=="end")rules.push(this.rule(name));return{name,populations,locals,body,rules};
  }
}

/** 🌱 Parse one complete expression without accepting trailing source. */
export function parseExpressExpression(source:string,maximumDepth=256):Expr{const parser=new ExpressParser(source,maximumDepth);const value=parser.expression();parser.done();return value;}

