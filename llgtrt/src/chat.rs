use crate::routes::openai::{ChatCompletionMessageContentPart, ChatCompletionMessageParams};
use liquid::{ParserBuilder, Template};
use serde_json::{json, Value};

pub struct ChatBuilder {
    template: Template,
}

const DEFAULT_TEMPLATE: &str = "{% for item in items %}{{ item.role }}: {{ item.content }}
{% endfor %}assistant:";

impl ChatBuilder {
    pub fn new(template: Option<&str>) -> anyhow::Result<Self> {
        let template = ParserBuilder::with_stdlib()
            .build()?
            .parse(template.unwrap_or(DEFAULT_TEMPLATE))?;
        Ok(ChatBuilder { template })
    }

    pub fn build(&self, messages: &Vec<ChatCompletionMessageParams>) -> anyhow::Result<String> {
        let items: Vec<_> = messages.iter().map(chat_to_json).collect();
        let context = liquid::object!({ "items": items });
        Ok(self.template.render(&context)?)
    }
}

fn chat_to_json(message: &ChatCompletionMessageParams) -> Value {
    match message {
        ChatCompletionMessageParams::System { content, name } => {
            json!({
                "role": "system",
                "content": content_to_string(content),
                "name": name,
            })
        }
        ChatCompletionMessageParams::User { content, name } => {
            json!({
                "role": "user",
                "content": content_to_string(content),
                "name": name,
            })
        }
        ChatCompletionMessageParams::Assistant { content, name, .. } => {
            json!({
                "role": "assistant",
                "name": name,
                "content": content_to_string(content),
            })
        }
        ChatCompletionMessageParams::Tool { content, .. } => {
            json!({
                "role": "tool",
                "content": content_to_string(content),
            })
        }
    }
}

fn content_to_string(content: &ChatCompletionMessageContentPart) -> String {
    match content {
        ChatCompletionMessageContentPart::Text(text) => text.clone(),
        ChatCompletionMessageContentPart::ContentParts(content_parts) => {
            content_parts.iter().map(|s| s.text.clone()).collect::<Vec<String>>().join("\n").to_string()
        },
    }
}
