-- Trigger before inserting laporan to assign nomor laporan
CREATE OR REPLACE FUNCTION sp_laporan_insert() RETURNS trigger AS $$
DECLARE _month int := EXTRACT(
        MONTH
        FROM NEW.tanggal_laporan
    );
_year int := EXTRACT(
    YEAR
    FROM NEW.tanggal_laporan
);
BEGIN NEW.nomor := sp_laporan_get_nomor(
    sp_laporan_count_month(NEW.jenis_id, NEW.satker_id, _month, _year),
    NEW.jenis_id,
    NEW.satker_id,
    _month,
    _year
);
RETURN NEW;
END $$ LANGUAGE plpgsql;

CREATE TRIGGER tr_laporan_insert BEFORE
INSERT ON public.laporan FOR EACH ROW
    WHEN (new.tanggal_laporan = CURRENT_TIMESTAMP) EXECUTE PROCEDURE public.sp_laporan_insert();

--- Trigger before inserting laporan to assign nomor laporan, with backdate tanggal_laporan
CREATE OR REPLACE FUNCTION sp_laporan_insert_backdate() RETURNS trigger AS $$
DECLARE _month int := EXTRACT(
        MONTH
        FROM NEW.tanggal_laporan
    );
_year int := EXTRACT(
    YEAR
    FROM NEW.tanggal_laporan
);
BEGIN NEW.nomor := sp_laporan_get_nomor(
    sp_laporan_get_nomor_position(NEW.jenis_id, NEW.satker_id, NEW.tanggal_laporan),
    NEW.jenis_id,
    NEW.satker_Id,
    _month,
    _year
);
SELECT sp_laporan_sort_nomor(sp_laporan_get_nomor_position(NEW.jenis_id,NEW.satker_id,NEW.tanggal_laporan) + 1, NEW.jenis_id, NEW.satker_id, NEW.tanggal_laporan);
RETURN NEW;
END $$ LANGUAGE plpgsql;

CREATE TRIGGER tr_laporan_insert_backdate BEFORE
INSERT ON public.laporan FOR EACH ROW
    WHEN (new.tanggal_laporan < CURRENT_TIMESTAMP) EXECUTE PROCEDURE public.sp_laporan_insert_backdate();

-- Edit Laporan Trigger

CREATE OR REPLACE FUNCTION sp_laporan_edit() RETURNS trigger AS $$
DECLARE _month int := EXTRACT(
        MONTH
        FROM NEW.tanggal_laporan
    );
_year int := EXTRACT(
    YEAR
    FROM NEW.tanggal_laporan
);
BEGIN NEW.nomor := sp_laporan_get_nomor(
    COALESCE(sp_laporan_get_nomor_position(NEW.jenis_id, NEW.satker_id, NEW.tanggal_laporan),sp_laporan_count_month(NEW.jenis_id, NEW.satker_id,_month,_year)),
    NEW.jenis_id,
    NEW.satker_id,
    _month,
    _year
);

SELECT sp_laporan_sort_nomor(sp_laporan_get_nomor_position(NEW.jenis_id,NEW.satker_id,NEW.tanggal_laporan) + 1, NEW.jenis_id, NEW.satker_id, NEW.tanggal_laporan);
SELECT sp_laporan_sort_nomor(sp_laporan_get_nomor_position(OLD.jenis_id,OLD.satker_id,OLD.tanggal_laporan), OLD.jenis_id, OLD.satker_id, OLD.tanggal_laporan);
RETURN NEW;
END $$ LANGUAGE plpgsql;

CREATE TRIGGER tr_laporan_edit BEFORE
UPDATE ON public.laporan FOR EACH ROW
    WHEN (OLD.tanggal_laporan IS DISTINCT FROM NEW.tanggal_laporan OR OLD.jenis_id IS DISTINCT FROM NEW.jenis_id OR OLD.satker_id IS DISTINCT FROM NEW.satker_id) EXECUTE PROCEDURE public.sp_laporan_edit();
	
