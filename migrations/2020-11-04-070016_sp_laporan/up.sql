-- Counting Laporan for Numbering
CREATE OR REPLACE FUNCTION sp_laporan_count_nomor(_id_jenis int, _id_satker int, _tanggal_laporan timestamp)
RETURNS INTEGER AS $$
BEGIN
	RETURN (SELECT COUNT(*) AS count FROM laporan WHERE satker_id = _id_satker AND jenis_id = _id_jenis AND tanggal_laporan >= date_trunc('month', _tanggal_laporan) AND tanggal_laporan < + interval '1' month);
END
$$ LANGUAGE plpgsql;

-- Get Satker Prefix
CREATE OR REPLACE FUNCTION sp_laporan_get_kode_satker(_id_satker int)
RETURNS VARCHAR AS $$
BEGIN
	RETURN (SELECT kode FROM satuan_kerja WHERE id = _id_satker);
END
$$ LANGUAGE plpgsql;

-- Get Kode Referensi (Jenis Laporan / Klasifikasi / Urgensi)
CREATE OR REPLACE FUNCTION sp_laporan_get_kode_referensi(_id int)
RETURNS VARCHAR AS $$
BEGIN
	RETURN (SELECT kode FROM referensi WHERE id = _id);
END
$$ LANGUAGE plpgsql;

-- Get Format Nomor Laporan
CREATE OR REPLACE FUNCTION sp_laporan_get_nomor(count_laporan int, prefix_satker varchar, prefix_jenis varchar, tanggal_laporan timestamp)
RETURNS VARCHAR AS $$
BEGIN
	RETURN (SELECT right('00000' || cast(count_laporan + 1 as varchar(5)), 5) || '\' || prefix_jenis || '\' || prefix_satker || '\' || date_part('month',tanggal_laporan) || '\' || date_part('year',tanggal_laporan));
END
$$ LANGUAGE plpgsql;


