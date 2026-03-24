use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsRaw {
    pub status: String,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub result_type: String,
    pub result: Vec<Result>,
    pub stats: Stats,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub metric: Metric,
    pub value: (f64, String),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metric {
    pub path: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub summary: Summary,
    pub store: Store,
    pub ingester: Ingester,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Summary {
    pub bytes_processed_per_second: i64,
    pub lines_processed_per_second: i64,
    pub total_bytes_processed: i64,
    pub total_lines_processed: i64,
    pub exec_time: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Store {
    pub total_chunks_ref: i64,
    pub total_chunks_downloaded: i64,
    pub chunks_download_time: f64,
    pub head_chunk_bytes: i64,
    pub head_chunk_lines: i64,
    pub decompressed_bytes: i64,
    pub decompressed_lines: i64,
    pub compressed_bytes: i64,
    pub total_duplicates: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ingester {
    pub total_reached: i64,
    pub total_chunks_matched: i64,
    pub total_batches: i64,
    pub total_lines_sent: i64,
    pub head_chunk_bytes: i64,
    pub head_chunk_lines: i64,
    pub decompressed_bytes: i64,
    pub decompressed_lines: i64,
    pub compressed_bytes: i64,
    pub total_duplicates: i64,
}
