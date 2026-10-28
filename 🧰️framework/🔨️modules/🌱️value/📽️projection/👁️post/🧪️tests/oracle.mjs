import Ajv from 'ajv/dist/2020.js';import{cloneDeep,isEqual}from'lodash-es';
export function postCustodyOracle(schema,fixture){const validate=new Ajv({strict:true}).compile(schema);if(!validate(fixture))throw Error(JSON.stringify(validate.errors));return{snapshot:cloneDeep,equal:isEqual,retire(value,original){if(value!==original)throw Error('Original snapshot issuer received another owner');}};}
