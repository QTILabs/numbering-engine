-- This file should undo anything in `up.sql`
DROP FUNCTION IF EXISTS sp_laporan_count_nomor(
    _id_jenis int,
    _id_satker int,
    _tanggal_laporan timestamp
);
DROP FUNCTION IF EXISTS sp_laporan_get_kode_satker(_id_satker int);
DROP FUNCTION IF EXISTS sp_laporan_get_kode_referensi(_id int);
DROP FUNCTION IF EXISTS sp_laporan_get_nomor(
    count_laporan int,
    prefix_satker varchar,
    prefix_jenis varchar,
    tanggal_laporan timestamp
);
DROP FUNCTION IF EXISTS sp_laporan_get_nomor_position(
    _id_jenis int,
    _id_satker int,
    _tanggal_laporan timestamp
);
DROP FUNCTION IF EXISTS sp_laporan_sort_nomor(
    num_count int,
    _id_jenis int,
    _id_satker int,
    _tanggal_laporan timestamp
);
