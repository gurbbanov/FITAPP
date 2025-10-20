use std::{collections::HashMap, option};
use egui::{Context, TextureHandle, ImageSource, include_image};
use serde::{Deserialize, Serialize};
use crate::tools::{load_png, weekday_iso};
use time::OffsetDateTime;
use chrono::{Local, NaiveDate, NaiveTime};

#[derive(Clone)]
pub struct AppMedia<'a> {
    pub ambient_blue: ImageSource<'a>,
    pub ambient_red: ImageSource<'a>,
    pub ambient_green: ImageSource<'a>,
    pub left_arrow: ImageSource<'a>,
    pub right_arrow: ImageSource<'a>,
    pub default_pp: ImageSource<'a>,
    pub pen: ImageSource<'a>,
    pub plus: ImageSource<'a>,
    pub fullscreen: ImageSource<'a>,
    pub home: ImageSource<'a>,
    pub friends: ImageSource<'a>,
    pub workouts: ImageSource<'a>,
    pub calories: ImageSource<'a>,
    pub water: ImageSource<'a>,
    pub statistics: ImageSource<'a>,
    pub calendar: ImageSource<'a>,
    pub save_button_l: ImageSource<'a>,
    pub cancel_button_l: ImageSource<'a>,
    pub save_button_d: ImageSource<'a>,
    pub cancel_button_d: ImageSource<'a>,
    pub switch: ImageSource<'a>,
    // pub ambient_test: TextureHandle,
}

