-- Counting Laporan for Numbering
CREATE OR REPLACE FUNCTION sp_laporan_count_month(
		_id_jenis int,
		_id_satker int,
		_month integer,
		_year integer
	) RETURNS INTEGER AS $$ BEGIN RETURN (
		SELECT COUNT(*) AS count
		FROM laporan
		WHERE satker_id = _id_satker
			AND jenis_id = _id_jenis
			AND EXTRACT(
				MONTH
				FROM tanggal_laporan
			) = _month
			AND EXTRACT(
				YEAR
				FROM tanggal_laporan
			) = _year
	);
END $$ LANGUAGE plpgsql;
-- Get Satker Prefix
CREATE OR REPLACE FUNCTION sp_laporan_get_kode_satker(_id_satker int) RETURNS VARCHAR AS $$ BEGIN RETURN (
		SELECT kode
		FROM satuan_kerja
		WHERE id = _id_satker
	);
END $$ LANGUAGE plpgsql;
-- Get Kode Referensi (Jenis Laporan / Klasifikasi / Urgensi)
CREATE OR REPLACE FUNCTION sp_laporan_get_kode_referensi(_id int) RETURNS VARCHAR AS $$ BEGIN RETURN (
		SELECT kode
		FROM referensi
		WHERE id = _id
	);
END $$ LANGUAGE plpgsql;
-- Get Format Nomor Laporan
CREATE OR REPLACE FUNCTION sp_laporan_get_nomor(
		count_laporan int,
		prefix_satker varchar,
		prefix_jenis varchar,
		tanggal_laporan timestamp
	) RETURNS VARCHAR AS $$ BEGIN RETURN (
		SELECT right(
				'00000' || cast(count_laporan + 1 as varchar(5)),
				5
			) || '\' || prefix_jenis || ' \ ' || prefix_satker || ' \ ' || date_part(' month ',tanggal_laporan) || ' \ ' || date_part(' year ',tanggal_laporan));
END
$$ LANGUAGE plpgsql;

-- Get Nomor Position
CREATE OR REPLACE FUNCTION sp_laporan_get_nomor_position(_id_jenis int, _id_satker int, _tanggal_laporan timestamp)
RETURNS INTEGER AS $$
BEGIN
	RETURN (SELECT CAST(substring(nomor,0,5) AS INTEGER) FROM laporan WHERE satker_id = _id_satker AND jenis_id = _id_jenis AND EXTRACT(MONTH FROM tanggal_laporan) = EXTRACT(MONTH FROM _tanggal_laporan) AND EXTRACT(YEAR FROM tanggal_laporan) = EXTRACT(YEAR FROM _tanggal_laporan) AND tanggal_laporan >= _tanggal_laporan ORDER BY tanggal_laporan ASC);
END
$$ LANGUAGE plpgsql;

-- Sort Nomor
CREATE OR REPLACE FUNCTION sp_laporan_sort_nomor(num_count int, _id_jenis int, _id_satker int, _tanggal_laporan timestamp)
RETURNS RECORD AS $$
DECLARE	
	data_laporan record;
BEGIN
	FOR data_laporan IN SELECT * FROM laporan WHERE satker_id = _id_satker AND jenis_id = _id_jenis AND EXTRACT(MONTH FROM tanggal_laporan) = EXTRACT(MONTH FROM _tanggal_laporan) AND EXTRACT(YEAR FROM tanggal_laporan) = EXTRACT(YEAR FROM _tanggal_laporan) AND tanggal_laporan > _tanggal_laporan ORDER BY tanggal_laporan ASC LOOP
		UPDATE laporan SET nomor = sp_laporan_get_nomor(num_count,sp_laporan_get_kode_satker(_id_satker),sp_laporan_get_kode_referensi(_id_jenis),data_laporan.tanggal_laporan) WHERE id = data_laporan.id;
		num_count := num_count + 1;
	END LOOP;
END
$$ LANGUAGE plpgsql;
