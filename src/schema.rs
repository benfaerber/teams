use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Color {
    Default,
    Dark,
    Light,
    Accent,
    Good,
    Warning,
    Attention,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Font {
    Default,
    Monospace,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
}

#[derive(Serialize, Deserialize)]
pub struct BasicMessage {
    text: String,
}

impl BasicMessage {
    pub fn text(text: &str) -> Self {
        Self {
            text: text.into()
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
enum CardType {
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardBlock {
    #[serde(rename = "type")]
    _type: String,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    font_type: Option<Font>,
    #[serde(skip_serializing_if = "Option::is_none")]
    horizontal_alignment: Option<HorizontalAlignment>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    is_subtle: Option<bool>,
}

impl CardBlock {
    pub fn text(text: &str) -> Self {
        Self {
            _type: "TextBlock".into(),
            text: text.into(),
            color: None,
            font_type: None,
            horizontal_alignment: None,
            is_subtle: None,
        }
    }

    pub fn colored_text(text: &str, color: Color) -> Self {
        Self {
            _type: "TextBlock".into(),
            text: text.into(),
            color: Some(color),
            font_type: None,
            horizontal_alignment: None,
            is_subtle: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CardContent {
    #[serde(rename = "$schema")]
    schema: String,
    #[serde(rename = "type")]
    _type: String,
    version: String,
    body: Vec<CardBlock>,
}

impl CardContent {
    pub fn body(body: Vec<CardBlock>) -> Self {
        Self {
            schema: "http://adaptivecards.io/schemas/adaptive-card.json".into(),
            _type: "AdaptiveCard".into(),
            version: "1.2".into(),
            body
        }
    }
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardAttachment {
    content_type: String,
    content_url: Option<String>,
    content: CardContent,
}

impl CardAttachment {
    pub fn content(content: CardContent) -> Self {
        Self {
            content_type: "application/vnd.microsoft.card.adaptive".into(),
            content_url: None,
            content
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "type")]
    _type: CardType,
    attachments: Vec<CardAttachment>,
}

impl Message {
    pub fn attachments(attachments: Vec<CardAttachment>) -> Self {
        Self {
            _type: CardType::Message,
            attachments
        }
    }

    pub fn attachment(attachment: CardAttachment) -> Self {
        Self::attachments(vec![attachment])
    }
}

