use bevy::prelude::*;
use lightyear::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SeverBeginReplicateLevel {
    pub num_level_entities: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClientLevelLoadComplete;

#[derive(Channel)]
pub struct UnorderedReliable;

pub fn register_messages(app: &mut App) {
    app.register_message::<SeverBeginReplicateLevel>(ChannelDirection::ServerToClient);

    app.register_message::<ClientLevelLoadComplete>(ChannelDirection::ClientToServer);

    app.add_channel::<UnorderedReliable>(ChannelSettings {
        mode: ChannelMode::UnorderedReliable(ReliableSettings::default()),
        ..default()
    });
}
