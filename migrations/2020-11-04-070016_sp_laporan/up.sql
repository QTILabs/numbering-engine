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
			AND status = 'terkirim_sudah_diapprove'
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
		tanggal_laporan timestampTz
	) RETURNS VARCHAR AS $$ BEGIN RETURN (
		SELECT right(
				'00000' || cast(count_laporan + 1 as varchar(5)),
				5
			) || '\' || prefix_jenis || '\' || prefix_satker || '\' || date_part('month',tanggal_laporan) || '\' || date_part('year',tanggal_laporan));
END
$$ LANGUAGE plpgsql;

-- Get Nomor Position
CREATE OR REPLACE FUNCTION sp_laporan_get_nomor_position(_id_jenis int, _id_satker int, _tanggal_laporan timestampTz)
RETURNS INTEGER AS $$
BEGIN
	RETURN (SELECT urutan FROM laporan WHERE satker_id = _id_satker AND jenis_id = _id_jenis AND EXTRACT(MONTH FROM tanggal_laporan) = EXTRACT(MONTH FROM _tanggal_laporan) AND EXTRACT(YEAR FROM tanggal_laporan) = EXTRACT(YEAR FROM _tanggal_laporan) AND tanggal_laporan >= _tanggal_laporan AND status = 'terkirim_sudah_diapprove' ORDER BY tanggal_laporan ASC LIMIT 1);
END
$$ LANGUAGE plpgsql;

-- Get Last Draft Nomor
CREATE OR REPLACE FUNCTION sp_laporan_get_last_draft_nomor(_id_jenis int, _id_satker int, _tanggal_laporan timestampTz)
RETURNS INTEGER AS $$
BEGIN
	RETURN (SELECT urutan FROM laporan WHERE satker_id = _id_satker AND jenis_id = _id_jenis AND EXTRACT(MONTH FROM tanggal_laporan) = EXTRACT(MONTH FROM _tanggal_laporan) AND EXTRACT(YEAR FROM tanggal_laporan) = EXTRACT(YEAR FROM _tanggal_laporan) AND tanggal_laporan <= _tanggal_laporan AND status = 'draft' ORDER BY tanggal_laporan DESC LIMIT 1);
END
$$ LANGUAGE plpgsql;

-- Sort Nomor
CREATE OR REPLACE FUNCTION sp_laporan_sort_nomor(num_count int, _id_jenis int, _id_satker int, _tanggal_laporan timestampTz)
RETURNS void AS $$
DECLARE	
	data_laporan record;
	_urutan int;
BEGIN
	FOR data_laporan IN SELECT * FROM laporan WHERE satker_id = _id_satker AND jenis_id = _id_jenis AND EXTRACT(MONTH FROM tanggal_laporan) = EXTRACT(MONTH FROM _tanggal_laporan) AND EXTRACT(YEAR FROM tanggal_laporan) = EXTRACT(YEAR FROM _tanggal_laporan) AND tanggal_laporan >= _tanggal_laporan AND status = 'terkirim_sudah_diapprove' ORDER BY tanggal_laporan ASC LOOP
		_urutan := num_count + 1;
		UPDATE laporan SET nomor = sp_laporan_get_nomor(num_count,sp_laporan_get_kode_satker(_id_satker),sp_laporan_get_kode_referensi(_id_jenis),data_laporan.tanggal_laporan), urutan = _urutan WHERE id = data_laporan.id;
		num_count := _urutan;
	END LOOP;
END
$$ LANGUAGE plpgsql;

--- Sort Old Nomor With Exception from old id (so newly updated laporan wont be updated)
CREATE OR REPLACE FUNCTION sp_laporan_sort_old_nomor(num_count int, _id_jenis int, _id_satker int, _tanggal_laporan timestampTz, old_id uuid)
RETURNS void AS $$
DECLARE	
	data_laporan record;
	_urutan int;
BEGIN
	FOR data_laporan IN SELECT * FROM laporan WHERE satker_id = _id_satker AND jenis_id = _id_jenis AND EXTRACT(MONTH FROM tanggal_laporan) = EXTRACT(MONTH FROM _tanggal_laporan) AND EXTRACT(YEAR FROM tanggal_laporan) = EXTRACT(YEAR FROM _tanggal_laporan) AND tanggal_laporan >= _tanggal_laporan AND status = 'terkirim_sudah_diapprove' AND id != old_id ORDER BY tanggal_laporan ASC LOOP
		_urutan := num_count + 1;
		UPDATE laporan SET nomor = sp_laporan_get_nomor(num_count,sp_laporan_get_kode_satker(_id_satker),sp_laporan_get_kode_referensi(_id_jenis),data_laporan.tanggal_laporan), urutan = _urutan WHERE id = data_laporan.id;
		num_count := _urutan;
	END LOOP;
END
$$ LANGUAGE plpgsql;

--- Re-sorting Nomor Laporan
CREATE OR REPLACE FUNCTION sp_laporan_re_sort_nomor(_id_jenis int, _id_satker int, _month int, _year int)
RETURNS void AS $$
DECLARE	
	data_laporan record;
	num_count int := 0;
BEGIN
	FOR data_laporan IN SELECT * FROM laporan WHERE satker_id = _id_satker AND jenis_id = _id_jenis AND EXTRACT(MONTH FROM tanggal_laporan) = _month AND EXTRACT(YEAR FROM tanggal_laporan) = _year AND status = 'terkirim_sudah_diapprove' ORDER BY tanggal_laporan ASC LOOP
		UPDATE laporan SET nomor = sp_laporan_get_nomor(num_count,sp_laporan_get_kode_satker(_id_satker),sp_laporan_get_kode_referensi(_id_jenis),data_laporan.tanggal_laporan), urutan = num_count + 1 WHERE id = data_laporan.id;
		num_count := num_count + 1;
	END LOOP;
END
$$ LANGUAGE plpgsql;

-- Get Format Nomor Draft Laporan
CREATE OR REPLACE FUNCTION sp_laporan_get_nomor_draft(
		count_laporan int,
		prefix_satker varchar,
		prefix_jenis varchar,
		tanggal_laporan timestampTz
	) RETURNS VARCHAR AS $$ BEGIN RETURN (
		SELECT 'DRAFT-' || right(
				'00000' || cast(count_laporan + 1 as varchar(5)),
				5
			) || '\' || prefix_jenis || '\' || prefix_satker || '\' || date_part('month',tanggal_laporan) || '\' || date_part('year',tanggal_laporan));
END
$$ LANGUAGE plpgsql;

-- Counting Draft Laporan for Numbering
CREATE OR REPLACE FUNCTION sp_laporan_count_month_draft(
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
			AND status = 'draft'
	);
END $$ LANGUAGE plpgsql;

