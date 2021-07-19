use google_drive3::DriveHub;
use hyper_rustls;
use std::path::Path;
use yup_oauth2;
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
}
