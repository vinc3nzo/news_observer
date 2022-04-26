#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // warning as errors in release build
#![warn(clippy::all, rust_2018_idioms)]

use eframe::NativeOptions;
use eframe::egui;
use eframe::epi;
use eframe::egui::*;

const NEWSCARD_PADDING: f32 = 5.0;
const TITLE_COLOR: Color32 = Color32::from_rgb(255, 255, 255);
const READ_MORE_COLOR: Color32 = Color32::from_rgb(0, 255, 30);
const DEFAULT_FONT_SIZE: f32 = 18.0;
const HEADING_FONT_SIZE: f32 = 32.0;
const READ_MORE_SIZE: f32 = 14.0;
const DESCRIPTION_SIZE: f32 = 16.0;

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
        let iter = (1..21).map(|a| Headline {
            title: format!("Title {}", a),
            link: format!("https://example.com/article-{}", a),
            description: format!("description_{}", a),
        });

        Self {
            headlines: Vec::from_iter(iter),
        }
    }
}

impl NewsObserverApp {
    fn configure_look(&self, ctx: &egui::Context) {
        let mut font_definition = FontDefinitions::default();
        font_definition.font_data.insert("InterRegular".to_owned(), FontData::from_static(include_bytes!("../fonts/Inter-Regular.ttf")));
        font_definition.families.get_mut(&FontFamily::Proportional).unwrap()
            .insert(0, "InterRegular".to_owned());

        let mut app_style = Style::default();
        app_style.text_styles.insert(TextStyle::Body, FontId::proportional(DEFAULT_FONT_SIZE));
        app_style.text_styles.insert(TextStyle::Heading, FontId::proportional(HEADING_FONT_SIZE));
        app_style.visuals.hyperlink_color = READ_MORE_COLOR;

        ctx.set_fonts(font_definition);
        ctx.set_style(app_style);
    }

    fn render_news_cards(&self, ui: &mut eframe::egui::Ui) {
        for a in &self.headlines {
            ui.add_space(NEWSCARD_PADDING);
            ui.colored_label(TITLE_COLOR, format!("▶ {}", a.title));
            ui.add_space(NEWSCARD_PADDING);
            ui.style_mut().text_styles.insert(TextStyle::Body, FontId::proportional(DESCRIPTION_SIZE));
            ui.label(&a.description);
            ui.style_mut().text_styles.insert(TextStyle::Body, FontId::proportional(DEFAULT_FONT_SIZE));
            ui.add_space(NEWSCARD_PADDING);
            ui.with_layout(Layout::right_to_left(), |ui| {
                ui.style_mut().text_styles.insert(TextStyle::Body, FontId::proportional(READ_MORE_SIZE));
                ui.hyperlink_to("MORE ↪", &a.link);
                ui.style_mut().text_styles.insert(TextStyle::Body, FontId::proportional(DEFAULT_FONT_SIZE));
            });
            ui.add_space(NEWSCARD_PADDING);
            ui.add(Separator::default());
        }
    }
}

impl epi::App for NewsObserverApp {
    fn name(&self) -> &str {
        "Top Headlines"
    }

    fn setup(&mut self, ctx: &egui::Context, _frame: &epi::Frame, _storage: Option<&dyn epi::Storage>) {
        self.configure_look(ctx);
    }

    fn save(&mut self, _storage: &mut dyn epi::Storage) {
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::both().show(ui, |ui| {
                self.render_news_cards(ui);
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
