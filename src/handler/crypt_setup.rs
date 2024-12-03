use crate::client::{ClientRef};
use crate::error::MumbleError;
use crate::handler::Handler;
use crate::proto::mumble::CryptSetup;
use crate::state::ServerStateRef;
use async_trait::async_trait;

#[async_trait]
impl Handler for CryptSetup {
    async fn handle(&self, _state: ServerStateRef, client: ClientRef) -> Result<(), MumbleError> {
        if self.has_client_nonce() {
            client.crypt_state.write().await.set_decrypt_nonce(self.get_client_nonce());
        } else {
            client.send_crypt_setup(false).await?;
        }

        Ok(())
    }
}
