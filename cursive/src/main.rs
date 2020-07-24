use cursive::views::Dialog;
use cursive::Cursive;

fn main() {
    let mut siv = cursive::default();

    siv.add_layer(
        Dialog::text("This is an urgent announcement!\nPress <Next> now!")
            .title("IMPORTANT MESSAGE")
            .button("Next", show_next),
    );

    siv.run();
}

fn show_next(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(Dialog::text("yeet").title("danny says:"));
}
