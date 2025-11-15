use eframe::egui;
use eframe::egui::{vec2, Color32, Pos2, Shape, Stroke, Vec2};
use serde::de;
use crate::models::{Exercise, Muscle, muscle_for_workout};

pub fn workout_tracker_widget_front(ctx: &egui::Context, ui: &mut egui::Ui, size: Vec2, exercises: &Vec<Exercise>) {
    let (primary_muscles, secondary_muscles) = muscle_for_workout(exercises);

    let (rect, _response) = ui.allocate_exact_size(size, egui::Sense::hover());

    let default_size = Vec2::new(120.0, 270.0);
    let scale = (size.x * size.y) / (default_size.x * default_size.y);

    let painter = ui.painter_at(rect);

    let center = rect.center() - vec2(0.0, 120.0 * scale);

    painter.circle_filled(
        center,
        20.0 * scale,
        Color32::GRAY,
    );

    //NECKS
    let neck_width = 10.0 * scale;
    let neck_height = 15.0 * scale;

    let neck_center = center + egui::vec2(0.0, 30.0 * scale); 
    let neck_min = neck_center - egui::vec2(neck_width / 2.0, neck_height / 2.0);
    let neck_max = neck_center + egui::vec2(neck_width / 2.0, neck_height / 2.0);

    let neck_to_draw = egui::Rect::from_min_max(neck_min, neck_max);

    painter.rect_filled(
        neck_to_draw,
        0.0, 
        Color32::GRAY,
    );

    //TRAPS
    let T_left= vec![
        Pos2::new(neck_min.x - 2.0 * scale, neck_min.y + 5.0 * scale),
        Pos2::new(neck_min.x - 20.0 * scale, neck_max.y),
        Pos2::new(neck_min.x - 2.0 * scale, neck_max.y),
    ];

    let T_right= vec![
        Pos2::new(neck_max.x + 2.0 * scale, neck_min.y + 5.0 * scale),
        Pos2::new(neck_max.x + 20.0 * scale, neck_max.y),
        Pos2::new(neck_max.x + 2.0 * scale, neck_max.y),
    ];

    let muscle_color = Color32::GRAY;
    let border = Stroke::new(0.0, Color32::GRAY);

    painter.add(Shape::convex_polygon(T_left, ColorQualifier(Muscle::Traps, &primary_muscles, &secondary_muscles), border));
    painter.add(Shape::convex_polygon(T_right, ColorQualifier(Muscle::Traps, &primary_muscles, &secondary_muscles), border));

    //TOP CHEST (TC)
    let chest_center = neck_center + vec2(0.0, 9.0 * scale);
    let TC_chest_width = 35.0 * scale;
    let TC_chest_height = 20.0 * scale;
    let offset = 1.5 * scale;

    //left chest
    let TC_left_top_l = Pos2::new(chest_center.x - offset - TC_chest_width + 10.0 * scale, chest_center.y);
    let TC_right_top_l = Pos2::new(chest_center.x - offset, chest_center.y);
    let TC_right_bot_l = Pos2::new(chest_center.x - offset, chest_center.y + TC_chest_height);
    let TC_left_bot_l = Pos2::new(chest_center.x - offset - TC_chest_width, chest_center.y + TC_chest_height);
    let TC_left = vec![TC_left_top_l, TC_right_top_l, TC_right_bot_l, TC_left_bot_l];

    //right chest
    let TC_left_top_r = Pos2::new(chest_center.x + offset, chest_center.y);
    let TC_right_top_r = Pos2::new(chest_center.x + offset + TC_chest_width - 10.0 * scale, chest_center.y);
    let TC_right_bottom_r = Pos2::new(chest_center.x + offset + TC_chest_width, chest_center.y + TC_chest_height);
    let TC_left_bottom_r = Pos2::new(chest_center.x + offset, chest_center.y + TC_chest_height);
    let TC_right= vec![TC_left_top_r, TC_right_top_r, TC_right_bottom_r, TC_left_bottom_r];

    // let chest_color = Color32::from_rgb(0, 75, 141);

    painter.add(Shape::convex_polygon(TC_left, ColorQualifier(Muscle::UpperChest, &primary_muscles, &secondary_muscles), border));
    painter.add(Shape::convex_polygon(TC_right, ColorQualifier(Muscle::UpperChest, &primary_muscles, &secondary_muscles), border));
    // Color32::from_rgb(0, 136, 255)

    //BOTTOM CHEST (BC)
    let BC_chest_height = 10.0 * scale;
    //left chest
    let BC_left_top_l = Pos2::new(TC_left_bot_l.x, TC_left_bot_l.y + offset);
    let BC_right_top_l = Pos2::new(TC_right_bot_l.x, TC_right_bot_l.y + offset);
    let BC_right_bot_l = Pos2::new(BC_right_top_l.x - 1.5 * scale, BC_right_top_l.y + BC_chest_height);
    let BC_left_bot_l = Pos2::new(BC_left_top_l.x + 4.5 * scale, BC_left_top_l.y + BC_chest_height);
    let BC_left = vec![BC_left_top_l, BC_right_top_l, BC_right_bot_l, BC_left_bot_l];
    //right chest
    let BC_left_top_r = Pos2::new(TC_left_bottom_r.x, TC_left_bottom_r.y + offset);
    let BC_right_top_r = Pos2::new(TC_right_bottom_r.x, TC_right_bottom_r.y + offset);
    let BC_right_bot_r = Pos2::new(BC_right_top_r.x - 4.5 * scale, BC_right_top_r.y + BC_chest_height);
    let BC_left_bot_r = Pos2::new(BC_left_top_r.x + 1.5 * scale, BC_left_top_r.y + BC_chest_height);
    let BC_right = vec![BC_left_top_r, BC_right_top_r, BC_right_bot_r, BC_left_bot_r];

    painter.add(Shape::convex_polygon(BC_left, ColorQualifier(Muscle::LowerChest, &primary_muscles, &secondary_muscles), border));
    painter.add(Shape::convex_polygon(BC_right, ColorQualifier(Muscle::LowerChest, &primary_muscles, &secondary_muscles), border));
    // painter.add(Shape::convex_polygon(BC_left, ColorQualifier(target_muscle.contains(&Muscle::LowerChest)), border));
    // painter.add(Shape::convex_polygon(BC_right, ColorQualifier(target_muscle.contains(&Muscle::LowerChest)), border));

    //SHOULDERS/DELTS
    //FRONT DELT (FD)
    let FD_left_top_l = Pos2::new(TC_left_top_l.x - offset - 10.0 * scale, TC_left_top_l.y - 3.0 * scale);
    let FD_right_top_l = Pos2::new(TC_left_top_l.x - offset, TC_left_top_l.y);
    let FD_bottom_l = Pos2::new(TC_left_bot_l.x - offset, TC_left_bot_l.y);
    let FD_left= vec![FD_right_top_l, FD_left_top_l, FD_bottom_l];

    let FD_left_top_r = Pos2::new(TC_right_top_r.x + offset, TC_right_top_r.y);
    let FD_right_top_r = Pos2::new(TC_right_top_r.x + offset + 10.0 * scale, TC_right_top_r.y - 3.0 * scale);
    let FD_bottom_r = Pos2::new(TC_right_bottom_r.x + offset, TC_right_bottom_r.y);
    let FD_right = vec![FD_left_top_r, FD_right_top_r, FD_bottom_r];

    painter.add(Shape::convex_polygon(FD_left, ColorQualifier(Muscle::FrontDelt, &primary_muscles, &secondary_muscles), border));
    painter.add(Shape::convex_polygon(FD_right, ColorQualifier(Muscle::FrontDelt, &primary_muscles, &secondary_muscles), border));
    // painter.add(Shape::convex_polygon(FD_left, ColorQualifier(target_muscle.contains(&Muscle::FrontDelt)), border));
    // painter.add(Shape::convex_polygon(FD_right, ColorQualifier(target_muscle.contains(&Muscle::FrontDelt)), border));

    //SIDE DELT (SD)
    let SD_left_top_l = Pos2::new(FD_left_top_l.x - 8.0 * scale - offset, FD_left_top_l.y + 2.0 * scale);
    let SD_right_top_l = Pos2::new(FD_left_top_l.x - offset, FD_left_top_l.y - 1.0 * scale);
    let SD_right_bot_l = Pos2::new(FD_bottom_l.x - offset, FD_bottom_l.y);
    let SD_left_bot_l = Pos2::new(FD_bottom_l.x - 10.0 * scale - offset, FD_bottom_l.y - 4.0 * scale);
    let SD_left= vec![SD_left_top_l, SD_right_top_l, SD_right_bot_l, SD_left_bot_l];

    let SD_right_top_r = Pos2::new(FD_right_top_r.x + 8.0 * scale + offset, FD_right_top_r.y + 2.0 * scale);
    let SD_left_top_r = Pos2::new(FD_right_top_r.x + offset, FD_right_top_r.y - 1.0 * scale);
    let SD_left_bot_r = Pos2::new(FD_bottom_r.x + offset, FD_bottom_r.y);
    let SD_right_bot_r = Pos2::new(FD_bottom_r.x + 10.0 * scale + offset, FD_bottom_r.y - 4.0 * scale);
    let SD_right= vec![SD_left_top_r, SD_right_top_r, SD_right_bot_r, SD_left_bot_r];

    painter.add(Shape::convex_polygon(SD_left, ColorQualifier(Muscle::SideDelt, &primary_muscles, &secondary_muscles), border));
    painter.add(Shape::convex_polygon(SD_right, ColorQualifier(Muscle::SideDelt, &primary_muscles, &secondary_muscles), border));
    // painter.add(Shape::convex_polygon(SD_left, ColorQualifier(target_muscle.contains(&Muscle::SideDelt)), border));
    // painter.add(Shape::convex_polygon(SD_right, ColorQualifier(target_muscle.contains(&Muscle::SideDelt)), border));

    //BICEPS
    let BIC_left_top_l = Pos2::new(SD_left_bot_l.x, SD_left_bot_l.y + offset);
    let BIC_right_top_l = Pos2::new(SD_right_bot_l.x, SD_right_bot_l.y + offset);
    let BIC_right_med1_l = Pos2::new(BIC_right_top_l.x + 2.0 * scale, BIC_right_top_l.y + 13.0 * scale);
    let BIC_right_med2_l = Pos2::new(BIC_right_med1_l.x, BIC_right_med1_l.y + 5.0 * scale);
    let BIC_right_bot_l = Pos2::new(BIC_right_med2_l.x - 2.0 * scale, BIC_right_med2_l.y + 13.0 * scale);
    let BIC_left_bot_l = Pos2::new(BIC_left_top_l.x, BIC_left_top_l.y + 31.0 * scale);
    let BIC_left = vec![BIC_left_top_l, BIC_right_top_l, BIC_right_med1_l, BIC_right_med2_l, BIC_right_bot_l, BIC_left_bot_l];

    let BIC_right_top_r = Pos2::new(SD_right_bot_r.x, SD_right_bot_r.y + offset);
    let BIC_left_top_r = Pos2::new(SD_left_bot_r.x, SD_left_bot_r.y + offset);
    let BIC_left_med1_r = Pos2::new(BIC_left_top_r.x - 2.0 * scale, BIC_left_top_r.y + 13.0 * scale);
    let BIC_left_med2_r = Pos2::new(BIC_left_med1_r.x, BIC_left_med1_r.y + 5.0 * scale);
    let BIC_left_bot_r = Pos2::new(BIC_left_med2_r.x + 2.0 * scale, BIC_left_med2_r.y + 13.0 * scale);
    let BIC_right_bot_r = Pos2::new(BIC_right_top_r.x, BIC_right_top_r.y + 31.0 * scale);
    let BIC_right = vec![BIC_left_top_r, BIC_right_top_r, BIC_right_bot_r, BIC_left_bot_r, BIC_left_med2_r, BIC_left_med1_r];

    painter.add(Shape::convex_polygon(BIC_left, ColorQualifier(Muscle::Biceps, &primary_muscles, &secondary_muscles), border));
    painter.add(Shape::convex_polygon(BIC_right, ColorQualifier(Muscle::Biceps, &primary_muscles, &secondary_muscles), border));
    // painter.add(Shape::convex_polygon(BIC_left, ColorQualifier(target_muscle.contains(&Muscle::Biceps)), border));
    // painter.add(Shape::convex_polygon(BIC_right, ColorQualifier(target_muscle.contains(&Muscle::Biceps)), border));

    //TRICEPS
    let TRI_top_l = Pos2::new(BIC_left_top_l.x - offset, BIC_left_top_l.y);
    let TRI_bot_l = Pos2::new(BIC_left_bot_l.x - offset, BIC_left_bot_l.y);
    let TRI_mid_l = Pos2::new(TRI_top_l.x - 5.0 * scale, (TRI_top_l.y + TRI_bot_l.y) / 2.0);
    let TRI_left = vec![TRI_top_l, TRI_bot_l, TRI_mid_l];

    let TRI_top_r = Pos2::new(BIC_right_top_r.x + offset, BIC_right_top_r.y);
    let TRI_bot_r = Pos2::new(BIC_right_bot_r.x + offset, BIC_right_bot_r.y);
    let TRI_mid_r = Pos2::new(TRI_top_r.x + 5.0 * scale, (TRI_top_r.y + TRI_bot_r.y) / 2.0);
    let TRI_right = vec![TRI_top_r, TRI_bot_r, TRI_mid_r];

    painter.add(Shape::convex_polygon(TRI_left, ColorQualifier(Muscle::Triceps, &primary_muscles, &secondary_muscles), border));
    painter.add(Shape::convex_polygon(TRI_right, ColorQualifier(Muscle::Triceps, &primary_muscles, &secondary_muscles), border));
    // painter.add(Shape::convex_polygon(TRI_left, ColorQualifier(target_muscle.contains(&Muscle::Triceps)), border));
    // painter.add(Shape::convex_polygon(TRI_right, ColorQualifier(target_muscle.contains(&Muscle::Triceps)), border));

    //FOREARMS
    let F_in_right_top_l = Pos2::new(BIC_right_bot_l.x, BIC_right_bot_l.y + offset);
    let F_in_med_l = Pos2::new(F_in_right_top_l.x + 2.0 * scale, F_in_right_top_l.y + 17.5 * scale);
    let F_in_right_bot_l = Pos2::new(F_in_right_top_l.x, F_in_right_top_l.y + 35.0 * scale);
    let F_in_left_bot_l = Pos2::new(F_in_right_bot_l.x - 5.0 * scale, F_in_right_bot_l.y);
    let F_in_left_top_l = Pos2::new(F_in_left_bot_l.x, F_in_left_bot_l.y - 37.0 * scale);
    let F_in_left = vec![F_in_right_top_l, F_in_med_l, F_in_right_bot_l, F_in_left_bot_l, F_in_left_top_l];

    let F_out_left_top_l = Pos2::new(TRI_bot_l.x, TRI_bot_l.y + offset);
    let F_out_right_top_l = Pos2::new(F_out_left_top_l.x + 5.0 * scale, F_out_left_top_l.y + offset);
    let F_out_right_bot_l = Pos2::new(F_out_right_top_l.x, F_out_right_top_l.y + 37.0 * scale);
    let F_out_left_bot_l = Pos2::new(F_out_right_bot_l.x - 2.0 * scale, F_out_right_bot_l.y);
    let F_out_left = vec![F_out_left_top_l, F_out_right_top_l, F_out_right_bot_l, F_out_left_bot_l];

    let F_in_left_top_r = Pos2::new(BIC_left_bot_r.x, BIC_left_bot_r.y + offset);
    let F_in_med_r = Pos2::new(F_in_left_top_r.x - 2.0 * scale, F_in_left_top_r.y + 17.5 * scale);
    let F_in_left_bot_r = Pos2::new(F_in_left_top_r.x, F_in_left_top_r.y + 35.0 * scale);
    let F_in_right_bot_r = Pos2::new(F_in_left_bot_r.x + 5.0 * scale, F_in_left_bot_r.y);
    let F_in_right_top_r = Pos2::new(F_in_right_bot_r.x, F_in_right_bot_r.y - 37.0 * scale);
    let F_in_right = vec![F_in_left_top_r, F_in_med_r, F_in_left_bot_r, F_in_right_bot_r, F_in_right_top_r];

    let F_out_right_top_r = Pos2::new(TRI_bot_r.x, TRI_bot_r.y + offset);
    let F_out_left_top_r = Pos2::new(F_out_right_top_r.x - 5.0 * scale, F_out_right_top_r.y + offset);
    let F_out_left_bot_r = Pos2::new(F_out_left_top_r.x, F_out_left_top_r.y + 37.0 * scale);
    let F_out_right_bot_r = Pos2::new(F_out_left_bot_r.x + 2.0 * scale, F_out_left_bot_r.y);
    let F_out_right = vec![F_out_right_top_r, F_out_left_top_r, F_out_left_bot_r, F_out_right_bot_r];

    painter.add(Shape::convex_polygon(F_in_left, ColorQualifier(Muscle::Forearms, &primary_muscles, &secondary_muscles), border));
    painter.add(Shape::convex_polygon(F_in_right, ColorQualifier(Muscle::Forearms, &primary_muscles, &secondary_muscles), border));
    painter.add(Shape::convex_polygon(F_out_left, ColorQualifier(Muscle::Forearms, &primary_muscles, &secondary_muscles), border));
    painter.add(Shape::convex_polygon(F_out_right, ColorQualifier(Muscle::Forearms, &primary_muscles, &secondary_muscles), border));
    // painter.add(Shape::convex_polygon(F_in_left, ColorQualifier(target_muscle.contains(&Muscle::Forearms)), border));
    // painter.add(Shape::convex_polygon(F_in_right, ColorQualifier(target_muscle.contains(&Muscle::Forearms)), border));
    // painter.add(Shape::convex_polygon(F_out_left, ColorQualifier(target_muscle.contains(&Muscle::Forearms)), border));
    // painter.add(Shape::convex_polygon(F_out_right, ColorQualifier(target_muscle.contains(&Muscle::Forearms)), border));

    //HAND
    let H_left_top_l = Pos2::new(F_out_left_bot_l.x - 3.0 * scale, F_out_left_bot_l.y + offset);
    let H_right_top_l = Pos2::new(F_in_right_bot_l.x + 3.0 * scale, F_in_right_bot_l.y + offset);
    let H_right_bot_l = Pos2::new(H_right_top_l.x, H_right_top_l.y + 10.0 * scale);
    let H_left_bot_l = Pos2::new(H_left_top_l.x, H_right_bot_l.y);
    let H_left = vec![H_left_top_l, H_right_top_l, H_right_bot_l, H_left_bot_l];

    let H_left_top_r = Pos2::new(F_in_left_bot_r.x - 3.0 * scale, F_in_left_bot_r.y + offset);
    let H_right_top_r = Pos2::new(F_out_right_bot_r.x + 3.0 * scale, F_out_right_bot_r.y + offset);
    let H_right_bot_r = Pos2::new(H_right_top_r.x, H_right_top_r.y + 10.0 * scale);
    let H_left_bot_r = Pos2::new(H_left_top_r.x, H_right_bot_r.y);
    let H_right = vec![H_left_top_r, H_right_top_r, H_right_bot_r, H_left_bot_r];

    painter.add(Shape::convex_polygon(H_left, muscle_color, border));
    painter.add(Shape::convex_polygon(H_right, muscle_color, border));

    //SIDE ABS (SA)
    let SA_left_top_l = Pos2::new(BC_left_bot_l.x + 2.0 * scale, BC_left_bot_l.y + offset);
    let SA_right_top_l = Pos2::new(SA_left_top_l.x + 12.0 * scale, SA_left_top_l.y);
    let SA_bot_l = Pos2::new(SA_right_top_l.x, SA_right_top_l.y + 50.0 * scale);
    let SA_left = vec![SA_left_top_l, SA_right_top_l, SA_bot_l];

    let SA_left_top_r= Pos2::new(BC_right_bot_r.x - 13.0 * scale, BC_right_bot_r.y + offset);
    let SA_right_top_r = Pos2::new(SA_left_top_r.x + 11.0 * scale, SA_left_top_r.y);
    let SA_bot_r = Pos2::new(SA_left_top_r.x, SA_left_top_r.y + 50.0 * scale);
    let SA_right = vec![SA_left_top_r, SA_right_top_r, SA_bot_r];

    painter.add(Shape::convex_polygon(SA_left, ColorQualifier(Muscle::Abs, &primary_muscles, &secondary_muscles), border));
    painter.add(Shape::convex_polygon(SA_right, ColorQualifier(Muscle::Abs, &primary_muscles, &secondary_muscles), border));

    //ABS
    let abs_width = 15.0 * scale;
    let abs_center = chest_center + vec2(0.0, 37.0 * scale);
    let left_center = abs_center - vec2((abs_width / 2.0) + offset, 0.0);
    let right_center = abs_center + vec2((abs_width / 2.0) + offset, 0.0);
    let abs_height = 7.5 * scale;
    let mut abs_min_l = left_center - egui::vec2((abs_width / 2.0), abs_height / 2.0);
    let mut abs_max_l = left_center + egui::vec2((abs_width / 2.0), abs_height / 2.0);
    let mut abs_min_r= right_center - egui::vec2((abs_width / 2.0), abs_height / 2.0);
    let mut abs_max_r = right_center + egui::vec2((abs_width / 2.0), abs_height / 2.0);

    let mut abs_to_draw;

    for _ in 0..3 {
        abs_to_draw = egui::Rect::from_min_max(abs_min_l, abs_max_l);

        painter.rect_filled(
            abs_to_draw,
            0.0, 
            ColorQualifier(Muscle::Abs, &primary_muscles, &secondary_muscles),
        );

        abs_to_draw = egui::Rect::from_min_max(abs_min_r, abs_max_r);

        painter.rect_filled(
            abs_to_draw,
            0.0, 
            ColorQualifier(Muscle::Abs, &primary_muscles, &secondary_muscles),
            // Color32::GRAY,
        );

        abs_min_l += vec2(0.0, 10.0 * scale);
        abs_max_l += vec2(0.0, 10.0 * scale);
        abs_min_r += vec2(0.0, 10.0 * scale);
        abs_max_r += vec2(0.0, 10.0 * scale);
    }

    // BOTTOM ABS (BA)
    let BA_left_top_l = Pos2::new(abs_min_l.x, abs_min_l.y);
    let BA_right_top_l = Pos2::new(BA_left_top_l.x + abs_width, BA_left_top_l.y);
    let BA_right_bot_l = Pos2::new(BA_right_top_l.x, BA_right_top_l.y + 25.0 * scale);
    let BA_left_bot_l = Pos2::new(BA_left_top_l.x, BA_left_top_l.y + 20.0 * scale);
    let BA_left = vec![BA_left_top_l, BA_right_top_l, BA_right_bot_l, BA_left_bot_l];

    let BA_left_top_r = Pos2::new(abs_min_r.x, abs_min_r.y);
    let BA_right_top_r = Pos2::new(BA_left_top_r.x + abs_width, BA_left_top_r.y);
    let BA_right_bot_r = Pos2::new(BA_right_top_r.x, BA_right_top_r.y + 20.0 * scale);
    let BA_left_bot_r = Pos2::new(BA_left_top_r.x, BA_left_top_r.y + 25.0 * scale);
    let BA_right = vec![BA_left_top_r, BA_right_top_r, BA_right_bot_r, BA_left_bot_r];

    painter.add(Shape::convex_polygon(BA_left, ColorQualifier(Muscle::Abs, &primary_muscles, &secondary_muscles), border));
    painter.add(Shape::convex_polygon(BA_right, ColorQualifier(Muscle::Abs, &primary_muscles, &secondary_muscles), border));
    // painter.add(Shape::convex_polygon(BA_left, muscle_color, border));
    // painter.add(Shape::convex_polygon(BA_right, muscle_color, border));

    // HIPS
    let HP_left_top_l = Pos2::new(SA_bot_l.x, SA_bot_l.y + offset);
    let HP_right_top_l = Pos2::new(HP_left_top_l.x + 8.0 * scale, HP_left_top_l.y + 3.0 * scale);
    let HP_bot_l = Pos2::new(HP_left_top_l.x - 8.0 * scale, HP_left_top_l.y + 15.0 * scale);
    let HP_left = vec![HP_left_top_l, HP_right_top_l, HP_bot_l];

    let HP_right_top_r = Pos2::new(SA_bot_r.x, SA_bot_r.y + offset);
    let HP_left_top_r = Pos2::new(HP_right_top_r.x - 8.0 * scale, HP_right_top_r.y + 3.0 * scale);
    let HP_bot_r = Pos2::new(HP_right_top_r.x + 8.0 * scale, HP_right_top_r.y + 15.0 * scale);
    let HP_right = vec![HP_right_top_r, HP_left_top_r, HP_bot_r];

    painter.add(Shape::convex_polygon(HP_left, ColorQualifier(Muscle::Hips, &primary_muscles, &secondary_muscles), border));
    painter.add(Shape::convex_polygon(HP_right, ColorQualifier(Muscle::Hips, &primary_muscles, &secondary_muscles), border));

    //ADDUCTORS
    let AD_left_top_l= Pos2::new(HP_right_top_l.x + offset, HP_right_top_l.y + 0.3 * scale);
    let AD_right_top_l = Pos2::new(BA_right_bot_l.x, BA_right_bot_l.y + offset);
    let AD_bot_l = Pos2::new(AD_right_top_l.x, AD_right_top_l.y + 10.0 * scale);
    let AD_left = vec![AD_left_top_l, AD_right_top_l, AD_bot_l];

    let AD_right_top_r = Pos2::new(HP_left_top_r.x - offset, HP_left_top_r.y + 0.3 * scale);
    let AD_left_top_r = Pos2::new(BA_left_bot_r.x, BA_left_bot_r.y + offset);
    let AD_bot_r = Pos2::new(AD_left_top_r.x, AD_left_top_r.y + 10.0 * scale);
    let AD_right = vec![AD_right_top_r, AD_left_top_r, AD_bot_r];

    painter.add(Shape::convex_polygon(AD_left, ColorQualifier(Muscle::Adductors, &primary_muscles, &secondary_muscles), border));
    painter.add(Shape::convex_polygon(AD_right, ColorQualifier(Muscle::Adductors, &primary_muscles, &secondary_muscles), border));
    // painter.add(Shape::convex_polygon(AD_left, muscle_color, border));
    // painter.add(Shape::convex_polygon(AD_right, muscle_color, border));

    //QUADS
    let QU_top_l = Pos2::new(HP_right_top_l.x + 0.2 * scale, HP_right_top_l.y + offset);
    let QU_right_mid_l = Pos2::new(AD_bot_l.x, AD_bot_l.y + offset);
    let QU_right_bot_l = Pos2::new(QU_right_mid_l.x - 5.0 * scale, QU_right_mid_l.y + 45.0 * scale);
    let QU_left_bot_l = Pos2::new(QU_right_bot_l.x - 15.0 * scale, QU_right_bot_l.y);
    let QU_left_mid_l = Pos2::new(HP_bot_l.x, HP_bot_l.y + offset);
    let QU_left = vec![QU_top_l, QU_right_mid_l, QU_right_bot_l, QU_left_bot_l, QU_left_mid_l];

    let QU_top_r = Pos2::new(HP_left_top_r.x - 0.2 * scale, HP_left_top_r.y + offset);
    let QU_left_mid_r = Pos2::new(AD_bot_r.x, AD_bot_r.y + offset);
    let QU_left_bot_r = Pos2::new(QU_left_mid_r.x + 5.0 * scale, QU_left_mid_r.y + 45.0 * scale);
    let QU_right_bot_r = Pos2::new(QU_left_bot_r.x + 15.0 * scale, QU_left_bot_r.y);
    let QU_right_mid_r = Pos2::new(HP_bot_r.x, HP_bot_r.y + offset);
    let QU_right = vec![QU_top_r, QU_left_mid_r, QU_left_bot_r, QU_right_bot_r, QU_right_mid_r];

    painter.add(Shape::convex_polygon(QU_left, ColorQualifier(Muscle::Quads, &primary_muscles, &secondary_muscles), border));
    painter.add(Shape::convex_polygon(QU_right, ColorQualifier(Muscle::Quads, &primary_muscles, &secondary_muscles), border));
    // painter.add(Shape::convex_polygon(QU_left, muscle_color, border));
    // painter.add(Shape::convex_polygon(QU_right, muscle_color, border));

    //EXTERNAL HIPS (EH)
    let EH_top_l = Pos2::new(QU_left_mid_l.x - offset, QU_left_mid_l.y + 4.0 * scale);
    let EH_right_bot_l = Pos2::new(QU_left_bot_l.x - offset, QU_left_bot_l.y);
    let EH_left_bot_l = Pos2::new(EH_right_bot_l.x - 5.0 * scale, EH_right_bot_l.y - 10.0 * scale);
    let EH_left = vec![EH_top_l, EH_right_bot_l, EH_left_bot_l];

    let EH_top_r = Pos2::new(QU_right_mid_r.x + offset, QU_right_mid_l.y + 4.0 * scale);
    let EH_left_bot_r = Pos2::new(QU_right_bot_r.x + offset, QU_right_bot_r.y);
    let EH_right_bot_r = Pos2::new(EH_left_bot_r.x + 5.0 * scale, EH_left_bot_r.y - 10.0 * scale);
    let EH_right = vec![EH_top_r, EH_left_bot_r, EH_right_bot_r];

    painter.add(Shape::convex_polygon(EH_left, ColorQualifier(Muscle::ExtHips, &primary_muscles, &secondary_muscles), border));
    painter.add(Shape::convex_polygon(EH_right, ColorQualifier(Muscle::ExtHips, &primary_muscles, &secondary_muscles), border));
    // painter.add(Shape::convex_polygon(EH_left, muscle_color, border));
    // painter.add(Shape::convex_polygon(EH_right, muscle_color, border));

    //KNESS
    let KN_min_l = Pos2::new(QU_left_bot_l.x, QU_left_bot_l.y + offset);
    let KN_max_l = Pos2::new(QU_right_bot_l.x, QU_right_bot_l.y + offset + 6.0 * scale);
    let KN_left = egui::Rect::from_min_max(KN_min_l, KN_max_l);

    painter.rect_filled(
        KN_left,
        0.0, 
        Color32::GRAY,
    );

    let KN_min_r = Pos2::new(QU_left_bot_r.x, QU_left_bot_r.y + offset);
    let KN_max_r = Pos2::new(QU_right_bot_r.x, QU_right_bot_r.y + offset + 6.0 * scale);
    let KN_right = egui::Rect::from_min_max(KN_min_r, KN_max_r);

    painter.rect_filled(
        KN_right,
        0.0, 
        Color32::GRAY,
    );

    //CALFS
    let CF_out_left_top_l = Pos2::new(EH_right_bot_l.x, KN_max_l.y + offset);
    let CF_out_right_top_l = Pos2::new(CF_out_left_top_l.x + 10.0 * scale, CF_out_left_top_l.y);
    let CF_out_right_bot_l = Pos2::new(CF_out_right_top_l.x, CF_out_right_top_l.y + 50.0 * scale);
    let CF_out_left_bot_l = Pos2::new(CF_out_right_bot_l.x - 5.0 * scale, CF_out_right_bot_l.y);
    let CF_out_left_med_l = Pos2::new(CF_out_left_top_l.x - 3.0 * scale, CF_out_left_bot_l.y - 30.0 * scale);
    let CF_out_left = vec![CF_out_left_top_l, CF_out_right_top_l, CF_out_right_bot_l, CF_out_left_bot_l, CF_out_left_med_l];

    let CF_in_left_top_l = Pos2::new(CF_out_right_top_l.x + offset, CF_out_right_top_l.y);
    let CF_in_right_top_l = Pos2::new(CF_in_left_top_l.x + 5.0 * scale, CF_in_left_top_l.y);
    let CF_in_right_med_l = Pos2::new(CF_in_right_top_l.x + 2.0 * scale, CF_in_right_top_l.y + 20.0 * scale);
    let CF_in_right_bot_l = Pos2::new(CF_in_right_top_l.x, CF_in_right_top_l.y + 50.0 * scale);
    let CF_in_left_bot_l = Pos2::new(CF_in_right_bot_l.x - 5.0 * scale, CF_in_right_bot_l.y);
    let CF_in_left = vec![CF_in_left_top_l, CF_in_right_top_l, CF_in_right_med_l, CF_in_right_bot_l, CF_in_left_bot_l];

    let CF_out_right_top_r = Pos2::new(EH_left_bot_r.x, KN_max_r.y + offset);
    let CF_out_left_top_r = Pos2::new(CF_out_right_top_r.x - 10.0 * scale, CF_out_right_top_r.y);
    let CF_out_left_bot_r = Pos2::new(CF_out_left_top_r.x, CF_out_left_top_r.y + 50.0 * scale);
    let CF_out_right_bot_r = Pos2::new(CF_out_left_bot_r.x + 5.0 * scale, CF_out_left_bot_r.y);
    let CF_out_right_med_r = Pos2::new(CF_out_right_top_r.x + 3.0 * scale, CF_out_right_bot_r.y - 30.0 * scale);
    let CF_out_right = vec![CF_out_right_top_r, CF_out_left_top_r, CF_out_left_bot_r, CF_out_right_bot_r, CF_out_right_med_r];

    let CF_in_right_top_r = Pos2::new(CF_out_left_top_r.x - offset, CF_out_left_top_r.y);
    let CF_in_left_top_r = Pos2::new(CF_in_right_top_r.x - 5.0 * scale, CF_in_right_top_r.y);
    let CF_in_left_mid_r = Pos2::new(CF_in_left_top_r.x - 2.0 * scale, CF_in_left_top_r.y + 20.0 * scale);
    let CF_in_left_bot_r = Pos2::new(CF_in_left_top_r.x, CF_in_left_top_r.y + 50.0 * scale);
    let CF_in_right_bot_r = Pos2::new(CF_in_left_bot_r.x + 5.0 * scale, CF_in_left_bot_r.y);
    let CF_in_right = vec![CF_in_right_top_r, CF_in_left_top_r, CF_in_left_mid_r, CF_in_left_bot_r, CF_in_right_bot_r];

    painter.add(Shape::convex_polygon(CF_out_left, ColorQualifier(Muscle::Calfs, &primary_muscles, &secondary_muscles), border));
    painter.add(Shape::convex_polygon(CF_out_right, ColorQualifier(Muscle::Calfs, &primary_muscles, &secondary_muscles), border));
    painter.add(Shape::convex_polygon(CF_in_left, ColorQualifier(Muscle::Calfs, &primary_muscles, &secondary_muscles), border));
    painter.add(Shape::convex_polygon(CF_in_right, ColorQualifier(Muscle::Calfs, &primary_muscles, &secondary_muscles), border));
    // painter.add(Shape::convex_polygon(CF_out_left, muscle_color, border));
    // painter.add(Shape::convex_polygon(CF_out_right, muscle_color, border));
    // painter.add(Shape::convex_polygon(CF_in_left, muscle_color, border));
    // painter.add(Shape::convex_polygon(CF_in_right, muscle_color, border));

    //FEET
    let F_min_l = Pos2::new(CF_out_left_bot_l.x - 4.0 * scale, CF_out_left_bot_l.y + offset);
    let F_max_l = Pos2::new(CF_in_right_bot_l.x + 3.0 * scale, CF_in_right_bot_l.y + offset + 10.0 * scale);
    let F_left = egui::Rect::from_min_max(F_min_l, F_max_l);

    painter.rect_filled(
        F_left,
        0.0, 
        Color32::GRAY,
    );

    let F_min_r = Pos2::new(CF_in_left_bot_r.x - 4.0 * scale, CF_in_left_bot_r.y + offset);
    let F_max_r = Pos2::new(CF_out_right_bot_r.x + 3.0 * scale, CF_out_right_bot_r.y + offset + 10.0 * scale);
    let F_right = egui::Rect::from_min_max(F_min_r, F_max_r);

    painter.rect_filled(
        F_right,
        0.0, 
        Color32::GRAY,
    );
}

