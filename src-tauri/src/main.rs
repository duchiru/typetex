// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
	let mut builder = tauri::Builder::default();

	builder = builder.setup(|app| {
		let mut main_window_builder =
			tauri::WebviewWindowBuilder::new(app, "main", tauri::WebviewUrl::default());

		main_window_builder = main_window_builder
			.title("TypeTeX")
			.inner_size(800.0, 600.0)
			.fullscreen(false)
			.resizable(true)
			.visible(false);

		#[cfg(target_os = "windows")]
		{
			main_window_builder = main_window_builder.decorations(false).shadow(true);
		}

		match main_window_builder.build() {
			Ok(window) => {
				let _ = window.set_focus();
			}
			Err(err) => {
				// GUI is the only way to interact with TypeTeX, so we must panic if it fails
				panic!("TypeTeX could not initialize application GUI: {}", err);
			}
		}

		Ok(())
	});

	match builder.build(tauri::generate_context!()) {
		Ok(app) => {
			app.run(|_app_handle, _event| {});
		}
		Err(err) => {
			panic!("TypeTeX ran into an error while starting: {}", err);
		}
	}
}
