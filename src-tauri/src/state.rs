use waapi_rs::{SubscriptionHandle, WaapiClient};

pub struct AppState {
    pub client: Option<WaapiClient>,
    pub sub_handle: Option<SubscriptionHandle>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            client: None,
            sub_handle: None,
        }
    }
}

impl Drop for AppState {
    fn drop(&mut self) {
        let _ = self.sub_handle.take();
        let _ = self.client.take();
    }
}
