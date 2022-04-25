#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // warning as errors in release build
#![warn(clippy::all, rust_2018_idioms)]

use eframe::NativeOptions;
use eframe::egui;
use eframe::epi;
use eframe::egui::*;

pub struct Headline {
    title: String,
    link: String,
    description: String,
}

pub struct NewsObserverApp {
    headlines: Vec<Headline>,
}

impl Default for NewsObserverApp {
    fn default() -> Self {
        let iter = (0..20).map(|a| Headline {
            title: format!("title-{}", a),
            link: format!("https://example.com/article-{}", a),
            description: format!("description-{}", a),
        });

        Self {
            headlines: Vec::from_iter(iter),
        }
    }
}

impl NewsObserverApp {
    fn configure_fonts(&self, ctx: &egui::Context) {
        let mut font_definition = FontDefinitions::default();
        font_definition.font_data.insert("InterRegular".to_owned(), FontData::from_static(include_bytes!("../fonts/Inter-Regular.ttf")));
        font_definition.families.get_mut(&FontFamily::Proportional).unwrap()
            .insert(0, "InterRegular".to_owned());

        let mut app_style = Style::default();
        app_style.text_styles.insert(TextStyle::Body, FontId::proportional(18.0));
        app_style.text_styles.insert(TextStyle::Heading, FontId::proportional(32.0));
        ctx.set_fonts(font_definition);
        ctx.set_style(app_style);
    }
}

impl epi::App for NewsObserverApp {
    fn name(&self) -> &str {
        "Top Headlines"
    }

    fn setup(&mut self, ctx: &egui::Context, _frame: &epi::Frame, _storage: Option<&dyn epi::Storage>) {
        self.configure_fonts(ctx);
    }

    fn save(&mut self, _storage: &mut dyn epi::Storage) {
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::both().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("News");
                });
                for a in &self.headlines {
                    ui.label(&a.title);
                    ui.label(&a.description);
                    ui.label(&a.link);
                }

                warn_if_debug_build(ui);
            });
        });
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = NewsObserverApp::default();
    let native_options = NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
