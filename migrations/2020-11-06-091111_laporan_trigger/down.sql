-- This file should undo anything in `up.sql`

DROP TRIGGER IF EXISTS tr_laporan_insert ON laporan CASCADE;
DROP TRIGGER IF EXISTS tr_laporan_insert_backdate ON laporan CASCADE;
DROP TRIGGER IF EXISTS tr_laporan_insert_draft ON laporan CASCADE;
DROP TRIGGER IF EXISTS tr_laporan_edit ON laporan CASCADE;
DROP TRIGGER IF EXISTS tr_laporan_edit_updated_date ON laporan CASCADE;
DROP TRIGGER IF EXISTS tr_laporan_edit_draft_status ON laporan CASCADE;
DROP TRIGGER IF EXISTS tr_laporan_edit_back_to_draft ON laporan CASCADE;
DROP TRIGGER IF EXISTS tr_laporan_delete ON laporan CASCADE;
DROP TRIGGER IF EXISTS tr_laporan_hard_delete ON laporan CASCADE;
