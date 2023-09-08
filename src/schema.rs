use builder_pattern::Builder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum Color {
    #[default]
    Default,
    Dark,
    Light,
    Accent,
    Good,
    Warning,
    Attention,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum FontSize {
    #[default]
    Default,
    Small,
    Medium,
    ExtraLarge,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum FontWeight {
    #[default]
    Default,
    Lighter,
    Darker,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum Font {
    #[default]
    Default,
    Monospace,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum TextBlockStyle {
    #[default]
    Default,
    Heading,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum HorizontalAlignment {
    #[default]
    Left,
    Center,
    Right,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum BlockType {
    TextBlock,
}

#[derive(Serialize, Deserialize)]
pub struct BasicMessage {
    text: String,
}

impl BasicMessage {
    pub fn text(text: &str) -> Self {
        Self { text: text.into() }
    }
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
enum CardType {
    #[default]
    Message,
}

#[derive(Builder, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextBlock {
    #[into]
    text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[default(None)]
    color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[default(None)]
    font_type: Option<Font>,

    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    horizontal_alignment: Option<HorizontalAlignment>,

    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    is_subtle: Option<bool>,

    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    max_lines: Option<i32>,

    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<FontSize>,

    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<FontWeight>,

    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    wrap: Option<bool>,

    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    text_block_style: Option<TextBlockStyle>,
}

impl TextBlock {
    pub fn with_text(text: &str) -> Self {
        Self::new().text(text).build()
    }
}

#[derive(Serialize, Deserialize)]
pub enum CardBlock {
    Text(TextBlock),
}

impl CardBlock {
    fn get_type_name(&self) -> String {
        match &self {
            CardBlock::Text(_) => "TextBlock",
        }
        .into()
    }

    pub fn to_json(&self) -> serde_json::Value {
        let mut raw = serde_json::to_value(match self {
            CardBlock::Text(text) => text,
        })
        .unwrap();
        raw["_type"] = serde_json::Value::String(self.get_type_name().to_string());
        raw
    }
}

#[derive(Serialize, Deserialize)]
pub struct CardContent {
    #[serde(rename = "$schema")]
    schema: String,
    #[serde(rename = "type")]
    _type: String,
    version: String,
    body: Vec<serde_json::Value>,
}

impl CardContent {
    pub fn body(body: Vec<TextBlock>) -> Self {
        Self {
            schema: "http://adaptivecards.io/schemas/adaptive-card.json".into(),
            _type: "AdaptiveCard".into(),
            version: "1.2".into(),
            body: body
                .iter()
                .map(|item| serde_json::to_value(item).unwrap())
                .collect(),
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
            content,
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
            attachments,
        }
    }

    pub fn attachment(attachment: CardAttachment) -> Self {
        Self::attachments(vec![attachment])
    }
}
