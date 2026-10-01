/** 🗄️ The owner's authored SQLite DDL is an intrinsic UTF-8 source asset. */
declare module "*.sql" { const sql: string; export default sql; }
