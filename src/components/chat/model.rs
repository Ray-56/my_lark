use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct ChatListModel {
    pub items: Vec<ChatListItem>,
    pub filter: ChatFilter,
}

#[derive(Clone, Debug, Default)]
pub struct ChatListItem {
    pub id: String,
    pub name: String,
    pub avatar: String,
    pub last_message: Option<String>,
    pub is_selected: bool,
    pub unread_count: Option<i32>,
    pub is_pinned: bool,
}

#[derive(Clone, Debug, Copy, PartialEq, Default)]
pub enum ChatFilter {
    #[default]
    All,
    Pinned,
}

#[derive(Clone, Debug, Default)]
pub struct Chat {
    pub id: String,
    pub name: String,
    pub avatar: String,
    pub member_count: i32,
    pub last_message: Option<String>,
    pub chat_type: ChatType,
    pub pin: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum ChatType {
    #[default]
    Group,
}

#[derive(Debug, Clone, Default)]
pub struct ChatMessage {
    pub id: String,
    pub chat_id: String,
    pub sender: String,
    pub avatar: String,
    pub content: String,
    pub timestamp: String,
    pub message_type: MessageType,
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub enum MessageType {
    #[default]
    Text,
    Images,
    File,
    Code,
}

impl ChatListModel {
    pub fn new(chats: &[Chat], selected_id: &str, unread_count: &HashMap<String, i32>) -> Self {
        let items = chats
            .iter()
            .map(|chat| ChatListItem {
                id: chat.id.clone(),
                name: chat.name.clone(),
                avatar: chat.avatar.clone(),
                last_message: chat.last_message.clone(),
                is_selected: chat.id == selected_id,
                unread_count: unread_count.get(&chat.id).copied(),
                is_pinned: chat.pin,
                ..Default::default()
            })
            .collect();

        Self {
            items,
            ..Default::default()
        }
    }

	pub fn filtered(&self) -> Vec<&ChatListItem> {
		self.items
			.iter()
			.filter(|item| match self.filter {
				ChatFilter::All => true,
				ChatFilter::Pinned => item.is_pinned,
			})
			.collect()
	}
}
