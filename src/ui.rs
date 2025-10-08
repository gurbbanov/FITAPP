use egui::{Layout, Context, ColorImage, ImageSource, ScrollArea, Ui, Image, Color32, TextStyle, RichText, Align, Vec2, Rounding, Label, Button, vec2, ImageButton, Rect, Pos2, scroll_area::ScrollBarVisibility, Stroke, StrokeKind};
use eframe::{Frame};
use egui_extras::{Size, Strip, StripBuilder};
use time::{OffsetDateTime};
use chrono::{Local, Datelike, Duration, NaiveDate};

use crate::models::{AppMedia, States, UserDataPack, WorkoutPlanned, WorkoutTemplate};
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

    pub fn home(&mut self, ctx: &Context, frame: &mut Frame, ui: &mut Ui) {
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
                                        ui.label(egui::RichText::new(format!("{}", self.datas.user_information.name)).size(20.0).strong());
                                        ui.label(egui::RichText::new(format!("@{}", self.datas.user_information.username)).size(15.0));
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
                                    ui.add_sized(egui::vec2(ui.available_width(), 10.0), egui::ProgressBar::new(0.0).show_percentage());
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
                                ui.spacing_mut().item_spacing = egui::vec2(1.0, -3.0);

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
                                    ui.spacing_mut().item_spacing = egui::vec2(1.0, -3.0);
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
                                                    workout_tracker_widget_front(ctx, frame, ui, Vec2::new(100.0, 226.0));
                                                });
                                            });

                                            strip.cell(|ui| {
                                                ui.vertical_centered(|ui| {
                                                    workout_tracker_widget_behind(ctx, frame, ui, Vec2::new(100.0, 226.0));
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
    }

    pub fn workouts_ui(&mut self, ctx: &Context, frame: &mut Frame, ui: &mut Ui, elements_color: Color32, tint_color: Color32) {
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
                            let side_rect =egui::Rect::from_min_size(
                                // top_rect.left_top() + egui::vec2(50.0, top_rect.height() + 25.0),
                                ctx.screen_rect().left_top() + vec2(50.0, 125.0),
                                egui::vec2(ui.available_width() - 100.0, 600.0),
                            );

                            let id = egui::Id::new("drag_x_rect");

                            let mut pos = ctx.data(|data| {
                                data.get_temp::<Pos2>(id).unwrap_or(side_rect.left_top())
                            });

                            let response = ui.allocate_rect(side_rect, egui::Sense::drag());

                            if self.datas.planned_workout_data.workouts.contains_key(&self.states.selected_day) {
                                ui.vertical_centered(|ui| {
                                    if response.dragged() {
                                        let delta = response.drag_delta();
                                        pos.y += delta.y; 

                                        ctx.data_mut(|data| {
                                            data.insert_temp(id, pos);
                                        });
                                    } else if pos.y > ctx.screen_rect().center().y - side_rect.height() / 1.85 {
                                        // while pos.y > ctx.screen_rect().center().y - side_rect.height() / 1.9 {
                                        //     pos.y -= 0.1;
                                        // }

                                        // pos.y = ctx.screen_rect().center().y - side_rect.height() / 1.9;
                                        let side_height = side_rect.height();
                                        let target_y = ctx.screen_rect().center().y - side_height / 1.85;

                                        let dt = ctx.input(|i| i.stable_dt);
                                        let speed = 600.0; // пикселей в секунду

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

                                    // self.draw_workout_card(ctx, frame, ui, pos, side_rect, self.datas.planned_workout_data[0].template.clone());
                                    self.draw_workout_card(ctx, frame, ui, pos, side_rect, self.states.selected_day.clone());

                                    let bot_rect =egui::Rect::from_min_size(
                                        // top_rect.left_top() + egui::vec2(50.0, top_rect.height() + 25.0),
                                        side_rect.left_bottom() + vec2(0.0, 50.0),
                                        egui::vec2(side_rect.width(), side_rect.height()),
                                    );

                                    ui.painter().rect_filled(
                                        Rect::from_min_size(pos + vec2(0.0, side_rect.height() + 50.0), vec2(bot_rect.width(), bot_rect.height())), 
                                        egui::epaint::Rounding {
                                            nw: 24,
                                            ne: 24,
                                            sw: 24,
                                            se: 24,
                                        },
                                        elements_color,
                                    );

                                    ui.allocate_ui_at_rect(Rect::from_min_size(pos + vec2(0.0, side_rect.height() + 50.0), vec2(side_rect.width(), side_rect.height())),|ui| {
                                        ui.vertical_centered(|ui| {
                                            ui.add_space(70.0);
                                            ui.add(Label::new(RichText::new("add more workout").size(30.0)).selectable(false));
                                            ui.add_space(150.0);
                                            ui.add_sized(vec2(100.0, 100.0), Image::new(self.medias.plus.clone()));
                                            ui.add_space(160.0);

                                            if (ui.add(
                                                Button::new(RichText::new("add workout").size(22.0).color(Color32::WHITE))
                                                    //     egui::Color32::from_rgb(91, 0, 113),
                                                    .fill(Color32::from_rgb(0, 75, 141)) 
                                                    .min_size(Vec2::new(side_rect.width() / 2.5, 40.0))
                                                    .rounding(10),
                                                // .stroke(egui::Stroke::new(1.0, Color32::WHITE)), 
                                            )).clicked() {
                                                self.datas.planned_workout_data.add_workout(self.states.selected_day, WorkoutPlanned::leg_day(self.states.selected_day));
                                            };
                                        });
                                    });
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
                                    ui.vertical_centered(|ui| {
                                        ui.add_space(70.0);
                                        ui.add(Label::new(RichText::new("not planned").size(30.0)).selectable(false));
                                        ui.add_space(40.0);
                                        ui.add_sized(vec2(250.0,250.0), Image::new(self.medias.calendar.clone()));
                                        ui.add_space(120.0);
                                    });

                                    ui.horizontal(|ui| {
                                        ui.add_space(side_rect.width() / 13.0);
                                    
                                        ui.add(
                                            Button::new(RichText::new("rest").size(22.0).color(Color32::WHITE))
                                                //     egui::Color32::from_rgb(91, 0, 113),
                                                .fill(Color32::from_rgb(91, 0, 113)) // цвет фона кнопки
                                                .min_size(Vec2::new(side_rect.width() / 2.5, 40.0))
                                                .rounding(10),
                                                // .stroke(egui::Stroke::new(1.0, Color32::WHITE)), // рамка
                                        );
                                    
                                        let padding = side_rect.width() - (((side_rect.width() / 13.0) * 2.0) + ((side_rect.width() / 2.5) * 2.0)) - 8.0;
                                        ui.add_space(padding);
                                    
                                        if (ui.add(
                                            Button::new(RichText::new("add workout").size(22.0).color(Color32::WHITE))
                                                //     egui::Color32::from_rgb(91, 0, 113),
                                                .fill(Color32::from_rgb(0, 75, 141)) 
                                                .min_size(Vec2::new(side_rect.width() / 2.5, 40.0))
                                                .rounding(10),
                                            // .stroke(egui::Stroke::new(1.0, Color32::WHITE)), 
                                        )).clicked() {
                                            self.datas.planned_workout_data.add_workout(self.states.selected_day, WorkoutPlanned::leg_day(self.states.selected_day));
                                        };
                                    });
                                });
                            };

                            // let bot_rect =egui::Rect::from_min_size(
                            //     // top_rect.left_top() + egui::vec2(50.0, top_rect.height() + 25.0),
                            //     side_rect.left_bottom() + vec2(0.0, 50.0),
                            //     egui::vec2(side_rect.width(), side_rect.height()),
                            // );

                            // ui.vertical_centered(|ui| {
                            //     let id = egui::Id::new("drag_x+1_rect");

                            //     let mut pos = ctx.data(|data| {
                            //         data.get_temp::<Pos2>(id).unwrap_or(bot_rect.left_bottom())
                            //     });

                            //     let response = ui.allocate_rect(bot_rect, egui::Sense::drag());

                            //     if response.dragged() {
                            //         let delta = response.drag_delta();
                            //         pos.y += delta.y; 

                            //         ctx.data_mut(|data| {
                            //             data.insert_temp(id, pos);
                            //         });
                            //     }
                                    
                            //         // ctx.data_mut(|data| {
                            //         //     data.insert_temp(id, pos);
                            //         // });

                            //     ui.painter().rect_filled(
                            //         Rect::from_min_size(pos, vec2(bot_rect.width(), bot_rect.height())), 
                            //         egui::epaint::Rounding {
                            //             nw: 24,
                            //             ne: 24,
                            //             sw: 24,
                            //             se: 24,
                            //         },
                            //         elements_color,
                            //     );
                            // });

                        });

                        let top_rect = egui::Rect::from_min_size(
                            ctx.screen_rect().left_top(),
                            egui::vec2(ui.available_width(), 100.0),
                        );

                        Self::draw_rect_with_black_shadow(ui.painter(), top_rect, 24, elements_color, 6.0, [(5.0, 20), (3.0, 25), (2.0, 30),], Rounding {
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

                        ui.with_layout(Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                            ui.add(Image::new(self.medias.ambient.clone()).tint(tint_color).fit_to_exact_size(vec2(ui.available_width(), ui.available_height() * 1.5)));
                        });
                    });

                strip.cell(|ui| {
                    let bot_rect = egui::Rect::from_min_size(
                        ctx.screen_rect().left_bottom() - vec2(0.0, 150.0),
                        ctx.screen_rect().right_bottom().to_vec2(),
                    );

                    // let bot_rect = ui.available_rect_before_wrap();

                    Self::draw_rect_with_black_shadow(ui.painter(), bot_rect, 24, elements_color, -4.0, [(2.0, 20), (3.0, 25), (5.0, 30)], Rounding {
                        nw: 24,
                        ne: 24,
                        sw: 0,
                        se: 0,
                    });

                    // StripBuilder::new(ui)
                    //     .size(Size::remainder())
                    //     // .size(Size::relative(0.08))
                    //     // .size(Size::relative(0.8))
                    //     // .size(Size::relative(0.1))
                    //     .horizontal(|mut strip| {
                            // strip.cell(|ui| {
                            //     ui.with_layout(Layout::bottom_up(egui::Align::TOP), |ui| {
                            //     // ui.with_layout(Layout::bottom_up(egui::Align::TOP), add_contents)
                            //         ui.add_space(100.0);
                            //         if self.states.skip_days >= 0 {
                            //             if ui.add_sized([30.0, 30.0], ImageButton::new(self.medias.left_arrow.clone()).frame(false)).clicked() {
                            //                 self.states.skip_days -= 7;
                            //             };
                            //         }
                            //     });
                            // });

                            let weekdays = ["mo", "tu", "we", "th", "fr", "sa", "su"];

                            // strip.cell(|ui| {
                                ui.allocate_ui_at_rect(bot_rect, |ui| {
                                    ui.vertical_centered_justified(|ui| {
                                        ui.add_space(REMAINDER);

                                        // let mut available_width = ui.available_width() * 0.8;
                                        let mut available_width = ui.available_width();
                                        ui.set_width(available_width);

                                        let rect_width = ui.available_width() / 10.0;

                                        let today = now.date_naive();

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
                                                        println!("selected: {:?}", self.states.selected_day);
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
                            // });

                            // strip.cell(|ui| {
                            //     ui.vertical_centered(|ui| {
                            //         // ui.add_space(45.0);
                            //         if self.states.skip_days <= 0 {
                            //             if ui.add_sized([30.0, 30.0], ImageButton::new(self.medias.right_arrow.clone()).frame(false)).clicked() {
                            //                 self.states.skip_days += 7;
                            //             };
                            //         }
                            //     });
                            // });
                        // });
                    });
                });
    }

    pub fn calory_tracker_ui(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, ui: &mut egui::Ui, elements_color: Color32, tint_color: Color32) {
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

        StripBuilder::new(ui)
            // .size(Size::exact(100.0))
            // .size(Size::relative(0.8))
            // .size(Size::relative(0.73))
            .size(Size::remainder())
            .size(Size::exact(self.states.strip_size))
            .vertical(|mut strip|{
                strip.cell(|ui| {
                    ScrollArea::vertical()
                        .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
                        .show(ui, |ui| {
                        let top_rect = egui::Rect::from_min_size(
                            ctx.screen_rect().left_top(),
                            egui::vec2(ui.available_width(), 100.0),
                        );
                    // let top_rect = ui.available_rect_before_wrap();

                    Self::draw_rect_with_black_shadow(ui.painter(), top_rect, 24, elements_color, 6.0, [(5.0, 20), (3.0, 25), (2.0, 30),], Rounding {
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
                // });

                // strip.cell(|ui| {
                    ui.add_space(45.0);

                    ui.vertical_centered(|ui| {
                        ui.add(Label::new(RichText::new("CALORIES").size(25.0).strong()).selectable(false));

                        let calory_rect =egui::Rect::from_min_size(
                            // top_rect.left_top() + egui::vec2(50.0, top_rect.height() + 25.0),
                            ctx.screen_rect().left_top() + vec2(150.0, 155.0),
                            egui::vec2(ui.available_width() - 300.0, 100.0),
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
                            ui.add_space(calory_rect.width() / 10.0);

                            ui.vertical_centered(|ui| {
                                ui.label(RichText::new(format!("{}/{}", self.datas.macro_data.calory_registered, self.datas.macro_data.calory_goal)).size(35.0).strong());
                            });
                        });
                    });

                    ui.add_space(50.0);

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
                            Pos2::new((available_width/ 2.0 ) - (rect_length / 2.0), ctx.screen_rect().min.y + 405.0),
                            Vec2::new(rect_length, ctx.screen_rect().min.y  + 50.0)
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
                                    ui.label(RichText::new("carbs").strong().color(egui::Color32::from_rgb(141, 54, 0)).size(14.0));
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
                                    ui.label(RichText::new("proteins").strong().color(egui::Color32::from_rgb(0, 75, 140)).size(14.0));
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
                                    ui.label(RichText::new("fats").strong().color(egui::Color32::from_rgb(141, 0, 19)).size(14.0));
                                    ui.label(RichText::new(format!("{}/{}", self.datas.macro_data.fat_registered, self.datas.macro_data.fat_goal)).size(23.0).strong());
                                    ui.add_space(5.0);
                                    ui.horizontal(|ui| {
                                        ui.add_space(31.0);
                                        self.mini_tracker_bar(ctx, frame, ui, spacing, rect_size, 5, self.datas.macro_data.fat_registered, self.datas.macro_data.fat_goal);
                                    });
                                });
                            });
                        });
                                    // ui.with_layout(Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                                    //     ui.add(Image::new(self.medias.ambient.clone()).tint(tint_color).fit_to_exact_size(vec2(ui.available_width(), ui.available_height() * 1.5)));
                                    // });

                        StripBuilder::new(ui)
                        .size(Size::exact(30.0))
                        .size(Size::exact(10.0))
                        .size(Size::remainder())
                        .size(Size::exact(10.0))
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
                                });

                                strip.empty();
                            });


                        // let bot_rect = ui.available_rect_before_wrap();
                        // let bot_rect = egui::Rect::from_min_size(
                        //     ctx.screen_rect().left_bottom() - vec2(0.0, 130.0),
                        //     ctx.screen_rect().right_bottom().to_vec2(),
                        // );

                        // Self::draw_rect_with_black_shadow(ui.painter(), bot_rect, 110, elements_color, -4.0, [(2.0, 20), (3.0, 25), (5.0, 30)], Rounding {
                        //     nw: 110,
                        //     ne: 110,
                        //     sw: 0,
                        //     se: 0,
                        // });

                        // ui.vertical_centered(|ui| {
                        //     ui.allocate_ui_at_rect(bot_rect, |ui| {
                        //         ui.add_space(30.0);
                        //         if ui.add(
                        //             Button::new(RichText::new("add calories").size(18.0).strong().color(Color32::WHITE))
                        //                 //     egui::Color32::from_rgb(91, 0, 113),
                        //                 .fill(Color32::from_rgb(21, 141, 0)) 
                        //                 .min_size(Vec2::new(120.0, 40.0))
                        //                 .rounding(12),
                        //         ).clicked() {
                        //             self.states.strip_size = 400.0;
                        //         };
                        //     });
                        // });
                    });
                });
            });

                strip.cell(|ui| {
                    let bot_rect = ui.available_rect_before_wrap();
                    let mut target_height = 150.0;

                    if self.states.calory_add_clicked {
                        target_height = 300.0;
                    };

                    self.states.strip_size = ui.ctx().animate_value_with_time(
                        ui.id().with("history_height"),
                        target_height,
                        1.0, 
                    );


                    Self::draw_rect_with_black_shadow(ui.painter(), bot_rect, 110, elements_color, -4.0, [(2.0, 20), (3.0, 25), (5.0, 30)], Rounding {
                        nw: 110,
                        ne: 110,
                        sw: 0,
                        se: 0,
                    });

                    ui.vertical_centered(|ui| {
                        ui.allocate_ui_at_rect(bot_rect, |ui| {
                            if !self.states.calory_add_clicked {
                                ui.add_space(30.0);
                                if ui.add(
                                    Button::new(RichText::new("add calories").size(18.0).strong().color(Color32::WHITE))
                                        //     egui::Color32::from_rgb(91, 0, 113),
                                        .fill(Color32::from_rgb(21, 141, 0)) 
                                        .min_size(Vec2::new(120.0, 40.0))
                                        .rounding(12),
                                ).clicked() {
                                    self.states.calory_add_clicked = true;
                                };
                            } else {
                                // ui.add_space(7.0);
                                StripBuilder::new(ui)
                                    .size(Size::exact(110.0))
                                    .size(Size::exact(10.0))
                                    .size(Size::remainder())
                                    .size(Size::exact(75.0))
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
                                                                    self.states.calory_add_clicked = false;
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
                                                            ui.allocate_ui_at_rect(calory_rect, |ui| {
                                                            // let frame = egui::Frame::none()
                                                            //     .fill(other_elements_color)
                                                            //     .rounding(25.0)
                                                            //     .stroke(Stroke::new(2.0, Color32::from_rgb(158, 91, 50)));

                                                            // frame.show(ui, |ui| {
        //                                                             let frame = egui::Frame::none()
        //     .fill(other_elements_color)
        //     .rounding(25.0)
        //     .stroke(Stroke::new(2.0, Color32::from_rgb(37, 99, 153)));

        // frame.show(ui, |ui| {
        ui.centered_and_justified(|ui| {

            ui.add_sized([calory_rect.width() - 20.0, calory_rect.height() - 30.0], egui::TextEdit::singleline(&mut self.states.calory_add_value));
        });
        // });
                                                            // });
                                                        });

                                                            // ui.painter().rect_filled(calory_rect, 25.0, other_elements_color);
                                                            ui.painter().rect_stroke(calory_rect, 25.0, Stroke::new(2.0, Color32::BLACK), StrokeKind::Outside);
                                                        });

                                                    });

                                                    strip.cell(|ui| {
                                                        ui.add_space(7.0);
                                                        ui.vertical_centered(|ui| {
                                                            ui.add_sized(vec2(85.0, 60.0), ImageButton::new(if is_dark {self.medias.save_button_d.clone()} else {self.medias.save_button_l.clone()})
                                                                .frame(false));
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
                                                            ui.painter().rect_stroke(carbs_rect, 25.0, Stroke::new(2.0, Color32::from_rgb(158, 91, 50)), StrokeKind::Outside);
                                                        });
                                                    });

                                                    strip.cell(|ui| {
                                                        ui.set_width(130.0);
                                                        ui.vertical_centered(|ui| {
                                                            ui.add(Label::new(RichText::new("add carbs:").size(18.0).color(text_color).strong()).selectable(false));
                                                            ui.add_space(REMAINDER);

                                                            let fats_rect = ui.available_rect_before_wrap();

                                                            ui.painter().rect_filled(fats_rect, 25.0, other_elements_color);
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

        // StripBuilder::new(ui)
        //     .size(Size::relative(0.1))
        //     .size(Size::relative(0.8))
        //     .size(Size::relative(0.1))
        //     .horizontal(|mut strip|{

        //         strip.empty();

        //         strip.cell(|ui| {
        //             // ui.allocate_ui_at_rect(ui.available_rect(), |ui| {
        //             //     ui.set_min_size(vec2(500.0, 600.0));

        //             ScrollArea::vertical()
        //                 .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
        //                 .show(ui, |ui| {
        //                     ui.vertical_centered(|ui| {
        //                         ui.group(|ui| {
        //                             ui.set_min_size(vec2(450.0, 600.0));
        //                                 ui.set_min_size(vec2(0.0, 150.0));
        //                                 let top_left = ui.min_rect().min;

        //                                 let my_rect = Rect::from_min_size(top_left + vec2(0.0, 120.0), vec2(450.0,600.0));

        //                                 ui.painter().rect_filled(my_rect, 5.0, Color32::from_rgb(60, 120, 200));

        //                         });

        //                         ui.add_space(20.0);

        //                         ui.group(|ui| {
        //                             ui.set_min_height(300.0);
        //                             ui.label("Нижний блок");
        //                         });

        //                         ui.add_space(100.0);
        //                     })
        //                 });
        //             });

        //         strip.empty();
        //     });
    }

    pub fn water_tracker_ui(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, ui: &mut egui::Ui, elements_color: Color32, tint_color: Color32) {
        // let ctx = ui.ctx();
        // let rect = ui.max_rect();

        // let layer1 = ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new("layer1")));
        // layer1.rect_filled(rect.shrink(20.0), 10.0, Color32::LIGHT_BLUE);


        // // let layer2 = ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new("layer2")));

        // // layer2.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
        // // layer2.add(Image::new(self.medias.ambient.clone()));
        // let layer2 = ctx.layer_painter(egui::LayerId::new(
        //     egui::Order::Foreground,
        //     egui::Id::new("layer2"),
        // ));

        // let rect = ui.max_rect();

        // layer2.image(
        //     self.medias.ambient_test.id(),
        //     rect,
        //     Rect::from_min_max(egui::pos2(0.0, 0.0),
        //     egui::pos2(1.0, 1.0)),
        //     egui::Color32::WHITE, // tint
        // );

        // // });
    }
            
    pub fn navigation_bar(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, ui: &mut egui::Ui) {
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
            // 0 => self.testing(ctx, frame, ui),
            0 => self.home(ctx, frame, ui),
            // 1 => self.friends_ui(ctx, frame, ui),
            2 => self.workouts_ui(ctx, frame, ui, elements_color, tint_color),
            3 => self.calory_tracker_ui(ctx, frame, ui, elements_color, tint_color),
            4 => self.water_tracker_ui(ctx, frame, ui, elements_color, tint_color),
            // 5 => self.statistics_ui(ctx, frame, ui),
            _ => {ui.label("empty");},
        }
    }
        //     0 => self.home(ctx, frame, ui),
        //     // 1 => self.friends_ui(ctx, frame, ui),
        //     2 => self.workouts_ui(ctx, frame, ui),
        //     3 => self.calory_tracker_ui(ctx, frame, ui),
        //     // 4 => self.water_tracker_ui(ctx, frame, ui, Some(ambient)),
        //     // 5 => self.statistics_ui(ctx, frame, ui),
        //     _ => {ui.label("empty");},
        // }
    fn calory_tracker_bar(&mut self, ctx: &Context, frame: &mut eframe::Frame, ui: &mut egui::Ui, spacing: f32, rect_size: f32, rows: u32, cols: u32, calory_percent: u32) {
        ui.spacing_mut().item_spacing = egui::vec2(1.0, -3.0);

        let mut green_rects = {
            if self.datas.macro_data.calory_goal > self.datas.macro_data.calory_registered{
                (((rows * cols) as f32 / 100.0) * calory_percent as f32).round() as u32
            } else {
                0
            //    rows * cols 
            }
        };

        ui.vertical_centered(|ui| {
            for _ in 0..rows {
                ui.horizontal(|ui| {
                    for col in 0..cols {
                        let (rect, _) = ui.allocate_exact_size(
                            egui::vec2(rect_size, rect_size),
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

    fn water_tracker_bar(&mut self, ctx: &Context, frame: &mut eframe::Frame, ui: &mut egui::Ui, spacing: f32, circle_size: f32, rows: u32, cols: u32, water_percent: u32) {
        ui.spacing_mut().item_spacing = egui::vec2(1.0, 3.0);

        let mut done_marks = {
            if self.datas.macro_data.calory_goal> self.datas.macro_data.calory_registered{
                (((rows * cols) as f32 / 100.0) * water_percent as f32).round() as u32
            } else {
            //    rows * cols
                0
            }
        };

        ui.vertical_centered(|ui| {
            for _ in 0..rows {
                ui.horizontal(|ui| {
                    for col in 0..cols {
                        let (rect, _) = ui.allocate_exact_size(
                            egui::vec2(circle_size, circle_size),
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

    fn mini_tracker_bar(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, ui: &mut egui::Ui, spacing: f32, rect_size:f32, cols: i32, registered: u32, goal: u32) {
        ui.spacing_mut().item_spacing = egui::vec2(1.0, -3.0); 

        let ROWS = 5;
        let COLUMNS = 5;

        let calory_percent = ((registered as f32 / goal as f32) * 100.0) as u32;

        let mut remaining = if goal > registered {
            (((ROWS * COLUMNS) as f32 / 100.0) * calory_percent as f32).round() as u32
        } else {
            0
        };

        ui.vertical(|ui| {
            for _ in 0..ROWS {
                ui.horizontal(|ui| {
                    for col in 0..COLUMNS {
                        let (rect, _) = ui.allocate_exact_size(
                            egui::vec2(rect_size, rect_size),
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

    fn draw_workout_card(&mut self, ctx: &Context, frame: &mut Frame, ui: &mut Ui, pos: Pos2, side_rect: Rect, selected_day: NaiveDate) {
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
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                    ui.vertical_centered(|ui| {
                        ui.add_space(30.0);
                        ui.label(RichText::new(&self.datas.planned_workout_data.workouts.get(&selected_day).unwrap()[0].template.workout_name).size(27.0).strong());
                        ui.add_space(50.0);
                    
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.set_width(side_rect.width() / 2.0);
                                ui.vertical_centered(|ui| {
                                    workout_tracker_widget_front(ctx, frame, ui, Vec2::new(110.0, 249.0));
                                });
                            });
                    
                            ui.vertical(|ui| {
                                ui.set_width(side_rect.width() / 2.0);
                                ui.vertical_centered(|ui| {
                                    workout_tracker_widget_behind(ctx, frame, ui, Vec2::new(110.0, 249.0));
                                });
                            });
                        });
                    
                        ui.add_space(50.0);
                    
                        ui.add(
                            Button::new(RichText::new("start").size(22.0).strong().color(Color32::WHITE))
                                //     egui::Color32::from_rgb(91, 0, 113),
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
                    
                            ui.add(
                                Button::new(RichText::new("change workout").size(15.0).strong().color(Color32::WHITE))
                                    .fill(Color32::from_rgb(0, 75, 141))
                                    .min_size(Vec2::new(button_width, 30.0))
                                    .rounding(8),
                            );
                    
                            ui.add_space(spacing);
                    
                            ui.add(
                                Button::new(RichText::new("rest").size(18.0).strong().color(Color32::WHITE))
                                    .fill(Color32::from_rgb(91, 0, 113))
                                    .min_size(Vec2::new(button_width, 30.0))
                                    .rounding(8),
                            );
                    
                            ui.add_space(spacing);
                    
                            if ui.add(
                                Button::new(RichText::new("skip").size(18.0).color(Color32::WHITE))
                                    .fill(Color32::from_rgb(141, 0, 19))
                                    .min_size(Vec2::new(button_width, 30.0))
                                    .rounding(8),
                            ).clicked() {
                                println!("{:?}", self.datas.planned_workout_data.remove_workout(selected_day, self.datas.planned_workout_data.workouts.get(&selected_day).unwrap()[0].clone()));
                            };
                        });
                    });
                });
                // }
            });
    }

    fn draw_rect_with_black_shadow(painter: &egui::Painter, rect: egui::Rect, rounding: u8, fill: Color32, offset_y: f32, layer: [(f32, u8); 3], corners: Rounding) {
        let shadow_color = |alpha: u8| Color32::from_rgba_unmultiplied(0, 0, 0, alpha);

        let shadow_offset = Vec2::new(0.0, offset_y);

        for (inflate_by, alpha) in layer {
            let shadow_rect = rect
                .translate(shadow_offset)
                .expand(inflate_by);
            painter.rect_filled(shadow_rect, egui::Rounding::same(rounding + inflate_by as u8), shadow_color(alpha));
        }

        painter.rect_filled(rect, corners
        ,fill);
    }
}