//! Request parameters types of Telegram bot methods.
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::default::Default;
use std::error::Error;
use std::fmt;
use super::types;
use super::types::{ChatId, ForceReply, InlineKeyboardMarkup, MessageId,
                   ParseMode, ReplyKeyboardMarkup, ReplyKeyboardRemove, UpdateId, UserId};


/// Chat integer identifier or username
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(untagged)]
pub enum ChatTarget {
    Id(ChatId),
    Username(String),
}


/// Use this method to receive incoming updates using long
/// polling ([wiki](https://en.wikipedia.org/wiki/Push_technology#Long_polling)).
/// An Array of [`Update`](types::Update) objects is returned.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct GetUpdates {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<UpdateId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
}


impl GetUpdates {
    pub fn new() -> GetUpdates {
        Default::default()
    }

    pub fn offset(&mut self, x: UpdateId) {
        self.offset = Some(x)
    }
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct ApiError {
    error_code: Option<i32>,
    description: String,
}


impl<T> From<TelegramResult<T>> for ApiError {
    fn from(result: TelegramResult<T>) -> ApiError {
        ApiError {
            error_code: result.err_code,
            description: result.description.unwrap_or_default(),
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ERROR] {}", self.description)
    }
}


impl Error for ApiError {
    fn description(&self) -> &str {
        self.description.as_ref()
    }
}



/// Use this method to specify a url and receive incoming updates via an outgoing webhook.
/// Whenever there is an update for the bot, we will send an HTTPS POST request to the specified
/// url, containing a JSON-serialized [`Update`](types::Update). In case of an unsuccessful request, we will give up
/// after a reasonable amount of attempts. Returns True on success.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SetWebhook {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
}


impl SetWebhook {
    pub fn new(url: String) -> SetWebhook {
        SetWebhook {
            url,
            max_connections: None,
            allowed_updates: None,
        }
    }
}


/// Kinds of reply markup.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(untagged)]
pub enum ReplyMarkup {
    InlineKeyboard(InlineKeyboardMarkup),
    ReplyKeyboard(ReplyKeyboardMarkup),
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    ForceReply(ForceReply),
}


/// Send text messages. On success, the sent [`Message`](types::Message) is returned.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SendMessage {
    pub chat_id: ChatTarget,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}


impl SendMessage {
    pub fn new(chat_id: ChatTarget, text: String) -> SendMessage {
        SendMessage {
            chat_id,
            text,
            parse_mode: None,
            disable_web_page_preview: Some(false),
            reply_to_message_id: None,
            disable_notification: Some(false),
            reply_markup: None,
        }
    }

    pub fn reply(chat_id: ChatTarget, text: String, message_id: MessageId) -> SendMessage {
        let message = Self::new(chat_id, text);
        SendMessage {
            reply_to_message_id: Some(message_id),
            ..message
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(untagged)]
pub enum File {
    Id(types::FileId),
    Url(String),
//    Upload,
}


/// Use this method to send .webp stickers.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SendSticker {
    pub chat_id: ChatTarget,
    pub sticker: File,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}


impl SendSticker {
    pub fn new(chat_id: ChatTarget, sticker: File) -> SendSticker {
        SendSticker {
            chat_id,
            sticker,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }
}


/// Use this method to send photos.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SendPhoto {
    pub chat_id: ChatTarget,
    pub photo: File,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}


impl SendPhoto {
    pub fn new(chat_id: ChatTarget, photo: File) -> SendPhoto {
        SendPhoto {
            chat_id,
            photo,
            caption: None,
            parse_mode: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SendDocument {
    pub chat_id: ChatTarget,
    pub document: File,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}


impl SendDocument {
    pub fn new(chat_id: ChatTarget, document: File) -> SendDocument {
        SendDocument {
            chat_id,
            document,
            caption: None,
            parse_mode: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }
}


/// Use this method to forward messages of any kind.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ForwardMessage {
    pub chat_id: ChatTarget,
    pub from_chat_id: ChatTarget,
    pub message_id: MessageId,
}

/// To get a list of profile pictures for a user. Returns a [`UserProfilePhotos`](types::UserProfilePhotos) object.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetUserProfilePhotos {
    pub user_id: UserId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}


/// Use this method to get up to date information about the chat (current name of the user
/// for one-on-one conversations, current username of a user, group or channel, etc.).
///
/// Returns a [`Chat`] object on success.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetChat {
    pub chat_id: ChatTarget,
}

/// Use this method to get the number of members in a chat. Returns `Int` on success.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetChatMembersCount {
    pub chat_id: ChatTarget,
}

/// Use this method to get a list of administrators in a chat. On success, returns an Array
/// of `ChatMember` objects that contains information about all chat administrators except
/// other bots. If the chat is a group or a supergroup and no administrators were appointed,
/// only the creator will be returned.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetChatAdministrators {
    pub chat_id: ChatTarget,
}