impl AppMedia<'_> {
    pub fn load_media(ctx: &Context) -> Self {
        Self {
            ambient_blue:  include_image!("../medias/ambient_blue.png"),
            ambient_red: include_image!("../medias/ambient_red.png"),
            ambient_green: include_image!("../medias/ambient_green.png"),
            // left_arrow: load_png(ctx, include_bytes!("../arrow_left.png")).expect("не удалось загрузить left_arrow"),
            left_arrow: include_image!("../medias/arrow_left.png"),
            right_arrow: include_image!("../medias/arrow_right.png"),
            default_pp: include_image!("../medias/user.jpg"),
            pen: include_image!("../medias/pen.png"),
            plus: include_image!("../medias/plus.png"),
            fullscreen: include_image!("../medias/fullscreen.png"),
            home: include_image!("../medias/home.png"),
            friends: include_image!("../medias/friends.png"),
            workouts: include_image!("../medias/workouts.png"),
            calories: include_image!("../medias/calories.png"),
            water: include_image!("../medias/water.png"),
            statistics: include_image!("../medias/statistics.png"),
            calendar: include_image!("../medias/calendar.png"),
            save_button_l: include_image!("../medias/save_button_l.png"),
            cancel_button_l: include_image!("../medias/cancel_button_l.png"),
            save_button_d: include_image!("../medias/save_button_d.png"),
            cancel_button_d: include_image!("../medias/cancel_button_d.png"),
            switch: include_image!("../medias/switch.png"),
            // ambient_test: load_png(ctx, include_bytes!("../test.png")).expect("не удалось загрузить ambient_test"),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct UserDataPack {
    pub user_information: UserInformation,
    pub account_data: AccountData,
    pub all_workout_data: AllWorkoutData,
    // pub planned_workout_data: HashMap<NaiveDate, Vec<WorkoutPlanned>>,
    pub planned_workout_data: WorkoutPlannedData,
    pub macro_data: MacroData,
    pub water_data: WaterData,
}

impl UserDataPack {
    pub fn default() -> Self {
        Self {
            user_information: UserInformation::default(),
            account_data: AccountData::default(),
            all_workout_data: AllWorkoutData::default(),
            // planned_workout_data: HashMap::from([(NaiveDate::from_ymd(2025, 9, 21), vec![WorkoutPlanned::new(WorkoutTemplate::default(), NaiveDate::from_ymd(2025, 9, 21))])]),
            planned_workout_data: WorkoutPlannedData::default(),
            macro_data: MacroData::default(),
            water_data: WaterData::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct UserInformation {
    pub name: String,
    pub username: String,
    pub age: u32,
    pub weight: u32,
    pub height: u32,
    pub registration_date: String,
}

impl UserInformation {
    pub fn default() -> Self {
        Self {
            name: String::from("name"),
            username: String::from("username"),
            age: 0,
            weight: 0,
            height: 0,
            registration_date: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct AccountData {
    pub level: u32,
    pub xp: u32,
    pub current_streak: u32,
    pub lifted_weight: u32,
    pub registrated_cals: u32,
    pub registrated_meals: u32,
}

impl AccountData {
    pub fn default() -> Self {
        Self {
            level: 0,
            xp: 0,
            current_streak: 0,
            lifted_weight: 0,
            registrated_cals: 0,
            registrated_meals: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct AllWorkoutData {
    pub total_volume: u32,
    pub total_sets: u32,
    pub total_reps: u32,
    pub total_time: u32,
    pub worked_out: u32,
    pub prs: u32,
    pub week_volume: u32,
    pub week_sets: u32,
    pub week_reps: u32,
    pub week_time: u32,
    pub workouts: Vec<WorkoutDone>,
}

impl AllWorkoutData {
    pub fn default() -> Self {
        Self {
            total_volume: 0,
            total_sets: 0,
            total_reps: 0,
            total_time: 0,
            worked_out: 0,
            prs: 0,
            week_volume: 0,
            week_sets: 0,
            week_reps: 0,
            week_time: 0,
            workouts: vec![WorkoutDone::default()],
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub enum Exercise {
    #[default] BenchPress,
    Deadlift,
    Squat,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct WorkoutTemplate {
    pub workout_name: String,
    pub exercises: Vec<Exercise>,
}

impl WorkoutTemplate {
    pub fn default() -> Self {
        Self {
            workout_name: String::from("full body"),
            exercises: vec![Exercise::BenchPress, Exercise::Deadlift, Exercise::Squat],
        }
    }

    pub fn legs() -> Self {
        Self {
            workout_name: String::from("legs"),
            exercises: vec![Exercise::Squat],
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct WorkoutDone {
    pub template: WorkoutTemplate,
    pub date: String,
    pub volume: u32,
    pub length: u32,
    pub prs: u32,
}

impl WorkoutDone {
    pub fn default() -> Self {
        Self {
            template: WorkoutTemplate::default(),
            date: String::new(),
            volume: 0,
            length: 0,
            prs: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct WorkoutPlanned {
    pub template: WorkoutTemplate,
    pub date: NaiveDate,
}

impl WorkoutPlanned {
    pub fn new(template: WorkoutTemplate, date: NaiveDate) -> Self {
        Self {
            template,
            date,
        }
    }

    pub fn leg_day(date: NaiveDate) -> Self {
        Self {
            template: WorkoutTemplate::legs(),
            date,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct WorkoutPlannedData {
    pub workouts: HashMap<NaiveDate, Vec<WorkoutPlanned>>,
}

impl WorkoutPlannedData {
    pub fn default() -> Self {
        Self {
            workouts: HashMap::from([]),
// (NaiveDate::from_ymd(2025, 9,27), vec![WorkoutPlanned::new(WorkoutTemplate::default(), NaiveDate::from_ymd(2025, 9, 21))])
        }
    }

    pub fn add_workout(&mut self, date: NaiveDate, workout: WorkoutPlanned) -> Result<(), String> {
        self.workouts.insert(date, vec![workout]);
        Ok(())
    }

    pub fn remove_workout(&mut self, data: NaiveDate, workout: WorkoutPlanned) -> Result<(), String> {
        if let Some(workouts) = self.workouts.get_mut(&data) {
            if let Some(index) = workouts.iter().position(|w| w.date == data) {
                workouts.remove(index);
                Ok(())
            } else {
                Err("no such date".to_string())
            }
        } else {
            Err("no such date".to_string())
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct WorkoutData {
    pub workout_name: String,
    pub workout_date: String,
    pub workout_volume: u32,
    pub workout_length: u32,
    pub workout_prs: u32,
}

impl WorkoutData {
    pub fn default() -> Self {
        Self {
            workout_name: String::new(),
            workout_date: String::new(),
            workout_volume: 0,
            workout_length: 0,
            workout_prs: 0,
        }
    }
}   

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct MacroData {
    pub meal_history: HashMap<NaiveDate, Vec<Eat>>,

    pub calory_goal: u32,
    pub protein_goal: u32,
    pub carb_goal: u32,
    pub fat_goal: u32,

    pub calory_registered: u32,
    pub protein_registered: u32,
    pub carb_registered: u32,
    pub fat_registered: u32,
    pub meal_registered: u32,
}

impl MacroData {
    pub fn default() -> Self {
        Self {
            meal_history: HashMap::from([(Local::now().date_naive(), vec![Eat::new(chrono::Local::now().time(), Meal::new(100, 100, 100, 100)), Eat::new(chrono::Local::now().time(), Meal::new(10, 10, 10, 10))])]),
            calory_goal: 1000,
            protein_goal: 200,
            carb_goal: 90,
            fat_goal: 50,
            calory_registered: 0,
            protein_registered: 0,
            carb_registered: 0,
            fat_registered: 0,
            meal_registered: 0,
        }
    }

    // pub fn update(&mut self, calory: &Option<String>, protein: &Option<String>, carb: &Option<String>, fat: &Option<String>) {
    //     if let Some(calory) = calory {
    //         self.calory_registered += calory.parse::<u32>().unwrap();
    //     }
    //     if let Some(protein) = protein {
    //         // self.protein_registered += protein.parse::<u32>();
    //     }
    //     if let Some(carb) = carb {
    //         // self.carb_registered += carb.parse::<u32>();
    //     }
    //     if let Some(fat) = fat {
    //         // self.fat_registered += fat.parse::<u32>();
    //     }
    // }

    pub fn update(&mut self, calory: &str, protein: &str, carb: &str, fat: &str) {
        self.calory_registered += calory.trim().parse::<u32>().unwrap_or(0);
        self.protein_registered += protein.trim().parse::<u32>().unwrap_or(0);
        self.carb_registered += carb.trim().parse::<u32>().unwrap_or(0);
        self.fat_registered += fat.trim().parse::<u32>().unwrap_or(0);
    }

    pub fn add_meal(&mut self, selected_date: NaiveDate, calory: &str, protein: &str, carb: &str, fat: &str) {
        self.meal_history.entry(selected_date).or_default().insert(0, Eat::new(chrono::Local::now().time(), Meal::new(
            calory.trim().parse::<u32>().unwrap_or(0),
            protein.trim().parse::<u32>().unwrap_or(0),
            carb.trim().parse::<u32>().unwrap_or(0),
            fat.trim().parse::<u32>().unwrap_or(0),
        )));
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Eat {
    pub date: NaiveTime,
    pub meal: Meal,
}

impl Eat {
    pub fn new(date: NaiveTime, meal: Meal) -> Self {
        Self {
            date,
            meal,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Meal {
    pub name: String,
    pub calory: u32,
    pub protein: u32,
    pub carb: u32,
    pub fat: u32,
}

impl Meal {
    pub fn new(calory: u32, protein: u32, carb: u32, fat: u32) -> Self {
        Self {
            name: String::from("Meal"),
            calory,
            protein,
            carb,
            fat,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct WaterData {
    pub water_goal: u32,
    pub water_registered: u32,
}

impl WaterData {
    pub fn default() -> Self {
        Self {
            water_goal: 0,
            water_registered: 0,
        }
    }
}  

#[derive(Debug, Clone)]
pub struct States {
    pub calory_add_modal: bool,
    pub selected_tab: usize,
    pub skip_days: i16,
    pub selected_day: NaiveDate,
    // pub selected_day_workouts_ui: NaiveDate,
    // pub selected_day_calory_ui: NaiveDate,
    pub strip_size: f32,
    pub calendar_mode_calory_ui: bool,
    pub calory_add_clicked: bool,
    pub calory_add_value: String,
    pub protein_add_value: String,
    pub carb_add_value: String,
    pub fat_add_value: String,
    pub alert_modal: bool,
    pub delete_was_positive: bool,
    pub water_add_clicked: bool,
    // pub calories:u32,
    // pub proteins: u32,
    // pub carbs: u32,
    // pub fats: u32,
}

impl States {
    pub fn default() -> Self {
        Self {
            calory_add_modal: false,
            selected_tab: 4,
            skip_days: 0,
            // selected_day: OffsetDateTime::now_local().unwrap(),
            selected_day: Local::now().date_naive(),
            // selected_day_workouts_ui: Local::now().date_naive(),
            // selected_day_calory_ui: Local::now().date_naive(),
            strip_size: 150.0,
            calendar_mode_calory_ui: true,
            calory_add_clicked: false,
            calory_add_value: String::from("0"), 
            protein_add_value: String::from("0"),
            carb_add_value: String::from("0"),
            fat_add_value: String::from("0"),
            alert_modal: false,
            delete_was_positive: false,
            water_add_clicked: false,
            // calories: 0,
            // proteins: 0,
            // carbs: 0,
            // fats: 0,
        }
    }

    pub fn reset_macros(&mut self) {
        self.calory_add_value = String::from("0");
        self.protein_add_value = String::from("0");
        self.carb_add_value = String::from("0");
        self.fat_add_value = String::from("0");
    }
}

pub trait Summary {
    fn summarize(&mut self, selected_day: Option<NaiveDate>);
}

impl Summary for MacroData {
    fn summarize(&mut self, selected_day: Option<NaiveDate>) {
        if let Some(eats) = self.meal_history.get(&selected_day.unwrap()) {
            let mut calories: u32 = 0;
            let mut proteins: u32 = 0;
            let mut carbs: u32 = 0;
            let mut fats: u32 = 0;

            for eat in eats {
                calories += eat.meal.calory;
                proteins += eat.meal.protein;
                carbs += eat.meal.carb;
                fats += eat.meal.fat;
            }

            self.calory_registered = calories;
            self.protein_registered = proteins;
            self.carb_registered = carbs;
            self.fat_registered = fats;
        } else {
            self.calory_registered = 0;
            self.protein_registered = 0;
            self.carb_registered = 0;
            self.fat_registered = 0;
            // println!("нет данных за выбранный день");
            // MacroData::default()
        }
    }
}
