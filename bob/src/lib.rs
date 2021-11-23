pub fn reply(message: &str) -> &str {
    let message = message.trim();

    if message.is_empty() {
        return "Fine. Be that way!";
    }
    // message is all caps
    let uppercase = message.to_uppercase() == message && message.chars().any(|c| c.is_alphabetic());

    if message.ends_with('?') {
        if uppercase {
            return "Calm down, I know what I'm doing!";
        }
        return "Sure.";
    }

    if uppercase {
        return "Whoa, chill out!";
    }

    "Whatever."
}
