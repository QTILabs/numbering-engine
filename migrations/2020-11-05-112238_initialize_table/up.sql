-- Your SQL goes here
CREATE TYPE Laporan_status AS ENUM (
    'Draft',
    'Terhapus',
    'Terkirim & Belum Di Approve',
    'Terkirim & Sudah Di Approve'
);

CREATE TABLE public.attachment (
    id serial PRIMARY KEY,
    id_laporan uuid NOT NULL,
    nama_file character varying(128) NOT NULL,
    data bytea NOT NULL
);

CREATE TABLE public.laporan (
    id uuid PRIMARY KEY,
    created_date timestamp with time zone NOT NULL,
    updated_date timestamp with time zone,
    judul character varying(150) NOT NULL,
    nomor character varying(128),
    isi text NOT NULL,
    status Laporan_status NOT NULL,
    satker_id integer NOT NULL,
    klasifikasi_id integer NOT NULL,
    pelapor_id integer NOT NULL,
    urgensi_id integer NOT NULL,
    uploader_id integer NOT NULL,
    updated_by_id integer,
    jenis_id integer NOT NULL,
    tanggal_laporan timestamp with time zone NOT NULL
);

CREATE TABLE public.pelapor (
    id serial PRIMARY KEY,
    nama character varying(128) NOT NULL,
    satker_id integer NOT NULL
);

CREATE TABLE public.referensi (
    id serial PRIMARY KEY,
    nama character varying(64) NOT NULL,
    kode character varying(32) NOT NULL,
    deskripsi text
);

CREATE TABLE public.satuan_kerja (
    id serial PRIMARY KEY,
    nama character varying(64) NOT NULL,
    kode character varying(32) NOT NULL,
    deskripsi text
);

CREATE TABLE public.user (
    id serial PRIMARY KEY,
    email character varying(128) NOT NULL,
    nama character varying(64) NOT NULL,
    password character varying(128) NOT NULL
);
