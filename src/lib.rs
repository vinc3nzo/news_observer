#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // warning as errors in release build
#![warn(clippy::all, rust_2018_idioms)]

pub mod news_fetcher;

pub mod news_gui {
    use std::sync::mpsc::{channel, Receiver};
    use std::thread;
    use eframe::epi;
    use eframe::egui::*;
    use serde::{Serialize, Deserialize};
    use crate::news_fetcher::*;

    const APPLICATION_NAME: &str = "news_observer";

    const FOOTER_PADDING: f32 = 5.0;
    const HEADING_PADDING: f32 = 10.0;
    const CONTROLS_PADDING: f32 = 2.5;
    const NEWSCARD_PADDING: f32 = 5.0;
    const NEWSCARD_DARK_TITLE_COLOR: Color32 = Color32::from_rgb(255, 255, 255);
    const NEWSCARD_LIGHT_TITLE_COLOR: Color32 = Color32::from_rgb(10, 10, 10);
    const DARK_HYPERLINK_COLOR: Color32 = Color32::from_rgb(0, 255, 30);
    const LIGHT_HYPERLINK_COLOR: Color32 = Color32::from_rgb(0, 0, 200);
    const DEFAULT_FONT_SIZE: f32 = 18.0;
    const HEADING_FONT_SIZE: f32 = 28.0;
    const SMALL_FONT_SIZE: f32 = 14.0;
    const READ_MORE_SIZE: f32 = 14.0;
    const DESCRIPTION_SIZE: f32 = 16.0;
    const CONTROL_BUTTON_SIZE: f32 = 22.0;
    const RED: Color32 = Color32::from_rgb(200, 0, 0);

    pub struct NewsObserverApp {
        articles: Vec<Article>,
        config: ApplicationConfig,
        light_visuals: Visuals,
        dark_visuals: Visuals,
        newscard_title_color: Color32,
        show_config_window: bool,
        news_rx: Option<Receiver<Vec<Article>>>,
    }

    #[derive(Clone, Serialize, Deserialize)]
    struct ApplicationConfig {
        dark_theme: bool,
        api_key: String,
        country: Country,
    }

    impl Default for ApplicationConfig {
        fn default() -> Self {
            Self {
                dark_theme: true,
                api_key: String::new(),
                country: Country::US,
            }
        }
    }

    impl Default for NewsObserverApp {
        fn default() -> Self {
            let mut light_visuals = Visuals::light();
            light_visuals.hyperlink_color = LIGHT_HYPERLINK_COLOR;
            let mut dark_visuals = Visuals::dark();
            dark_visuals.hyperlink_color = DARK_HYPERLINK_COLOR;

            let config: ApplicationConfig = confy::load(APPLICATION_NAME).unwrap_or_default();

            Self {
                show_config_window: config.api_key.is_empty(),
                articles: vec![],
                config,
                light_visuals,
                dark_visuals,
                newscard_title_color: NEWSCARD_DARK_TITLE_COLOR,
                news_rx: None,
            }
        }
    }

    fn parse_unicode(input: &str) -> Option<char> {
        let unicode = u32::from_str_radix(input, 16).ok()?;
        char::from_u32(unicode)
    }

    impl NewsObserverApp {
        fn save_config(&self) {
            if let Err(e) = confy::store(APPLICATION_NAME, self.config.to_owned()) {
                tracing::error!("Failed to save config onto the disk: {}", e);
            } else {
                tracing::info!("Successfully saved application config onto the disk");
            }
        }

        fn fetch_news(&mut self) {
            let (news_tx, news_rx) = channel();
            self.news_rx = Some(news_rx);
            let api_key = self.config.api_key.to_string();
            let country = self.config.country.clone();
            thread::spawn(move || {
                let a = match get_articles_in_country(&api_key, &country) {
                    Ok(articles) => articles.articles,
                    Err(_e) => vec![],
                };
                if let Err(e) = news_tx.send(a) {
                    tracing::error!("Could not send the news cards between the threads: {}", e);
                }
            });
        }

        fn preload_articles(&mut self) {
            if let Some(rx) = &self.news_rx {
                if let Ok(a) = rx.try_recv() {
                    self.articles = a;
                }
            }
        }

        fn configure_look(&self, ctx: &Context) {
            let mut font_definition = FontDefinitions::default();
            font_definition.font_data.insert("Inter".to_owned(),
                                             FontData::from_static(include_bytes!("../fonts/Inter-Regular.ttf")));
            font_definition.families.get_mut(&FontFamily::Proportional).unwrap()
                .insert(0, "Inter".to_owned());

            let mut app_style = Style::default();
            app_style.text_styles.insert(TextStyle::Small, FontId::proportional(SMALL_FONT_SIZE));
            app_style.text_styles.insert(TextStyle::Body, FontId::proportional(DEFAULT_FONT_SIZE));
            app_style.text_styles.insert(TextStyle::Heading, FontId::proportional(HEADING_FONT_SIZE));

            ctx.set_fonts(font_definition);
            ctx.set_style(app_style);
        }

