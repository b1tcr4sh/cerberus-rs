use serenity::model::id::{MessageId, RoleId};
use serenity::model::prelude::{Member, ChannelId, ReactionType};
use serenity::prelude::{TypeMapKey, Context};

pub struct ReactionHandler {
    pub reaction_events: Vec<ReactionRole>,
    pub active_creation_listener: Option<MessageId>,
    reaction_paylod: Option<ReactionType>
}
impl ReactionHandler {
    pub fn new()  -> ReactionHandler {
        ReactionHandler {
            reaction_events: Vec::new(),
            active_creation_listener: None,
            reaction_paylod: None
        }
    }
    pub fn register(&mut self, message_id: u64, role_id: u64, emoji: ReactionType) {
        let listener = ReactionRole {
            message_id: MessageId(message_id),
            role_id: RoleId(role_id),
            emoji: emoji
        };

        self.reaction_events.push(listener);
    }
    pub fn get(&self, message: &MessageId, emoji: &ReactionType) -> Option<&ReactionRole> {
        if self.reaction_events.is_empty() {
            return None;
        }

        for reaction in self.reaction_events.iter() {
            if reaction.message_id.0 == message.0 && reaction.emoji == emoji.to_owned() {
                return Some(&reaction);
            }
        }

        return None;
    }
    pub fn send_paylod(&self, emoji: ReactionType) {
        self.reaction_paylod = Some(emoji);
    }
    pub fn paylod(&self) -> Option<ReactionType> {
        self.reaction_paylod
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