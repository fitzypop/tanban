use cursive::views::{Dialog, TextView};

const THEME_TOML: &'static str = include_str!("theme.toml");

fn main() {
    let mut siv = cursive::default();

    // set themeing via toml tile
    siv.load_toml(THEME_TOML).unwrap();

    siv.add_layer(
        Dialog::around(TextView::new("Hello TUI!!"))
            .title("Tanban")
            .button("Quit", |s| s.quit()),
    );

    siv.run();
}