        fn render_controls(&mut self, ctx: &Context) {
            TopBottomPanel::top("controls").show(ctx, |ui| {
                ui.add_space(CONTROLS_PADDING);
                menu::bar(ui, |ui| {
                    ui.with_layout(Layout::left_to_right(), |ui| {
                        let refresh_button = ui.button(
                            RichText::new(parse_unicode("21ba").unwrap())
                                .size(CONTROL_BUTTON_SIZE - 2.0));
                        if refresh_button.clicked() {
                            self.fetch_news();
                        }

                        let configure_button = ui.button(
                            RichText::new(parse_unicode("1f527").unwrap())
                                .size(CONTROL_BUTTON_SIZE)
                        );
                        if configure_button.clicked() {
                            self.show_config_window = !self.show_config_window;
                        }
                    });
                    ui.with_layout(Layout::right_to_left(), |ui| {
                        let theme_button = ui.button(
                            RichText::new(parse_unicode(
                                if self.config.dark_theme { "2600" } else { "1f318" }
                            ).unwrap())
                                .size(CONTROL_BUTTON_SIZE));
                        if theme_button.clicked() {
                            self.config.dark_theme = !self.config.dark_theme;
                            self.save_config();
                        }

                        let country_button = ui.button(
                            RichText::new(resolve_country(&self.config.country))
                                .size(CONTROL_BUTTON_SIZE)
                        );
                        if country_button.clicked() {
                            self.config.country = match &self.config.country {
                                Country::US => Country::RU,
                                Country::RU => Country::US,
                            };
                            self.fetch_news();
                            self.save_config();
                        }
                    });
                });
                ui.add_space(CONTROLS_PADDING);
            });
        }

        fn render_news_cards(&self, ui: &mut Ui) {
            if self.articles.is_empty() {
                ui.add_space(NEWSCARD_PADDING);
                ui.colored_label(RED, RichText::new(String::from("There is nothing to show you.\n") +
                    "1) Check your API key setting in the Configuration window. (Click on the wrench in the top-left corner of the window.) " +
                        "After that, don't forget to refresh the news cards by pressing the refresh button!\n" +
                    "2) If the key is valid, but you're still seeing this message, probably, the News API servers are down. Try again later.")
                    .size(DESCRIPTION_SIZE));
                ui.add_space(NEWSCARD_PADDING);
                ui.separator();
            }
            for a in &self.articles {
                ui.add_space(NEWSCARD_PADDING);
                ui.colored_label(self.newscard_title_color, format!("▶ {}", a.title));
                ui.add_space(NEWSCARD_PADDING);
                ui.label(RichText::new(a.description.as_ref().map(|s| s.to_string()).unwrap_or("...".to_string()))
                    .size(DESCRIPTION_SIZE));
                ui.add_space(NEWSCARD_PADDING);
                let right_to_left_layout = Layout::right_to_left().with_cross_align(Align::TOP);
                ui.with_layout(right_to_left_layout, |ui| {
                    ui.hyperlink_to(RichText::new("MORE ↪").size(READ_MORE_SIZE), &a.url);
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
            ui.vertical_centered_justified(|ui| {
                ui.add_space(FOOTER_PADDING);
                ui.label(
                    RichText::new(
                        "This is a so-called \"pet application\" created by me to complete a particular university assignment. ".to_owned() +
                            "Do note though, that the idea behind this application is not entirely mine. " +
                            "It is based on a creativcoder's sample Rust GUI project for his YouTube series and has some " +
                            "major code improvements made by me.")
                        .small());
                ui.add_space(FOOTER_PADDING);
                ui.hyperlink_to(RichText::new("The news are provided by the News API.").size(READ_MORE_SIZE),
                                "https://newsapi.org");
                ui.hyperlink_to(RichText::new("The GUI was made using the eframe Rust crate.").size(READ_MORE_SIZE),
                                "https://github.com/emilk/egui/tree/master/eframe");
                ui.hyperlink_to(RichText::new("Sources are available at GitHub.").size(READ_MORE_SIZE),
                                "https://github.com/vinc3nzo/news_observer");
                ui.add_space(FOOTER_PADDING);
                warn_if_debug_build(ui);
            });
        }

        fn render_config_window(&mut self, ctx: &Context) {
            Window::new("Configuration")
                .collapsible(false).show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label("Enter your key for the News API and press Enter:");
                    let key_input = ui.text_edit_singleline(&mut self.config.api_key);
                    if key_input.lost_focus() && ui.input().key_pressed(Key::Enter) {
                        self.show_config_window = false;
                        self.fetch_news();
                        if let Err(e) = confy::store(APPLICATION_NAME, self.config.to_owned()) {
                            tracing::error!("Failed to write the config onto the disk: {}", e);
                        } else {
                            tracing::info!("Successfully wrote config onto the disk");
                        }
                    }
                    ui.label("If you don't have it, you can register one at");
                    ui.hyperlink("https://newsapi.org");
                });
            });
        }
    }

    impl epi::App for NewsObserverApp {
        fn name(&self) -> &str {
            "News Observer"
        }

        fn setup(&mut self, ctx: &Context, _frame: &epi::Frame, _storage: Option<&dyn epi::Storage>) {
            self.configure_look(ctx);
            self.fetch_news();
        }

        fn update(&mut self, ctx: &Context, _frame: &epi::Frame) {
            ctx.request_repaint();

            if self.config.dark_theme {
                ctx.set_visuals(self.dark_visuals.to_owned());
                self.newscard_title_color = NEWSCARD_DARK_TITLE_COLOR;
            } else {
                ctx.set_visuals(self.light_visuals.to_owned());
                self.newscard_title_color = NEWSCARD_LIGHT_TITLE_COLOR;
            }

            if self.show_config_window {
                self.render_config_window(ctx);
            }
            self.preload_articles();
            self.render_controls(ctx);
            CentralPanel::default().show(ctx, |ui| {
                self.render_header(ui);
                ScrollArea::both().show(ui, |ui| {
                    self.render_news_cards(ui);
                    self.render_footer(ui);
                });
            });
        }
    }
}