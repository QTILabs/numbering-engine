pub(crate) mod auth;
pub(crate) mod jwt_laporan;
pub(crate) mod redis_helper;

use chrono::{DateTime, FixedOffset};
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

#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) struct LaporanAttachment {
    pub(crate) id: u32,
    pub(crate) nama_file: String,
    pub(crate) data: Vec<u8>,
}

impl IJsonSerializable for LaporanAttachment {}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) struct Laporan {
    pub(crate) id: Option<IdLaporan>,
    pub(crate) created_date: DateTime<FixedOffset>,
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
    pub(crate) tanggal_laporan: DateTime<FixedOffset>,
    pub(crate) attachment: Option<LaporanAttachment>,
}

impl IJsonSerializable for Laporan {}

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
}
