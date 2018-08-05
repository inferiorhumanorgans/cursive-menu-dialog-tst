extern crate cursive;

use cursive::Cursive;
use cursive::align::HAlign;
use cursive::traits::*;
use cursive::views::{Dialog, SelectView};

fn action_dialog(siv: &mut cursive::Cursive) {
    let mut select = SelectView::new().h_align(HAlign::Center);
    select.add_item("One", 1);
    select.add_item("Two", 2);

    let dialog = Dialog::around(
        select.fixed_size((40, 10))
    ).title("Dialog Box");
    siv.add_layer(dialog);
}

fn action_quit(siv: &mut cursive::Cursive) {
    siv.quit()
}

fn main() {
    let mut siv = Cursive::ncurses();
    siv.menubar()
        .add_leaf("Dialog", action_dialog)
        .add_leaf("Quit", action_quit);
    siv.set_autohide_menu(false);
    siv.run();
}
