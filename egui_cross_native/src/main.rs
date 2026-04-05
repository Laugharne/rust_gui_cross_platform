use eframe::egui;

fn main() -> eframe::Result {
	let options = eframe::NativeOptions::default();

	eframe::run_native(
		"egui App",
		options,
		Box::new(|cc| {
			cc.egui_ctx.set_visuals(egui::Visuals::dark());
			Ok(Box::new(MyApp::default()))
		}),
	)
}

#[derive(Default)]
struct MyApp {
	name: String,
	age: u32,
	counter: i32,
}

impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("Welcome to egui!");

			ui.separator();

			ui.horizontal(|ui| {
				ui.label("Your name: ");
				ui.text_edit_singleline(&mut self.name);
			});

			ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));

			if ui.button("Increment Counter").clicked() {
				self.counter += 1;
			}

			ui.label(format!("Counter: {}", self.counter));

			ui.separator();

			if !self.name.is_empty() {
				ui.label(format!("Hello, {}! You are {} years old.", self.name, self.age));
			}
		});
	}
}