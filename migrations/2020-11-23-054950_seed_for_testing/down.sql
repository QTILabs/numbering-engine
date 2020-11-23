-- This file should undo anything in `up.sql`

DELETE FROM laporan;

DELETE FROM public.user;

DELETE FROM referensi;
ALTER TABLE referensi DROP COLUMN IF EXISTS parent_id;

DELETE FROM satuan_kerja;
ALTER SEQUENCE satuan_kerja_id_seq RESTART WITH 1;
UPDATE referensi SET id=nextval('satuan_kerja_id_seq');
