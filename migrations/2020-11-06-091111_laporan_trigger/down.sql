-- This file should undo anything in `up.sql`

DROP TRIGGER IF EXISTS tr_laporan_insert ON laporan CASCADE;
DROP TRIGGER IF EXISTS tr_laporan_insert_backdate ON laporan CASCADE;
DROP TRIGGER IF EXISTS tr_laporan_edit ON laporan CASCADE;