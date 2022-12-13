use serenity::model::id::{MessageId, RoleId, EmojiId};
use serenity::prelude::TypeMapKey;

pub struct ReactionHandler {
    pub reaction_events: Vec<ReactionRole>
}
impl ReactionHandler {
    pub fn new()  -> ReactionHandler {
        ReactionHandler {
            reaction_events: Vec::new()
        }
    }
    pub fn register(&mut self, message: MessageId, role: RoleId, emoji: EmojiId) {
        let listener = ReactionRole {
            message_id: message,
            role_id: role,
            emoji_id: emoji
        };

        self.reaction_events.push(listener);
    }
    pub fn get(&self, message: &MessageId, emoji: &EmojiId) -> Option<&ReactionRole> {
        if self.reaction_events.is_empty() {
            return None;
        }

        for reaction in self.reaction_events.iter() {
            if &reaction.message_id == message && &reaction.emoji_id == emoji {
                return Some(&reaction);
            }
        }

        return None;
    }
}

impl TypeMapKey for ReactionHandler {
    type Value = ReactionHandler;
}

pub struct ReactionRole {
    pub message_id: MessageId,
    pub role_id: RoleId,
    pub emoji_id: EmojiId
}

impl ReactionRole {
    pub fn assign_role(&self, ) {

    }
}