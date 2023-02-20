use serenity::model::id::{MessageId, RoleId};
use serenity::model::prelude::{Member, ChannelId, ReactionType};
use serenity::prelude::{TypeMapKey, Context};

pub struct ReactionHandler<'a> {
    pub reaction_events: Vec<ReactionRole<'a>>,
    pub active_creation_listener: Option<MessageId>,
    reaction_paylod: Option<ReactionType>
}
impl<'a> ReactionHandler<'a> {
    pub fn new<'b>()  -> ReactionHandler<'b> {
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

        self.reaction_events.push(&listener);
    }
    pub fn get(&self, message: &'a MessageId, emoji: &'a ReactionType) -> Option<&'a ReactionRole> {
        if self.reaction_events.is_empty() {
            return None;
        }

        for reaction in self.reaction_events.iter() {
            if reaction.message_id.0 == message.0 && reaction.emoji == emoji.to_owned() {
                return Some(reaction);
            }
        }

        return None;
    }
    pub fn send_paylod(&self, emoji: ReactionType) {
        self.reaction_paylod = Some(&emoji);
    }
    pub fn paylod(&self) -> Option<ReactionType> {
        self.reaction_paylod
    }
}

impl<'a> TypeMapKey for ReactionHandler<'a> {
    type Value = ReactionHandler<'a>;
}

pub struct ReactionRole<'a> {
    pub message_id: MessageId,
    pub role_id: RoleId,
    pub emoji: ReactionType
}

impl<'a> ReactionRole<'a> {
    pub async fn assign_role(&self, ctx: &Context, member: &mut Member, channel: &ChannelId) {
        if let Err(why) = member.add_role(&ctx, self.role_id).await {
            channel.say(&ctx, "Hey, adding the role failed for some reason, ping an admin? Idk").await.expect("Failed to send error message to channel!");
        }
    }
}