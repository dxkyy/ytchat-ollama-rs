pub type Result<T> = core::result::Result<T, Error>;

pub type Error = Box<dyn std::error::Error>;

pub mod consts {
    pub const MODEL: &str = "llama3";

    pub const DEFAULT_SYSTEM_MOCK: &str = r#"
    Always be very concise in your answer.

    If asked about the previous question, only give the user messages, not system message.
    "#;
}
