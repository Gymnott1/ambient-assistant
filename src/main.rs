use gtk4::prelude::*;
use gtk4::{
    Application, Window, Box, Button, Label, Orientation, 
    CssProvider, PopoverMenu, GestureClick
};
use gio::{SimpleAction, Menu, MenuItem};
use std::rc::Rc;
use std::cell::Cell;

fn main() {
    let app = Application::builder()
        .application_id("com.ambient.assistant")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = Window::builder()
        .application(app)
        .title("Ambient Assistant")
        .decorated(false) // Custom glass look
        .default_width(320)
        .build();

    let is_pinned = Rc::new(Cell::new(true)); // Default to always on top
    let is_hidden = Rc::new(Cell::new(false));

    // ROOT CONTAINER
    let root = Box::new(Orientation::Vertical, 0);
    root.add_css_class("root-container");

    // 1. HEADER (The Drag Handle)
    let header = Box::new(Orientation::Horizontal, 0);
    header.add_css_class("drag-handle");
    
    let drag_label = Label::new(Some("⠿ Ambient Assistant"));
    drag_label.set_hexpand(true);
    drag_label.set_halign(gtk4::Align::Start);
    drag_label.set_margin_start(12);
    header.append(&drag_label);

    let shortcut_hint = Label::new(Some("📌 Always on Top"));
    shortcut_hint.add_css_class("hint-text");
    header.append(&shortcut_hint);

    // Make the header the OS-level drag handle
    window.set_titlebar(Some(&header));

    // RIGHT-CLICK MENU
    let menu = Menu::new();
    let pin_item = MenuItem::new(Some("Toggle Always on Top"), Some("app.toggle-pin"));
    let hide_item = MenuItem::new(Some("Toggle Hide Content"), Some("app.toggle-hide"));
    menu.append_item(&pin_item);
    menu.append_item(&hide_item);
    
    let popover = PopoverMenu::from_model(Some(&menu));
    popover.set_parent(&window);
    
    let right_click = GestureClick::new();
    right_click.set_button(3); // Right mouse button
    let popover_clone = popover.clone();
    right_click.connect_pressed(move |_gesture, _, x, y| {
        popover_clone.set_pointing_to(Some(&gtk4::gdk::Rectangle::new(x as i32, y as i32, 1, 1)));
        popover_clone.popup();
    });
    window.add_controller(right_click);

    // 2. SUGGESTIONS CONTENT
    let content_box = Box::new(Orientation::Vertical, 8);
    content_box.set_margin_all(15);

    let suggestions = vec![
        ("🎵 Play Focus Music", "Spotify • Lofi Beats"),
        ("📝 Git: Commit Changes", "5 files modified"),
        ("🔑 SSH: Connect to Server", "Root access ready"),
        ("🧹 Cleanup Downloads", "1.2GB of temp files"),
        ("🚀 Deploy App", "All tests passed"),
    ];

    for (title, sub) in suggestions {
        content_box.append(&create_suggestion_item(title, sub));
    }

    // 3. ACTION: TOGGLE PIN (Super+P + Right-click menu)
    let pin_action = SimpleAction::new("toggle-pin", None);
    let _win_pin = window.clone();
    let is_pinned_clone = Rc::clone(&is_pinned);
    let shortcut_hint_clone = shortcut_hint.clone();
    
    pin_action.connect_activate(move |_, _| {
        let pinned = !is_pinned_clone.get();
        is_pinned_clone.set(pinned);
        
        // Use X11/Wayland compatible method
        use std::process::Command;
        
        if pinned {
            // Get window ID and set always on top
            if let Ok(output) = Command::new("xdotool")
                .args(["search", "--name", "Ambient Assistant"])
                .output() {
                if let Ok(window_id) = String::from_utf8(output.stdout) {
                    let window_id = window_id.trim();
                    if !window_id.is_empty() {
                        let _ = Command::new("wmctrl")
                            .args(["-i", "-r", window_id, "-b", "add,above"])
                            .output();
                    }
                }
            }
            shortcut_hint_clone.set_label("📌 Always on Top");
            println!("📌 Always on top: ENABLED");
        } else {
            // Remove always on top
            if let Ok(output) = Command::new("xdotool")
                .args(["search", "--name", "Ambient Assistant"])
                .output() {
                if let Ok(window_id) = String::from_utf8(output.stdout) {
                    let window_id = window_id.trim();
                    if !window_id.is_empty() {
                        let _ = Command::new("wmctrl")
                            .args(["-i", "-r", window_id, "-b", "remove,above"])
                            .output();
                    }
                }
            }
            shortcut_hint_clone.set_label("Super+P to Pin");
            println!("📍 Always on top: DISABLED");
        }
    });
    app.add_action(&pin_action);
    app.set_accels_for_action("app.toggle-pin", &["<Super>p"]);

    // 4. ACTION: TOGGLE HIDE (Super+H)
    let hide_action = SimpleAction::new("toggle-hide", None);
    let content_copy = content_box.clone();
    let is_hidden_clone = Rc::clone(&is_hidden);
    
    hide_action.connect_activate(move |_, _| {
        let hidden = !is_hidden_clone.get();
        is_hidden_clone.set(hidden);
        content_copy.set_visible(!hidden);
    });
    app.add_action(&hide_action);
    app.set_accels_for_action("app.toggle-hide", &["<Super>h"]);

    root.append(&content_box);

    load_css();
    window.set_child(Some(&root));
    
    // Set always on top after window is shown
    glib::timeout_add_local(std::time::Duration::from_millis(500), move || {
        let _ = std::process::Command::new("wmctrl")
            .args(["-r", "Ambient Assistant", "-b", "add,above"])
            .output();
        println!("📌 Always on top enabled by default");
        glib::ControlFlow::Break
    });
    
    window.present();
}

