/** 📋️ Contract: the prepare commands render the signed tag, squash, push and all-in-one blocks with every operand single-quoted for the shell, and the agent reply appends the tag name and the message; the corpus is language-agnostic and `🐍️oracle.py` recovers the operands with `shlex` as the independent shell oracle. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {formatCommitPrepareAgentReply,formatCommitPrepareCommands,formatGitSignedTagCommand} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1();
test("the signed tag command quotes the name as one shell word, twice",()=>{
 for(const tag of corpus.tags){const command=formatGitSignedTagCommand(tag);expect(command.startsWith("git tag -s -m ")).toBe(true);expect(command.endsWith(" HEAD")).toBe(true);}
 expect(formatGitSignedTagCommand("v1","abc")).toBe("git tag -s -m 'v1' 'v1' abc");expect(formatGitSignedTagCommand("it's")).toBe("git tag -s -m 'it'\"'\"'s' 'it'\"'\"'s' HEAD");
});
for(const[index,row]of corpus.cases.entries())test("prepare commands "+index+": "+JSON.stringify(row.tagName),()=>{
 const text=formatCommitPrepareCommands({tagName:row.tagName,wipSha:row.wipSha,messageFile:(row as {messageFile?:string}).messageFile}),blocks=text.trimEnd().split("\n\n");
 expect(text).toBe(row.expected);expect(blocks.length).toBe(4);expect(text.endsWith("\n")).toBe(true);for(const block of blocks){expect(block.startsWith("```\n")).toBe(true);expect(block.endsWith("\n```")).toBe(true);}
 const lines=blocks.map(block=>block.slice(4,-4));expect(lines[0]).toBe(formatGitSignedTagCommand(row.tagName));expect(lines[1]!.startsWith("git reset --soft "+row.wipSha+" && git commit -S -F ")).toBe(true);expect(lines[2]).toBe("git push --follow-tags");
 expect(lines[3]).toBe(lines[0]+" && "+lines[1]+" && "+lines[2]);
});
test("the agent reply is the four blocks, the tag name block and the message block",()=>{
 const reply=formatCommitPrepareAgentReply(corpus.reply),commands=formatCommitPrepareCommands({tagName:corpus.reply.tagName,wipSha:corpus.reply.wipSha,messageFile:corpus.reply.messageFile});
 expect(reply.startsWith(commands)).toBe(true);expect(reply.slice(commands.length)).toBe("```\n"+corpus.reply.tagName.trim()+"\n```\n\n```\nline one\nline two\n```\n");
});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)return;
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️oracle.py"),join(here,"🔣️.json")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
