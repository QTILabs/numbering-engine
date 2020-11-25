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
_urutan int := sp_laporan_count_month(NEW.jenis_id, NEW.satker_id, _month, _year);
BEGIN NEW.nomor := sp_laporan_get_nomor(
    _urutan,
    sp_laporan_get_kode_satker(NEW.satker_id),
    sp_laporan_get_kode_referensi(NEW.jenis_id),
    NEW.tanggal_laporan
);
NEW.urutan := _urutan + 1;
RETURN NEW;
END $$ LANGUAGE plpgsql;

CREATE TRIGGER tr_laporan_insert BEFORE
INSERT ON public.laporan FOR EACH ROW
    WHEN (NEW.tanggal_laporan = CURRENT_TIMESTAMP AND NEW.status = 'terkirim_sudah_diapprove') EXECUTE PROCEDURE public.sp_laporan_insert();

-- Trigger before inserting draft laporan to assign nomor laporan
CREATE OR REPLACE FUNCTION sp_laporan_insert_draft() RETURNS trigger AS $$
DECLARE _month int := EXTRACT(
        MONTH
        FROM NEW.tanggal_laporan
    );
_year int := EXTRACT(
    YEAR
    FROM NEW.tanggal_laporan
);
_urutan int := COALESCE(sp_laporan_get_last_draft_nomor(NEW.jenis_id, NEW.satker_id, NEW.tanggal_laporan),sp_laporan_count_month_draft(NEW.jenis_id, NEW.satker_id,_month,_year));
BEGIN NEW.nomor := sp_laporan_get_nomor_draft(
    _urutan,
    sp_laporan_get_kode_satker(NEW.satker_id),
    sp_laporan_get_kode_referensi(NEW.jenis_id),
    NEW.tanggal_laporan
);
NEW.urutan := _urutan + 1;
RETURN NEW;
END $$ LANGUAGE plpgsql;

CREATE TRIGGER tr_laporan_insert_draft BEFORE
INSERT ON public.laporan FOR EACH ROW
    WHEN (NEW.status = 'draft') EXECUTE PROCEDURE public.sp_laporan_insert_draft();

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
_urutan int := sp_laporan_get_nomor_position(NEW.jenis_id,NEW.satker_id,NEW.tanggal_laporan);
BEGIN NEW.nomor := sp_laporan_get_nomor(
    _urutan - 1,
    sp_laporan_get_kode_satker(NEW.satker_id),
    sp_laporan_get_kode_referensi(NEW.jenis_id),
    NEW.tanggal_laporan
);
NEW.urutan := _urutan;
PERFORM sp_laporan_sort_nomor( _urutan, NEW.jenis_id, NEW.satker_id, NEW.tanggal_laporan);
RETURN NEW;
END $$ LANGUAGE plpgsql;

CREATE TRIGGER tr_laporan_insert_backdate BEFORE
INSERT ON public.laporan FOR EACH ROW
    WHEN (new.tanggal_laporan < CURRENT_TIMESTAMP AND NEW.status = 'terkirim_sudah_diapprove') EXECUTE PROCEDURE public.sp_laporan_insert_backdate();

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
_urutan int := COALESCE(sp_laporan_get_nomor_position(NEW.jenis_id, NEW.satker_id, NEW.tanggal_laporan),sp_laporan_count_month_edit(NEW.jenis_id, NEW.satker_id,_month,_year,OLD.id) + 1);
BEGIN 

NEW.nomor := sp_laporan_get_nomor(
    _urutan - 1,
    sp_laporan_get_kode_satker(NEW.satker_id),
    sp_laporan_get_kode_referensi(NEW.jenis_id),
    NEW.tanggal_laporan
);
NEW.urutan := _urutan;

PERFORM sp_laporan_sort_old_nomor(_urutan, NEW.jenis_id, NEW.satker_id, NEW.tanggal_laporan,OLD.id);
PERFORM sp_laporan_sort_old_nomor(OLD.urutan - 1, OLD.jenis_id, OLD.satker_id, OLD.tanggal_laporan,OLD.id);

RETURN NEW;
END $$ LANGUAGE plpgsql;

CREATE TRIGGER tr_laporan_edit BEFORE
UPDATE ON public.laporan FOR EACH ROW
    WHEN (OLD.tanggal_laporan IS DISTINCT FROM NEW.tanggal_laporan OR OLD.jenis_id IS DISTINCT FROM NEW.jenis_id OR OLD.satker_id IS DISTINCT FROM NEW.satker_id AND NEW.status = 'terkirim_sudah_diapprove') EXECUTE PROCEDURE public.sp_laporan_edit();

--- Add or update updated_date when updating table

CREATE OR REPLACE FUNCTION sp_laporan_edit_updated_date() RETURNS trigger AS $$
BEGIN
    NEW.updated_date := CURRENT_TIMESTAMP;
    RETURN NEW;
