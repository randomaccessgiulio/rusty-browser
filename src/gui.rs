#![allow(unsafe_op_in_unsafe_fn)]

use crate::make_target;
use std::ffi::{CStr, CString, c_char, c_int, c_uint, c_ulong, c_void};
use std::mem;
use std::ptr;

type Widget = c_void;
type Callback = Option<unsafe extern "C" fn()>;
type ClosureNotify = Option<unsafe extern "C" fn(*mut c_void, *mut c_void)>;

const GTK_WINDOW_TOPLEVEL: c_int = 0;
const GTK_ORIENTATION_HORIZONTAL: c_int = 0;
const GTK_ORIENTATION_VERTICAL: c_int = 1;
const WEBKIT_LOAD_STARTED: c_int = 0;
const WEBKIT_LOAD_REDIRECTED: c_int = 1;
const WEBKIT_LOAD_COMMITTED: c_int = 2;
const WEBKIT_LOAD_FINISHED: c_int = 3;
const WEBKIT_CACHE_MODEL_DOCUMENT_VIEWER: c_int = 0;
const WEBKIT_HARDWARE_ACCELERATION_POLICY_NEVER: c_int = 2;

#[link(name = "libgtk-3.so.0", modifiers = "+verbatim")]
unsafe extern "C" {
    fn gtk_init_check(argc: *mut c_int, argv: *mut *mut *mut c_char) -> c_int;
    fn gtk_window_new(window_type: c_int) -> *mut Widget;
    fn gtk_window_set_title(window: *mut Widget, title: *const c_char);
    fn gtk_window_set_icon_name(window: *mut Widget, name: *const c_char);
    fn gtk_window_set_default_size(window: *mut Widget, width: c_int, height: c_int);
    fn gtk_window_set_position(window: *mut Widget, position: c_int);
    fn gtk_container_add(container: *mut Widget, widget: *mut Widget);
    fn gtk_box_new(orientation: c_int, spacing: c_int) -> *mut Widget;
    fn gtk_box_pack_start(
        box_: *mut Widget,
        child: *mut Widget,
        expand: c_int,
        fill: c_int,
        padding: c_uint,
    );
    fn gtk_button_new_with_label(label: *const c_char) -> *mut Widget;
    fn gtk_entry_new() -> *mut Widget;
    fn gtk_entry_set_text(entry: *mut Widget, text: *const c_char);
    fn gtk_entry_get_text(entry: *mut Widget) -> *const c_char;
    fn gtk_entry_set_placeholder_text(entry: *mut Widget, text: *const c_char);
    fn gtk_label_new(text: *const c_char) -> *mut Widget;
    fn gtk_label_set_text(label: *mut Widget, text: *const c_char);
    fn gtk_label_set_xalign(label: *mut Widget, xalign: f32);
    fn gtk_widget_set_hexpand(widget: *mut Widget, expand: c_int);
    fn gtk_widget_set_vexpand(widget: *mut Widget, expand: c_int);
    fn gtk_widget_set_margin_start(widget: *mut Widget, margin: c_int);
    fn gtk_widget_set_margin_end(widget: *mut Widget, margin: c_int);
    fn gtk_widget_set_margin_top(widget: *mut Widget, margin: c_int);
    fn gtk_widget_set_margin_bottom(widget: *mut Widget, margin: c_int);
    fn gtk_widget_set_size_request(widget: *mut Widget, width: c_int, height: c_int);
    fn gtk_widget_set_tooltip_text(widget: *mut Widget, text: *const c_char);
    fn gtk_widget_grab_focus(widget: *mut Widget) -> c_int;
    fn gtk_widget_show_all(widget: *mut Widget);
    fn gtk_main();
    fn gtk_main_quit();
}

#[link(name = "libwebkit2gtk-4.1.so.0", modifiers = "+verbatim")]
unsafe extern "C" {
    fn webkit_web_view_new() -> *mut Widget;
    fn webkit_web_context_get_default() -> *mut c_void;
    fn webkit_web_context_set_cache_model(context: *mut c_void, cache_model: c_int);
    fn webkit_web_view_load_uri(web_view: *mut Widget, uri: *const c_char);
    fn webkit_web_view_get_uri(web_view: *mut Widget) -> *const c_char;
    fn webkit_web_view_get_title(web_view: *mut Widget) -> *const c_char;
    fn webkit_web_view_get_settings(web_view: *mut Widget) -> *mut c_void;
    fn webkit_web_view_can_go_back(web_view: *mut Widget) -> c_int;
    fn webkit_web_view_go_back(web_view: *mut Widget);
    fn webkit_web_view_can_go_forward(web_view: *mut Widget) -> c_int;
    fn webkit_web_view_go_forward(web_view: *mut Widget);
    fn webkit_web_view_reload(web_view: *mut Widget);
    fn webkit_settings_set_enable_javascript(settings: *mut c_void, enabled: c_int);
    fn webkit_settings_set_enable_webgl(settings: *mut c_void, enabled: c_int);
    fn webkit_settings_set_enable_page_cache(settings: *mut c_void, enabled: c_int);
    fn webkit_settings_set_hardware_acceleration_policy(settings: *mut c_void, policy: c_int);
}

