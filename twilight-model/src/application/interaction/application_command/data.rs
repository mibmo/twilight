use crate::{
    application::{
        command::CommandType,
        interaction::application_command::{CommandDataOption, CommandInteractionDataResolved},
    },
    id::{
        marker::{CommandMarker, GenericMarker, GuildMarker},
        Id,
    },
};
use serde::{Deserialize, Serialize};

/// Data received when an [`ApplicationCommand`] interaction is executed.
///
/// See [Discord Docs/Interaction Object].
///
/// [`ApplicationCommand`]: crate::application::interaction::Interaction::ApplicationCommand
/// [Discord Docs/Interaction Object]: https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-data-structure
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CommandData {
    /// ID of the guild the command is registered to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    /// ID of the command.
    pub id: Id<CommandMarker>,
    /// Name of the command.
    pub name: String,
    /// Type of the command.
    #[serde(rename = "type")]
    pub kind: CommandType,
    /// List of parsed options specified by the user.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<CommandDataOption>,
    /// Data sent if any of the options are Discord types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<CommandInteractionDataResolved>,
    /// If this is a user or message command, the ID of the targeted user/message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<Id<GenericMarker>>,
}
