struct Message<'a> {
    text: &'a str,
}

fn main() {
    let message;

    {
        let text = String::from("hello");

        message = Message { text: &text };
    }

    println!("{}", message.text);
}
