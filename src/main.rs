use cursive::views::{Dialog, TextView, ListView, Button, EditView};
use cursive::traits::*;
use cursive::Cursive;
use cursive::With;
use cursive::menu;
use cursive::event;

mod json;

pub fn test(list: ListView) -> ListView {
    list.child("", Button::new_raw("Quit", |s| s.quit()))
}

fn main() {
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();
    
    // Loading default theme
    siv.load_toml(include_str!("../themes/dark.toml")).unwrap();


    let main_list = ListView::new()
                        .child("Test1", TextView::new("Hello Dialog!"));
    // Creates a dialog with a single "Quit" button
    siv.add_layer(
            test(main_list) 
    );

    siv.menubar()
    .add_subtree(
        "File",
        menu::Tree::new()
            .leaf("New", |s| s.add_layer(Dialog::info("New file!")))
            .subtree(
                "Recent",
                menu::Tree::new().with(|tree| {
                    for i in 1..100 {
                        tree.add_leaf(format!("Item {}", i), |_| ())
                    }
                }),
            )
            .delimiter()
            .with(|tree| {
                for i in 1..10 {
                    tree.add_leaf(format!("Option {}", i), |_| ());
                }
            })
            .delimiter()
            .leaf("Quit", |s| s.quit()),
    )
    .add_subtree(
        "Help",
        menu::Tree::new()
            .subtree(
                "Help",
                menu::Tree::new()
                    .leaf("General", |s| {
                        s.add_layer(Dialog::info("Help message!"))
                    })
                    .leaf("Online", |s| {
                        s.add_layer(Dialog::info("Online help?"))
                    }),
            )
            .leaf("About", |s| {
                s.add_layer(Dialog::info("Cursive v0.0.0"))
            }),
    );

    siv.add_global_callback(event::Key::Esc, |s| s.select_menubar());
    siv.add_global_callback(event::Event::Char('q'), |s| s.quit());
    siv.add_global_callback(event::Event::Char('a'), |s| show_add_dialog(s));

    // Starts the event loop.
    siv.run();
}


// This will replace the current layer with a new popup.
// If the name is empty, we'll show an error message instead.
fn submit(s: &mut Cursive, name: &str) {
    if name.is_empty() {
        // Try again as many times as we need!
        s.add_layer(Dialog::info("Please enter a name!"));
    } else {
        json::save_test(name).unwrap();
        let content = format!("Hello {}!", name);
        // Remove the initial popup
        s.pop_layer();
        // And put a new one instead
        s.add_layer(
            Dialog::around(TextView::new(content))
                .button("Quit", |s| s.quit()),
        );
    }
}

fn show_add_dialog(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(
        Dialog::new()
            .title("Enter name for new list item")
            // Padding is (left, right, top, bottom)
            .padding_lrtb(1, 1, 1, 0)
            .content(
                EditView::new()
                    // Call `submit` when the user presses `Enter`
                    .on_submit(submit)
                    // Give the `EditView` a name so we can refer to it later.
                    .with_name("name")
                    // Wrap this in a `ResizedView` with a fixed width.
                    // Do this _after_ `with_name` or the name will point to the
                    // `ResizedView` instead of `EditView`!
                    .fixed_width(20),
            )
            .button("Ok", |s| {
                // This will run the given closure, *ONLY* if a view with the
                // correct type and the given name is found.
                let name = s
                    .call_on_name("name", |view: &mut EditView| {
                        // We can return content from the closure!
                        view.get_content()
                    })
                    .unwrap();

                // Run the next step
                submit(s, &name);
            }),
    );
}
