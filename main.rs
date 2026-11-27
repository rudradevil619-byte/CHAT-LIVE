mod app;
mod theme;

use app::RustChat;

fn main() -> iced::Result {
    iced::application(
        "RustChat",
        RustChat::update,
        RustChat::view,
    )
        .theme(RustChat::theme)
        .run()
}