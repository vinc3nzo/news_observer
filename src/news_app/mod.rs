pub mod entities;

use eframe::egui;
use eframe::epi;
use entities::Article;

pub struct NewsObserverApp {
    label: String,
    articles: Vec<Article>,
}

impl Default for NewsObserverApp {
    fn default() -> Self {
        Self {
            label: "News Observer".to_owned(),
            articles: vec![],
        }
    }
}

impl epi::App for NewsObserverApp {
    fn name(&self) -> &str {
        "Top Headlines"
    }

    fn setup(&mut self, _ctx: &egui::Context, _frame: &epi::Frame, _storage: Option<&dyn epi::Storage>) {
    }

    fn save(&mut self, _storage: &mut dyn epi::Storage) {
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        let Self { label, articles } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("News");
            ui.vertical_centered_justified(|ui| {
                ui.label("Article #1");
                ui.label("Article #2");
                ui.label("Article #3");
                ui.label("Article #4");
            });
            egui::warn_if_debug_build(ui);
        });
    }
}
