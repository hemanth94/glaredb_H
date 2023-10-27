import glaredb from './glaredb.js'

let con = await glaredb.connect();
let res = await con.sql("select * from '../testdata/json/userdata1.json' limit 1");
let df = await res.toPolars();
console.log("polars\n---\n ", df.toString());
