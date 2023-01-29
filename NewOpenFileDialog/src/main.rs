use gtk::FileChooserNative;
use gtk::ResponseType;
use gtk::prelude::*;
use gtk::glib;
use gtk::Application;
use gtk::ApplicationWindow;
use gtk::Button;

fn
build_ui
(
	app: &Application
)
{
	let open_button = Button::builder()
		.label("Open...")
		.margin_top(12)
		.margin_bottom(12)
		.margin_start(12)
		.margin_end(12)
		.build();


	let window = ApplicationWindow::builder()
		.application(app)
		.title("NewOpenFileDialog")
		.child(&open_button)
		.build();

	// Sources:
	// - https://discourse.gnome.org/t/gtk-rs-gtk4-filechoosernative-tutorial/8278
	// - https://github.com/gtk-rs/gtk4-rs/blob/55724488e76b512ac8388fa3d4eee5ab4ac12a76/examples/video_player/video_player_window/imp.rs#L55
	// - https://gtk-rs.org/gtk3-rs/git/docs/gtk/struct.FileChooserNative.html
	// - https://docs.gtk.org/gtk3/class.FileChooserNative.html
	open_button.connect_clicked(glib::clone!(@weak window => move |_|
	{
		println!("Open button pressed");
		let file_chooser = gtk::FileChooserNative::new(
			Some("Open File"),
			Some(&window),
			gtk::FileChooserAction::Open,
			Some("Open"),
			Some("Cancel"),
		);
		file_chooser.set_modal(true);

		// file_chooser.connect_response(handle_file_chooser_selection);
		file_chooser.connect_response(
			glib::clone!(
				@weak window => move |file_chooser, response| handle_file_chooser_selection(file_chooser, response)
			)
		);
		
		file_chooser.show();
	}
	));

	window.present();
}

fn
handle_file_chooser_selection
(
	file_chooser: &FileChooserNative,
	response: ResponseType
)
{
	if response == gtk::ResponseType::Accept
	{
		println!("{:?}", file_chooser.file().unwrap().path().unwrap())
	}
	file_chooser.destroy();
}

fn main() 
{
	let app = Application::builder()
		.application_id("com.tp.rust.gtk.test.NewOpenFileDialog")
		.build();

	app.connect_activate(build_ui);
	app.run();
}
