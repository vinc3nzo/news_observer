#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // warning as errors in release build
#![warn(clippy::all, rust_2018_idioms)]

use eframe::NativeOptions;
use eframe::epi;
use eframe::egui::*;

const FOOTER_PADDING: f32 = 5.0;
const HEADING_PADDING: f32 = 10.0;
const NEWSCARD_PADDING: f32 = 5.0;
const TITLE_COLOR: Color32 = Color32::from_rgb(255, 255, 255);
const READ_MORE_COLOR: Color32 = Color32::from_rgb(0, 255, 30);
const DEFAULT_FONT_SIZE: f32 = 18.0;
const HEADING_FONT_SIZE: f32 = 32.0;
const SMALL_FONT_SIZE: f32 = 14.0;
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
    fn configure_look(&self, ctx: &Context) {
        let mut font_definition = FontDefinitions::default();
        font_definition.font_data.insert("ComicMono".to_owned(),
                                         FontData::from_static(include_bytes!("../fonts/ComicMono.ttf")));
        font_definition.families.get_mut(&FontFamily::Proportional).unwrap()
            .insert(0, "ComicMono".to_owned());

        let mut app_style = Style::default();
        app_style.text_styles.insert(TextStyle::Small, FontId::proportional(SMALL_FONT_SIZE));
        app_style.text_styles.insert(TextStyle::Body, FontId::proportional(DEFAULT_FONT_SIZE));
        app_style.text_styles.insert(TextStyle::Heading, FontId::proportional(HEADING_FONT_SIZE));
        app_style.visuals.hyperlink_color = READ_MORE_COLOR;

        ctx.set_fonts(font_definition);
        ctx.set_style(app_style);
    }

    fn render_news_cards(&self, ui: &mut Ui) {
        for a in &self.headlines {
            ui.add_space(NEWSCARD_PADDING);
            ui.colored_label(TITLE_COLOR, format!("▶ {}", a.title));
            ui.add_space(NEWSCARD_PADDING);
            ui.label(RichText::new(&a.description).size(DESCRIPTION_SIZE));
            ui.add_space(NEWSCARD_PADDING);
            let right_to_left_layout = Layout::right_to_left().with_cross_align(Align::TOP);
            ui.with_layout(right_to_left_layout, |ui| {
                ui.hyperlink_to(RichText::new("MORE ↪").size(READ_MORE_SIZE), &a.link);
            });
            ui.add_space(NEWSCARD_PADDING);
            ui.separator();
        }
    }

    fn render_header(&self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("🔥Top News🔥");
        });
        ui.add_space(HEADING_PADDING);
        ui.separator();
    }

    fn render_footer(&self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(FOOTER_PADDING);
            ui.label(
                RichText::new(
                    "This is a \"pet application\" created by me to complete a particular university assignment. ".to_owned() +
                        "Do note though, that the idea behind this application is not entirely mine. " +
                        "It is based on a creativcoder's sample Rust GUI project for his YouTube series and has some " +
                        "major code improvements made by me.")
                    .small());
            ui.hyperlink_to(RichText::new("The news are provided by the News API.").size(READ_MORE_SIZE),
                            "https://newsapi.org");
            ui.hyperlink_to(RichText::new("The GUI was made using the eframe Rust crate.").size(READ_MORE_SIZE),
                            "https://github.com/emilk/egui/tree/master/eframe");
            ui.hyperlink_to(RichText::new("Sources are available at GitHub.").size(READ_MORE_SIZE),
                            "https://github.com/vinc3nzo/news_observer");
            warn_if_debug_build(ui);
        });
    }
}

impl epi::App for NewsObserverApp {
    fn name(&self) -> &str {
        "The News Observer"
    }

    fn setup(&mut self, ctx: &Context, _frame: &epi::Frame, _storage: Option<&dyn epi::Storage>) {
        self.configure_look(ctx);
    }

    fn save(&mut self, _storage: &mut dyn epi::Storage) {
    }

    fn update(&mut self, ctx: &Context, _frame: &epi::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.render_header(ui);
            ScrollArea::both().show(ui, |ui| {
                self.render_news_cards(ui);
                self.render_footer(ui);
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