END $$ LANGUAGE plpgsql;

CREATE TRIGGER tr_laporan_edit_updated_date BEFORE
UPDATE ON public.laporan FOR EACH ROW
   EXECUTE PROCEDURE public.sp_laporan_edit_updated_date();

--- Trigger Edit Laporan Status Draft to Laporan Terkirim Terapprove
CREATE OR REPLACE FUNCTION sp_laporan_edit_draft_status() RETURNS trigger AS $$
DECLARE _month int := EXTRACT(
        MONTH
        FROM NEW.tanggal_laporan
    );
_year int := EXTRACT(
    YEAR
    FROM NEW.tanggal_laporan
);
_urutan int := COALESCE(sp_laporan_get_nomor_position(NEW.jenis_id, NEW.satker_id, NEW.tanggal_laporan),sp_laporan_count_month(NEW.jenis_id, NEW.satker_id,_month,_year) + 1);
BEGIN NEW.nomor := sp_laporan_get_nomor(
    _urutan - 1,
    sp_laporan_get_kode_satker(NEW.satker_id),
    sp_laporan_get_kode_referensi(NEW.jenis_id),
    NEW.tanggal_laporan
);
NEW.urutan := _urutan;
PERFORM sp_laporan_sort_nomor( _urutan, NEW.jenis_id, NEW.satker_id, NEW.tanggal_laporan);
RETURN NEW;
END $$ LANGUAGE plpgsql;

CREATE TRIGGER tr_laporan_edit_draft_status BEFORE
UPDATE ON public.laporan FOR EACH ROW
    WHEN (OLD.status = 'draft' AND NEW.status = 'terkirim_sudah_diapprove') EXECUTE PROCEDURE public.sp_laporan_edit_draft_status();


--- Trigger Edit Laporan Status Back to Draft
CREATE OR REPLACE FUNCTION sp_laporan_edit_back_to_draft() RETURNS trigger AS $$
DECLARE _month int := EXTRACT(
        MONTH
        FROM NEW.tanggal_laporan
    );
_year int := EXTRACT(
    YEAR
    FROM NEW.tanggal_laporan
);
_urutan int := COALESCE(sp_laporan_get_last_draft_nomor(NEW.jenis_id, NEW.satker_id, NEW.tanggal_laporan),sp_laporan_count_month_draft(NEW.jenis_id, NEW.satker_id,_month,_year));
BEGIN NEW.nomor := sp_laporan_get_nomor_draft(
    _urutan,
    sp_laporan_get_kode_satker(NEW.satker_id),
    sp_laporan_get_kode_referensi(NEW.jenis_id),
    NEW.tanggal_laporan
);
NEW.urutan := _urutan + 1;
PERFORM sp_laporan_sort_old_nomor(OLD.urutan - 1, OLD.jenis_id, OLD.satker_id, OLD.tanggal_laporan,OLD.id);
RETURN NEW;
END $$ LANGUAGE plpgsql;

CREATE TRIGGER tr_laporan_edit_back_to_draft BEFORE
UPDATE ON public.laporan FOR EACH ROW
    WHEN (NEW.status = 'draft') EXECUTE PROCEDURE public.sp_laporan_edit_back_to_draft();

--- Trigger Edit Laporan Status to terhapus (soft delete laporan)
CREATE OR REPLACE FUNCTION sp_laporan_delete() RETURNS trigger AS $$
BEGIN 
NEW.nomor := ''; -- NOMOR SET EMPTY
NEW.urutan := 0; -- SET SEQUENCE TO 0
PERFORM sp_laporan_sort_old_nomor(OLD.urutan - 1, OLD.jenis_id, OLD.satker_id, OLD.tanggal_laporan,OLD.id);
RETURN NEW;
END $$ LANGUAGE plpgsql;

CREATE TRIGGER tr_laporan_delete BEFORE
UPDATE ON public.laporan FOR EACH ROW
    WHEN (NEW.status = 'terhapus' AND OLD.status = 'terkirim_sudah_diapprove') EXECUTE PROCEDURE public.sp_laporan_delete();


--- Trigger Delete Laporan (HARD DELETE laporan)
CREATE OR REPLACE FUNCTION sp_laporan_hard_delete() RETURNS trigger AS $$
BEGIN 
PERFORM sp_laporan_sort_old_nomor(OLD.urutan - 1, OLD.jenis_id, OLD.satker_id, OLD.tanggal_laporan,OLD.id);
RETURN OLD;
END $$ LANGUAGE plpgsql;

CREATE TRIGGER tr_laporan_hard_delete BEFORE
DELETE ON public.laporan FOR EACH ROW
   WHEN (OLD.status = 'terkirim_sudah_diapprove') EXECUTE PROCEDURE public.sp_laporan_hard_delete();

