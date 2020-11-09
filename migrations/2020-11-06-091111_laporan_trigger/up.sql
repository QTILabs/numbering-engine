-- Trigger before inserting laporan to assign nomor laporan
CREATE FUNCTION sp_laporan_insert() RETURNS trigger AS $$
DECLARE _month int := EXTRACT(
        MONTH
        FROM NEW.tanggal_laporan
    );
_year int := EXTRACT(
    YEAR
    FROM NEW.tanggal_laporan
);
BEGIN NEW.nomor := sp_laporan_get_nomor(
    sp_laporan_count_month(NEW.id_jenis, NEW.id_satker, _month, _year),
    NEW.id_jenis,
    NEW.id_satker,
    _month,
    _year
);
RETURN NEW;
END $$ LANGUAGE plpgsql;

CREATE TRIGGER tr_laporan_insert BEFORE
INSERT ON public.laporan FOR EACH ROW
    WHEN (new.tanggal_laporan = CURRENT_TIMESTAMP) EXECUTE PROCEDURE public.sp_laporan_insert();

--- Trigger before inserting laporan to assign nomor laporan, with backdate tanggal_laporan
CREATE FUNCTION sp_laporan_insert_backdate() RETURNS trigger AS $$
DECLARE _month int := EXTRACT(
        MONTH
        FROM NEW.tanggal_laporan
    );
_year int := EXTRACT(
    YEAR
    FROM NEW.tanggal_laporan
);
BEGIN NEW.nomor := sp_laporan_get_nomor(
    sp_laporan_get_nomor_position(NEW.id_jenis, NEW.id_satker, NEW.tanggal_laporan),
    NEW.id_jenis,
    NEW.id_satker,
    _month,
    _year
);
SELECT sp_laporan_sort_nomor(NEW.id_jenis, NEW.id_satker, NEW.tanggal_laporan);
RETURN NEW;
END $$ LANGUAGE plpgsql;

CREATE TRIGGER tr_laporan_insert_backdate BEFORE
INSERT ON public.laporan FOR EACH ROW
    WHEN (new.tanggal_laporan < CURRENT_TIMESTAMP) EXECUTE PROCEDURE public.sp_laporan_insert_backdate();
