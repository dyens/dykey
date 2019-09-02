extern crate gdk;
extern crate gio;
extern crate gtk;

use std::cell::RefCell;
use std::env::args;
use std::rc::Rc;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::Box;
use gtk::Entry;
use gtk::EntryCompletion;
use gtk::Inhibit;
use gtk::ListStore;
use gtk::Orientation;
use gtk::Stack;
use gtk::TextView;
use gtk::Type;

struct DyKey {
    stack: Stack,

    main_password_entry: Entry,
    _main_password_entered: bool,

    search_entry: Entry,
    search_result: TextView,
}

impl DyKey {
    fn new(
        stack: Stack,
        main_password_entry: Entry,
        search_entry: Entry,
        search_result: TextView,
    ) -> Self {
        DyKey {
            stack: stack,
            main_password_entry: main_password_entry,
            _main_password_entered: false,
            search_entry: search_entry,
            search_result: search_result,
        }
    }
}

fn create_list_model() -> ListStore {
    let col_types: [Type; 1] = [Type::String];

    let data: [String; 4] = [
        "France".to_string(),
        "Italy".to_string(),
        "Sweden".to_string(),
        "Switzerland".to_string(),
    ];
    let store = ListStore::new(&col_types);
    let col_indices: [u32; 1] = [0];
    for d in data.iter() {
        let values: [&dyn ToValue; 1] = [&d];
        store.set(&store.append(), &col_indices, &values);
    }
    store
}

fn connect_to_ui(dykey_rc: Rc<RefCell<DyKey>>) {
    let dykey_outer = dykey_rc.clone();
    dykey_outer
        .borrow()
        .main_password_entry
        .connect_activate(move |password_entry| {
            let dykey = dykey_rc.borrow_mut();
            let text = password_entry.get_text();
            dykey.stack.set_visible_child_name("search_box");
            dykey.search_result.get_buffer().unwrap().set_text("test");
            println!("Password {:?}", text);
        });
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_application(Some(application));
    window.set_title("dykey: gtk::Focus");
    window.set_border_width(2);
    window.set_resizable(false);

    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let stack = Stack::new();
    let main_password_entry = Entry::new();

    let search_entry_completion = EntryCompletion::new();
    search_entry_completion.set_text_column(0);
    search_entry_completion.set_minimum_key_length(1);
    search_entry_completion.set_popup_completion(true);
    let model_list = create_list_model();
    search_entry_completion.set_model(Some(&model_list));

    let search_entry = Entry::new();
    search_entry.set_completion(Some(&search_entry_completion));
    let search_result = TextView::new();

    search_entry.set_text("test1");
    search_result.get_buffer().unwrap().set_text("test2");

    let search_box = Box::new(Orientation::Vertical, 2);
    search_box.pack_start(&search_entry, true, true, 0);
    search_box.pack_start(&search_result, true, true, 0);

    stack.add_named(&main_password_entry, "main_password");
    stack.add_named(&search_box, "search_box");

    window.add(&stack);

    let dykey = Rc::new(RefCell::new(DyKey::new(
        stack,
        main_password_entry,
        search_entry,
        search_result,
    )));
    connect_to_ui(dykey.clone());
    window.show_all();
}

pub fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let application =
        gtk::Application::new(Some("com.github.dykey"), gio::ApplicationFlags::empty())
            .expect("Initialization failed...");

    application.connect_startup(move |app| {
        build_ui(app);
    });
    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());
}
