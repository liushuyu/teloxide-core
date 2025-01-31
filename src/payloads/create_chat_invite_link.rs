// This file is auto generated by [`cg`] from [`schema`].
//
// **DO NOT EDIT THIS FILE**,
//
// Edit `cg` or `schema` instead.
//
// [cg]: https://github.com/teloxide/cg
// [`schema`]: https://github.com/WaffleLapkin/tg-methods-schema
use serde::Serialize;

use crate::types::{ChatId, ChatInviteLink};

impl_payload! {
    /// Use this method to create an additional invite link for a chat. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. The link can be revoked using the method [`RevokeChatInviteLink`]. Returns the new invite link as [`ChatInviteLink`] object.
    ///
    /// [`ChatInviteLink`]: crate::types::ChatInviteLink
    /// [`RevokeChatInviteLink`]: crate::payloads::RevokeChatInviteLink
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub CreateChatInviteLink (CreateChatInviteLinkSetters) => ChatInviteLink {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: ChatId [into],
        }
        optional {
            /// Point in time (Unix timestamp) when the link will expire
            pub expire_date: i64,
            /// Maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
            pub member_limit: u32,
        }
    }
}
