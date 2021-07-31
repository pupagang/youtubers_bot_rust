use google_drive3::api::Scope::Full;
use google_drive3::Result;
use google_drive3::{api::File, DriveHub};
use std::fs;
use std::path::Path;
use yup_oauth2::InstalledFlowReturnMethod;

pub struct Drive {
    pub hub: DriveHub,
}

impl Drive {
    pub async fn init(persist_path: &str) -> Self {
        if !Path::new("token.json").exists() {
            let secret = yup_oauth2::read_application_secret(persist_path)
                .await
                .expect("failed to read \"credentials.json\" file");
            let authenticator = yup_oauth2::InstalledFlowAuthenticator::builder(
                secret,
                InstalledFlowReturnMethod::Interactive,
            )
            .persist_tokens_to_disk("test.json")
            .build()
            .await
            .expect("failed to create authenticator");

            let hub = DriveHub::new(
                hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()),
                authenticator,
            );
            Self { hub }
        } else {
            let secret = yup_oauth2::read_application_secret(persist_path)
                .await
                .expect("failed to read \"credentials.json\" file");
            let authenticator = yup_oauth2::DeviceFlowAuthenticator::builder(secret)
                .persist_tokens_to_disk("test.json")
                .build()
                .await
                .expect("failed to create authenticator");

            let hub = DriveHub::new(
                hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()),
                authenticator,
            );
            Self { hub }
        }
    }

    pub async fn get_shared_drives(&self) -> google_drive3::api::DriveList {
        self.hub.drives().list().doit().await.unwrap().1
    }

    pub async fn upload_file(
        &self,
        file_path: &str,
        file_name: &str,
        drive_root: &str,
    ) -> Result<google_drive3::api::File> {
        let mut file = File::default();
        file.name = Some(file_name.to_string());
        file.parents = Some(vec![drive_root.to_string()]);
        let upload = self
            .hub
            .files()
            .create(file)
            .add_scope(Full)
            .supports_all_drives(true)
            .upload_resumable(
                fs::File::open(file_path.to_string()).unwrap(),
                "application/octet-stream".parse().unwrap(),
            )
            .await;
        Ok(upload.unwrap().1)
    }
}