#[link(name = "libgobject-2.0.so.0", modifiers = "+verbatim")]
unsafe extern "C" {
    fn g_signal_connect_data(
        instance: *mut c_void,
        detailed_signal: *const c_char,
        c_handler: Callback,
        data: *mut c_void,
        destroy_data: ClosureNotify,
        connect_flags: c_uint,
    ) -> c_ulong;
}

struct Ui {
    window: *mut Widget,
    address: *mut Widget,
    web_view: *mut Widget,
    status: *mut Widget,
}

pub(crate) fn run() -> Result<(), String> {
    unsafe {
        if gtk_init_check(ptr::null_mut(), ptr::null_mut()) == 0 {
            return Err("non è disponibile una sessione grafica GTK".to_owned());
        }

        let window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
        if window.is_null() {
            return Err("non riesco a creare la finestra".to_owned());
        }
        let title = cstring("Rusty Browser");
        gtk_window_set_title(window, title.as_ptr());
        let icon_name = cstring("rusty-browser");
        gtk_window_set_icon_name(window, icon_name.as_ptr());
        gtk_window_set_default_size(window, 1200, 820);
        gtk_window_set_position(window, 1);

        let root = gtk_box_new(GTK_ORIENTATION_VERTICAL, 0);
        let toolbar = gtk_box_new(GTK_ORIENTATION_HORIZONTAL, 4);
        let back = button("‹", "Indietro");
        let forward = button("›", "Avanti");
        let reload = button("↻", "Ricarica");
        let address = gtk_entry_new();
        let placeholder = cstring("Indirizzo web o ricerca");
        gtk_entry_set_placeholder_text(address, placeholder.as_ptr());
        gtk_widget_set_hexpand(address, 1);
        let go = button("Vai", "Apri indirizzo");

        for widget in [back, forward, reload, address, go] {
            gtk_box_pack_start(toolbar, widget, if widget == address { 1 } else { 0 }, 1, 0);
        }
        for (widget, width) in [(back, 34), (forward, 34), (reload, 34), (go, 48)] {
            gtk_widget_set_size_request(widget, width, 28);
        }
        gtk_widget_set_margin_start(toolbar, 5);
        gtk_widget_set_margin_end(toolbar, 5);
        gtk_widget_set_margin_top(toolbar, 5);
        gtk_widget_set_margin_bottom(toolbar, 5);

        let context = webkit_web_context_get_default();
        if !context.is_null() {
            webkit_web_context_set_cache_model(context, WEBKIT_CACHE_MODEL_DOCUMENT_VIEWER);
        }

        let web_view = webkit_web_view_new();
        if web_view.is_null() {
            return Err("il motore WebKitGTK non è disponibile".to_owned());
        }
        gtk_widget_set_hexpand(web_view, 1);
        gtk_widget_set_vexpand(web_view, 1);

        let status_text = cstring("Pronto — inserisci un indirizzo o una ricerca");
        let status = gtk_label_new(status_text.as_ptr());
        gtk_label_set_xalign(status, 0.0);
        gtk_widget_set_margin_start(status, 10);
        gtk_widget_set_margin_end(status, 10);
        gtk_widget_set_margin_top(status, 5);
        gtk_widget_set_margin_bottom(status, 5);

        gtk_box_pack_start(root, toolbar, 0, 1, 0);
        gtk_box_pack_start(root, web_view, 1, 1, 0);
        gtk_box_pack_start(root, status, 0, 1, 0);
        gtk_container_add(window, root);

        let settings = webkit_web_view_get_settings(web_view);
        if !settings.is_null() {
            webkit_settings_set_enable_javascript(settings, 1);
            webkit_settings_set_enable_webgl(settings, 0);
            webkit_settings_set_enable_page_cache(settings, 0);
            webkit_settings_set_hardware_acceleration_policy(
                settings,
                WEBKIT_HARDWARE_ACCELERATION_POLICY_NEVER,
            );
        }

        let mut ui = Box::new(Ui {
            window,
            address,
            web_view,
            status,
        });
        let user_data = (&mut *ui as *mut Ui).cast::<c_void>();

        connect(window, "destroy", on_destroy as *const (), ptr::null_mut());
        connect(go, "clicked", on_go as *const (), user_data);
        connect(address, "activate", on_go as *const (), user_data);
        connect(back, "clicked", on_back as *const (), user_data);
        connect(forward, "clicked", on_forward as *const (), user_data);
        connect(reload, "clicked", on_reload as *const (), user_data);
        connect(
            web_view,
            "load-changed",
            on_load_changed as *const (),
            user_data,
        );
        connect(
            web_view,
            "load-failed",
            on_load_failed as *const (),
            user_data,
        );

        gtk_widget_show_all(window);
        gtk_widget_grab_focus(address);
        gtk_main();
        drop(ui);
    }
    Ok(())
}

