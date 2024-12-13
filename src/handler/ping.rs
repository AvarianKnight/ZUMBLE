use crate::client::ClientRef;
use crate::error::MumbleError;
use crate::handler::Handler;
use crate::proto::mumble::Ping;
use crate::proto::MessageKind;
use crate::state::ServerStateRef;
use std::time::Instant;

impl Handler for Ping {
    async fn handle(&self, _state: &ServerStateRef, client: &ClientRef) -> Result<(), MumbleError> {
        let mut ping = Ping::default();
        ping.set_timestamp(self.get_timestamp());

        {
            client.last_ping.swap(Instant::now());
        }

        {
            let crypt_state_read = client.crypt_state.lock().await;
            ping.set_good(crypt_state_read.good);
            ping.set_late(crypt_state_read.late);
            ping.set_lost(crypt_state_read.lost);
            ping.set_resync(crypt_state_read.resync);
        }

        client.send_message(MessageKind::Ping, &ping).await
    }
}
