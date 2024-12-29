use anyhow::Error;

use crate::client::ClientArc;
use crate::handler::Handler;
use crate::proto::mumble::ChannelState;
use crate::proto::MessageKind;
use crate::state::ServerStateRef;

use super::MumbleResult;

impl Handler for ChannelState {
    async fn handle(&self, state: &ServerStateRef, client: &ClientArc) -> MumbleResult {
        if self.has_channel_id() {
            tracing::warn!("editing channel is not supported");

            return Ok(());
        }

        if !self.has_parent() {
            tracing::warn!("cannot create channel: channel must have a parent");

            return Ok(());
        }

        if !self.has_name() {
            tracing::warn!("cannot create channel: channel must have a name");

            return Ok(());
        }

        if !self.get_temporary() {
            tracing::warn!("cannot create channel: channel must be temporary");

            return Ok(());
        }

        let name = self.get_name();

        if name.len() > 512 {
            return Ok(());
        }

        if !state.channels.contains(&self.get_parent()) {
            tracing::warn!("cannot create channel: parent channel does not exist");

            return Ok(());
        }

        let existing_channel = state.get_channel_by_name(name).await;
        if existing_channel.is_some() {
            return Ok(());
        }

        let channel = state.add_channel(self).await;
        let channel_state = { channel.get_channel_state() };

        tracing::debug!("Created channel {}, requested by {}", channel.id, client.session_id);

        let err = state.broadcast_message(MessageKind::ChannelState, channel_state.as_ref());

        state.set_client_channel(client, channel.id).await?;

        err.map_err(Error::new)
    }
}
