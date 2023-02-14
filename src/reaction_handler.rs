use serenity::model::id::{MessageId, RoleId, EmojiId};
use serenity::model::prelude::{Member, ChannelId, ReactionType};
use serenity::prelude::{TypeMapKey, Context};

pub struct ReactionHandler {
    pub reaction_events: Vec<ReactionRole>
}
impl ReactionHandler {
    pub fn new()  -> ReactionHandler {
        ReactionHandler {
            reaction_events: Vec::new()
        }
    }
    pub fn register(&mut self, message: MessageId, role: RoleId, emoji: ReactionType) {
        let listener = ReactionRole {
            message_id: message,
            role_id: role,
            emoji: emoji
        };

        self.reaction_events.push(listener);
    }
    pub fn get(&self, message: &MessageId, emoji: &ReactionType) -> Option<&ReactionRole> {
        if self.reaction_events.is_empty() {
            return None;
        }

        for reaction in self.reaction_events.iter() {
            if &reaction.message_id == message && reaction.emoji == emoji.to_owned() {
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
    pub emoji: ReactionType
}

impl ReactionRole {
    pub async fn assign_role(&self, ctx: &Context, member: &mut Member, channel: &ChannelId) {
        if let Err(why) = member.add_role(&ctx, self.role_id).await {
            channel.say(&ctx, "Hey, adding the role failed for some reason, ping an admin? Idk").await.expect("Failed to send error message to channel!");
        }
    }
}