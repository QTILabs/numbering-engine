table! {
    attachment (id) {
        id -> Int4,
        id_laporan -> Uuid,
        nama_file -> Varchar,
        data -> Bytea,
    }
}

table! {
    laporan (id) {
        id -> Uuid,
        created_date -> Timestamptz,
        updated_date -> Nullable<Timestamptz>,
        judul -> Varchar,
        nomor -> Nullable<Varchar>,
        isi -> Text,
        status -> Laporan_status,
        satker_id -> Int4,
        klasifikasi_id -> Int4,
        pelapor_id -> Int4,
        urgensi_id -> Int4,
        uploader_id -> Int4,
        updated_by_id -> Nullable<Int4>,
        jenis_id -> Int4,
    }
}

table! {
    pelapor (id) {
        id -> Int4,
        nama -> Varchar,
        satker_id -> Int4,
    }
}

table! {
    referensi (id) {
        id -> Int4,
        nama -> Varchar,
        kode -> Varchar,
        deskripsi -> Nullable<Text>,
    }
}

table! {
    satuan_kerja (id) {
        id -> Int4,
        nama -> Varchar,
        kode -> Varchar,
        deskripsi -> Nullable<Text>,
    }
}

table! {
    user (id) {
        id -> Int4,
        email -> Varchar,
        nama -> Varchar,
        password -> Varchar,
    }
}

joinable!(laporan -> pelapor (pelapor_id));
joinable!(laporan -> satuan_kerja (satker_id));
joinable!(pelapor -> satuan_kerja (satker_id));

allow_tables_to_appear_in_same_query!(
    attachment,
    laporan,
    pelapor,
    referensi,
    satuan_kerja,
    user,
);
