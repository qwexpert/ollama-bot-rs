pub fn console_log(username: Option<&str>, prompt: &str, res: &str, chat_id: String) {
    let reset = "\x1b[0m";
    let red = "\x1b[31m";
    let green = "\x1b[32m";
    let blue = "\x1b[34m";
    let magenta = "\x1b[35m";
    let bold = "\x1b[1m";

    let username = username.unwrap_or("Anonymous");

    println!(
        "\n\n{}{}{}: {}{}{}\n{}{}{}: {}{}{}\n{}{}{}: {}{}{}\n{}{}{}: {}{}{}",
        bold,
        "Username",
        reset,
        green,
        username,
        reset,
        bold,
        "Chat_ID",
        reset,
        magenta,
        chat_id,
        reset,
        bold,
        "Request",
        reset,
        red,
        prompt,
        reset,
        bold,
        "Response",
        reset,
        blue,
        res,
        reset
    );
}