unsafe fn button(label: &str, tooltip: &str) -> *mut Widget {
    let label = cstring(label);
    let button = gtk_button_new_with_label(label.as_ptr());
    let tooltip = cstring(tooltip);
    gtk_widget_set_tooltip_text(button, tooltip.as_ptr());
    button
}

unsafe fn connect(
    instance: *mut Widget,
    signal: &str,
    callback: *const (),
    user_data: *mut c_void,
) {
    let signal = cstring(signal);
    let callback: unsafe extern "C" fn() = mem::transmute(callback);
    g_signal_connect_data(
        instance.cast(),
        signal.as_ptr(),
        Some(callback),
        user_data,
        None,
        0,
    );
}

unsafe extern "C" fn on_destroy(_widget: *mut Widget, _user_data: *mut c_void) {
    gtk_main_quit();
}

unsafe extern "C" fn on_go(_widget: *mut Widget, user_data: *mut c_void) {
    let ui = &mut *user_data.cast::<Ui>();
    let input = gtk_entry_get_text(ui.address);
    if input.is_null() {
        return;
    }
    let input = CStr::from_ptr(input).to_string_lossy().trim().to_owned();
    if input.is_empty() {
        set_status(ui, "Inserisci un indirizzo o una ricerca");
        return;
    }
    let target = make_target(&input);
    let address = cstring(&target);
    gtk_entry_set_text(ui.address, address.as_ptr());
    set_status(ui, &format!("Caricamento di {target}…"));
    webkit_web_view_load_uri(ui.web_view, address.as_ptr());
}

unsafe extern "C" fn on_back(_widget: *mut Widget, user_data: *mut c_void) {
    let ui = &mut *user_data.cast::<Ui>();
    if webkit_web_view_can_go_back(ui.web_view) != 0 {
        webkit_web_view_go_back(ui.web_view);
    }
}

unsafe extern "C" fn on_forward(_widget: *mut Widget, user_data: *mut c_void) {
    let ui = &mut *user_data.cast::<Ui>();
    if webkit_web_view_can_go_forward(ui.web_view) != 0 {
        webkit_web_view_go_forward(ui.web_view);
    }
}

unsafe extern "C" fn on_reload(_widget: *mut Widget, user_data: *mut c_void) {
    let ui = &mut *user_data.cast::<Ui>();
    webkit_web_view_reload(ui.web_view);
}

unsafe extern "C" fn on_load_changed(
    web_view: *mut Widget,
    load_event: c_int,
    user_data: *mut c_void,
) {
    let ui = &mut *user_data.cast::<Ui>();
    if matches!(
        load_event,
        WEBKIT_LOAD_STARTED | WEBKIT_LOAD_REDIRECTED | WEBKIT_LOAD_COMMITTED | WEBKIT_LOAD_FINISHED
    ) {
        let uri = webkit_web_view_get_uri(web_view);
        if !uri.is_null() {
            gtk_entry_set_text(ui.address, uri);
        }
    }
    match load_event {
        WEBKIT_LOAD_STARTED => set_status(ui, "Caricamento pagina…"),
        WEBKIT_LOAD_FINISHED => {
            let page_title = webkit_web_view_get_title(web_view);
            if !page_title.is_null() {
                let page_title = CStr::from_ptr(page_title).to_string_lossy();
                let title = cstring(&format!("{page_title} — Rusty Browser"));
                gtk_window_set_title(ui.window, title.as_ptr());
            }
            set_status(ui, "Pagina caricata");
        }
        _ => {}
    }
}

unsafe extern "C" fn on_load_failed(
    _web_view: *mut Widget,
    _load_event: c_int,
    failing_uri: *const c_char,
    _error: *mut c_void,
    user_data: *mut c_void,
) -> c_int {
    let ui = &mut *user_data.cast::<Ui>();
    let uri = if failing_uri.is_null() {
        "la pagina".to_owned()
    } else {
        CStr::from_ptr(failing_uri).to_string_lossy().into_owned()
    };
    set_status(ui, &format!("Impossibile caricare {uri}"));
    0
}

unsafe fn set_status(ui: &Ui, message: &str) {
    let message = cstring(message);
    gtk_label_set_text(ui.status, message.as_ptr());
}

fn cstring(text: &str) -> CString {
    CString::new(text.replace('\0', "�")).expect("NUL bytes were replaced")
}
