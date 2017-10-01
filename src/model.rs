//! Models in struct form, parsed out from JSON in response bodies.

/// Representation of the body response to a file upload request.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileUploadResponse {
    /// The uploaded files.
    pub files: Vec<UploadedFile>,
    /// Whether uploading the file(s) was successful.
    pub success: bool,
}

/// Definition of the structure representing information of an uploaded file.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UploadedFile {
    /// Identifying hash of the uploaded file.
    pub hash: String,
    /// Name of the file when uploaded, if given.
    pub name: Option<String>,
    /// Size of the file in bytes.
    pub size: u64,
    /// URL fragment to the file.
    pub url: String,
}
