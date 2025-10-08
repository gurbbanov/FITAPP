use egui::Button;
use egui::{epaint::Rounding, vec2, Color32, Context, ImageButton, LayerId, SelectableLabel};
use crate::ui::Gui;
use crate::models::AppMedia;

pub struct AppRuntime<'a> {
    ui: Gui<'a>,
    // medias: AppMedia<'a>,
}

impl AppRuntime<'_> {
    pub fn new(ctx: &Context) -> Self {
        Self {
            ui: Gui::init(ctx),
            // medias: AppMedia::load_media(ctx)
        }
    }
}

impl eframe::App for AppRuntime<'_> {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        let is_dark = ctx.style().visuals.dark_mode;

        let bg_color = if is_dark {
            Color32::from_rgb(20, 20, 20)
        } else {
            Color32::from_rgb(240, 240, 240)
        };

        let elements_color = if is_dark {
            Color32::from_rgb(50, 100, 200)
        } else {
            Color32::from_rgb(200, 100, 50)
        };

        ctx.layer_painter(LayerId::background())
            .rect_filled(ctx.screen_rect(), Rounding::ZERO, bg_color);

        egui_extras::install_image_loaders(ctx);

        egui::CentralPanel::default()
            .frame(
                egui::Frame::NONE,
            )
            .show(ctx, |ui| {
                self.ui.navigation_bar(ctx, frame, ui);
            });

        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .min_height(50.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.add_space(5.0);
                    ui.horizontal(|ui| {
                //     ui.set_width(ui.available_width());

                //     let button_count = 6;
                //     let button_width = 85.0;
                //     let total_buttons_width = button_count as f32 * button_width;
                //     let left_padding =
                //         (ui.available_width() - total_buttons_width).max(0.0) / 2.0 - 20.0;

                //     ui.add_space(left_padding);

                //     let labels = ["Home", "Friends", "Workouts", "Calories", "Water", "Statistics"];

                //     for (i, label) in labels.iter().enumerate() {
                //         let selected = self.ui.states.selected_tab == i;

                //         let resp = ui.allocate_ui_with_layout(
                //             vec2(button_width, 70.0), 
                //             egui::Layout::top_down(egui::Align::Center),
                //             |ui| {
                //                 ui.vertical_centered(|ui| {
                //                     // let img_size = vec2(32.0, 32.0);
                //                     ui.image(self.ui.medias.plus.clone());

                //                     ui.label(
                //                         egui::RichText::new(*label)
                //                             .size(12.0)
                //                             .strong()
                //                             .color(if selected {
                //                                 egui::Color32::WHITE
                //                             } else {
                //                                 egui::Color32::GRAY
                //                             }),
                //                     );
                //                 });
                //             },
                //         );

                //         if resp.response.clicked() {
                //             self.ui.states.selected_tab = i;
                //         }

                //         if selected {
                //             let rect = resp.response.rect;
                //             ui.painter().rect_stroke(
                //                 rect,
                //                 6.0,
                //                 egui::Stroke::new(2.0, egui::Color32::LIGHT_BLUE),
                //                 egui::StrokeKind::Inside,
                //             );
                //         }
                //     }
                // });

                // ui.horizontal(|ui| {
                        ui.set_width(ui.available_width());

                        let button_count = 6;
                        let button_width = 85.0;
                        let total_buttons_width = button_count as f32 * button_width;
                        let left_padding = (ui.available_width() - total_buttons_width).max(0.0) / 2.0 - 20.0;

                        ui.add_space(left_padding);

                        let icons = [&self.ui.medias.home, &self.ui.medias.friends, &self.ui.medias.workouts, &self.ui.medias.calories, &self.ui.medias.water, &self.ui.medias.statistics];
                        for (i, img) in icons.iter().enumerate() {
                            let selected = self.ui.states.selected_tab == i;
                            // let button = SelectableLabel::new(selected, *label);
                            // let button = Button::image_and_text(self.medias.plus.clone(), String::from("home"))
                            //     .selected(selected);
                                // .frame(false);
                                // .tint(if selected { elements_color } else { Color32::GRAY });
                            
                            // let button = egui::Image::new(label);
                            let button = ImageButton::new(img.clone().clone())
                                // .selected(selected)
                                .frame(false)
                                .tint(if selected { Color32::from_rgb(0, 102, 190)} else {if is_dark {Color32::LIGHT_GRAY} else {Color32::DARK_GRAY}});

                            if ui.add_sized(vec2(button_width, 35.0), button).clicked() {
                                self.ui.states.selected_tab = i;
                            }
                        }
                    });
                }); 
            });
    }
}