fn create_suggestion_item(title: &str, subtitle: &str) -> Button {
    let inner = Box::new(Orientation::Vertical, 2);
    let t = Label::builder().label(title).halign(gtk4::Align::Start).build();
    t.add_css_class("s-title");
    let s = Label::builder().label(subtitle).halign(gtk4::Align::Start).build();
    s.add_css_class("s-sub");

    inner.append(&t);
    inner.append(&s);

    let btn = Button::builder().child(&inner).build();
    btn.add_css_class("suggestion-item");
    
    let title_owned = title.to_string();
    btn.connect_clicked(move |_| {
        println!("🚀 Executing: {}", title_owned);
    });

    btn
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data("
        window { background: transparent; }
        .root-container {
            background-color: rgba(30, 30, 46, 0.95);
            border-radius: 0 0 16px 16px;
            border: 1px solid rgba(255, 255, 255, 0.1);
            border-top: none;
        }
        .drag-handle {
            background: rgba(45, 45, 70, 0.98);
            padding: 10px;
            border-radius: 16px 16px 0 0;
            color: #89b4fa;
            font-weight: bold;
            border: 1px solid rgba(255, 255, 255, 0.1);
        }
        .hint-text {
            color: #6c7086;
            font-size: 10px;
            margin-right: 10px;
        }
        .suggestion-item {
            background: rgba(45, 45, 70, 0.6);
            border-radius: 12px;
            padding: 12px;
            margin-bottom: 4px;
            border: 1px solid transparent;
        }
        .suggestion-item:hover {
            background: rgba(137, 180, 250, 0.2);
            border: 1px solid #89b4fa;
        }
        .s-title { color: #f5e0dc; font-weight: bold; }
        .s-sub { color: #a6adc8; font-size: 11px; }
    ");

    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().expect("Display error"),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

trait MarginExt { fn set_margin_all(&self, margin: i32); }
impl MarginExt for Box {
    fn set_margin_all(&self, margin: i32) {
        self.set_margin_start(margin); self.set_margin_end(margin);
        self.set_margin_top(margin); self.set_margin_bottom(margin);
    }
}