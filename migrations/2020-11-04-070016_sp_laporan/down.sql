-- This file should undo anything in `up.sql`
DROP FUNCTION IF EXISTS sp_laporan_count_month(
    _id_jenis int,
    _id_satker int,
    _month integer,
	_year integer
);
DROP FUNCTION IF EXISTS sp_laporan_get_kode_satker(_id_satker int);
DROP FUNCTION IF EXISTS sp_laporan_get_kode_referensi(_id int);
DROP FUNCTION IF EXISTS sp_laporan_get_nomor(
    count_laporan int,
    prefix_satker varchar,
    prefix_jenis varchar,
    tanggal_laporan timestampTz
);
DROP FUNCTION IF EXISTS sp_laporan_get_nomor_position(
    _id_jenis int,
    _id_satker int,
    _tanggal_laporan timestampTz
);
DROP FUNCTION IF EXISTS sp_laporan_sort_nomor(
    num_count int,
    _id_jenis int,
    _id_satker int,
    _tanggal_laporan timestampTz
);

DROP FUNCTION IF EXISTS sp_laporan_re_sort_nomor(_id_jenis int, _id_satker int, _tanggal_laporan timestampTz);

DROP FUNCTION IF EXISTS sp_laporan_count_month_draft(
    _id_jenis int,
    _id_satker int,
    _month integer,
	_year integer
);

DROP FUNCTION IF EXISTS sp_laporan_get_nomor_draft(
    count_laporan int,
    prefix_satker varchar,
    prefix_jenis varchar,
    tanggal_laporan timestampTz
);

