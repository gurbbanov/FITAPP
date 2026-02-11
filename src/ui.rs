use eframe::{Frame};
use egui::{CornerRadius, TextEdit, Layout, Context, ColorImage, ImageSource, ScrollArea, Ui, Image, Color32, TextStyle, RichText, Align, Vec2, Rounding, Label, Button, vec2, ImageButton, Rect, Pos2, scroll_area::ScrollBarVisibility, Stroke, StrokeKind, FontFamily, FontId, Style, CursorIcon, Sense, Id, Window, Area, Order, LayerId};
use egui_extras::{Size, Strip, StripBuilder};
use time::{OffsetDateTime};
use chrono::{DateTime, Datelike, Duration, Local, NaiveDate};
use strum::IntoEnumIterator;

use crate::models::{AppMedia, States, Summary, UserDataPack, WorkoutPlanned, WorkoutPlannedData, WorkoutTemplate, Muscle, Exercises};
use crate::muscles::{workout_tracker_widget_front, workout_tracker_widget_behind};
use crate::tools::weekday_iso;

const REMAINDER: f32 = 10.0;

pub struct Gui<'a> {
    pub datas: UserDataPack,
    pub medias: AppMedia<'a>,
    pub states: States,
}

impl Gui<'_> {
    pub fn init(ctx: &Context) -> Self {
        Self {
            datas: UserDataPack::default(),
            medias: AppMedia::load_media(ctx),
            states: States::default(),
        }
    }

    pub fn home(&mut self, ctx: &Context, frame: &mut Frame, ui: &mut Ui, tint_color: Color32) {
        let available_width = ui.available_width();
        // let is_dark = ctx.style().visuals.dark_mode;

        let spacing = 3.0;
        let rect_size = 10.0;
        let calory_rows = 5;
        let calory_cols = 25;
        let calory_percent = ((self.datas.macro_data.calory_registered as f32 / self.datas.macro_data.calory_goal as f32) * 100.0) as u32;
        let calory_tracker_width = (rect_size * calory_cols as f32) + (spacing * (calory_cols as f32 - 1.0));

        let circle_size = 14.0;
        let water_rows = 2;
        let water_cols = 20;
        let water_percent = ((self.datas.water_data.water_registered as f32 / self.datas.water_data.water_goal as f32) * 100.0) as u32;
        let water_tracker_width = (circle_size * water_cols as f32) + (spacing * (water_cols as f32 - 1.0));

        StripBuilder::new(ui)
            .size(Size::exact(150.0))
            .size(Size::exact(40.0))
            .size(Size::exact(30.0))
            .size(Size::exact(170.0))
            .size(Size::exact(150.0))
            .size(Size::remainder())
            .vertical(|mut strip| {
                strip.cell(|ui| {
                    StripBuilder::new(ui)
                        .size(Size::relative(0.39))
                        .size(Size::relative(0.2))
                        .size(Size::relative(0.4))
                        .horizontal(|mut strip| {
                            strip.cell(|ui| {
                                // ui.painter().rect_filled(
                                //     ui.available_rect_before_wrap(),
                                //     0.0,
                                //     (Color32::BLUE),
                                // );
                            });

                            strip.cell(|ui| {
                                    ui.vertical_centered(|ui| {
                                        ui.add_sized([100.0, 100.0], Image::new(self.medias.default_pp.clone()).corner_radius(5.0));
                                        ui.label(RichText::new(format!("{}", self.datas.user_information.name)).size(20.0).strong());
                                        ui.label(RichText::new(format!("@{}", self.datas.user_information.username)).size(15.0));
                                    });
                            });

                            strip.cell(|ui| {
                                // ui.painter().rect_filled(
                                //     ui.available_rect_before_wrap(),
                                //     0.0,
                                //     (Color32::BLUE),
                                // );
                            });
                        });
                });

                strip.cell(|ui| {
                    StripBuilder::new(ui)
                        .size(Size::relative(0.2))
                        .size(Size::remainder())
                        .size(Size::relative(0.2))
                        .horizontal(|mut strip| {
                            strip.cell(|ui| {
                                ui.add_space(ui.available_height() / 4.0);
                                ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                                    ui.label(RichText::new("14 lev").size(14.0).strong());
                                });
                            });

                            strip.cell(|ui| {
                                ui.add_space(ui.available_height() / 4.0);
                                ui.vertical_centered(|ui| {
                                    ui.add_sized(vec2(ui.available_width(), 10.0), egui::ProgressBar::new(0.0).show_percentage());
                                });
                            });

                            strip.cell(|ui| {
                                ui.add_space(ui.available_height() / 4.0);
                                ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                                    ui.label(RichText::new("1300/2400").size(14.0).strong());
                                });
                            });
                        });
                });

                strip.cell(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.label(RichText::new("calory tracker").size(17.5).strong());
                        ui.add_space(REMAINDER);
                        ui.label(RichText::new(format!("{} %", &calory_percent)).size(18.0));
                    });
                });

                strip.cell(|ui| {
                    StripBuilder::new(ui)
                        .size(Size::relative(0.18))
                        .size(Size::remainder())
                        .size(Size::exact(calory_tracker_width))
                        .size(Size::remainder())
                        .size(Size::relative(0.22))
                        .horizontal(|mut strip| {
                            strip.cell(|ui| {
                                ui.vertical(|ui| {
                                    ui.vertical_centered(|ui| {
                                        ui.label(RichText::new("REGISTERED").size(15.0).strong());
                                        ui.add_space(5.0);
                                        ui.label(RichText::new(format!("proteins: {}", self.datas.macro_data.protein_registered)).size(13.0));
                                        ui.add_space(5.0);
                                        ui.label(RichText::new(format!("carbs: {}", self.datas.macro_data.carb_registered)).size(13.0));
                                        ui.add_space(5.0);
                                        ui.label(RichText::new(format!("fats: {}", self.datas.macro_data.fat_registered)).size(13.0));
                                    })
                                });
                            });

                            strip.empty();

                            strip.cell(|ui| {
                                ui.spacing_mut().item_spacing = vec2(1.0, -3.0);

                                ui.vertical_centered(|ui| {
                                    ui.add_space(REMAINDER * 2.0);
                                    self.calory_tracker_bar(ctx, frame, ui, spacing, rect_size, calory_rows, calory_cols, calory_percent);
                                });
                            });

                            strip.empty();

                            strip.cell(|ui| {
                                ui.vertical_centered(|ui| {
                                    ui.label(RichText::new("REMAINS").size(15.0).strong());
                                    ui.add_space(5.0);
                                    ui.label(RichText::new(format!(
                                        "proteins: {}",
                                        self.datas.macro_data.protein_goal.saturating_sub(self.datas.macro_data.protein_registered)
                                    )).size(13.0));
                                    ui.add_space(5.0);
                                    ui.label(RichText::new(format!(
                                        "carbs: {}",
                                        self.datas.macro_data.carb_goal.saturating_sub(self.datas.macro_data.carb_registered)
                                    )).size(13.0));
                                    ui.add_space(5.0);
                                    ui.label(RichText::new(format!(
                                        "fats: {}",
                                        self.datas.macro_data.fat_goal.saturating_sub(self.datas.macro_data.fat_registered)
                                    )).size(13.0));
                                });
                            });
                        });
                    });

                strip.cell(|ui| {
                    StripBuilder::new(ui)
                        .size(Size::remainder())
                        .size(Size::exact(water_tracker_width))
                        .size(Size::remainder())
                        .horizontal(|mut strip| {
                            strip.empty();

                            strip.cell(|ui| {
                                ui.vertical_centered(|ui| {
                                    ui.spacing_mut().item_spacing = vec2(1.0, -3.0);
                                    ui.label(RichText::new("water tracker").size(17.5).strong());

                                    ui.add_space(REMAINDER);

                                    ui.label(RichText::new(format!("{} %", &water_percent)).size(18.0));

                                    ui.add_space(REMAINDER);

                                    self.water_tracker_bar(ctx, frame, ui, spacing, circle_size, water_rows, water_cols, water_percent);

                                });
                            });

                            strip.empty();
                        });
                    });

                strip.cell(|ui| {
                    StripBuilder::new(ui)
                        .size(Size::relative(0.25))
                        .size(Size::remainder())
                        .size(Size::relative(0.25))
                        .horizontal(|mut strip| {
                            strip.cell(|ui| {
                                // ui.painter().rect_filled(
                                //     ui.available_rect_before_wrap(),
                                //     0.0,
                                //     (Color32::GREEN),
                                // );
                            });

                            strip.cell(|ui| {
                                ui.vertical_centered(|ui| {
                                    ui.label(RichText::new("workout tracker").size(17.5).strong());

                                    ui.add_space(REMAINDER);

                                    ui.label(RichText::new(format!("{} % of muscles worked out this week", self.datas.all_workout_data.worked_out)).size(15.0));
                                    StripBuilder::new(ui)
                                        .size(Size::relative(0.5))
                                        .size(Size::relative(0.5))
                                        .horizontal(|mut strip| {
                                            strip.cell(|ui| {
                                                ui.vertical_centered(|ui| {
                                                    workout_tracker_widget_front(ctx, ui, Vec2::new(100.0, 226.0), &vec![Exercises::Deadlift, Exercises::BenchPress]);
                                                });
                                            });

                                            strip.cell(|ui| {
                                                ui.vertical_centered(|ui| {
                                                    workout_tracker_widget_behind(ctx, ui, Vec2::new(100.0, 226.0), &vec![]);
                                                });
                                            });
                                        })
                                });
                            });

                            strip.cell(|ui| {
                                // ui.painter().rect_filled(
                                //     ui.available_rect_before_wrap(),
                                //     0.0,
                                //     (Color32::GREEN),
                                // );
                            });
                        });
                });
            });

        let screen_rect = ctx.screen_rect();
        let painter = ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("dark_backdrop")));
        painter.rect_filled(
            screen_rect,
            0.0,
            Color32::from_rgba_unmultiplied(20, 20, 20, 0),
        );

        ui.allocate_ui_at_rect(screen_rect, |ui| {
            ui.with_layout(Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                ui.add(Image::new(self.medias.ambient_blue.clone()).tint(tint_color).fit_to_exact_size(vec2(ui.available_width(), ui.available_height() * 1.5)));
            });
        });
    }

    pub fn workouts_ui(&mut self, ctx: &Context, frame: &mut Frame, ui: &mut Ui, elements_color: Color32, tint_color: Color32, is_dark: bool) {
        let mut other_elements_color;
        let mut text_color;

        if is_dark {
            other_elements_color = Color32::from_rgb(67, 67, 67);
            text_color = Color32::WHITE;
        } else {
            other_elements_color = Color32::from_rgb(240, 240, 240);
            text_color = Color32::BLACK;
        }

        StripBuilder::new(ui)
            // .size(Size::exact(100.0))
            .size(Size::remainder())
            .size(Size::exact(150.0))
            .vertical(|mut strip|{

                let now = chrono::Local::now();

                strip.cell(|ui| {
                    ScrollArea::vertical()
                        .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
                        .show(ui, |ui| {
                            let side_rect = Rect::from_min_size(
                            ctx.screen_rect().left_top() + vec2(50.0, 125.0),
                            vec2(ui.available_width() - 100.0, 600.0),
                            );

                            let rect_height = side_rect.height();
                            let spacing = 35.0;

                            let id = Id::new("scroll_pos");
                            let mut pos = ctx.data(|data| {
                                data.get_temp::<Pos2>(id).unwrap_or(side_rect.left_top())
                            });

                            let response = ui.allocate_rect(side_rect, egui::Sense::drag());
                            let dt = ctx.input(|i| i.stable_dt);
                            let speed = 600.0;
                            let workouts = &self.datas.planned_workout_data.workouts;

                            //TODO improve logic
                            if workouts.contains_key(&self.states.selected_day) && !workouts.get(&self.states.selected_day).expect("workout data error").is_empty() && workouts.get(&self.states.selected_day).unwrap()[0].template.workout_name != "rest" {
                            // if self.datas.planned_workout_data.workouts.contains_key(&self.states.selected_day) && self.datas.planned_workout_data.workouts.get(&self.states.selected_day).unwrap()[0].template.workout_name != "rest" {
                                ui.vertical_centered(|ui| {
                                    if response.dragged() {
                                        let delta = response.drag_delta();
                                        pos.y += delta.y;
                                        ctx.data_mut(|data| {
                                            data.insert_temp(id, pos);
                                        });
                                    } else {
                                        let mut closest_index = 0;
                                        let mut closest_dist = f32::MAX;
                                        let screen_center_y = ctx.screen_rect().center().y - 25.0;

                                        for i in 0..self.datas.planned_workout_data.workouts.get(&self.states.selected_day).unwrap().len() + 1 {
                                            let offset_y = i as f32 * (rect_height + spacing);
                                            let rect_center_y = pos.y + offset_y + rect_height / 2.0;
                                            let dist = (rect_center_y - screen_center_y).abs();

                                            if dist < closest_dist {
                                                closest_dist = dist;
                                                closest_index = i;
                                            }
                                        }

                                        let target_y = screen_center_y - closest_index as f32 * (rect_height + spacing) - rect_height / 2.0;

                                        let dy = target_y - pos.y;
                                        let step = speed * dt;

                                        if dy.abs() > 0.5 {
                                            if dy.abs() > step {
                                                pos.y += step * dy.signum();
                                                ctx.request_repaint();
                                            } else {
                                                pos.y = target_y;
                                            }
                                            ctx.data_mut(|data| {
                                                data.insert_temp(id, pos);
                                            });
                                        }
                                    }

                                    for i in 0..self.datas.planned_workout_data.workouts.get(&self.states.selected_day).unwrap().len() + 1 {
                                        let offset_y = i as f32 * (rect_height + spacing);
                                        let rect_pos = pos + vec2(0.0, offset_y);

                                        let rect = Rect::from_min_size(rect_pos, vec2(ui.available_width() - 100.0, rect_height));

                                        let screen_center_y = ctx.screen_rect().center().y;
                                        let dist_to_center = (rect.center().y - screen_center_y).abs();
                                        let intensity = (1.0 - (dist_to_center / 800.0)).clamp(0.0, 1.0) * 0.4;

                                        let [r, g, b, _] = elements_color.to_array();
                                        let brightness = 0.2126 * r as f32 + 0.7152 * g as f32 + 0.0722 * b as f32;

                                        let color = {
                                            if brightness < 128.0 {
                                                Color32::from_rgb(
                                                    (r as f32 * (0.95 - intensity)).max(0.0) as u8,
                                                    (g as f32 * (0.95 - intensity)).max(0.0) as u8,
                                                    (b as f32 * (0.95 - intensity)).max(0.0) as u8,
                                                )
                                            } else {
                                                Color32::from_rgb(
                                                    (r as f32 + (255.0 - r as f32) * intensity).min(255.0) as u8,
                                                    (g as f32 + (255.0 - g as f32) * intensity).min(255.0) as u8,
                                                    (b as f32 + (255.0 - b as f32) * intensity).min(255.0) as u8,
                                                )
                                            }
                                        };

                                        Self::draw_rect_with_black_shadow(ui.painter(), rect, 24, color, 3.0, 3.0, [(2.0, 20), (3.0, 25), (5.0, 30)], Rounding::same(24));

                                        if i != self.datas.planned_workout_data.workouts.get(&self.states.selected_day).unwrap().len() {
                                            self.draw_workout_card(ctx, frame, ui, rect_pos, rect, self.states.selected_day, i, is_dark, elements_color, other_elements_color, text_color);
                                        } else {
                                            ui.allocate_ui_at_rect(Rect::from_min_size(rect_pos, vec2(ui.available_width() - 100.0, 600.0)),|ui| {
                                                ui.vertical_centered(|ui| {
                                                    ui.add_space(70.0);
                                                    ui.add(Label::new(RichText::new("add more workout").size(30.0)).selectable(false));
                                                    ui.add_space(150.0);
                                                    ui.add_sized(vec2(100.0, 100.0), Image::new(self.medias.plus.clone()));
                                                    ui.add_space(160.0);

                                                    if (ui.add(
                                                        Button::new(RichText::new("add workout").size(22.0).color(Color32::WHITE))
                                                            .fill(Color32::from_rgb(0, 75, 141))
                                                            .min_size(Vec2::new(side_rect.width() / 2.5, 40.0))
                                                            .rounding(10),
                                                    )).clicked() {
                                                        self.states.templates_window = !self.states.templates_window;
                                                    }

                                                    if self.states.templates_window {
                                                        self.draw_templates_window(ui, ctx, is_dark, elements_color, other_elements_color, text_color, &mut true);
                                                    }
                                                });
                                            });
                                        }
                                    }
                                });
                            } else {
                                if response.dragged() {
                                    let delta = response.drag_delta();
                                    pos.y += delta.y;

                                    ctx.data_mut(|data| {
                                        data.insert_temp(id, pos);
                                    });
                                } else if (pos.y > ctx.screen_rect().center().y - side_rect.height() / 1.85) || (pos.y <ctx.screen_rect().center().y + side_rect.height() / 1.85) {
                                    let side_height = side_rect.height();
                                    let target_y = ctx.screen_rect().center().y - side_height / 1.85;

                                    let dt = ctx.input(|i| i.stable_dt);
                                    let speed = 600.0;

                                    let dy = target_y - pos.y;
                                    let step = speed * dt;

                                    if dy.abs() > 0.5 {
                                        if dy.abs() > step {
                                            pos.y += step * dy.signum();
                                            ctx.request_repaint();
                                        } else {
                                            pos.y = target_y;
                                        }
                                    }

                                    ctx.data_mut(|data| {
                                        data.insert_temp(id, pos);
                                    });
                                }
                                ui.painter().rect_filled(
                                    Rect::from_min_size(pos, vec2(ui.available_width() - 100.0, 600.0)),
                                    egui::epaint::Rounding {
                                        nw: 24,
                                        ne: 24,
                                        sw: 24,
                                        se: 24,
                                    },
                                    elements_color,
                                );

                                ui.allocate_ui_at_rect(Rect::from_min_size(pos, vec2(side_rect.width(), side_rect.height())),|ui| {
                                    if !self.datas.planned_workout_data.workouts.contains_key(&self.states.selected_day) || self.datas.planned_workout_data.workouts.get(&self.states.selected_day).unwrap().is_empty() {
                                        ui.vertical_centered(|ui| {
                                            ui.add_space(70.0);
                                            ui.add(Label::new(RichText::new("not planned").size(30.0)).selectable(false));
                                            ui.add_space(40.0);
                                            ui.add_sized(vec2(250.0,250.0), Image::new(self.medias.calendar.clone()));
                                            ui.add_space(120.0);
                                        });

                                        ui.horizontal(|ui| {
                                            ui.add_space(side_rect.width() / 13.0);

                                            if ui.add(
                                                Button::new(RichText::new("rest").size(22.0).color(Color32::WHITE))
                                                    .fill(Color32::from_rgb(91, 0, 113))
                                                    .min_size(Vec2::new(side_rect.width() / 2.5, 40.0))
                                                    .rounding(10),
                                            ).clicked() {
                                                self.datas.planned_workout_data.rest(self.states.selected_day);
                                            };

                                            let padding = side_rect.width() - (((side_rect.width() / 13.0) * 2.0) + ((side_rect.width() / 2.5) * 2.0)) - 8.0;
                                            ui.add_space(padding);

                                            if (ui.add(
                                                Button::new(RichText::new("add workout").size(22.0).color(Color32::WHITE))
                                                    .fill(Color32::from_rgb(0, 75, 141))
                                                    .min_size(Vec2::new(side_rect.width() / 2.5, 40.0))
                                                    .rounding(10),
                                            )).clicked() {
                                                self.states.templates_window = !self.states.templates_window;
                                            }

                                            if self.states.templates_window {
                                                self.draw_templates_window(ui, ctx, is_dark, elements_color, other_elements_color, text_color, &mut true);
                                            }
                                        });
                                    } else {
                                        ui.vertical_centered(|ui| {
                                            ui.add_space(70.0);
                                            ui.add(Label::new(RichText::new("rest").size(30.0)).selectable(false));
                                            ui.add_space(40.0);
                                            // ui.add_space(250.0);
                                            ui.add_sized(vec2(250.0,250.0), Image::new(self.medias.bed.clone()));
                                            ui.add_space(120.0);

                                            if (ui.add(
                                                Button::new(RichText::new("change workout").size(22.0).color(Color32::WHITE))
                                                    .fill(Color32::from_rgb(0, 75, 141))
                                                    .min_size(Vec2::new(side_rect.width() / 2.5, 40.0))
                                                    .rounding(10),
                                            )).clicked() {
                                                self.states.templates_window = !self.states.templates_window;
                                            }

                                            if self.states.templates_window {
                                                self.draw_templates_window(ui, ctx, is_dark, elements_color, other_elements_color, text_color, &mut true);
                                            }
                                        });
                                    }
                                });
                            };
                        });

                        if self.states.templates_window {
                            self.draw_templates_window(ui, ctx, is_dark, elements_color, other_elements_color, text_color, &mut true);
                        }

                        if self.states.exercises_window {
                            self.draw_exercises_window(ui, ctx, is_dark, elements_color, other_elements_color, text_color, &mut true);
                        }

                        if self.states.alert_modal {
                            self.draw_alert_window(ui, ctx, is_dark, "are you sure to remove workout?", "remove");
                        }

                        let top_rect = Rect::from_min_size(
                            ctx.screen_rect().left_top(),
                            vec2(ui.available_width(), 100.0),
                        );

                        Self::draw_rect_with_black_shadow(ui.painter(), top_rect, 24, elements_color, 0.0, 6.0, [(5.0, 20), (3.0, 25), (2.0, 30),], Rounding {
                            nw: 0,
                            ne: 0,
                            sw: 24,
                            se: 24,
                        });

                        ui.allocate_ui_at_rect(top_rect, |ui| {
                            ui.vertical_centered(|ui| {
                                StripBuilder::new(ui)
                                    .size(Size::relative(0.3))
                                    .size(Size::remainder())
                                    .size(Size::relative(0.3))
                                    .horizontal(|mut strip| {
                                        strip.cell(|ui | {
                                            ui.horizontal_centered(|ui| {
                                                ui.add_space(30.0);
                                                if ui.add(Button::image_and_text(self.medias.workout_templates.clone(), RichText::new("templates").size(13.0).strong().color(text_color))
                                                    .fill(other_elements_color)
                                                    .min_size(Vec2::new(75.0, 30.0))
                                                    .rounding(5.0),
                                                ).clicked() {
                                                    self.states.editable = true;
                                                    self.states.templates_window = !self.states.templates_window;
                                                }
                                            });
                                        });

                                        strip.cell(|ui| {
                                            ui.vertical_centered(|ui| {
                                                ui.add_space(20.0);
                                                ui.add(Label::new(RichText::new(format!("{} {}", self.states.selected_day.format("%B"), self.states.selected_day.format("%d"))).size(25.0).strong()).selectable(false));
                                                ui.add(Label::new(RichText::new(format!("{}", self.states.selected_day.format("%A"))).size(15.0).strong()).selectable(false));
                                            });
                                        });

                                        strip.cell(|ui| {
                                            ui.horizontal_centered(|ui| {
                                                ui.add_space(50.0);
                                                if ui.add(Button::image_and_text(self.medias.workouts.clone(), RichText::new("exercises").size(13.0).strong().color(text_color))
                                                    .fill(other_elements_color)
                                                    .min_size(Vec2::new(75.0, 30.0))
                                                    .rounding(5.0),
                                                ).clicked() {
                                                    self.states.exercises_window = !self.states.exercises_window;
                                                }
                                            });
                                        });
                                    });
                            });
                        });

                        let screen_rect = ctx.screen_rect();
                        let painter = ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("ambient layout")));

                        ui.allocate_ui_at_rect(screen_rect, |ui| {
                            if self.states.alert_modal {
                                ui.with_layout(Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                                    ui.add(Image::new(self.medias.ambient_red.clone()).fit_to_exact_size(vec2(ui.available_width(), ui.available_height() * 1.5)));
                                });
                            } else {
                                ui.with_layout(Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                                    ui.add(Image::new(self.medias.ambient_blue.clone()).tint(tint_color).fit_to_exact_size(vec2(ui.available_width(), ui.available_height() * 1.5)));
                                });
                            }
                        });
                    });

                strip.cell(|ui| {
                    let bot_rect = Rect::from_min_size(
                        ctx.screen_rect().left_bottom() - vec2(0.0, 150.0),
                        ctx.screen_rect().right_bottom().to_vec2(),
                    );

                    Self::draw_rect_with_black_shadow(ui.painter(), bot_rect, 24, elements_color, 0.0, -4.0, [(2.0, 20), (3.0, 25), (5.0, 30)], Rounding {
                        nw: 24,
                        ne: 24,
                        sw: 0,
                        se: 0,
                    });

                    self.draw_calendar(ui, bot_rect, now);
                });
            });
    }

    pub fn calory_tracker_ui(&mut self, ctx: &Context, frame: &mut Frame, ui: &mut Ui, elements_color: Color32, tint_color: Color32) {
        let is_dark = ctx.style().visuals.dark_mode;
        let mut other_elements_color;
        let mut text_color;

        if is_dark {
            other_elements_color = Color32::from_rgb(67, 67, 67);
            text_color = Color32::WHITE;
        } else {
            other_elements_color = Color32::from_rgb(240, 240, 240);
            text_color = Color32::BLACK;
        }

        let now = chrono::Local::now();

        StripBuilder::new(ui)
            // .size(Size::exact(100.0))
            .size(Size::remainder())
            .size(Size::exact(self.states.strip_size))
            .vertical(|mut strip|{

                // let now = chrono::Local::now();
                strip.cell(|ui| {
                    ScrollArea::vertical()
                        .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
                        .show(ui, |ui| {

                    ui.add_space(115.0);

                    ui.vertical_centered(|ui| {
                        ui.add(Label::new(RichText::new("CALORIES").size(25.0).strong()).selectable(false));

                        let calory_rect = Rect::from_min_size(
                            // top_rect.left_top() + egui::vec2(50.0, top_rect.height() + 25.0),
                            ctx.screen_rect().left_top() + vec2(150.0, 155.0),
                            vec2(ui.available_width() - 300.0, 90.0),
                        );

                        ui.painter().rect_filled(
                            calory_rect,
                            egui::epaint::Rounding {
                                nw: 24,
                                ne: 24,
                                sw: 24,
                                se: 24,
                            },
                            elements_color,
                        );

                        ui.allocate_ui_at_rect(calory_rect, |ui| {
                            ui.add_space(calory_rect.width() / 12.0);

                            ui.vertical_centered(|ui| {
                                ui.label(RichText::new(format!("{}/{}", self.datas.macro_data.calory_registered, self.datas.macro_data.calory_goal)).size(35.0).strong());
                            });
                        });
                    });

                    ui.add_space(40.0);

                    let spacing = 3.0;
                    let rect_size = 10.0;
                    let rows = 5;
                    let cols = 25;

                    let calory_percent = ((self.datas.macro_data.calory_registered as f32 / self.datas.macro_data.calory_goal as f32) * 100.0) as u32;

                    let total_width = (rect_size * cols as f32) + (spacing * (cols as f32 - 1.0));
                    let available_width = ui.available_width();

                    ui.horizontal(|ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.set_width((available_width - total_width - 30.0) / 2.0);
                            });
                        });

                        ui.vertical(|ui| {
                            ui.set_width(total_width + 25.0);
                            ui.vertical_centered(|ui| {
                                ui.set_width(total_width + 25.0);
                                ui.vertical_centered(|ui| {
                                    self.calory_tracker_bar(ctx, frame, ui, spacing, rect_size, rows, cols, calory_percent);
                                });
                            });
                        });
                    });

                    ui.add_space(10.0);

                    ui.vertical_centered(|ui| {

                        let rect_length = 130.0;

                        let available_width = ui.available_width();

                        let carbs_rect = Rect::from_min_size(
                            Pos2::new((available_width/ 2.0 ) - (rect_length / 2.0), ctx.screen_rect().min.y + 390.0),
                            Vec2::new(rect_length, ctx.screen_rect().min.y + 50.0)
                        );

                        let proteins_rect = Rect::from_min_size(
                            carbs_rect.left_top() - Vec2::new(rect_length + 10.0, 0.0),
                            Vec2::new(rect_length, ctx.screen_rect().min.y + 50.0),
                        );

                        let fats_rect = Rect::from_min_size(
                            carbs_rect.right_top() + Vec2::new(10.0, 0.0),
                            Vec2::new(rect_length, ctx.screen_rect().min.y + 50.0),
                        );

                        ui.add(Label::new(RichText::new("MACROS").size(25.0).strong()).selectable(false));

                        ui.horizontal(|ui| {
                            ui.painter().rect_filled(
                                carbs_rect,
                                egui::epaint::Rounding {
                                    nw: 14,
                                    ne: 14,
                                    sw: 14,
                                    se: 14,
                                },
                                elements_color,
                            );

                            ui.allocate_ui_at_rect(carbs_rect, |ui| {
                                ui.vertical_centered(|ui| {
                                    ui.add_space(2.0);
                                    ui.label(RichText::new("carbs").strong().color(Color32::from_rgb(141, 54, 0)).size(14.0));
                                    ui.label(RichText::new(format!("{}/{}", self.datas.macro_data.carb_registered, self.datas.macro_data.carb_goal)).size(23.0).strong());
                                    ui.add_space(5.0);
                                    ui.horizontal(|ui| {
                                        ui.add_space(31.0);
                                        self.mini_tracker_bar(ctx, frame, ui, spacing, rect_size, 5, self.datas.macro_data.carb_registered, self.datas.macro_data.carb_goal);
                                    });
                                });
                            });

                            ui.painter().rect_filled(
                                proteins_rect,
                                egui::epaint::Rounding {
                                    nw: 14,
                                    ne: 14,
                                    sw: 14,
                                    se: 14,
                                },
                                elements_color,
                            );

                            ui.allocate_ui_at_rect(proteins_rect, |ui| {
                                ui.vertical_centered(|ui| {
                                    ui.add_space(2.0);
                                    ui.label(RichText::new("proteins").strong().color(Color32::from_rgb(0, 75, 140)).size(14.0));
                                    ui.label(RichText::new(format!("{}/{}", self.datas.macro_data.protein_registered, self.datas.macro_data.protein_goal)).size(23.0).strong());
                                    ui.add_space(5.0);
                                    ui.horizontal(|ui| {
                                        ui.add_space(31.0);
                                        self.mini_tracker_bar(ctx, frame, ui, spacing, rect_size, 5, self.datas.macro_data.protein_registered, self.datas.macro_data.protein_goal);
                                    });
                                });
                            });

                            ui.painter().rect_filled(
                                fats_rect,
                                egui::epaint::Rounding {
                                    nw: 14,
                                    ne: 14,
                                    sw: 14,
                                    se: 14,
                                },
                                elements_color,
                            );

                            ui.allocate_ui_at_rect(fats_rect, |ui| {
                                ui.vertical_centered(|ui| {
                                    ui.add_space(2.0);
                                    ui.label(RichText::new("fats").strong().color(Color32::from_rgb(141, 0, 19)).size(14.0));
                                    ui.label(RichText::new(format!("{}/{}", self.datas.macro_data.fat_registered, self.datas.macro_data.fat_goal)).size(23.0).strong());
                                    ui.add_space(5.0);
                                    ui.horizontal(|ui| {
                                        ui.add_space(31.0);
                                        self.mini_tracker_bar(ctx, frame, ui, spacing, rect_size, 5, self.datas.macro_data.fat_registered, self.datas.macro_data.fat_goal);
                                    });
                                });
                            });
                        });

                        StripBuilder::new(ui)
                            .size(Size::exact(30.0))
                            .size(Size::exact(3.0))
                            .size(Size::remainder())
                            .size(Size::exact(20.0))
                            .vertical(|mut strip| {
                                strip.cell(|ui| {
                                    ui.vertical_centered(|ui| {
                                            ui.add(Label::new(RichText::new("HISTORY").size(25.0).strong()).selectable(false));
                                        });
                                    });

                                    strip.empty();

                                    strip.cell(|ui| {
                                        ui.set_max_width(480.0);

                                        let history_rect = ui.available_rect_before_wrap();

                                        ui.vertical_centered(|ui| {
                                            ui.painter().rect_filled(
                                                history_rect,
                                                egui::epaint::Rounding {
                                                    nw: 28,
                                                    ne: 28,
                                                    sw: 28,
                                                    se: 28,
                                                },
                                                elements_color,
                                            );
                                        });

                                        ui.allocate_ui_at_rect(history_rect.shrink2(vec2(20.0, 5.0)), |ui| {
                                            ScrollArea::vertical()
                                                .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
                                                .show(ui, |ui| {
                                                    // for i in 0..10 {
                                                    // for j in self.datas.macro_data.meal_history.get(&self.states.selected_day) {
                                                    //     for i in j {
                                                    if let Some(eats) = self.datas.macro_data.meal_history.get_mut(&self.states.selected_day) {
                                                        for (index, eat) in eats.clone().iter().enumerate() {
                                                            ui.vertical(|ui| {
                                                                ui.set_height(42.0);
                                                                ui.horizontal_centered(|ui| {
                                                                    ui.vertical(|ui| {
                                                                        ui.add_space(5.0);
                                                                        ui.add(Label::new(RichText::new("Meal").size(17.0)));
                                                                        ui.add(Label::new(RichText::new(format!("{} {}", self.states.selected_day.format("%b %e"), eat.date.format("%T"))).size(10.0)));
                                                                    });
                                                                    ui.add_space(140.0);

                                                                    if ui.add(Button::new(RichText::new("delete").size(14.0).strong().color(Color32::WHITE))
                                                                        .fill(Color32::from_rgb(140, 0, 0))
                                                                        .min_size(Vec2::new(65.0, 25.0))
                                                                        .rounding(9)).clicked() {
                                                                            self.states.alert_modal = !self.states.alert_modal;
                                                                        };

                                                                    if self.states.delete_was_positive {
                                                                        eats.remove(index);
                                                                        self.states.delete_was_positive = false;
                                                                    }

                                                                    ui.add_space(5.0);

                                                                    ui.add(Button::new(RichText::new("edit").size(14.0).strong().color(Color32::WHITE))
                                                                    .fill(Color32::from_rgb(0, 79, 148))
                                                                    .min_size(Vec2::new(50.0, 25.0))
                                                                    .rounding(9));

                                                                    ui.add_space(10.0);

                                                                    ui.vertical(|ui| {
                                                                        ui.add_space(6.0);
                                                                        ui.add(Label::new(RichText::new(format!("+{} cals", eat.meal.calory)).size(16.0).color(Color32::from_rgb(21, 141, 0))));
                                                                        ui.horizontal(|ui| {
                                                                            ui.add(Label::new(RichText::new(format!("{}p", eat.meal.protein)).size(10.0).color(Color32::BLUE)));
                                                                            ui.add(Label::new(RichText::new(format!("{}c", eat.meal.carb)).size(10.0).color(Color32::ORANGE)));
                                                                            ui.add(Label::new(RichText::new(format!("{}f", eat.meal.fat)).size(10.0).color(Color32::RED)));
                                                                        });
                                                                    });
                                                                });
                                                                ui.separator();
                                                            });
                                                        }
                                                    }
                                                });
                                        });
                                    });
                                    strip.empty();
                                });

                                if self.states.alert_modal {
                                    self.draw_alert_window(ui, ctx, is_dark, "are you sure to delete meal?", "delete");
                                }
                            });
                        });

                        let top_rect = Rect::from_min_size(
                            ctx.screen_rect().left_top(),
                            vec2(ui.available_width(), 100.0),
                        );

                        Self::draw_rect_with_black_shadow(ui.painter(), top_rect, 24, elements_color, 0.0, 6.0, [(5.0, 20), (3.0, 25), (2.0, 30),], Rounding {
                            nw: 0,
                            ne: 0,
                            sw: 24,
                            se: 24,
                        });

                        ui.allocate_ui_at_rect(top_rect, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.add_space(20.0);
                                ui.add(Label::new(RichText::new(format!("{} {}", self.states.selected_day.format("%B"), self.states.selected_day.format("%d"))).size(25.0).strong()).selectable(false));
                                ui.add(Label::new(RichText::new(format!("{}", self.states.selected_day.format("%A"))).size(15.0).strong()).selectable(false));
                            });
                        });

                    let screen_rect = ctx.screen_rect();
                    let painter = ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("ambient layout")));

                    ui.allocate_ui_at_rect(screen_rect, |ui| {
                        if self.states.alert_modal {
                            ui.with_layout(Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                                ui.add(Image::new(self.medias.ambient_red.clone()).fit_to_exact_size(vec2(ui.available_width(), ui.available_height() * 1.5)));
                            });
                        } else {
                            ui.with_layout(Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                                ui.add(Image::new(self.medias.ambient_blue.clone()).tint(tint_color).fit_to_exact_size(vec2(ui.available_width(), ui.available_height() * 1.5)));
                            });
                        }
                    });
                });

                strip.cell(|ui| {
                    let bot_rect = Rect::from_min_max(
                        // ctx.screen_rect().left_bottom() - vec2(0.0, 150.0),
                        // ctx.screen_rect().right_bottom().to_vec2(),
                        ctx.screen_rect().left_bottom() - vec2(0.0, self.states.strip_size),
                        ctx.screen_rect().right_bottom(),
                    );

                    let mut target_height = 150.0;

                    if self.states.macro_add_clicked {
                        target_height = 330.0;
                    };

                    self.states.strip_size = ui.ctx().animate_value_with_time(
                        ui.id().with("history_height"),
                        target_height,
                        0.6,
                    );

                    Self::draw_rect_with_black_shadow(ui.painter(), bot_rect, 110, elements_color, 0.0, -4.0, [(2.0, 20), (3.0, 25), (5.0, 30)], Rounding {
                        nw: 110,
                        ne: 110,
                        sw: 0,
                        se: 0,
                    });

                    ui.vertical_centered(|ui| {
                        ui.allocate_ui_at_rect(bot_rect, |ui| {
                            if !self.states.macro_add_clicked {
                                let rect = Rect::from_min_size(
                                    egui::pos2(bot_rect.center().x - 35.0, bot_rect.left_top().y - 20.0),
                                    vec2(70.0, 25.0),
                                );

                                Self::draw_rect_with_black_shadow(ui.painter(), rect, 5, other_elements_color, 0.0, 1.0, [(2.0, 20), (3.0, 25), (5.0, 30)], Rounding {
                                    nw: 5,
                                    ne: 5,
                                    sw: 5,
                                    se: 5,
                                });

                                ui.allocate_ui_at_rect(rect, |ui| {
                                    if ui.add(
                                        Button::image_and_text(self.medias.switch.clone(), RichText::new("switch").size(13.0).strong().color(text_color))
                                            .fill(elements_color)
                                            .min_size(Vec2::new(70.0, 25.0))
                                            .rounding(5.0),
                                    ).clicked() {
                                        self.states.calendar_mode_calory_ui = !self.states.calendar_mode_calory_ui;
                                    };
                                });
                                if self.states.calendar_mode_calory_ui {
                                    self.draw_calendar(ui, bot_rect, now);
                                } else {
                                    ui.add_space(20.0);
                                    if ui.add(
                                        Button::new(RichText::new("add macros").size(18.0).strong().color(Color32::WHITE))
                                            //     egui::Color32::from_rgb(91, 0, 113),
                                            .fill(Color32::from_rgb(21, 141, 0))
                                            .min_size(Vec2::new(120.0, 40.0))
                                            .rounding(12),
                                    ).clicked() {
                                        self.states.macro_add_clicked = !self.states.macro_add_clicked;
                                    };
                                }
                            } else {
                                StripBuilder::new(ui)
                                    .size(Size::exact(110.0))
                                    .size(Size::exact(25.0))
                                    .size(Size::remainder())
                                    .size(Size::exact(85.0))
                                    .vertical(|mut strip| {
                                        strip.cell(|ui| {
                                            StripBuilder::new(ui)
                                                .size(Size::exact(120.0))
                                                .size(Size::remainder())
                                                .size(Size::exact(120.0))
                                                .horizontal(|mut strip| {
                                                    strip.cell(|ui| {
                                                        ui.add_space(7.0);
                                                        ui.vertical_centered(|ui| {
                                                            if ui.add_sized(vec2(85.0, 60.0), ImageButton::new(if is_dark {self.medias.cancel_button_d.clone()} else {self.medias.cancel_button_l.clone()})
                                                                .frame(false)).clicked() {
                                                                    self.states.macro_add_clicked = !self.states.macro_add_clicked;
                                                                };
                                                        });
                                                    });

                                                    strip.cell(|ui| {
                                                        ui.set_width(150.0);

                                                        ui.vertical_centered(|ui| {
                                                            ui.add_space(REMAINDER);
                                                            ui.add(Label::new(RichText::new("add calories:").size(18.0).color(text_color).strong()).selectable(false));
                                                            ui.add_space(REMAINDER);
                                                            let calory_rect = ui.available_rect_before_wrap();
                                                            ui.painter().rect_filled(calory_rect, 25.0, other_elements_color);
                                                            ui.vertical_centered(|ui| {
                                                                ui.add_space(5.0);
                                                                ui.style_mut().override_text_style = Some(egui::TextStyle::Heading);

                                                                ui.add_sized(
                                                                    [calory_rect.width() - 20.0, calory_rect.height() - 30.0],
                                                                    egui::TextEdit::singleline(&mut self.states.calory_add_value)
                                                                        .font(egui::FontId::new(40.0, egui::FontFamily::Proportional)).background_color(other_elements_color).frame(false).char_limit(4),
                                                                );
                                                            });
                                                            ui.painter().rect_stroke(calory_rect, 25.0, Stroke::new(2.0, Color32::BLACK), StrokeKind::Outside);
                                                        });

                                                    });

                                                    strip.cell(|ui| {
                                                        ui.add_space(7.0);
                                                        ui.vertical_centered(|ui| {
                                                            if ui.add_sized(vec2(85.0, 60.0), ImageButton::new(if is_dark {self.medias.save_button_d.clone()} else {self.medias.save_button_l.clone()})
                                                                .frame(false)).clicked() {
                                                                    self.datas.macro_data.add_meal(
                                                                        self.states.selected_day,
                                                                        &self.states.calory_add_value,
                                                                        &self.states.protein_add_value,
                                                                        &self.states.carb_add_value,
                                                                        &self.states.fat_add_value);

                                                                    self.datas.macro_data.summarize(Some(self.states.selected_day));
                                                                    self.states.reset_macros();
                                                                    self.states.macro_add_clicked = !self.states.macro_add_clicked;
                                                                };
                                                        });
                                                    });
                                                });
                                        });

                                        strip.empty();

                                        strip.cell(|ui| {
                                            StripBuilder::new(ui)
                                                .size(Size::remainder())
                                                .size(Size::relative(0.3))
                                                .size(Size::relative(0.3))
                                                .size(Size::relative(0.3))
                                                .size(Size::remainder())
                                                .horizontal(|mut strip| {
                                                    strip.empty();

                                                    strip.cell(|ui| {
                                                        ui.set_width(130.0);
                                                        ui.vertical_centered(|ui| {
                                                            ui.add(Label::new(RichText::new("add proteins:").size(18.0).color(text_color).strong()).selectable(false));
                                                            ui.add_space(REMAINDER);

                                                            let proteins_rect = ui.available_rect_before_wrap();

                                                            ui.painter().rect_filled(proteins_rect, 25.0, other_elements_color);
                                                            ui.vertical_centered(|ui| {
                                                                ui.add_space(5.0);
                                                                ui.style_mut().override_text_style = Some(egui::TextStyle::Heading);

                                                                ui.add_sized(
                                                                    [proteins_rect.width() - 20.0, proteins_rect.height()],
                                                                    egui::TextEdit::singleline(&mut self.states.protein_add_value)
                                                                        .font(egui::FontId::new(40.0, egui::FontFamily::Proportional)).background_color(other_elements_color).frame(false).char_limit(4),
                                                                );
                                                            });
                                                            ui.painter().rect_stroke(proteins_rect, 25.0, Stroke::new(2.0, Color32::from_rgb(37, 99, 153)), StrokeKind::Outside);
                                                        });

                                                    });

                                                    strip.cell(|ui| {
                                                        ui.set_width(130.0);
                                                        ui.vertical_centered(|ui| {
                                                            ui.add(Label::new(RichText::new("add carbs:").size(18.0).color(text_color).strong()).selectable(false));
                                                            ui.add_space(REMAINDER);

                                                            let carbs_rect = ui.available_rect_before_wrap();

                                                            ui.painter().rect_filled(carbs_rect, 25.0, other_elements_color);
                                                            ui.vertical_centered(|ui| {
                                                                ui.add_space(5.0);
                                                                ui.style_mut().override_text_style = Some(egui::TextStyle::Heading);

                                                                ui.add_sized(
                                                                    [carbs_rect.width() - 20.0, carbs_rect.height()],
                                                                    egui::TextEdit::singleline(&mut self.states.carb_add_value)
                                                                        .font(egui::FontId::new(40.0, egui::FontFamily::Proportional)).background_color(other_elements_color).frame(false).char_limit(4),
                                                                );
                                                            });
                                                            ui.painter().rect_stroke(carbs_rect, 25.0, Stroke::new(2.0, Color32::from_rgb(158, 91, 50)), StrokeKind::Outside);
                                                        });
                                                    });

                                                    strip.cell(|ui| {
                                                        ui.set_width(130.0);
                                                        ui.vertical_centered(|ui| {
                                                            ui.add(Label::new(RichText::new("add fats:").size(18.0).color(text_color).strong()).selectable(false));
                                                            ui.add_space(REMAINDER);

                                                            let fats_rect = ui.available_rect_before_wrap();

                                                            ui.painter().rect_filled(fats_rect, 25.0, other_elements_color);
                                                            ui.vertical_centered(|ui| {
                                                                ui.add_space(5.0);
                                                                ui.style_mut().override_text_style = Some(egui::TextStyle::Heading);

                                                                ui.add_sized(
                                                                    [fats_rect.width() - 20.0, fats_rect.height()],
                                                                    egui::TextEdit::singleline(&mut self.states.fat_add_value)
                                                                        .font(egui::FontId::new(40.0, egui::FontFamily::Proportional)).background_color(other_elements_color).frame(false).char_limit(4),
                                                                );
                                                            });
                                                            ui.painter().rect_stroke(fats_rect, 25.0, Stroke::new(2.0, Color32::from_rgb(161, 59, 73)), StrokeKind::Outside);
                                                        });
                                                    });
                                                    strip.empty();
                                                });
                                        });
                                        strip.empty();
                                    });
                            }
                        });
                    });
                });
            });
    }

    pub fn water_tracker_ui(&mut self, ctx: &Context, frame: &mut Frame, ui: &mut Ui, elements_color: Color32, tint_color: Color32) {
        let is_dark = ctx.style().visuals.dark_mode;
        let mut other_elements_color;
        let mut text_color;

        let spacing = 3.0;

        let circle_size = 14.0;
        let water_rows = 4;
        let water_cols = 20;
        let water_percent = ((self.datas.water_data.hydrolized as f32 / self.datas.water_data.water_goal as f32) * 100.0) as u32;
        let water_tracker_width = (circle_size * water_cols as f32) + (spacing * (water_cols as f32 - 1.0));

        if is_dark {
            other_elements_color = Color32::from_rgb(67, 67, 67);
            text_color = Color32::WHITE;
        } else {
            other_elements_color = Color32::from_rgb(240, 240, 240);
            text_color = Color32::BLACK;
        }

        let now = chrono::Local::now();
        StripBuilder::new(ui)
            // .size(Size::exact(100.0))
            .size(Size::remainder())
            // .size(Size::exact(200.0))
            .size(Size::exact(self.states.strip_size))
            .vertical(|mut strip|{
                strip.cell(|ui| {
                    let top_rect = Rect::from_min_size(
                        ctx.screen_rect().left_top(),
                        vec2(ui.available_width(), 100.0),
                    );

                    Self::draw_rect_with_black_shadow(ui.painter(), top_rect, 24, elements_color, 0.0, 6.0, [(5.0, 20), (3.0, 25), (2.0, 30),], Rounding {
                        nw: 0,
                        ne: 0,
                        sw: 24,
                        se: 24,
                    });

                    ui.allocate_ui_at_rect(top_rect, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(20.0);
                            ui.add(Label::new(RichText::new(format!("{} {}", self.states.selected_day.format("%B"), self.states.selected_day.format("%d"))).size(25.0).strong()).selectable(false));
                            ui.add(Label::new(RichText::new(format!("{}", self.states.selected_day.format("%A"))).size(15.0).strong()).selectable(false));
                        });
                    });

                    ui.add_space(43.0);

                    ui.vertical_centered(|ui| {
                        ui.add(Label::new(RichText::new("WATER").size(25.0).strong()).selectable(false));

                        let water_rect =Rect::from_min_size(
                            // top_rect.left_top() + egui::vec2(50.0, top_rect.height() + 25.0),
                            ctx.screen_rect().left_top() + vec2(150.0, 155.0),
                            vec2(ui.available_width() - 300.0, 90.0),
                        );

                        ui.painter().rect_filled(
                            water_rect,
                            egui::epaint::Rounding {
                                nw: 24,
                                ne: 24,
                                sw: 24,
                                se: 24,
                            },
                            elements_color,
                        );

                        ui.allocate_ui_at_rect(water_rect, |ui| {
                            ui.add_space(water_rect.width() / 12.0);

                            ui.vertical_centered(|ui| {
                                ui.label(RichText::new(format!("{}/{}", self.datas.water_data.hydrolized, self.datas.water_data.water_goal)).size(35.0).strong());
                            });
                        });
                    });

                    ui.add_space(40.0);

                    ui.vertical_centered(|ui| {
                        ui.set_width(350.0);
                        ui.spacing_mut().item_spacing = vec2(1.0, -3.0);
                        self.water_tracker_bar(ctx, frame, ui, spacing, circle_size, water_rows, water_cols, water_percent);
                    });

                    // ui.add_space(REMAINDER);

                    ui.vertical_centered(|ui| {
                        let rect_length = 150.0;

                        let available_width = ui.available_width();

                        ui.add(Label::new(RichText::new("RECENT DRINKS").size(25.0).strong()).selectable(false));

                        let second_rect = Rect::from_min_size(
                            Pos2::new((available_width/ 2.0 ) - (rect_length / 2.0), ctx.screen_rect().min.y + 400.0),
                            Vec2::new(rect_length, ctx.screen_rect().min.y + 100.0)
                        );

                        let first_rect = Rect::from_min_size(
                            second_rect.left_top() - Vec2::new(rect_length + 30.0, 0.0),
                            Vec2::new(rect_length, ctx.screen_rect().min.y + 100.0),
                        );

                        let third_rect = Rect::from_min_size(
                            second_rect.right_top() + Vec2::new(30.0, 0.0),
                            Vec2::new(rect_length, ctx.screen_rect().min.y + 100.0),
                        );

                        ui.horizontal(|ui| {
                            ui.painter().rect_filled(
                                second_rect,
                                egui::epaint::Rounding {
                                    nw: 19,
                                    ne: 19,
                                    sw: 19,
                                    se: 19,
                                },
                                elements_color,
                            );

                            ui.allocate_ui_at_rect(second_rect, |ui| {
                                ui.vertical_centered(|ui| {
                                    StripBuilder::new(ui)
                                        .size(Size::relative(0.8))
                                        .size(Size::relative(0.2))
                                        .vertical(|mut strip| {
                                            strip.cell(|ui| {
                                                StripBuilder::new(ui)
                                                    .size(Size::relative(0.2))
                                                    .size(Size::relative(0.8))
                                                    .horizontal(|mut strip| {
                                                        strip.cell(|ui| {
                                                            ui.add_space(25.0);
                                                            ui.horizontal(|ui| {
                                                                ui.add_space(15.0);
                                                                ui.add_sized(vec2(45.0, 45.0), Image::new(self.medias.coffee.clone()));
                                                            });
                                                        });

                                                        strip.cell(|ui| {
                                                            ui.add_space(5.0);
                                                            ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                                                                ui.add_space(5.0);
                                                                ui.add(Label::new(RichText::new("COFFEE").size(17.0).color(text_color)));
                                                            });

                                                            ui.add_space(5.0);

                                                            ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                                                                ui.add_sized(vec2(15.0, 15.0), Image::new(self.medias.drop.clone()));
                                                                ui.add(Label::new(RichText::new("25 ml").size(15.0).color(text_color)));
                                                            });

                                                            ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                                                                ui.add_sized(vec2(18.0, 18.0), Image::new(self.medias.ml.clone()));
                                                                ui.add(Label::new(RichText::new("250 ml").size(15.0).color(text_color)));
                                                            });
                                                        })
                                                    });
                                            });

                                            strip.cell(|ui| {
                                                ui.add(Button::new(RichText::new("add").size(14.0).strong().color(Color32::WHITE))
                                                    // .fill(Color32::GRAY)
                                                    .fill(Color32::from_rgb(96, 96, 96))
                                                    .min_size(ui.available_rect_before_wrap().size())
                                                    .rounding(egui::epaint::Rounding {
                                                        nw: 0,
                                                        ne: 0,
                                                        sw: 25,
                                                        se: 25,
                                                    }));
                                            });
                                        });
                                });
                            });

                            ui.painter().rect_filled(
                                first_rect,
                                egui::epaint::Rounding {
                                    nw: 19,
                                    ne: 19,
                                    sw: 19,
                                    se: 19,
                                },
                                elements_color,
                            );

                            ui.allocate_ui_at_rect(first_rect, |ui| {
                                ui.vertical_centered(|ui| {
                                    StripBuilder::new(ui)
                                        .size(Size::relative(0.8))
                                        .size(Size::relative(0.2))
                                        .vertical(|mut strip| {
                                            strip.cell(|ui| {
                                                StripBuilder::new(ui)
                                                    .size(Size::relative(0.2))
                                                    .size(Size::relative(0.8))
                                                    .horizontal(|mut strip| {
                                                        strip.cell(|ui| {
                                                            ui.add_space(15.0);
                                                            ui.horizontal(|ui| {
                                                                ui.add_space(5.0);
                                                                ui.add_sized(vec2(55.0, 55.0), Image::new(self.medias.bottle.clone()));
                                                            });
                                                        });

                                                        strip.cell(|ui| {
                                                            ui.add_space(5.0);
                                                            ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                                                                ui.add_space(5.0);
                                                                ui.add(Label::new(RichText::new("WATER").size(15.0).color(text_color)));
                                                            });

                                                            ui.add_space(5.0);

                                                            ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                                                                ui.add_sized(vec2(15.0, 15.0), Image::new(self.medias.drop.clone()));
                                                                ui.add(Label::new(RichText::new("500 ml").size(15.0).color(text_color)));
                                                            });

                                                            ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                                                                ui.add_sized(vec2(18.0, 18.0), Image::new(self.medias.ml.clone()));
                                                                ui.add(Label::new(RichText::new("500 ml").size(15.0).color(text_color)));
                                                            });
                                                        })
                                                    });
                                            });

                                            strip.cell(|ui| {
                                                ui.add(Button::new(RichText::new("add").size(14.0).strong().color(Color32::WHITE))
                                                    // .fill(Color32::GRAY)
                                                    .fill(Color32::from_rgb(96, 96, 96))
                                                    .min_size(ui.available_rect_before_wrap().size())
                                                    .rounding(egui::epaint::Rounding {
                                                        nw: 0,
                                                        ne: 0,
                                                        sw: 25,
                                                        se: 25,
                                                    }));
                                            });
                                        });
                                });
                            });

                            ui.painter().rect_filled(
                                third_rect,
                                egui::epaint::Rounding {
                                    nw: 19,
                                    ne: 19,
                                    sw: 19,
                                    se: 19,
                                },
                                elements_color,
                            );

                            ui.allocate_ui_at_rect(third_rect, |ui| {
                                ui.vertical_centered(|ui| {
                                    StripBuilder::new(ui)
                                        .size(Size::relative(0.8))
                                        .size(Size::relative(0.2))
                                        .vertical(|mut strip| {
                                            strip.cell(|ui| {
                                                StripBuilder::new(ui)
                                                    .size(Size::relative(0.3))
                                                    .size(Size::relative(0.7))
                                                    .horizontal(|mut strip| {
                                                        strip.cell(|ui| {
                                                            ui.add_space(25.0);
                                                            ui.horizontal(|ui| {
                                                                ui.add_space(15.0);
                                                                ui.add_sized(vec2(45.0, 45.0), Image::new(self.medias.glass.clone()));
                                                            });
                                                        });

                                                        strip.cell(|ui| {
                                                            ui.add_space(5.0);
                                                            ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                                                                ui.add_space(5.0);
                                                                ui.add(Label::new(RichText::new("WATER").size(15.0).color(text_color)));
                                                            });

                                                            ui.add_space(5.0);

                                                            ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                                                                ui.add_sized(vec2(15.0, 15.0), Image::new(self.medias.drop.clone()));
                                                                ui.add(Label::new(RichText::new("500 ml").size(15.0).color(text_color)));
                                                            });

                                                            ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                                                                ui.add_sized(vec2(18.0, 18.0), Image::new(self.medias.ml.clone()));
                                                                ui.add(Label::new(RichText::new("500 ml").size(15.0).color(text_color)));
                                                            });
                                                        })
                                                    });
                                            });

                                            strip.cell(|ui| {
                                                ui.add(Button::new(RichText::new("add").size(14.0).strong().color(Color32::WHITE))
                                                    // .fill(Color32::GRAY)
                                                    .fill(Color32::from_rgb(96, 96, 96))
                                                    .min_size(ui.available_rect_before_wrap().size())
                                                    .rounding(egui::epaint::Rounding {
                                                        nw: 0,
                                                        ne: 0,
                                                        sw: 25,
                                                        se: 25,
                                                    }));
                                            });
                                        });
                                });
                            });
                        });
                    });

                    ui.add_space(REMAINDER);

                    ui.vertical_centered(|ui| {
                        StripBuilder::new(ui)
                            .size(Size::exact(30.0))
                            .size(Size::exact(3.0))
                            .size(Size::remainder())
                            .size(Size::exact(20.0))
                            .vertical(|mut strip| {
                                strip.cell(|ui| {
                                    ui.vertical_centered(|ui| {
                                        ui.add(Label::new(RichText::new("HISTORY").size(25.0).strong()).selectable(false));
                                    });
                                });

                                strip.empty();

                                strip.cell(|ui| {
                                    ui.set_max_width(480.0);

                                    let history_rect = ui.available_rect_before_wrap();

                                    ui.vertical_centered(|ui| {
                                        ui.painter().rect_filled(
                                            history_rect,
                                            egui::epaint::Rounding {
                                                nw: 28,
                                                ne: 28,
                                                sw: 28,
                                                se: 28,
                                            },
                                            elements_color,
                                        );
                                    });

                                    ui.allocate_ui_at_rect(history_rect.shrink2(vec2(20.0, 5.0)), |ui| {
                                        ScrollArea::vertical()
                                            .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
                                            .show(ui, |ui| {
                                                if let Some(drinks) = self.datas.water_data.water_history.get_mut(&self.states.selected_day) {
                                                    for (index, drink) in drinks.clone().iter().enumerate() {
                                                        ui.vertical(|ui| {
                                                            ui.set_height(42.0);
                                                            ui.horizontal_centered(|ui| {
                                                                ui.vertical(|ui| {
                                                                    ui.add_space(5.0);
                                                                    ui.add(Label::new(RichText::new(format!("{}", drink.beverage.name)).size(17.0)));
                                                                    ui.add(Label::new(RichText::new(format!("{} {}", self.states.selected_day.format("%b %e"), drink.date.format("%T"))).size(10.0)));
                                                                });
                                                                ui.add_space(140.0);

                                                                if ui.add(Button::new(RichText::new("delete").size(14.0).strong().color(Color32::WHITE))
                                                                    .fill(Color32::from_rgb(140, 0, 0))
                                                                    .min_size(Vec2::new(65.0, 25.0))
                                                                    .rounding(9)).clicked() {
                                                                        self.states.alert_modal = !self.states.alert_modal;
                                                                    };

                                                                if self.states.delete_was_positive {
                                                                    drinks.remove(index);
                                                                    self.states.delete_was_positive = false;
                                                                }

                                                                ui.add_space(5.0);

                                                                ui.add(Button::new(RichText::new("edit").size(14.0).strong().color(Color32::WHITE))
                                                                .fill(Color32::from_rgb(0, 79, 148))
                                                                .min_size(Vec2::new(50.0, 25.0))
                                                                .rounding(9));

                                                                ui.add_space(10.0);

                                                                ui.vertical(|ui| {
                                                                    ui.add_space(7.0);
                                                                    ui.add(Label::new(RichText::new(format!("+{} ml", drink.beverage.amount)).size(16.0).color(Color32::from_rgb(0, 79, 148))));
                                                                    ui.add(Label::new(RichText::new(format!("+{} ml", (drink.beverage.hydration_amount))).size(13.0).color(Color32::from_rgb(0, 170, 255))));
                                                                });
                                                            });
                                                            ui.separator();
                                                        });
                                                    }
                                                }
                                            });
                                        });
                                });

                                strip.empty();
                            });

                            if self.states.alert_modal {
                                self.draw_alert_window(ui, ctx, is_dark, "are sure to delete water?", "delete");
                            }
                    });

                    let screen_rect = ctx.screen_rect();
                    let painter = ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("ambient layout")));

                    ui.allocate_ui_at_rect(screen_rect, |ui| {
                        if self.states.alert_modal {
                            ui.with_layout(Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                                ui.add(Image::new(self.medias.ambient_red.clone()).fit_to_exact_size(vec2(ui.available_width(), ui.available_height() * 1.5)));
                            });
                        } else {
                            ui.with_layout(Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                                ui.add(Image::new(self.medias.ambient_blue.clone()).tint(tint_color).fit_to_exact_size(vec2(ui.available_width(), ui.available_height() * 1.5)));
                            });
                        }
                    });
                });

                strip.cell(|ui| {
                    let bot_rect = Rect::from_min_max(
                        // ctx.screen_rect().left_bottom() - vec2(0.0, 150.0),
                        // ctx.screen_rect().right_bottom().to_vec2(),
                        ctx.screen_rect().left_bottom() - vec2(0.0, self.states.strip_size),
                        ctx.screen_rect().right_bottom(),
                    );

                    let mut target_height = 150.0;

                    if self.states.water_add_clicked {
                        target_height = 350.0;
                    };

                    self.states.strip_size = ui.ctx().animate_value_with_time(
                        ui.id().with("history_height"),
                        target_height,
                        0.6,
                    );

                    Self::draw_rect_with_black_shadow(ui.painter(), bot_rect, 110, elements_color, 0.0, -4.0, [(2.0, 20), (3.0, 25), (5.0, 30)], Rounding {
                        nw: 110,
                        ne: 110,
                        sw: 0,
                        se: 0,
                    });

                    ui.vertical_centered(|ui| {
                        ui.allocate_ui_at_rect(bot_rect, |ui| {
                            if !self.states.water_add_clicked {
                                let rect = Rect::from_min_size(
                                    egui::pos2(bot_rect.center().x - 35.0, bot_rect.left_top().y - 20.0),
                                    vec2(70.0, 25.0),
                                );

                                Self::draw_rect_with_black_shadow(ui.painter(), rect, 5, other_elements_color, 0.0, 1.0, [(2.0, 20), (3.0, 25), (5.0, 30)], Rounding {
                                    nw: 5,
                                    ne: 5,
                                    sw: 5,
                                    se: 5,
                                });

                                ui.allocate_ui_at_rect(rect, |ui| {
                                    if ui.add(
                                        Button::image_and_text(self.medias.switch.clone(), RichText::new("switch").size(13.0).strong().color(text_color))
                                            .fill(elements_color)
                                            .min_size(Vec2::new(70.0, 25.0))
                                            .rounding(5.0),
                                    ).clicked() {
                                        self.states.calendar_mode_calory_ui = !self.states.calendar_mode_calory_ui;
                                    };
                                });
                                if self.states.calendar_mode_calory_ui {
                                    self.draw_calendar(ui, bot_rect, now);
                                } else {
                                    ui.add_space(20.0);
                                    if ui.add(
                                        Button::new(RichText::new("add water").size(18.0).strong().color(Color32::WHITE))
                                            //     egui::Color32::from_rgb(91, 0, 113),
                                            // .fill(Color32::from_rgb(21, 141, 0))
                                            .fill(Color32::from_rgb(0, 75, 142))
                                            .min_size(Vec2::new(120.0, 40.0))
                                            .rounding(12),
                                    ).clicked() {
                                        self.states.water_add_clicked= !self.states.water_add_clicked;
                                    };
                                }
                            } else {
                                StripBuilder::new(ui)
                                    .size(Size::exact(110.0))
                                    // .size(Size::exact(25.0))
                                    .size(Size::remainder())
                                    // .size(Size::exact(85.0))
                                    .vertical(|mut strip| {
                                        strip.cell(|ui| {
                                            StripBuilder::new(ui)
                                                .size(Size::exact(120.0))
                                                .size(Size::remainder())
                                                .size(Size::exact(120.0))
                                                .horizontal(|mut strip| {
                                                    strip.cell(|ui| {
                                                        ui.add_space(7.0);
                                                        ui.vertical_centered(|ui| {
                                                            if ui.add_sized(vec2(85.0, 60.0), ImageButton::new(if is_dark {self.medias.cancel_button_d.clone()} else {self.medias.cancel_button_l.clone()})
                                                                .frame(false)).clicked() {
                                                                    self.states.water_add_clicked = !self.states.water_add_clicked;
                                                                };
                                                        });
                                                    });

                                                    strip.cell(|ui| {
                                                        ui.set_width(200.0);

                                                        ui.vertical_centered(|ui| {
                                                            ui.add_space(REMAINDER * 3.0);
                                                            ui.add(Label::new(RichText::new("add beverage:").size(25.0).color(text_color).strong()).selectable(false));
                                                        });
                                                    });

                                                    strip.cell(|ui| {
                                                        ui.add_space(7.0);
                                                        ui.vertical_centered(|ui| {
                                                            if ui.add_sized(vec2(85.0, 60.0), ImageButton::new(if is_dark {self.medias.save_button_d.clone()} else {self.medias.save_button_l.clone()})
                                                                .frame(false)).clicked() {
                                                                    self.datas.water_data.add_drink(self.states.selected_day, &self.states.water_add_value, &self.states.hydration_percent);

                                                                    self.datas.water_data.summarize(Some(self.states.selected_day));
                                                                    self.states.reset_water();
                                                                    self.states.water_add_clicked = !self.states.water_add_clicked;
                                                                };
                                                        });
                                                    });
                                                });
                                        });

                                        strip.cell(|ui| {
                                            StripBuilder::new(ui)
                                                .size(Size::relative(0.5))
                                                .size(Size::relative(0.5))
                                                .horizontal(|mut strip| {
                                                    strip.cell(|ui| {
                                                        ui.set_width(150.0);
                                                        ui.set_height(100.0);
                                                        ui.vertical_centered(|ui| {
                                                            ui.add(Label::new(RichText::new("drink amount:").size(18.0).color(text_color).strong()).selectable(false));
                                                            let calory_rect = ui.available_rect_before_wrap();
                                                            ui.painter().rect_filled(calory_rect, 25.0, other_elements_color);
                                                            ui.vertical_centered(|ui| {
                                                                ui.add_space(5.0);
                                                                ui.style_mut().override_text_style = Some(egui::TextStyle::Heading);

                                                                ui.add_sized(
                                                                    [calory_rect.width() - 20.0, calory_rect.height() - 30.0],
                                                                    egui::TextEdit::singleline(&mut self.states.water_add_value)
                                                                        .font(egui::FontId::new(40.0, egui::FontFamily::Proportional)).background_color(other_elements_color).frame(false).char_limit(4),
                                                                );
                                                            });
                                                            ui.painter().rect_stroke(calory_rect, 25.0, Stroke::new(2.0, Color32::from_rgb(0, 79, 148)), StrokeKind::Outside);
                                                        });
                                                    });

                                                    strip.cell(|ui| {
                                                        ui.set_width(150.0);
                                                        ui.set_height(100.0);
                                                        ui.vertical_centered(|ui| {
                                                            ui.add(Label::new(RichText::new("hydration percent:").size(18.0).color(text_color).strong()).selectable(false));
                                                            let calory_rect = ui.available_rect_before_wrap();
                                                            ui.painter().rect_filled(calory_rect, 25.0, other_elements_color);
                                                            ui.vertical_centered(|ui| {
                                                                ui.add_space(5.0);
                                                                ui.style_mut().override_text_style = Some(egui::TextStyle::Heading);

                                                                ui.add_sized(
                                                                    [calory_rect.width() - 20.0, calory_rect.height() - 30.0],
                                                                    egui::TextEdit::singleline(&mut self.states.hydration_percent)
                                                                        .font(egui::FontId::new(40.0, egui::FontFamily::Proportional)).background_color(other_elements_color).frame(false).char_limit(4),
                                                                );
                                                            });
                                                            ui.painter().rect_stroke(calory_rect, 25.0, Stroke::new(2.0, Color32::from_rgb(0, 170, 255)), StrokeKind::Outside);
                                                        });
                                                    });
                                                });
                                        });
                                    });
                            }
                        });
                    });
                });

            });
    }

    pub fn statistics_ui(&mut self, ctx: &Context, frame: &mut Frame, ui: &mut Ui) {
    }

    pub fn navigation_bar(&mut self, ctx: &Context, frame: &mut Frame, ui: &mut Ui) {
        let is_dark = ctx.style().visuals.dark_mode;
        let mut elements_color;
        let mut tint_color;

        if is_dark {
            elements_color = Color32::from_rgb(27, 27, 27);
            tint_color = Color32::BLUE;
        } else {
            elements_color = Color32::from_rgb(217, 217, 217);
            tint_color = Color32::WHITE;
        }

        match self.states.selected_tab {
            0 => self.home(ctx, frame, ui, tint_color),
            // 1 => self.friends_ui(ctx, frame, ui),
            2 => self.workouts_ui(ctx, frame, ui, elements_color, tint_color, is_dark),
            3 => self.calory_tracker_ui(ctx, frame, ui, elements_color, tint_color),
            4 => self.water_tracker_ui(ctx, frame, ui, elements_color, tint_color),
            5 => self.statistics_ui(ctx, frame, ui),
            _ => {ui.label("empty");},
        }
    }

    fn calory_tracker_bar(&mut self, ctx: &Context, frame: &mut Frame, ui: &mut Ui, spacing: f32, rect_size: f32, rows: u32, cols: u32, calory_percent: u32) {
        ui.spacing_mut().item_spacing = vec2(1.0, -3.0);

        let mut green_rects = {
            if self.datas.macro_data.calory_registered == 0 {
                0
            } else if self.datas.macro_data.calory_goal > self.datas.macro_data.calory_registered{
                (((rows * cols) as f32 / 100.0) * calory_percent as f32).round() as u32
            } else {
               rows * cols
            }
        };

        ui.vertical_centered(|ui| {
            for _ in 0..rows {
                ui.horizontal(|ui| {
                    for col in 0..cols {
                        let (rect, _) = ui.allocate_exact_size(
                            vec2(rect_size, rect_size),
                            egui::Sense::hover(),
                        );

                        let color =  if green_rects > 0 {
                            green_rects -= 1;
                            Color32::from_rgb(0, 136, 255)
                            // Color32::DARK_BLUE
                        } else {
                            Color32::GRAY
                        };

                        ui.painter().rect_filled(rect, 1.0, color);

                        if col < cols - 1 {
                            ui.add_space(spacing);
                        }
                    }
                });
            }
            ui.add_space(5.0);
        });
    }

    fn water_tracker_bar(&mut self, ctx: &Context, frame: &mut Frame, ui: &mut Ui, spacing: f32, circle_size: f32, rows: u32, cols: u32, water_percent: u32) {
        ui.spacing_mut().item_spacing = vec2(1.0, 3.0);

        let mut done_marks = {
            if self.datas.water_data.water_registered == 0 {
                0
            } else if self.datas.water_data.water_goal > self.datas.water_data.hydrolized {
                (((rows * cols) as f32 / 100.0) * water_percent as f32).round() as u32
            } else {
               rows * cols
            }
        };

        ui.vertical_centered(|ui| {
            for _ in 0..rows {
                ui.horizontal(|ui| {
                    for col in 0..cols {
                        let (rect, _) = ui.allocate_exact_size(
                            vec2(circle_size, circle_size),
                            egui::Sense::hover(),
                        );

                        let color = if done_marks > 0 {
                            done_marks -= 1;
                            Color32::from_rgb(0, 136, 255)
                        } else {
                            Color32::GRAY
                        };

                        let center = rect.center();
                        let radius = circle_size / 2.0;
                        ui.painter().circle_filled(center, radius, color);

                        if col < cols - 1 {
                            ui.add_space(spacing);
                        }
                    }
                });
            }
            ui.add_space(5.0);
        });
    }

    fn mini_tracker_bar(&mut self, ctx: &Context, frame: &mut Frame, ui: &mut Ui, spacing: f32, rect_size:f32, cols: i32, registered: u32, goal: u32) {
        ui.spacing_mut().item_spacing = vec2(1.0, -3.0);

        let ROWS = 5;
        let COLUMNS = 5;

        let calory_percent = ((registered as f32 / goal as f32) * 100.0) as u32;

        let mut remaining = if registered == 0 {0}
        else if goal > registered {
            (((ROWS * COLUMNS) as f32 / 100.0) * calory_percent as f32).round() as u32
        } else {
            ROWS * COLUMNS
        };

        ui.vertical(|ui| {
            for _ in 0..ROWS {
                ui.horizontal(|ui| {
                    for col in 0..COLUMNS {
                        let (rect, _) = ui.allocate_exact_size(
                            vec2(rect_size, rect_size),
                            egui::Sense::hover(),
                        );

                        let color = if remaining > 0 {
                            remaining -= 1;
                            Color32::from_rgb(0, 136, 255)
                        } else {
                            Color32::GRAY
                        };

                        ui.painter().rect_filled(rect, 1.0, color);

                        if col < COLUMNS - 1 {
                            ui.add_space(spacing);
                        }
                    }
                });
            }
        });
    }

    fn draw_workout_card(&mut self, ctx: &Context, frame: &mut Frame, ui: &mut Ui, pos: Pos2, side_rect: Rect, selected_day: NaiveDate, index: usize, is_dark: bool, elements_color: Color32, other_elements_color: Color32, text_color: Color32) {
        ui.allocate_ui_at_rect(Rect::from_min_size(pos, vec2(ui.available_width() - 100.0, 600.0)),|ui| {
            // if !self.datas.planned_workout_data.workouts.contains_key(&selected_day) {
            //     ui.vertical_centered(|ui| {
            //         ui.add_space(280.0);
            //         ui.add(Label::new(RichText::new("not planned").size(24.0)).selectable(false));
            //         ui.add_space(200.0);
            //     });

            //     ui.horizontal(|ui| {
            //         ui.add_space(side_rect.width() / 13.0);

            //         ui.add(
            //             Button::new(RichText::new("rest").size(22.0).color(Color32::WHITE))
            //                 //     egui::Color32::from_rgb(91, 0, 113),
            //                 .fill(Color32::from_rgb(91, 0, 113))
            //                 .min_size(Vec2::new(side_rect.width() / 2.5, 40.0))
            //                 .rounding(10),
            //                 // .stroke(egui::Stroke::new(1.0, Color32::WHITE)),
            //         );

            //         let padding = side_rect.width() - (((side_rect.width() / 13.0) * 2.0) + ((side_rect.width() / 2.5) * 2.0)) - 8.0;
            //         ui.add_space(padding);

            //         if (ui.add(
            //             Button::new(RichText::new("add workout").size(22.0).color(Color32::WHITE))
            //                 //     egui::Color32::from_rgb(91, 0, 113),
            //                 .fill(Color32::from_rgb(0, 75, 141))
            //                 .min_size(Vec2::new(side_rect.width() / 2.5, 40.0))
            //                 .rounding(10),
            //             // .stroke(egui::Stroke::new(1.0, Color32::WHITE)),
            //         )).clicked() {
            //             println!("{:?}", self.datas.planned_workout_data.add_workout(selected_day, WorkoutPlanned::leg_day(selected_day)));
            //         };
            //     });
            // } else {

            if self.datas.planned_workout_data.workouts.get(&selected_day).unwrap().len() > index {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                        ui.vertical_centered(|ui| {
                            ui.add_space(30.0);
                            ui.label(RichText::new(&self.datas.planned_workout_data.workouts.get(&selected_day).unwrap()[index].template.workout_name).size(27.0).strong());
                            ui.add_space(50.0);

                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.set_width(side_rect.width() / 2.0);
                                    ui.vertical_centered(|ui| {
                                        workout_tracker_widget_front(ctx, ui, Vec2::new(110.0, 249.0), &self.datas.planned_workout_data.workouts.get(&selected_day).unwrap()[index].template.exercises);
                                    });
                                });

                                ui.vertical(|ui| {
                                    ui.set_width(side_rect.width() / 2.0);
                                    ui.vertical_centered(|ui| {
                                        workout_tracker_widget_behind(ctx, ui, Vec2::new(110.0, 249.0), &self.datas.planned_workout_data.workouts.get(&selected_day).unwrap()[index].template.exercises);
                                    });
                                });
                            });

                            ui.add_space(50.0);

                            ui.add(
                                Button::new(RichText::new("start").size(22.0).strong().color(Color32::WHITE))
                                    .fill(Color32::from_rgb(21, 141, 0))
                                    .min_size(Vec2::new(side_rect.width() / 4.0, 40.0))
                                    .rounding(10),
                            );

                            ui.add_space(12.0);

                            ui.horizontal(|ui| {
                                let button_width = side_rect.width() / 4.0;
                                let spacing = 6.0;
                                let total_width = button_width * 3.0 + spacing * 2.0;

                                let left_padding = (side_rect.width() - total_width) / 2.1;
                                ui.add_space(left_padding);

                                if ui.add(
                                    Button::new(RichText::new("change workout").size(15.0).strong().color(Color32::WHITE))
                                        .fill(Color32::from_rgb(0, 75, 141))
                                        .min_size(Vec2::new(button_width, 30.0))
                                        .rounding(8),
                                ).clicked() {
                                    self.states.templates_window = !self.states.templates_window;
                                }

                                if self.states.templates_window {
                                    self.draw_templates_window(ui, ctx, is_dark, elements_color, other_elements_color, text_color, &mut true);
                                }

                                ui.add_space(spacing);

                                if ui.add(
                                    Button::new(RichText::new("rest").size(18.0).strong().color(Color32::WHITE))
                                        .fill(Color32::from_rgb(91, 0, 113))
                                        .min_size(Vec2::new(button_width, 30.0))
                                        .rounding(8),
                                ).clicked() {
                                    self.datas.planned_workout_data.rest(self.states.selected_day);
                                };

                                ui.add_space(spacing);

                                if ui.add(
                                    Button::new(RichText::new("remove workout").size(18.0).color(Color32::WHITE))
                                        .fill(Color32::from_rgb(141, 0, 19))
                                        .min_size(Vec2::new(button_width, 30.0))
                                        .rounding(8),
                                ).clicked() {
                                    self.states.alert_modal = !self.states.alert_modal;
                                };

                                if self.states.delete_was_positive {
                                    self.datas.planned_workout_data.remove_workout(selected_day,  index);
                                    self.states.delete_was_positive = false;
                                }
                            });
                        });
                    });
                }
            });
    }

    fn draw_rect_with_black_shadow(painter: &egui::Painter, rect: Rect, rounding: u8, fill: Color32, offset_x: f32, offset_y: f32, layer: [(f32, u8); 3], corners: Rounding) {
        let shadow_color = |alpha: u8| Color32::from_rgba_unmultiplied(0, 0, 0, alpha);

        let shadow_offset = Vec2::new(offset_x, offset_y);

        for (inflate_by, alpha) in layer {
            let shadow_rect = rect
                .translate(shadow_offset)
                .expand(inflate_by);
            painter.rect_filled(shadow_rect, egui::Rounding::same(rounding + inflate_by as u8), shadow_color(alpha));
        }

        painter.rect_filled(rect, corners, fill);
    }

    fn draw_calendar(&mut self, ui: &mut Ui, rect: Rect, now: DateTime<Local>) {
        ui.allocate_ui_at_rect(rect, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.add_space(REMAINDER);

                let mut available_width = ui.available_width();
                ui.set_width(available_width);

                let rect_width = ui.available_width() / 10.0;

                let today = now.date_naive();
                let weekdays = ["mo", "tu", "we", "th", "fr", "sa", "su"];

                ui.horizontal(|ui| {
                    ui.add_space(REMAINDER * 3.0);
                    if self.states.skip_days >= 0 {
                        ui.vertical(|ui| {
                            ui.add_space(REMAINDER * 3.0);
                            if ui.add_sized([30.0, 30.0], ImageButton::new(self.medias.left_arrow.clone()).frame(false)).clicked() {
                                self.states.skip_days -= 7;
                            };
                        });
                    } else {
                        ui.add_space(REMAINDER * 3.8) ;
                    }

                    for i in 0..7 {
                        let offset = i as i64 - today.weekday().num_days_from_monday() as i64 + self.states.skip_days as i64;
                        let date = now.date_naive() + Duration::days(offset);

                        ui.vertical(|ui| {

                            ui.add(Label::new(format!(" {}", weekdays[i])).selectable(false));

                            let is_today = date == today;
                            let is_selected = Some(date) == Some(self.states.selected_day);

                            if ui.add(
                                Button::new(
                                    RichText::new(format!("{}", date.day()))
                                        .size(18.0)
                                        .color(Color32::WHITE),
                                )
                                .fill(
                                    if is_today {
                                        Color32::from_rgb(0, 75, 142)
                                    } else if is_selected {
                                        Color32::GRAY
                                    } else {
                                        Color32::from_rgb(96, 96, 96)
                                    },
                                )
                                .min_size(Vec2::new(rect_width, 60.0))
                                .rounding(8),
                            ).clicked() {
                                self.states.selected_day = date;
                                self.datas.macro_data.summarize(Some(self.states.selected_day));
                                self.datas.water_data.summarize(Some(self.states.selected_day));
                            }
                        });
                    }

                    if self.states.skip_days <= 0 {
                        ui.vertical(|ui| {
                            ui.add_space(REMAINDER * 3.0);
                            if ui.add_sized([30.0, 30.0], ImageButton::new(self.medias.right_arrow.clone()).frame(false)).clicked() {
                                self.states.skip_days += 7;
                            };
                            ui.add_space(REMAINDER * 4.0);
                        });
                    }
                });
            });
        });

    }

    pub fn draw_alert_window(&mut self, ui: &mut Ui, ctx: &Context, is_dark: bool, q_label: &str, conf_label: &str) {
        let screen_rect = ctx.screen_rect();
        let painter = ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("dark_backdrop")));

        ui.painter().rect_filled(
            screen_rect,
            0.0,
            if is_dark {
                Color32::from_rgba_unmultiplied(20, 20, 20,150)
            } else {
                Color32::from_rgba_unmultiplied(240, 240, 240, 150)
            }
        );

        Area::new("modal_blocker".into())
            .order(Order::Background)
            .fixed_pos(screen_rect.min)
            .show(ctx, |ui| {
                let _response = ui.allocate_response(screen_rect.size(), Sense::click());
            });

        let window_size = vec2(250.0, 70.0);

        Window::new("warning")
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .fixed_size(window_size)
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add(Label::new(RichText::new(q_label).size(18.0)));
                });

                ui.add_space(REMAINDER);

                StripBuilder::new(ui)
                    .size(Size::relative(0.5))
                    .size(Size::relative(0.5))
                    .horizontal(|mut strip| {
                        strip.cell(|ui| {
                            ui.vertical_centered(|ui| {
                                if ui.add(Button::new(RichText::new("cancel").size(14.0).strong().color(Color32::WHITE))
                                    // .fill(Color32::GRAY)
                                    .fill(Color32::from_rgb(96, 96, 96))
                                    .min_size(ui.available_rect_before_wrap().size())
                                    .rounding(egui::epaint::Rounding {
                                        nw: 0,
                                        ne: 0,
                                        sw: 9,
                                        se: 0,
                                    })).clicked() {
                                        self.states.alert_modal = !self.states.alert_modal;
                                    }
                            });
                        });

                        strip.cell(|ui| {
                            ui.vertical_centered(|ui| {
                                if ui.add(Button::new(RichText::new(conf_label).size(14.0).strong().color(Color32::WHITE))
                                    .fill(Color32::from_rgb(140, 0, 0))
                                    // .min_size(Vec2::new(65.0, 25.0))
                                    .min_size(ui.available_rect_before_wrap().size())
                                    .rounding(egui::epaint::Rounding {
                                        nw: 0,
                                        ne: 0,
                                        sw: 0,
                                        se: 9,
                                    })).clicked() {
                                        self.states.delete_was_positive = true;
                                        self.states.alert_modal = !self.states.alert_modal;
                                    }
                            });
                        });
                    });
            });
    }

    pub fn draw_templates_window(&mut self, ui: &mut Ui, ctx: &Context, is_dark: bool, elements_color: Color32, other_elements_color: Color32, text_color: Color32, open: &mut bool) {
        let screen_rect = ctx.screen_rect();
        let painter = ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("dark_backdrop")));

        ui.painter().rect_filled(
            screen_rect,
            0.0,
            if is_dark {
                Color32::from_rgba_unmultiplied(20, 20, 20,150)
            } else {
                Color32::from_rgba_unmultiplied(240, 240, 240, 150)
            }
        );

        Area::new("modal_blocker".into())
            .order(Order::Background)
            .fixed_pos(screen_rect.min)
            .show(ctx, |ui| {
                let _response = ui.allocate_response(screen_rect.size(), Sense::click());
            });

        let window_size = vec2(400.0, 500.0);
        let button_size = vec2(300.0, 60.0);

        Window::new("workout templates")
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .collapsible(false)
            .resizable(false)
            .interactable(false)
            .open(open)
            .fixed_size(window_size)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(REMAINDER);

                    if self.states.show_templates {
                        for template in self.datas.all_workout_data.workout_templates.values() {
                            if ui.add(
                                Button::new(
                                    RichText::new(format!("{}", template.workout_name))
                                        .size(18.0)
                                        .color(text_color),
                                )
                                .fill(other_elements_color)
                                .min_size(button_size)
                                .rounding(8),
                            ).clicked() {
                                if self.states.editable {
                                    self.states.current_template = template.workout_name.clone();
                                    self.states.show_templates = !self.states.show_templates;
                                } else {
                                    if let Some(rst) = self.datas.planned_workout_data.workouts.get_mut(&self.states.selected_day) {
                                        if rst[0].template.workout_name == "rest" {
                                            rst.remove(0);
                                        }
                                    };
                                    self.datas.planned_workout_data.add_workout(self.states.selected_day, WorkoutPlanned { template: template.clone() , date: self.states.selected_day });
                                    self.states.reset_template_window();
                                }
                            };
                            ui.add_space(10.0);
                        }
                        if ui.add(
                            Button::image_and_text(self.medias.plus.clone(),
                                RichText::new("create template")
                                    .size(18.0)
                                    .color(text_color),
                            )
                            .fill(other_elements_color)
                            .min_size(button_size)
                            .rounding(8),
                        ).clicked() {
                            self.states.create_template = !self.states.create_template;
                            self.states.show_templates = !self.states.show_templates;
                        };
                    } else if self.states.create_template {
                        // ui.vertical_centered(|ui| {
                        // ui.set_width(400.0);
                        //     if ui.add_sized(
                        //         vec2(70.0, 30.0),
                        //         Button::image_and_text(self.medias.left_arrow.clone(), "back").rounding(8)
                        //     ).clicked() {
                        //         self.states.create_template = !self.states.create_template;
                        //         self.states.show_templates = !self.states.show_templates;
                        //     }

                        //     ui.add_space(60.0);

                        //     ui.add_sized(vec2(130.0, 30.0), TextEdit::singleline(&mut self.states.new_template_name)
                        //                     .font(egui::FontId::new(18.0, egui::FontFamily::Proportional)));

                        //     if ui.add_sized(
                        //         vec2(70.0, 30.0),
                        //         Button::image_and_text(self.medias.left_arrow.clone(), "back").rounding(8)
                        //     ).clicked() {
                        // });
                        StripBuilder::new(ui)
                            .size(Size::relative(0.1))
                            .size(Size::remainder())
                            .vertical(|mut strip| {
                                strip.cell(|ui| {
                                    StripBuilder::new(ui)
                                        .size(Size::relative(0.2))
                                        .size(Size::relative(0.6))
                                        .size(Size::relative(0.2))
                                        .horizontal(|mut strip| {
                                            strip.cell(|ui| {
                                                if ui.add_sized(
                                                    vec2(70.0, 30.0),
                                                    Button::image_and_text(self.medias.left_arrow.clone(), "back").rounding(8)
                                                ).clicked() {
                                                    self.states.create_template = !self.states.create_template;
                                                    self.states.show_templates = !self.states.show_templates;
                                                }
                                            });

                                            strip.cell(|ui| {
                                                ui.add_sized(vec2(130.0, 30.0), TextEdit::singleline(&mut self.states.new_template_name)
                                                                .font(egui::FontId::new(18.0, egui::FontFamily::Proportional)));
                                            });

                                            strip.cell(|ui| {
                                                if ui.add_sized(
                                                    vec2(70.0, 30.0),
                                                    Button::image_and_text(self.medias.plus.clone(), "create").rounding(8)
                                                ).clicked() {
                                                    self.datas.all_workout_data.create_workout_template(self.states.new_template_name.clone(), self.states.new_template_exercises.clone());
                                                    self.states.reset_new_template_window();
                                                };
                                            });
                                        });
                                });

                                strip.cell(|ui| {
                                    StripBuilder::new(ui)
                                        .size(Size::exact(150.0))
                                        .size(Size::remainder())
                                        .vertical(|mut strip| {
                                            strip.cell(|ui| {
                                                StripBuilder::new(ui)
                                                    .size(Size::relative(0.5))
                                                    .size(Size::relative(0.5))
                                                    .horizontal(|mut strip| {
                                                        strip.cell(|ui| {
                                                            ui.vertical_centered(|ui| {
                                                                workout_tracker_widget_front(ctx, ui, Vec2::new(100.0, 226.0), &self.states.new_template_exercises);
                                                            });
                                                        });

                                                        strip.cell(|ui| {
                                                            ui.vertical_centered(|ui| {
                                                                workout_tracker_widget_behind(ctx, ui, Vec2::new(100.0, 226.0), &self.states.new_template_exercises);
                                                            });
                                                        });
                                                    });
                                            });

                                            strip.cell(|ui| {
                                                ScrollArea::vertical()
                                                    .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
                                                    .show(ui, |ui| {
                                                        if !self.states.new_template_exercises.is_empty() {
                                                            for (index, exercise) in self.states.new_template_exercises.clone().iter().enumerate() {
                                                                ui.vertical_centered(|ui| {
                                                                    ui.set_height(42.0);
                                                                    ui.set_width(350.0);

                                                                    let rect = ui.available_rect_before_wrap();

                                                                    ui.painter().rect_filled(rect, 8, other_elements_color);

                                                                    ui.allocate_ui_at_rect(rect, |ui| {
                                                                        // ui.add_space(10.0);
                                                                        ui.horizontal(|ui| {
                                                                            StripBuilder::new(ui)
                                                                                .size(Size::relative(0.05))
                                                                                .size(Size::relative(0.3))
                                                                                .size(Size::relative(0.3))
                                                                                .size(Size::remainder())
                                                                                .horizontal(|mut strip| {
                                                                                    strip.empty();

                                                                                    strip.cell(|ui| {
                                                                                        ui.vertical(|ui| {
                                                                                            ui.add_space(10.0);
                                                                                            ui.add(Label::new(RichText::new(exercise.to_string()).size(15.0).color(text_color)));
                                                                                        });
                                                                                    });

                                                                                    strip.empty();

                                                                                    strip.cell(|ui| {
                                                                                        ui.vertical_centered(|ui| {
                                                                                            ui.add_space(2.5);
                                                                                            if ui.add_sized(
                                                                                                vec2(35.0, 35.0),
                                                                                                ImageButton::new(Image::new(self.medias.remove.clone())).frame(false)
                                                                                            ).clicked() {
                                                                                                self.states.new_template_exercises.remove(index);
                                                                                            };
                                                                                        });
                                                                                    });
                                                                                });
                                                                            });
                                                                        });
                                                                    });
                                                                ui.add_space(REMAINDER);
                                                            }
                                                        }
                                                    if ui.add(
                                                        Button::image_and_text(self.medias.plus.clone(),
                                                            RichText::new("add exercise")
                                                                .size(16.0)
                                                                .color(text_color),
                                                        )
                                                        .fill(other_elements_color)
                                                        .min_size(vec2(350.0, 42.0))
                                                        .rounding(8),
                                                    ).clicked() {
                                                        self.states.exercises_window = true;
                                                        println!("{:?}", self.states.exercises_window);
                                                    }
                                                        // ui.vertical_centered(|ui| {
                                                        //     ui.set_height(42.0);
                                                        //     ui.set_width(350.0);

                                                        //     let rect = ui.available_rect_before_wrap();

                                                        //     ui.painter().rect_filled(rect, 8, other_elements_color);

                                                        //     ui.allocate_ui_at_rect(rect, |ui| {
                                                        //         ui.add
                                                });
                                            });
                                });
                            });
                        });
                    } else {
                        // ui.horizontal(|ui| {
                        //     if ui.add_sized(
                        //         vec2(70.0, 30.0),
                        //         Button::image_and_text(self.medias.left_arrow.clone(), "back").rounding(8)
                        //     ).clicked() {
                        //         self.states.current_template.clear();
                        //         self.states.show_templates = !self.states.show_templates;
                        //     }

                        //     ui.add_space(90.0);

                        //     if !self.states.current_template.is_empty() {
                        //         ui.add(Label::new(RichText::new(format!("{}", self.datas.all_workout_data.workout_templates.get(&self.states.current_template).unwrap().workout_name)).color(text_color).size(23.0)));
                        //     }
                        // });
                        StripBuilder::new(ui)
                            .size(Size::relative(0.1))
                            .size(Size::remainder())
                            .vertical(|mut strip| {
                                strip.cell(|ui| {
                                    StripBuilder::new(ui)
                                        .size(Size::relative(0.2))
                                        .size(Size::relative(0.6))
                                        .size(Size::relative(0.2))
                                        .horizontal(|mut strip| {
                                            strip.cell(|ui| {
                                                if ui.add_sized(
                                                    vec2(70.0, 30.0),
                                                    Button::image_and_text(self.medias.left_arrow.clone(), "back").rounding(8)
                                                ).clicked() {
                                                    self.states.current_template.clear();
                                                    self.states.show_templates = !self.states.show_templates;
                                                }
                                            });

                                            strip.cell(|ui| {
                                                if !self.states.current_template.is_empty() {
                                                    ui.add(Label::new(RichText::new(format!("{}", self.datas.all_workout_data.workout_templates.get(&self.states.current_template).unwrap().workout_name)).color(text_color).size(23.0)));
                                                }
                                            });

                                            strip.empty();
                                        });
                                });

                                strip.cell(|ui| {
                                    if !self.states.current_template.is_empty() {
                                        StripBuilder::new(ui)
                                            .size(Size::exact(150.0))
                                            .size(Size::remainder())
                                            .vertical(|mut strip| {
                                                strip.cell(|ui| {
                                                    StripBuilder::new(ui)
                                                        .size(Size::relative(0.5))
                                                        .size(Size::relative(0.5))
                                                        .horizontal(|mut strip| {
                                                            strip.cell(|ui| {
                                                                ui.vertical_centered(|ui| {
                                                                    workout_tracker_widget_front(ctx, ui, Vec2::new(100.0, 226.0), &self.datas.all_workout_data.workout_templates.get(&self.states.current_template).unwrap().exercises);
                                                                });
                                                            });

                                                            strip.cell(|ui| {
                                                                ui.vertical_centered(|ui| {
                                                                    workout_tracker_widget_behind(ctx, ui, Vec2::new(100.0, 226.0), &self.datas.all_workout_data.workout_templates.get(&self.states.current_template).unwrap().exercises);
                                                                });
                                                            });
                                                        });
                                                });

                                                strip.cell(|ui| {
                                                    ScrollArea::vertical()
                                                        .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
                                                        .show(ui, |ui| {
                                                            if let Some(template) = self.datas.all_workout_data.workout_templates.get_mut(&self.states.current_template) {
                                                                for (index, exercise) in template.exercises.clone().iter().enumerate() {
                                                                    ui.vertical_centered(|ui| {
                                                                        ui.set_height(42.0);
                                                                        ui.set_width(350.0);

                                                                        let rect = ui.available_rect_before_wrap();

                                                                        ui.painter().rect_filled(rect, 8, other_elements_color);

                                                                        ui.allocate_ui_at_rect(rect, |ui| {
                                                                            // ui.add_space(10.0);
                                                                            ui.horizontal(|ui| {
                                                                                StripBuilder::new(ui)
                                                                                    .size(Size::relative(0.05))
                                                                                    .size(Size::relative(0.3))
                                                                                    .size(Size::relative(0.3))
                                                                                    .size(Size::remainder())
                                                                                    .horizontal(|mut strip| {
                                                                                        strip.empty();

                                                                                        strip.cell(|ui| {
                                                                                            ui.vertical(|ui| {
                                                                                                ui.add_space(10.0);
                                                                                                ui.add(Label::new(RichText::new(exercise.to_string()).size(15.0).color(text_color)));
                                                                                            });
                                                                                        });

                                                                                        strip.empty();

                                                                                        strip.cell(|ui| {
                                                                                            ui.vertical_centered(|ui| {
                                                                                                ui.add_space(2.5);
                                                                                                if ui.add_sized(
                                                                                                    vec2(35.0, 35.0),
                                                                                                    ImageButton::new(Image::new(self.medias.remove.clone())).frame(false)
                                                                                                ).clicked() {
                                                                                                    template.exercises.remove(index);
                                                                                                };
                                                                                            });
                                                                                        });
                                                                                    });
                                                                                });
                                                                            });
                                                                        });
                                                                    ui.add_space(REMAINDER);
                                                                }
                                                            }
                                                        if ui.add(
                                                            Button::image_and_text(self.medias.plus.clone(),
                                                                RichText::new("add exercise")
                                                                    .size(16.0)
                                                                    .color(text_color),
                                                            )
                                                            .fill(other_elements_color)
                                                            .min_size(vec2(350.0, 42.0))
                                                            .rounding(8),
                                                        ).clicked() {
                                                            self.states.exercises_window = true;
                                                        }
                                                            // ui.vertical_centered(|ui| {
                                                            //     ui.set_height(42.0);
                                                            //     ui.set_width(350.0);

                                                            //     let rect = ui.available_rect_before_wrap();

                                                            //     ui.painter().rect_filled(rect, 8, other_elements_color);

                                                            //     ui.allocate_ui_at_rect(rect, |ui| {
                                                            //         ui.add
                                                    });
                                                });
                                            });
                                    };
                                });
                            });
                    }
                // return false;
            });
        });

        if !*open {
            self.states.reset_template_window();
        }

        // *open
    }

    pub fn draw_exercises_window(&mut self, ui: &mut Ui, ctx: &Context, is_dark: bool, elements_color: Color32, other_elements_color: Color32, text_color: Color32, open: &mut bool) {
        let screen_rect = ctx.screen_rect();
        let painter = ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("dark_backdrop")));

        ui.painter().rect_filled(
            screen_rect,
            0.0,
            if is_dark {
                Color32::from_rgba_unmultiplied(20, 20, 20,150)
            } else {
                Color32::from_rgba_unmultiplied(240, 240, 240, 150)
            }
        );

        Area::new("modal_blocker".into())
            .order(Order::Background)
            .fixed_pos(screen_rect.min)
            .show(ctx, |ui| {
                let _response = ui.allocate_response(screen_rect.size(), Sense::click());
            });

        let window_size = vec2(400.0, 500.0);
        let button_size = vec2(300.0, 60.0);

        Window::new("exercises")
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .collapsible(false)
            .resizable(false)
            .interactable(false)
            .open(open)
            .fixed_size(window_size)
            .show(ctx, |ui| {
                ui.add_space(REMAINDER);
                ui.vertical_centered(|ui| {
                    if self.states.show_exercises {
                        ScrollArea::vertical().show(ui, |ui| {
                            ui.add_space(REMAINDER);
                            for exercise in Exercises::iter() {
                                if ui.add(
                                    Button::new(
                                        RichText::new(format!("{}", exercise))
                                            .size(18.0)
                                            .color(text_color),
                                    )
                                    .fill(other_elements_color)
                                    .min_size(button_size)
                                    .rounding(8),
                                ).clicked() {
                                    if self.states.templates_window {
                                        if self.states.create_template {
                                            self.states.new_template_exercises.push(exercise);
                                        } else {
                                            self.datas.all_workout_data.workout_templates.get_mut(&self.states.current_template).unwrap().exercises.push(exercise);
                                        }
                                        self.states.exercises_window = false;
                                    } else {
                                        self.states.current_exercise = exercise.clone();
                                        self.states.show_exercises = !self.states.show_exercises;
                                    }
                                };
                                ui.add_space(10.0);
                            }
                            if ui.add(
                                Button::image_and_text(self.medias.plus.clone(),
                                    RichText::new("create exercise")
                                        .size(18.0)
                                        .color(text_color),
                                )
                                .fill(other_elements_color)
                                .min_size(button_size)
                                .rounding(8),
                            ).clicked() {
                                self.states.show_exercises = !self.states.show_exercises;
                                self.states.create_exercise = !self.states.create_exercise;
                            }
                        });
                    } else {
                    // } else if self.states.create_exercise {
                        StripBuilder::new(ui)
                            .size(Size::relative(0.1))
                            .size(Size::remainder())
                            .vertical(|mut strip| {
                                strip.cell(|ui| {
                                    StripBuilder::new(ui)
                                        .size(Size::relative(0.2))
                                        .size(Size::relative(0.6))
                                        .size(Size::relative(0.2))
                                        .horizontal(|mut strip| {
                                            strip.cell(|ui| {
                                                if ui.add_sized(
                                                    vec2(70.0, 30.0),
                                                    Button::image_and_text(self.medias.left_arrow.clone(), "back").rounding(8)
                                                ).clicked() {
                                                    if self.states.create_exercise {
                                                        self.states.create_exercise = !self.states.create_exercise;
                                                        self.states.show_exercises = !self.states.show_exercises;
                                                    } else {
                                                        self.states.show_exercises = !self.states.show_exercises;
                                                    }
                                                }
                                            });

                                            strip.cell(|ui| {
                                                if self.states.create_exercise {

                                                } else {
                                                    ui.add(Label::new(RichText::new(format!("{}", self.states.current_exercise)).color(text_color).size(23.0)));
                                                }
                                                // ui.add_sized(vec2(130.0, 30.0), TextEdit::singleline(&mut self.states.new_template_name)
                                                //                 .font(egui::FontId::new(18.0, egui::FontFamily::Proportional)));
                                            });

                                            if self.states.create_exercise {
                                                strip.cell(|ui| {
                                                    if ui.add_sized(
                                                        vec2(70.0, 30.0),
                                                        Button::image_and_text(self.medias.plus.clone(), "create").rounding(8)
                                                    ).clicked() {
                                                        self.states.create_exercise = !self.states.create_exercise;
                                                        self.states.show_exercises = !self.states.show_exercises;
                                                        // self.datas.all_workout_data.create_workout_template(self.states.new_template_name.clone(), self.states.new_template_exercises.clone());
                                                        // self.states.reset_new_template_window();
                                                    };
                                                });
                                            } else {
                                                strip.empty();
                                            }
                                        });
                                });

                                strip.cell(|ui| {
                                    if self.states.create_exercise {

                                    } else {
                                        StripBuilder::new(ui)
                                            .size(Size::exact(150.0))
                                            .size(Size::remainder())
                                            .vertical(|mut strip| {
                                                strip.cell(|ui| {
                                                    StripBuilder::new(ui)
                                                        .size(Size::relative(0.5))
                                                        .size(Size::relative(0.5))
                                                        .horizontal(|mut strip| {
                                                            strip.cell(|ui| {
                                                                ui.vertical_centered(|ui| {
                                                                    workout_tracker_widget_front(ctx, ui, Vec2::new(100.0, 226.0), &vec![self.states.current_exercise.clone()]);
                                                                });
                                                            });

                                                            strip.cell(|ui| {
                                                                ui.vertical_centered(|ui| {
                                                                    workout_tracker_widget_behind(ctx, ui, Vec2::new(100.0, 226.0), &vec![self.states.current_exercise.clone()]);
                                                                });
                                                            });
                                                        });
                                                });

                                                strip.cell(|ui| {
                                                    ui.add(Label::new(RichText::new("statistics:").size(18.0).color(text_color)));
                                                })
                                            });
                                    }
                                });
                            });
                        }
                    // } else {
                    //     StripBuilder::new(ui)
                    //         .size(Size::relative(0.1))
                    //         .size(Size::remainder())
                    //         .vertical(|mut strip| {
                    //             strip.cell(|ui| {
                    //                 StripBuilder::new(ui)
                    //                     .size(Size::relative(0.2))
                    //                     .size(Size::relative(0.6))
                    //                     .size(Size::relative(0.2))
                    //                     .horizontal(|mut strip| {
                    //                         strip.cell(|ui| {
                    //                             if ui.add_sized(
                    //                                 vec2(70.0, 30.0),
                    //                                 Button::image_and_text(self.medias.left_arrow.clone(), "back").rounding(8)
                    //                             ).clicked() {
                    //                                 self.states.show_exercises = !self.states.show_exercises;
                    //                             }
                    //                         });

                    //                         strip.cell(|ui| {
                    //                             ui.add(Label::new(RichText::new(format!("{}", self.states.current_exercise)).color(text_color).size(23.0)));
                    //                         });

                    //                         strip.empty();
                    //                     });
                    //             });

                    //             strip.cell(|ui| {
                    //                 StripBuilder::new(ui)
                    //                     .size(Size::exact(150.0))
                    //                     .size(Size::remainder())
                    //                     .vertical(|mut strip| {
                    //                         strip.cell(|ui| {
                    //                             StripBuilder::new(ui)
                    //                                 .size(Size::relative(0.5))
                    //                                 .size(Size::relative(0.5))
                    //                                 .horizontal(|mut strip| {
                    //                                     strip.cell(|ui| {
                    //                                         ui.vertical_centered(|ui| {
                    //                                             workout_tracker_widget_front(ctx, ui, Vec2::new(100.0, 226.0), &vec![self.states.current_exercise.clone()]);
                    //                                         });
                    //                                     });

                    //                                     strip.cell(|ui| {
                    //                                         ui.vertical_centered(|ui| {
                    //                                             workout_tracker_widget_behind(ctx, ui, Vec2::new(100.0, 226.0), &vec![self.states.current_exercise.clone()]);
                    //                                         });
                    //                                     });
                    //                                 });
                    //                         });

                    //                         strip.cell(|ui| {
                    //                             ui.add(Label::new(RichText::new("statistics:").size(18.0).color(text_color)));
                    //                         })
                    //                     });
                                // });

                            // });
                    // }
                });
            });

        if !*open {
            self.states.exercises_window = false;
        }
    }
}
