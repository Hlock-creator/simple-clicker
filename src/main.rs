use eframe::egui;

fn main() -> eframe::Result<()> {
    // создаем опции окна
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 300.0]), // РАЗМЕРЫ ОКНА
        ..Default::default()
    };

    // ЗАПУСК ПРИЛОЖЕНИЯ
    eframe::run_native(
        "gui_window",                               // ИДЕНТИФИКАТОР ПРИЛОЖУХИ
        options,                                    // ОПЦИИ
        Box::new(|_cc| Ok(Box::new(MyApp::new()))), // СОЗДАЕМ ЭКЗЕМПЛЯР ПРИЛОЖЕНИЯ
    )
}

struct MyApp {
    piska: i64,
}

impl MyApp {
    fn new() -> Self {
        Self { piska: 0 }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.6);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(format!("Clicked: {}", self.piska));
                if ui.button("click").clicked() {
                    self.piska += 1;
                }
            });
        });
    }
}
