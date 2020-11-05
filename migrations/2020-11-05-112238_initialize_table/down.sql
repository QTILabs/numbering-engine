-- This file should undo anything in `up.sql`
DROP TYPE IF EXISTS Laporan_status;

DROP TABLE IF EXISTS public.attachment;
DROP TABLE IF EXISTS public.laporan;
DROP TABLE IF EXISTS public.pelapor;
DROP TABLE IF EXISTS public.referensi;
DROP TABLE IF EXISTS public.satuan_kerja;
DROP TABLE IF EXISTS public.user;