/// Use this method to get information about a member of a chat. Returns a `ChatMember`
/// object on success.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetChatMember {
    pub chat_id: ChatTarget,
    pub user_id: UserId,
}


/// Use this method to edit text and game messages sent by the bot or via the bot (for inline bots).
/// On success, if edited message is sent by the bot, the edited [`Message`](types::Message) is
/// returned, otherwise True is returned.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct EditMessageText {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatTarget>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Use this method to edit captions of messages sent by the bot or via the bot (for inline bots).
/// On success, if edited message is sent by the bot, the edited [`Message`](types::Message) is
/// returned, otherwise True is returned.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct EditMessageCaption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatTarget>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}


/// Use this method to edit only the reply markup of messages sent by the bot or via the bot (for
/// inline bots). On success, if edited message is sent by the bot, the edited [`Message`](types::Message)
/// is returned, otherwise True is returned.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct EditMessageReplyMarkup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatTarget>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}


/// Use this method to delete a message, including service messages, with the following limitations:
///
/// - A message can only be deleted if it was sent less than 48 hours ago.
/// - Bots can delete outgoing messages in groups and supergroups.
/// - Bots granted can_post_messages permissions can delete outgoing messages in channels.
/// - If the bot is an administrator of a group, it can delete any message there.
/// - If the bot has can_delete_messages permission in a supergroup or a channel, it can delete any message there.
///
/// Returns True on success.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeleteMessage {
    pub chat_id: ChatTarget,
    pub message_id: MessageId,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct GetMe;

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteWebhook;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetWebhookInfo;

/// Telegram methods.
pub trait Method: Serialize {
    /// Method name in the Telegram Bot API url.
    const NAME: &'static str;
    /// Method return type.
    type Item: DeserializeOwned + fmt::Debug + 'static;

    /// Get method url.
    fn url(token: &str) -> String {
        format!("https://api.telegram.org/bot{}/{}", token, Self::NAME)
    }
}

macro_rules! impl_method {
    ($Type: ty, $name: expr, $Item: ty) => {
        impl Method for $Type {
            const NAME: &'static str = $name;
            type Item = $Item;
        }
    };
}


//           Type                   Method                   Return
impl_method!(GetUpdates           , "getUpdates"           , Vec<types::Update>    );
impl_method!(GetMe                , "getMe"                , types::User           );
impl_method!(SetWebhook           , "setWebhook"           , bool                  );
impl_method!(DeleteWebhook        , "deleteWebhook"        , bool                  );
impl_method!(GetWebhookInfo       , "getWebhookInfo"       , types::WebhookInfo    );
impl_method!(SendMessage          , "sendMessage"          , types::Message        );
impl_method!(ForwardMessage       , "forwardMessage"       , types::Message        );
impl_method!(EditMessageText      , "editMessageText"      , types::Message        );
impl_method!(DeleteMessage        , "deleteMessage"        , bool                  );
impl_method!(EditMessageCaption   , "editMessageCaption"   , bool                  );
impl_method!(SendSticker          , "sendSticker"          , types::Message        );
impl_method!(SendPhoto            , "sendPhoto"            , types::Message        );
impl_method!(SendDocument         , "sendDocument"         , types::Message        );
impl_method!(GetChat              , "getChat"              , types::Chat           );
impl_method!(GetChatAdministrators, "getChatAdministrators", Vec<types::ChatMember>);
impl_method!(GetChatMembersCount  , "getChatMembersCount"  , i32                   );
impl_method!(GetChatMember        , "getChatMember"        , types::ChatMember     );


#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TelegramResult<T>
{
    pub ok: bool,
    pub description: Option<String>,
    pub err_code: Option<i32>,
    pub result: Option<T>,
}


impl<T> Into<Result<T, ApiError>> for TelegramResult<T> {
    fn into(self) -> Result<T, ApiError> {
        if self.ok {
            self.result.ok_or(ApiError { error_code: Some(0), description: "empty return".to_string() })
        } else {
            Err(self.into())
        }
    }
}


pub type UpdateList = TelegramResult<Vec<types::Update>>;
