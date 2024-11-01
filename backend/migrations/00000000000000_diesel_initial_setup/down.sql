-- This file was automatically created by Diesel to setup helper functions
-- and other internal bookkeeping. This file is safe to edit, any future
-- changes will be added to existing projects as new migrations.

DROP TABLE IF EXISTS embeddings;
DROP TABLE IF EXISTS article_chunks;
DROP TABLE IF EXISTS articles;
DROP TABLE IF EXISTS collections;

DROP EXTENSION IF EXISTS vector;
DROP EXTENSION IF EXISTS pg_trgm;
DROP EXTENSION IF EXISTS uuid-ossp;

DROP FUNCTION IF EXISTS diesel_manage_updated_at(_tbl regclass);
DROP FUNCTION IF EXISTS diesel_set_updated_at();