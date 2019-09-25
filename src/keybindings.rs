/// Map key bindings to events in GTK applications
///
/// This macro creates a `gio::SimpleAction` for each defined event, adds that action to
/// a given `gtk::Application`, and then associates that action with a specified slice of
/// possible key binding accelerators.
///
/// See [gdk::enums::key](https://gtk-rs.org/docs/gdk/enums/key/index.html) for a list of
/// supported keys.
///
/// # Example
///
/// Where `application` is a `&gtk::Application`, and `sender` is a `&glib::Sender<T>`:
///
/// ```rust
/// use gio::prelude::*;
/// use gtk::prelude::*;
/// use gtk_extras::keybindings;
///
/// const APP_ID: &str = "org.Organization.App";
///
/// enum UiEvent {
///     Back,
///     Quit,
///     StackNext,
///     StackPrev
/// }
///
/// let app_flags = gio::ApplicationFlags::empty();
/// let application = gtk::Application::new(APP_ID.into(), app_flags).unwrap();
///
/// application.connect_startup(|app| {
///     let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
///
///     keybindings!((app, &sender) {
///         "back" => (UiEvent::Back, &["<Primary>BackSpace"]),
///         "quit" => (UiEvent::Quit, &["<Primary>Q"]),
///         "stkn" => (UiEvent::StackNext, &["<Primary>Right"]),
///         "stkp" => (UiEvent::StackPrev, &["<Primary>Left"]),
///     });
///
///     receiver.attach(None, move |event| {
///         match event {
///             UiEvent::Back => (),
///             UiEvent::Quit => {
///                 // Destroy main window here.
///                 return glib::Continue(false);
///             },
///             UiEvent::StackNext => (),
///             UiEvent::StackPrev => (),
///         }
///
///         glib::Continue(true)
///     });
/// });
///
/// application.run(&[]);
/// ```
#[macro_export]
macro_rules! keybindings {
    (($app:expr, $sender:expr) { $( $name:expr => ($value:expr, $keys:expr) ),+ $(,)? }) => (
        use gio::ActionMapExt;

        $({
            let action = gio::SimpleAction::new($name, None);

            let sender = $sender.clone();
            action.connect_activate(move |_, _| {
                let _ = sender.send($value);
            });

            $app.add_action(&action);
            $app.set_accels_for_action(concat!("app.", $name), $keys);
        })+
    );
}
