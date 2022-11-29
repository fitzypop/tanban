use cursive::views::{Dialog, TextView};

fn main() {
    let mut siv = cursive::default();

    // set themeing via toml tile
    siv.load_toml(include_str!("theme.toml")).unwrap();

    siv.add_layer(
        Dialog::around(TextView::new("Hello TUI!!"))
            .title("Tanban")
            .button("Quit", |s| s.quit()),
    );

    siv.run();
}
