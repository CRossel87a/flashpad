use eframe::egui;
use std::time::{Duration, Instant};
use std::sync::Arc;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    
    eframe::run_native(
        "FlashPad",
        options,
        Box::new(|cc| {
            // Configure the font
            let mut fonts = egui::FontDefinitions::default();
            fonts.font_data.insert(
                "consolas".to_owned(),
                Arc::new(egui::FontData::from_static(include_bytes!("C:\\Windows\\Fonts\\consola.ttf"))),
            );
            fonts.families
                .get_mut(&egui::FontFamily::Monospace)
                .unwrap()
                .insert(0, "consolas".to_owned());
            cc.egui_ctx.set_fonts(fonts);

            Ok(Box::new(MyApp::new(cc)))
        }),
    )
}

struct MyApp {
    text: String,
    start_time: Instant,
    lifetime: Duration,
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            text: String::new(),
            start_time: Instant::now(),
            lifetime: Duration::from_secs(12 * 60 * 60), // 12 hours
        }
    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            text: String::new(),
            start_time: Instant::now(),
            lifetime: Duration::from_secs(12 * 60 * 60), // 12 hours
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check if we should close
        if self.start_time.elapsed() >= self.lifetime {
            std::process::exit(0);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // Display remaining time and reset button in a horizontal layout
            ui.horizontal(|ui| {
                let remaining = self.lifetime - self.start_time.elapsed();
                let hours = remaining.as_secs() / 3600;
                let minutes = (remaining.as_secs() % 3600) / 60;
                let seconds = remaining.as_secs() % 60;
                ui.label(format!("Remaining time: {:02}:{:02}:{:02}", hours, minutes, seconds));
                
                if ui.button("+12h").clicked() {
                    self.lifetime += Duration::from_secs(12 * 60 * 60);
                }
            });
            
            // Calculate number of rows based on available height
            let available_height = ui.available_height();
            let row_height = ui.text_style_height(&egui::TextStyle::Monospace);
            let rows = (available_height / row_height).round() as usize;
            
            // Create a scrollable text edit that fills the available space
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::TextEdit::multiline(&mut self.text)
                    .desired_width(f32::INFINITY)
                    .desired_rows(rows)
                    .font(egui::FontId::new(14.0, egui::FontFamily::Monospace))
                    .show(ui);
            });
        });
    }
}
