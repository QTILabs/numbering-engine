pub(crate) mod auth;
pub(crate) mod jwt_auth;
pub(crate) mod jwt_laporan;
pub(crate) mod redis_helper;

use chrono::{DateTime, FixedOffset, TimeZone};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub(crate) type NomorLaporan = String;
pub(crate) type IdLaporan = Uuid;

pub trait IJsonSerializable<T = Self>
where
    Self: DeserializeOwned + Serialize + Clone + Send + Sized,
{
    fn from_json(json_string: &str) -> Option<Self> {
        serde_json::from_str::<Self>(json_string).ok()
    }

    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn to_pretty_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) enum AuthResult {
    Ok,
    NotAuthenticated,
    TokenInvalid,
    TokenExpired(DateTime<FixedOffset>),
    ForbiddenAccess(String, String),
}

impl IJsonSerializable for AuthResult {}

#[derive(Deserialize, Serialize, Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum LaporanStatus {
    Draft,
    Terhapus,
    TerkirimBelumDiApprove,
    TerkirimSudahDiApprove,
}

impl IJsonSerializable for LaporanStatus {}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub(crate) struct LaporanAttachment {
    pub(crate) id: Option<u32>,
    pub(crate) nama_file: String,
    pub(crate) data: Vec<u8>,
}

impl IJsonSerializable for LaporanAttachment {}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub(crate) struct Laporan {
    pub(crate) id: Option<IdLaporan>,
    pub(crate) created_date: Option<DateTime<FixedOffset>>,
    pub(crate) updated_date: Option<DateTime<FixedOffset>>,
    pub(crate) judul: String,
    pub(crate) nomor: Option<NomorLaporan>,
    pub(crate) urutan: Option<u32>,
    pub(crate) isi: String,
    pub(crate) status: LaporanStatus,
    pub(crate) satker_id: u32,
    pub(crate) klasifikasi_id: u32,
    pub(crate) pelapor_id: u32,
    pub(crate) urgensi_id: u32,
    pub(crate) uploader_id: u32,
    pub(crate) jenis_id: u32,
    pub(crate) tanggal_laporan: Option<DateTime<FixedOffset>>,
    pub(crate) attachment: Option<LaporanAttachment>,
}

impl IJsonSerializable for Laporan {}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) struct Pelapor {
    pub(crate) id: Option<u32>,
    pub(crate) nama: String,
    pub(crate) kode: String,
    pub(crate) deskripsi: Option<String>,
}

impl IJsonSerializable for Pelapor {}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) struct SatuanKerja {
    pub(crate) id: Option<u32>,
    pub(crate) nama: String,
    pub(crate) kode: String,
    pub(crate) deskripsi: Option<String>,
}

impl IJsonSerializable for SatuanKerja {}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) struct User {
    pub(crate) id: Option<u32>,
    pub(crate) email: String,
    pub(crate) nama: String,
    pub(crate) password: Option<String>,
}

impl IJsonSerializable for User {}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) struct Referensi {
    pub(crate) id: Option<u32>,
    pub(crate) nama: String,
    pub(crate) kode: String,
    pub(crate) deskripsi: Option<String>,
}

impl IJsonSerializable for Referensi {}

#[cfg(test)]
mod test_data_type {
    use super::*;

    #[test]
    fn test_serde_laporan_status() {
        let laporan_status = LaporanStatus::TerkirimBelumDiApprove;
        let laporan_status_in_json = laporan_status.to_json();
        let expected_laporan_status_in_json = "\"terkirim_belum_di_approve\"";
        assert_eq!(laporan_status_in_json, expected_laporan_status_in_json);
    }

    #[test]
    fn test_serde_laporan() {
        let tanggal_buat_laporan = FixedOffset::east(7 * 3600).ymd(2020, 11, 13).and_hms(11, 00, 00);
        let laporan = Laporan {
            id: None,
            created_date: None,
            updated_date: None,
            judul: "Judul Laporan".into(),
            nomor: None,
            urutan: None,
            isi: "Isi Laporan".into(),
            status: LaporanStatus::TerkirimSudahDiApprove,
            satker_id: 1,
            klasifikasi_id: 6,
            pelapor_id: 1,
            uploader_id: 1,
            urgensi_id: 10,
            jenis_id: 1,
            tanggal_laporan: Some(tanggal_buat_laporan),
            attachment: None,
        };
        let laporan_in_json = laporan.to_json();
        let expected_laporan_from_json = Laporan::from_json(&laporan_in_json);
        assert_eq!(laporan, expected_laporan_from_json.unwrap());
    }
}