pub fn workout_tracker_widget_behind(ctx: &egui::Context, ui: &mut egui::Ui, size: Vec2, exercises: &Vec<Exercise>) {
    let (primary_muscles, secondary_muscles) = muscle_for_workout(exercises);

    let (rect, _response) = ui.allocate_exact_size(size, egui::Sense::hover());
    let default_size = Vec2::new(120.0, 270.0);
    let scale = (size.x * size.y) / (default_size.x * default_size.y);

    let painter = ui.painter_at(rect);

    let center = rect.center() - vec2(0.0, 120.0 * scale);
    let muscle_color = Color32::GRAY;
    let border = Stroke::new(0.0, Color32::GRAY);
    let offset = 1.5 * scale;

    painter.circle_filled(
        center,
        20.0 * scale,
        Color32::GRAY,
    );

    //TOP TRAPS (TT)
    let traps_center = center + egui::vec2(0.0, 40.0 * scale);
    let top_trap_height= 15.0 * scale;
    let top_trap_width = 24.0 * scale;

    let TT_top_l= Pos2::new(traps_center.x - 1.0 * scale, traps_center.y - 18.0 * scale);
    let TT_right_bot_l = Pos2::new(TT_top_l.x, TT_top_l.y + top_trap_height);
    let TT_left_bot_l = Pos2::new(TT_right_bot_l.x - top_trap_width, TT_right_bot_l.y);
    let TT_left = vec![TT_top_l, TT_right_bot_l, TT_left_bot_l];

    let TT_top_r = Pos2::new(traps_center.x + 1.0 * scale, traps_center.y - 18.0 * scale);
    let TT_left_bot_r = Pos2::new(TT_top_r.x, TT_top_r.y + top_trap_height);
    let TT_right_bot_r = Pos2::new(TT_left_bot_r.x + top_trap_width, TT_left_bot_r.y);
    let TT_right = vec![TT_top_r, TT_left_bot_r, TT_right_bot_r];

    painter.add(Shape::convex_polygon(TT_left, ColorQualifier(Muscle::Traps, &primary_muscles, &secondary_muscles), border));
    painter.add(Shape::convex_polygon(TT_right, ColorQualifier(Muscle::Traps, &primary_muscles, &secondary_muscles), border));
    // painter.add(Shape::convex_polygon(TT_left, muscle_color, border));
    // painter.add(Shape::convex_polygon(TT_right, muscle_color, border));

    //TRAPS
    let trap_height = 55.0 * scale;

    let T_left_top_l = Pos2::new(TT_left_bot_l.x, TT_left_bot_l.y + offset);
    let T_right_top_l = Pos2::new(TT_right_bot_l.x, TT_right_bot_l.y + offset);
    let T_bot_l = Pos2::new(T_right_top_l.x, T_right_top_l.y + trap_height);
    let T_left = vec![T_left_top_l, T_right_top_l, T_bot_l];

    let T_right_top_r = Pos2::new(TT_right_bot_r.x, TT_right_bot_l.y + offset);
    let T_left_top_r = Pos2::new(TT_left_bot_r.x, TT_left_bot_l.y + offset);
    let T_bot_r = Pos2::new(T_left_top_r.x, T_left_top_r.y + trap_height);
    let T_right = vec![T_right_top_r, T_left_top_r, T_bot_r];

    painter.add(Shape::convex_polygon(T_left, ColorQualifier(Muscle::Traps, &primary_muscles, &secondary_muscles), border));
    painter.add(Shape::convex_polygon(T_right, ColorQualifier(Muscle::Traps, &primary_muscles, &secondary_muscles), border));
    // painter.add(Shape::convex_polygon(T_left, muscle_color, border));
    // painter.add(Shape::convex_polygon(T_right, muscle_color, border));

    //INFRASPINATUS
    let infraspinatus_height = 20.0 * scale;
    let infraspinatus_width = 10.0 * scale;

    let I_top_l = Pos2::new(T_left_top_l.x - offset, T_left_top_l.y + offset);
    let I_left_bot_l = Pos2::new(I_top_l.x - infraspinatus_width * 0.2, I_top_l.y + infraspinatus_height);
    let I_right_bot_l = Pos2::new(I_top_l.x + infraspinatus_width, I_left_bot_l.y);
    let I_left = vec![I_top_l, I_left_bot_l, I_right_bot_l];

    let I_top_r = Pos2::new(T_right_top_r.x + offset, T_right_top_r.y + offset);
    let I_right_bot_r = Pos2::new(I_top_r.x + infraspinatus_width * 0.2, I_top_r.y + infraspinatus_height);
    let I_left_bot_r = Pos2::new(I_top_r.x - infraspinatus_width, I_right_bot_r.y);
    let I_right = vec![I_top_r, I_right_bot_r, I_left_bot_r];

    painter.add(Shape::convex_polygon(I_left, ColorQualifier(Muscle::Infraspinatus, &primary_muscles, &secondary_muscles), border));
    painter.add(Shape::convex_polygon(I_right, ColorQualifier(Muscle::Infraspinatus, &primary_muscles, &secondary_muscles), border));
    // painter.add(Shape::convex_polygon(I_left, muscle_color, border));
    // painter.add(Shape::convex_polygon(I_right, muscle_color, border));

    //REAR DELT (RD)
    let rear_delt_width = 5.0 * scale;

    let RD_right_top_l = Pos2::new(I_top_l.x - offset, I_top_l.y - offset);
    let RD_left_top_l = Pos2::new(RD_right_top_l.x - rear_delt_width * 1.4, T_left_top_l.y - offset);
    let RD_right_bot_l = Pos2::new(I_left_bot_l.x - offset, I_left_bot_l.y);
    let RD_left_bot_l = Pos2::new(RD_left_top_l.x - offset * 4.0, RD_right_bot_l.y - offset * 1.4);
    let RD_left = vec![RD_right_top_l, RD_left_top_l, RD_left_bot_l, RD_right_bot_l];

    let RD_left_top_r = Pos2::new(I_top_r.x + offset, I_top_r.y - offset);
    let RD_right_top_r = Pos2::new(RD_left_top_r.x + rear_delt_width * 1.4, T_right_top_r.y - offset);
    let RD_left_bot_r = Pos2::new(I_right_bot_r.x + offset, I_right_bot_r.y);
    let RD_right_bot_r = Pos2::new(RD_right_top_r.x + offset * 4.0, RD_left_bot_r.y - offset * 1.4);
    let RD_right = vec![RD_left_top_r, RD_right_top_r, RD_right_bot_r, RD_left_bot_r];

    painter.add(Shape::convex_polygon(RD_left, ColorQualifier(Muscle::RearDelt, &primary_muscles, &secondary_muscles), border));
    painter.add(Shape::convex_polygon(RD_right, ColorQualifier(Muscle::RearDelt, &primary_muscles, &secondary_muscles), border));
    // painter.add(Shape::convex_polygon(RD_left, muscle_color, border));
    // painter.add(Shape::convex_polygon(RD_right, muscle_color, border));

    //SIDE DELT (SD)
    let side_delt_width = 6.5 * scale;

    let SD_right_top_l = Pos2::new(RD_left_top_l.x - offset, RD_left_top_l.y);
    let SD_right_bot_l = Pos2::new(RD_left_bot_l.x - offset, RD_left_bot_l.y - 0.5 * scale);
    let SD_left_top_l = Pos2::new(SD_right_top_l.x - side_delt_width, SD_right_top_l.y + offset * 2.0);
    let SD_left_bot_l = Pos2::new(SD_right_bot_l.x - side_delt_width, SD_right_bot_l.y - offset * 1.4);
    let SD_left = vec![SD_right_top_l, SD_right_bot_l, SD_left_bot_l, SD_left_top_l];

    let SD_left_top_r = Pos2::new(RD_right_top_r.x + offset, RD_right_top_r.y);
    let SD_left_bot_r = Pos2::new(RD_right_bot_r.x + offset, RD_right_bot_r.y - 0.5 * scale);
    let SD_right_top_r = Pos2::new(SD_left_top_r.x + side_delt_width, SD_left_top_r.y + offset * 2.0);
    let SD_right_bot_r = Pos2::new(SD_left_bot_r.x + side_delt_width, SD_left_bot_r.y - offset * 1.4);
    let SD_right = vec![SD_left_top_r, SD_left_bot_r, SD_right_bot_r, SD_right_top_r];

    painter.add(Shape::convex_polygon(SD_left, ColorQualifier(Muscle::SideDelt, &primary_muscles, &secondary_muscles), border));
    painter.add(Shape::convex_polygon(SD_right, ColorQualifier(Muscle::SideDelt, &primary_muscles, &secondary_muscles), border));
    // painter.add(Shape::convex_polygon(SD_left, muscle_color, border));
    // painter.add(Shape::convex_polygon(SD_right, muscle_color, border));

    //OUTER TRICEPS (OT)
    let outer_triceps_width = 3.7 * scale;
    let outer_triceps_height = 17.0 * scale;

    let OT_left_top_l = Pos2::new(SD_left_bot_l.x, SD_left_bot_l.y + offset);
    let OT_right_top_l = Pos2::new(OT_left_top_l.x + outer_triceps_width, OT_left_top_l.y + offset);
    let OT_right_bot_l = Pos2::new(OT_right_top_l.x - 10.5 * scale, OT_left_top_l.y + outer_triceps_height + 6.5 * scale);
    let OT_left_bot_l = Pos2::new(OT_left_top_l.x - 9.7 * scale, OT_left_top_l.y + outer_triceps_height);
    let OT_left = vec![OT_left_top_l, OT_right_top_l, OT_right_bot_l, OT_left_bot_l];

    let OT_right_top_r = Pos2::new(SD_right_bot_r.x, SD_right_bot_r.y + offset);
    let OT_left_top_r = Pos2::new(OT_right_top_r.x - outer_triceps_width, OT_right_top_r.y + offset);
    let OT_left_bot_r = Pos2::new(OT_left_top_r.x + 10.5 * scale, OT_left_top_r.y + outer_triceps_height + 6.5 * scale);
    let OT_right_bot_r = Pos2::new(OT_right_top_r.x + 9.7 * scale, OT_right_top_r.y + outer_triceps_height);
    let OT_right = vec![OT_left_top_r, OT_right_top_r, OT_right_bot_r, OT_left_bot_r];

    painter.add(Shape::convex_polygon(OT_left, ColorQualifier(Muscle::Triceps, &primary_muscles, &secondary_muscles), border));
    painter.add(Shape::convex_polygon(OT_right, ColorQualifier(Muscle::Triceps, &primary_muscles, &secondary_muscles), border));
    // painter.add(Shape::convex_polygon(OT_left, muscle_color, border));
    // painter.add(Shape::convex_polygon(OT_right, muscle_color, border));

    //INNER TRICEPS (IT)
    let inner_triceps_width = 4.0 * scale;
    let inner_triceps_height = 24.0 * scale;

    let IT_left_top_l = Pos2::new(OT_right_top_l.x + 3.0 * scale, OT_right_top_l.y + offset * 0.5);
    let IT_right_top_l = Pos2::new(IT_left_top_l.x + inner_triceps_width, IT_left_top_l.y + 3.0 * scale);
    let IT_right_bot_l = Pos2::new(IT_right_top_l.x, IT_right_top_l.y + inner_triceps_height - 5.0 * scale);
    let IT_left_bot_l = Pos2::new(IT_left_top_l.x, IT_left_top_l.y + inner_triceps_height);
    let IT_left = vec![IT_left_top_l, IT_right_top_l, IT_right_bot_l, IT_left_bot_l];

    let IT_right_top_r = Pos2::new(OT_left_top_r.x - 3.0 * scale, OT_left_top_r.y + offset * 0.5 * scale);
    let IT_left_top_r = Pos2::new(IT_right_top_r.x - inner_triceps_width, IT_right_top_r.y + 3.0 * scale);
    let IT_left_bot_r = Pos2::new(IT_left_top_r.x, IT_left_top_r.y + inner_triceps_height - 5.0 * scale);
    let IT_right_bot_r = Pos2::new(IT_right_top_r.x, IT_right_top_r.y + inner_triceps_height);
    let IT_right= vec![IT_left_top_r, IT_right_top_r, IT_right_bot_r, IT_left_bot_r];

    painter.add(Shape::convex_polygon(IT_left, ColorQualifier(Muscle::Triceps, &primary_muscles, &secondary_muscles), border));
    painter.add(Shape::convex_polygon(IT_right, ColorQualifier(Muscle::Triceps, &primary_muscles, &secondary_muscles), border));
    // painter.add(Shape::convex_polygon(IT_left, muscle_color, border));
    // painter.add(Shape::convex_polygon(IT_right, muscle_color, border));

    //MEDIAL TRICEPS (MT)
    let medial_triceps_width = 7.0 * scale;
    let medial_triceps_height = 23.4 * scale;

    let MT_top_l = Pos2::new(SD_right_bot_l.x - offset, SD_right_bot_l.y + 3.5 * scale);
    let MT_right_mid_l = Pos2::new(MT_top_l.x, MT_top_l.y + medial_triceps_height);
    let MT_right_bot_l = Pos2::new(MT_right_mid_l.x - 3.0 * scale, MT_right_mid_l.y + 3.0 * scale);
    let MT_left_bot_l = Pos2::new(MT_right_bot_l.x - medial_triceps_width, MT_right_bot_l.y - 2.0 * scale);
    let MT_left_mid_l = Pos2::new(MT_left_bot_l.x, MT_left_bot_l.y - 4.0 * scale);
    let MT_left = vec![MT_top_l, MT_right_mid_l, MT_right_bot_l, MT_left_bot_l, MT_left_mid_l];

    let MT_top_r = Pos2::new(SD_left_bot_r.x + offset, SD_left_bot_r.y + 3.5 * scale);
    let MT_left_mid_r = Pos2::new(MT_top_r.x, MT_top_r.y + medial_triceps_height);
    let MT_left_bot_r = Pos2::new(MT_left_mid_r.x + 3.0 * scale, MT_left_mid_r.y + 3.0 * scale);
    let MT_right_bot_r = Pos2::new(MT_left_bot_r.x + medial_triceps_width, MT_left_bot_r.y - 2.0 * scale);
    let MT_right_mid_r = Pos2::new(MT_right_bot_r.x, MT_right_bot_r.y - 4.0 * scale);
    let MT_right = vec![MT_top_r, MT_right_mid_r, MT_right_bot_r, MT_left_bot_r, MT_left_mid_r];

    painter.add(Shape::convex_polygon(MT_left, ColorQualifier(Muscle::Triceps, &primary_muscles, &secondary_muscles), border));
    painter.add(Shape::convex_polygon(MT_right, ColorQualifier(Muscle::Triceps, &primary_muscles, &secondary_muscles), border));
    // painter.add(Shape::convex_polygon(MT_left, muscle_color, border));
    // painter.add(Shape::convex_polygon(MT_right, muscle_color, border));
    
    //FOREARMS
    let F_in_right_top_l = Pos2::new(MT_right_mid_l.x, MT_right_bot_l.y + offset * 2.0);
    let F_in_med_l = Pos2::new(F_in_right_top_l.x + 2.0 * scale, F_in_right_top_l.y + 17.5 * scale);
    let F_in_right_bot_l = Pos2::new(F_in_right_top_l.x, F_in_right_top_l.y + 35.0 * scale);
    let F_in_left_bot_l = Pos2::new(F_in_right_bot_l.x - 5.0 * scale, F_in_right_bot_l.y);
    let F_in_left_top_l = Pos2::new(F_in_left_bot_l.x, F_in_left_bot_l.y - 37.0 * scale);
    let F_in_left = vec![F_in_right_top_l, F_in_med_l, F_in_right_bot_l, F_in_left_bot_l, F_in_left_top_l];

    let F_out_left_top_l = Pos2::new(OT_right_bot_l.x, MT_left_bot_l.y + offset);
    let F_out_right_top_l = Pos2::new(F_out_left_top_l.x + 5.0 * scale, F_out_left_top_l.y + offset);
    let F_out_right_bot_l = Pos2::new(F_out_right_top_l.x, F_out_right_top_l.y + 37.0 * scale);
    let F_out_left_bot_l = Pos2::new(F_out_right_bot_l.x - 2.0 * scale, F_out_right_bot_l.y);
    let F_out_left = vec![F_out_left_top_l, F_out_right_top_l, F_out_right_bot_l, F_out_left_bot_l];

    let F_in_left_top_r = Pos2::new(MT_left_mid_r.x, MT_left_bot_r.y + offset * 2.0);
    let F_in_med_r = Pos2::new(F_in_left_top_r.x - 2.0 * scale, F_in_left_top_r.y + 17.5 * scale);
    let F_in_left_bot_r = Pos2::new(F_in_left_top_r.x, F_in_left_top_r.y + 35.0 * scale);
    let F_in_right_bot_r = Pos2::new(F_in_left_bot_r.x + 5.0 * scale, F_in_left_bot_r.y);
    let F_in_right_top_r = Pos2::new(F_in_right_bot_r.x, F_in_right_bot_r.y - 37.0 * scale);
    let F_in_right = vec![F_in_left_top_r, F_in_med_r, F_in_left_bot_r, F_in_right_bot_r, F_in_right_top_r];

    let F_out_right_top_r = Pos2::new(OT_left_bot_r.x, MT_right_bot_r.y + offset);
    let F_out_left_top_r = Pos2::new(F_out_right_top_r.x - 5.0 * scale, F_out_right_top_r.y + offset);
    let F_out_left_bot_r = Pos2::new(F_out_left_top_r.x, F_out_left_top_r.y + 37.0 * scale);
    let F_out_right_bot_r = Pos2::new(F_out_left_bot_r.x + 2.0 * scale, F_out_left_bot_r.y);
    let F_out_right = vec![F_out_right_top_r, F_out_left_top_r, F_out_left_bot_r, F_out_right_bot_r];

    painter.add(Shape::convex_polygon(F_in_left, ColorQualifier(Muscle::Forearms, &primary_muscles, &secondary_muscles), border));
    painter.add(Shape::convex_polygon(F_in_right, ColorQualifier(Muscle::Forearms, &primary_muscles, &secondary_muscles), border));
    painter.add(Shape::convex_polygon(F_out_left, ColorQualifier(Muscle::Forearms, &primary_muscles, &secondary_muscles), border));
    painter.add(Shape::convex_polygon(F_out_right, ColorQualifier(Muscle::Forearms, &primary_muscles, &secondary_muscles), border));
    // painter.add(Shape::convex_polygon(F_in_left, muscle_color, border));
    // painter.add(Shape::convex_polygon(F_out_left, muscle_color, border));
    // painter.add(Shape::convex_polygon(F_in_right, muscle_color, border));
    // painter.add(Shape::convex_polygon(F_out_right, muscle_color, border));

    //HAND
    let H_left_top_l = Pos2::new(F_out_left_bot_l.x - 3.0 * scale, F_out_left_bot_l.y + offset);
    let H_right_top_l = Pos2::new(F_in_right_bot_l.x + 3.0 * scale, F_in_right_bot_l.y + offset);
    let H_right_bot_l = Pos2::new(H_right_top_l.x, H_right_top_l.y + 10.0 * scale);
    let H_left_bot_l = Pos2::new(H_left_top_l.x, H_right_bot_l.y);
    let H_left = vec![H_left_top_l, H_right_top_l, H_right_bot_l, H_left_bot_l];

    let H_left_top_r = Pos2::new(F_in_left_bot_r.x - 3.0 * scale, F_in_left_bot_r.y + offset);
    let H_right_top_r = Pos2::new(F_out_right_bot_r.x + 3.0 * scale, F_out_right_bot_r.y + offset);
    let H_right_bot_r = Pos2::new(H_right_top_r.x, H_right_top_r.y + 10.0 * scale);
    let H_left_bot_r = Pos2::new(H_left_top_r.x, H_right_bot_r.y);
    let H_right = vec![H_left_top_r, H_right_top_r, H_right_bot_r, H_left_bot_r];

    painter.add(Shape::convex_polygon(H_left, muscle_color, border));
    painter.add(Shape::convex_polygon(H_right, muscle_color, border));

    //LATS
    let lats_height = 60.0 * scale;

    let L_top_right_l = Pos2::new(I_right_bot_l.x + 0.3 * scale, I_right_bot_l.y + offset);
    let L_top_left_l = Pos2::new(I_left_bot_l.x + 0.1 * scale, I_left_bot_l.y + offset);
    let L_bot_l = Pos2::new(L_top_left_l.x + 9.0 * scale, L_top_left_l.y + lats_height);
    let L_mid_l = Pos2::new(T_bot_l.x - offset, T_bot_l.y);
    let L_left = vec![L_top_right_l, L_top_left_l, L_bot_l, L_mid_l];

    let L_top_left_r = Pos2::new(I_left_bot_r.x - 0.3 * scale, I_left_bot_r.y + offset);
    let L_top_right_r = Pos2::new(I_right_bot_r.x - 0.1 * scale, I_right_bot_r.y + offset);
    let L_bot_r = Pos2::new(L_top_right_r.x - 9.0 * scale, L_top_right_r.y + lats_height);
    let L_mid_r = Pos2::new(T_bot_r.x + offset, T_bot_r.y);
    let L_right = vec![L_top_left_r, L_top_right_r, L_bot_r, L_mid_r];

    painter.add(Shape::convex_polygon(L_left, ColorQualifier(Muscle::Lats, &primary_muscles, &secondary_muscles), border));
    painter.add(Shape::convex_polygon(L_right, ColorQualifier(Muscle::Lats, &primary_muscles, &secondary_muscles), border));
    // painter.add(Shape::convex_polygon(L_left, muscle_color, border));
    // painter.add(Shape::convex_polygon(L_right, muscle_color, border));

    //LOWER BACK (LB)
    let lower_back_height = 35.0 * scale;

    let LB_top_l = Pos2::new(T_bot_l.x, T_bot_l.y + 0.15 * scale);
    let LB_bot_l = Pos2::new(LB_top_l.x, LB_top_l.y + lower_back_height);
    let LB_mid_l = Pos2::new(L_bot_l.x + offset, L_bot_l.y);
    let LB_left = vec![LB_top_l, LB_bot_l, LB_mid_l];

    let LB_top_r = Pos2::new(T_bot_r.x, T_bot_r.y + 0.15 * scale);
    let LB_bot_r = Pos2::new(LB_top_r.x, LB_top_r.y + lower_back_height);
    let LB_mid_r = Pos2::new(L_bot_r.x - offset, L_bot_r.y);
    let LB_right = vec![LB_top_r, LB_bot_r, LB_mid_r];

    painter.add(Shape::convex_polygon(LB_left, ColorQualifier(Muscle::LowerBack, &primary_muscles, &secondary_muscles), border));
    painter.add(Shape::convex_polygon(LB_right, ColorQualifier(Muscle::LowerBack, &primary_muscles, &secondary_muscles), border));
    // painter.add(Shape::convex_polygon(LB_left, muscle_color, border));
    // painter.add(Shape::convex_polygon(LB_right, muscle_color, border));

    //GLUTES
    let glutes_height = 20.0 * scale;

    let G_left_top_l = Pos2::new(L_bot_l.x, L_bot_l.y + offset);
    let G_right_top_l = Pos2::new(LB_bot_l.x, LB_bot_l.y + offset);
    let G_right_bot_l = Pos2::new(G_right_top_l.x, G_right_top_l.y + glutes_height);
    let G_mid_bot_l = Pos2::new(G_right_bot_l.x - 12.0 * scale, G_right_bot_l.y + glutes_height * 0.3 * scale);
    let G_left_bot_l = Pos2::new(G_left_top_l.x - 9.0 * scale, G_right_bot_l.y - 8.0 * scale);
    let G_left = vec![G_left_top_l, G_right_top_l, G_right_bot_l, G_mid_bot_l, G_left_bot_l];

    let G_right_top_r = Pos2::new(L_bot_r.x, L_bot_r.y + offset);
    let G_left_top_r = Pos2::new(LB_bot_r.x, LB_bot_r.y + offset);
    let G_left_bot_r = Pos2::new(G_left_top_r.x, G_left_top_r.y + glutes_height);
    let G_mid_bot_r = Pos2::new(G_left_bot_r.x + 12.0 * scale, G_left_bot_r.y + glutes_height * 0.3 * scale);
    let G_right_bot_r = Pos2::new(G_right_top_r.x + 9.0 * scale, G_left_bot_r.y - 8.0 * scale);
    let G_right = vec![G_right_top_r, G_left_top_r, G_left_bot_r, G_mid_bot_r, G_right_bot_r];

    painter.add(Shape::convex_polygon(G_left, ColorQualifier(Muscle::Glutes, &primary_muscles, &secondary_muscles), border));
    painter.add(Shape::convex_polygon(G_right, ColorQualifier(Muscle::Glutes, &primary_muscles, &secondary_muscles), border));
    // painter.add(Shape::convex_polygon(G_left, muscle_color, border));
    // painter.add(Shape::convex_polygon(G_right, muscle_color, border));

    //INNER HAMSTRING (IH)
    let inner_hamstring_height = 20.0 * scale;

    let IH_right_top_l = Pos2::new(G_right_bot_l.x, G_right_bot_l.y + offset);
    let IH_bot_l = Pos2::new(IH_right_top_l.x - offset * 3.0, IH_right_top_l.y + inner_hamstring_height);
    let IH_left_top_l = Pos2::new(IH_bot_l.x, IH_bot_l.y - inner_hamstring_height * 0.86);
    let IH_left = vec![IH_left_top_l, IH_bot_l, IH_right_top_l];

    let IH_left_top_r = Pos2::new(G_left_bot_r.x, G_left_bot_r.y + offset);
    let IH_bot_r = Pos2::new(IH_left_top_r.x + offset * 3.0, IH_left_top_r.y + inner_hamstring_height);
    let IH_right_top_r = Pos2::new(IH_bot_r.x, IH_bot_r.y - inner_hamstring_height * 0.86);
    let IH_right = vec![IH_right_top_r, IH_bot_r, IH_left_top_r];

    painter.add(Shape::convex_polygon(IH_left, ColorQualifier(Muscle::Hamstrings, &primary_muscles, &secondary_muscles), border));
    painter.add(Shape::convex_polygon(IH_right, ColorQualifier(Muscle::Hamstrings, &primary_muscles, &secondary_muscles), border));
    // painter.add(Shape::convex_polygon(IH_left, muscle_color, border));
    // painter.add(Shape::convex_polygon(IH_right, muscle_color, border));

    //HAMSTRINGS
    let hamstring_height = 45.0 * scale;

    let H_right_top_l = Pos2::new(IH_left_top_l.x - offset, IH_left_top_l.y - 0.1 * scale);
    let H_mid_top_l = Pos2::new(G_mid_bot_l.x, G_mid_bot_l.y + offset);
    let H_left_top_l = Pos2::new(L_bot_l.x - offset * 2.0, H_mid_top_l.y - 9.0 * scale);
    let H_left_bot_l = Pos2::new(H_left_top_l.x - 3.0 * scale, H_left_top_l.y + hamstring_height);
    let H_right_bot_l = Pos2::new(H_right_top_l.x, H_left_bot_l.y);
    let H_left = vec![H_right_top_l, H_mid_top_l, H_left_top_l, H_left_bot_l, H_right_bot_l];

    let H_left_top_r = Pos2::new(IH_right_top_r.x + offset, IH_right_top_r.y - 0.1 * scale);
    let H_mid_top_r = Pos2::new(G_mid_bot_r.x, G_mid_bot_r.y + offset);
    let H_right_top_r = Pos2::new(L_bot_r.x + offset * 2.0, H_mid_top_r.y - 9.0 * scale);
    let H_right_bot_r = Pos2::new(H_right_top_r.x + 3.0 * scale, H_right_top_r.y + hamstring_height);
    let H_left_bot_r = Pos2::new(H_left_top_r.x, H_right_bot_r.y);
    let H_right = vec![H_right_top_r , H_mid_top_r, H_left_top_r, H_left_bot_r, H_right_bot_r];

    painter.add(Shape::convex_polygon(H_left, ColorQualifier(Muscle::Hamstrings, &primary_muscles, &secondary_muscles), border));
    painter.add(Shape::convex_polygon(H_right, ColorQualifier(Muscle::Hamstrings, &primary_muscles, &secondary_muscles), border));
    // painter.add(Shape::convex_polygon(H_left, muscle_color, border));
    // painter.add(Shape::convex_polygon(H_right, muscle_color, border));

    //OUTER HAMSTRINGS (OH)
    let outer_hamstrings_height = 35.0 * scale;

    let OH_left_top_l = Pos2::new(G_left_bot_l.x, G_left_bot_l.y + offset);
    let OH_right_top_l = Pos2::new(H_left_top_l.x - offset, H_left_top_l.y - 0.1 * scale);
    let OH_bot_l = Pos2::new(H_left_bot_l.x - offset, OH_right_top_l.y + outer_hamstrings_height);
    let OH_left = vec![OH_left_top_l, OH_right_top_l, OH_bot_l];

    let OH_right_top_r = Pos2::new(G_right_bot_r.x, G_right_bot_r.y + offset);
    let OH_left_top_r = Pos2::new(H_right_top_r.x + offset, H_right_top_r.y - 0.1 * scale);
    let OH_bot_r = Pos2::new(H_right_bot_r.x + offset, OH_left_top_r.y + outer_hamstrings_height);
    let OH_right = vec![OH_right_top_r, OH_left_top_r, OH_bot_r];

    painter.add(Shape::convex_polygon(OH_left, ColorQualifier(Muscle::Hamstrings, &primary_muscles, &secondary_muscles), border));
    painter.add(Shape::convex_polygon(OH_right, ColorQualifier(Muscle::Hamstrings, &primary_muscles, &secondary_muscles), border));
    // painter.add(Shape::convex_polygon(OH_left, muscle_color, border));
    // painter.add(Shape::convex_polygon(OH_right, muscle_color, border));

    //OUTER CALFS (OC)
    let outer_calfs_height = 50.0 * scale;

    let OC_left_top_l = Pos2::new(H_left_bot_l.x, H_left_bot_l.y + offset);
    let OC_right_top_l = Pos2::new(I_right_bot_l.x, H_right_bot_l.y + offset);
    let OC_right_bot_l = Pos2::new(OC_right_top_l.x, OC_right_top_l.y + outer_calfs_height);
    let OC_left_bot_l = Pos2::new(OC_right_bot_l.x - 5.0 * scale, OC_right_bot_l.y);
    let OC_left_mid_l = Pos2::new(OC_left_top_l.x - 4.0 * scale, OC_left_top_l.y + outer_calfs_height * 0.4);
    let OC_left = vec![OC_left_top_l, OC_right_top_l, OC_right_bot_l, OC_left_bot_l, OC_left_mid_l];

    let OC_right_top_r = Pos2::new(H_right_bot_r.x, H_right_bot_r.y + offset);
    let OC_left_top_r = Pos2::new(I_left_bot_r.x, H_left_bot_r.y + offset);
    let OC_left_bot_r = Pos2::new(OC_left_top_r.x, OC_left_top_r.y + outer_calfs_height);
    let OC_right_bot_r = Pos2::new(OC_left_bot_r.x + 5.0 * scale, OC_left_bot_r.y);
    let OC_right_mid_r = Pos2::new(OC_right_top_r.x + 4.0 * scale, OC_right_top_r.y + outer_calfs_height * 0.4);
    let OC_right = vec![OC_right_top_r, OC_left_top_r, OC_left_bot_r, OC_right_bot_r, OC_right_mid_r];

    painter.add(Shape::convex_polygon(OC_left, ColorQualifier(Muscle::Calfs, &primary_muscles, &secondary_muscles), border));
    painter.add(Shape::convex_polygon(OC_right, ColorQualifier(Muscle::Calfs, &primary_muscles, &secondary_muscles), border));
    // painter.add(Shape::convex_polygon(OC_left, muscle_color, border));
    // painter.add(Shape::convex_polygon(OC_right, muscle_color, border));

    //INNER CALFS (IC)
    let IC_left_top_l = Pos2::new(OC_right_top_l.x + offset, OC_right_top_l.y);
    let IC_right_top_l = Pos2::new(H_right_bot_l.x, H_right_bot_l.y + offset);
    let IC_right_mid_l = Pos2::new(IC_right_top_l.x + 2.0 * scale, OC_left_mid_l.y);
    let IC_right_bot_l = Pos2::new(IC_right_top_l.x, OC_right_bot_l.y);
    let IC_left_bot_l = Pos2::new(IC_left_top_l.x, OC_right_bot_l.y);
    let IC_left = vec![IC_left_top_l, IC_right_top_l, IC_right_mid_l, IC_right_bot_l, IC_left_bot_l];

    let IC_right_top_r = Pos2::new(OC_left_top_r.x - offset, OC_left_top_r.y);
    let IC_left_top_r = Pos2::new(H_left_top_r.x, H_left_bot_r.y + offset);
    let IC_left_mid_r = Pos2::new(IC_left_top_r.x - 2.0 * scale, OC_right_mid_r.y);
    let IC_left_bot_r = Pos2::new(IC_left_top_r.x, OC_left_bot_r.y);
    let IC_right_bot_r = Pos2::new(IC_right_top_r.x,OC_left_bot_r.y);
    let IC_right = vec![IC_left_top_r, IC_left_mid_r, IC_left_bot_r, IC_right_bot_r, IC_right_top_r];

    painter.add(Shape::convex_polygon(IC_left, ColorQualifier(Muscle::Calfs, &primary_muscles, &secondary_muscles), border));
    painter.add(Shape::convex_polygon(IC_right, ColorQualifier(Muscle::Calfs, &primary_muscles, &secondary_muscles), border));
    // painter.add(Shape::convex_polygon(IC_left, muscle_color, border));
    // painter.add(Shape::convex_polygon(IC_right, muscle_color, border));

    //FEET
    let F_min_l = Pos2::new(OC_left_bot_l.x - 3.0 * scale, OC_left_bot_l.y + offset);
    let F_max_l = Pos2::new(IC_right_bot_l.x + 2.0 * scale, IC_right_bot_l.y + offset + 10.0 * scale);
    let F_left = egui::Rect::from_min_max(F_min_l, F_max_l);

    painter.rect_filled(
        F_left,
        0.0,
        Color32::GRAY,
    );

    let F_min_r = Pos2::new(IC_left_bot_r.x - 3.0 * scale, IC_left_bot_r.y + offset);
    let F_max_r = Pos2::new(OC_right_bot_r.x + 2.0 * scale, OC_right_bot_r.y + offset + 10.0 * scale);
    let F_right = egui::Rect::from_min_max(F_min_r, F_max_r);

    painter.rect_filled(
        F_right,
        0.0,
        Color32::GRAY,
    );
    // let SD_right_top_l = Pos2::new(RD_left_top_l.x - offset, RD_left_top_l.y + offset * 2.0);
    // let SD_
    // let SD_right_bot_l = Pos2::new(RD_left_bot_l.x - offset, RD_left_bot_l.y );
    // let SD_left_bot_l = Pos2::new(SD_right_bot_l.x - side_delt_width, SD_right_bot_l.y - offset);
    // let SD_left = vec![SD_top_l, SD_right_bot_l, SD_left_bot_l];
    //
    // let SD_top_r = Pos2::new(RD_right_top_r.x + offset, RD_right_top_r.y + offset * 2.0);
    // let SD_left_bot_r = Pos2::new(RD_right_bot_r.x + offset, RD_right_bot_r.y);
    // let SD_right_bot_r = Pos2::new(SD_left_bot_r.x + side_delt_width, SD_left_bot_r.y - offset);
    // let SD_right = vec![SD_top_r, SD_left_bot_r, SD_right_bot_r];

}

pub fn ColorQualifier(muscle: Muscle, primary_muscles: &Vec<Muscle>, secondary_muscles: &Vec<Muscle>) -> Color32 {
    if primary_muscles.contains(&muscle) {
        Color32::from_rgb(0, 75, 141)
        // Color32::BLUE
    } else if secondary_muscles.contains(&muscle) {
        Color32::from_rgb(59, 163, 255)
    } else {
        Color32::GRAY
    }
}