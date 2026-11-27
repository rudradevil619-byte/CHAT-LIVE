use iced::{
    widget::{
        button, column, container, row, scrollable, text, text_input,
    },
    Element, Length, Task, Theme,
};

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    SendPressed,
}

pub struct RustChat {
    username: String,
    input_message: String,
    messages: Vec<String>,
}

impl Default for RustChat {
    fn default() -> Self {
        Self {
            username: "You".to_string(),
            input_message: String::new(),
            messages: vec![
                "Welcome to RustChat!".to_string(),
                "Lesson 2 - Chat UI".to_string(),
            ],
        }
    }
}

impl RustChat {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::InputChanged(value) => {
                self.input_message = value;
            }

            Message::SendPressed => {
                let message = self.input_message.trim();

                if !message.is_empty() {
                    let chat_line =
                        format!("{}: {}", self.username, message);

                    self.messages.push(chat_line);

                    self.input_message.clear();
                }
            }
        }

        Task::none()
    }

    pub fn view(&self) -> Element<Message> {
        //-----------------------------
        // Sidebar
        //-----------------------------

        let sidebar = container(
            column![
                text("RustChat").size(30),
                text("----------------"),
                text("Friends"),
                text("🟢 Alice"),
                text("🟢 Bob"),
                text("🔴 Charlie"),
            ]
                .spacing(10)
                .padding(15),
        )
            .width(220);

        //-----------------------------
        // Chat messages
        //-----------------------------

        let mut chat_column = column![].spacing(10);

        for message in &self.messages {
            chat_column = chat_column.push(
                container(
                    text(message)
                        .size(18)
                )
                    .padding(10)
                    .width(Length::Fill),
            );
        }

        let chat_area = scrollable(chat_column)
            .height(Length::Fill);

        //-----------------------------
        // Message box
        //-----------------------------

        let input = text_input(
            "Type your message...",
            &self.input_message,
        )
            .on_input(Message::InputChanged)
            .padding(10)
            .width(Length::Fill);

        let send_button = button("Send")
            .padding(10)
            .on_press(Message::SendPressed);

        let bottom_bar = row![
            input,
            send_button,
        ]
            .spacing(10);

        //-----------------------------
        // Right side
        //-----------------------------

        let right_side = column![
            text("Chat")
                .size(30),

            chat_area,

            bottom_bar,
        ]
            .spacing(15)
            .padding(20);

        //-----------------------------
        // Main Layout
        //-----------------------------

        let content = row![
            sidebar,
            right_side,
        ];

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn theme(&self) -> Theme {
        crate::theme::dark_theme()
    }
}