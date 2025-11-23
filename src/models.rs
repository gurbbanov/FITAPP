use std::{collections::HashMap, option, fmt};
use egui::{Context, ImageSource, TextureHandle, ahash::HashSet, include_image};
use serde::{Deserialize, Serialize};
use crate::tools::{load_png, weekday_iso};
use time::OffsetDateTime;
use chrono::{Local, NaiveDate, NaiveTime};
use egui::ahash::HashSetExt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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
    pub coffee: ImageSource<'a>,
    pub bottle: ImageSource<'a>,
    pub glass: ImageSource<'a>,
    pub drop: ImageSource<'a>,
    pub ml: ImageSource<'a>,
    pub workout_templates: ImageSource<'a>,
    pub remove: ImageSource<'a>,
    pub bed: ImageSource<'a>,
}

impl AppMedia<'_> {
    pub fn load_media(ctx: &Context) -> Self {
        Self {
            ambient_blue:  include_image!("../medias/ambient_blue.png"),
            ambient_red: include_image!("../medias/ambient_red.png"),
            ambient_green: include_image!("../medias/ambient_green.png"),
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
            coffee: include_image!("../medias/coffee.png"),
            bottle: include_image!("../medias/bottle.png"),
            glass: include_image!("../medias/glass.png"),
            drop: include_image!("../medias/drop.png"),
            ml: include_image!("../medias/ml.png"),
            workout_templates: include_image!("../medias/workout_templates.png"),
            remove: include_image!("../medias/remove.png"),
            bed: include_image!("../medias/bed.png"),
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
    pub workout_templates: HashMap<String, WorkoutTemplate>,
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
            workout_templates: HashMap::from(
                [
                (String::from("leg day"), 
                    WorkoutTemplate{workout_name: String::from("leg day"), 
                    exercises: vec![Exercises::HackSquat, Exercises::LegExtension, Exercises::LegCurl]}
                ),
                (String::from("pull day"),
                    WorkoutTemplate{workout_name: String::from("pull day"),
                    exercises: vec![Exercises::BenchPress, Exercises::TricepDips]}
                )
                ]
            )
        }
    }

    pub fn create_workout_template(&mut self, workout_name: String, exercises: Vec<Exercises>) {
        self.workout_templates.entry(workout_name.clone()).insert_entry(WorkoutTemplate { workout_name, exercises });
    }

}

#[derive(Serialize, Deserialize, Default, Debug, Clone, EnumIter)]
pub enum Exercises {
    #[default] BenchPress,
    TricepDips,
    Deadlift,
    Squat,
    HackSquat,
    LegPress,
    LegExtension,
    LegCurl
}

impl fmt::Display for Exercises {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Exercises::BenchPress => "Bench Press",
            Exercises::TricepDips => "Tricep Dips",
            Exercises::Deadlift => "Deadlift",
            Exercises::Squat => "Squat",
            Exercises::HackSquat => "Hack Squat",
            Exercises::LegPress => "Leg Press",
            Exercises::LegExtension => "Leg Extension",
            Exercises::LegCurl => "Leg Curl",
        };
        write!(f, "{name}")
    }
}

pub fn muscles_for(ex: &Exercises) -> (Vec<Muscle>, Vec<Muscle>) {
    match ex {
        Exercises::BenchPress => (
            vec![Muscle::LowerChest, Muscle::UpperChest], 
            vec![Muscle::Triceps],                   
        ),
        Exercises::TricepDips => (
            vec![Muscle::Triceps],
            vec![Muscle::LowerChest],
        ),
        Exercises::Deadlift => (
            vec![Muscle::LowerBack, Muscle::Glutes],      
            vec![Muscle::Hamstrings],                
        ),
        Exercises::Squat => (
            vec![Muscle::Quads],                     
            vec![Muscle::Glutes, Muscle::Hamstrings],
        ),
        Exercises::HackSquat => (
            vec![Muscle::Quads, Muscle::Hips],
            vec![Muscle::Hamstrings],
        ),
        Exercises::LegPress => (
            vec![Muscle::Quads, Muscle::Hamstrings],
            vec![Muscle::ExtHips],
        ),
        Exercises::LegExtension => (
            vec![Muscle::Quads, Muscle::ExtHips],
            vec![Muscle::Hips],
        ),
        Exercises::LegCurl => (
            vec![Muscle::Hamstrings],
            vec![Muscle::Hips],
        ),
    }
}

pub fn muscle_for_workout(exercises: &Vec<Exercises>) -> (Vec<Muscle>, Vec<Muscle>) {
    let mut primary_muscle = HashSet::new();
    let mut secondary_muscle = HashSet::new();

    for ex in exercises {
        let m = muscles_for(ex);
        primary_muscle.extend(m.0);
        secondary_muscle.extend(m.1);
    }

    (primary_muscle.into_iter().collect::<Vec<Muscle>>(), secondary_muscle.into_iter().collect::<Vec<Muscle>>())
}


#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct WorkoutTemplate {
    pub workout_name: String,
    pub exercises: Vec<Exercises>,
    // pub rimary_muscles: Vec<Muscle>,
    // pub secondary_muscles: Vec<Muscle>,
}

impl WorkoutTemplate {
    pub fn default() -> Self {
        Self {
            workout_name: String::from("full body"),
            exercises: vec![Exercises::BenchPress, Exercises::Deadlift, Exercises::Squat],
            // primary_muscles: vec![Muscle::LowerChest, Muscle::Quads, Muscle::Hips, Muscle::Hamstrings, Muscle::Calfs],
            // secondary_muscles: vec![Muscle::Forearms, Muscle::UpperChest, Muscle::LowerBack],
        }
    }

    pub fn rest() -> Self {
        Self {
            workout_name: String::from("rest"),
            exercises: vec![],
        }
    }

    // pub fn new(name: String, exercises: Vec<Exercises>) -> Self {
    //     Self {
    //         workout_name: name,
    //         exercises: exercises,
    //     }
    // }

    pub fn legs() -> Self {
        Self {
            workout_name: String::from("legs"),
            exercises: vec![Exercises::Squat],
            // primary_muscles: vec![Muscle::Quads, Muscle::Hips, Muscle::Hamstrings],
            // secondary_muscles: vec![Muscle::Calfs],
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

    pub fn rest(date: NaiveDate) -> Self {
        Self {
            template: WorkoutTemplate::rest(),
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

// #[derive(Serialize, Deserialize, Default, Debug, Clone, Allocator)]
// pub struct Rest {
//     date: NaiveDate,
// }

impl WorkoutPlannedData {
    pub fn default() -> Self {
        Self {
            workouts: HashMap::from([
            (NaiveDate::from_ymd(2025, 10,29), vec![WorkoutPlanned::new(WorkoutTemplate::default(), NaiveDate::from_ymd(2025, 10, 29))])]),
        }
    }

    pub fn add_workout(&mut self, date: NaiveDate, workout: WorkoutPlanned) -> Result<(), String> {
        self.workouts.entry(date).or_default().push(workout);
        Ok(())
    }

    pub fn rest(&mut self, date: NaiveDate) {
        if let Some(workouts) = self.workouts.get_mut(&date) {
            workouts.clear();
        }
        self.workouts.entry(date).or_default().push(WorkoutPlanned::rest(date));
    }

    pub fn remove_workout(&mut self, date: NaiveDate, index: usize) -> Result<(), String> {
        if let workouts = self.workouts.get_mut(&date).unwrap() {
            workouts.remove(index);
            Ok(())
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub enum Muscle {
    Necks,
    Traps,
    UpperChest,
    LowerChest,
    FrontDelt,
    SideDelt,
    RearDelt,
    Biceps,
    Triceps,
    Forearms,
    Abs,
    Hips,
    Adductors,
    Quads,
    ExtHips,
    Calfs,
    Infraspinatus,
    Lats,
    LowerBack,
    Glutes,
    Hamstrings,
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
    pub water_history: HashMap<NaiveDate, Vec<Drink>>,

    pub water_goal: u32,
    pub water_registered: u32,
    pub hydrolized: u32,
}

impl WaterData {
    pub fn default() -> Self {
        Self {
            // meal_history: HashMap::from([(Local::now().date_naive(), vec![Eat::new(chrono::Local::now().time(), Meal::new(100, 100, 100, 100)), Eat::new(chrono::Local::now().time(), Meal::new(10, 10, 10, 10))])]),
            water_history: HashMap::from([(Local::now().date_naive(), vec![Drink::new(chrono::Local::now().time(), Beverage::new(BeverageCategory::Water, 400, Some(100))), Drink::new(chrono::Local::now().time(), Beverage::new(BeverageCategory::Coffee, 250, Some(100)))])]),

            water_goal: 1000,
            water_registered: 0,
            hydrolized: 0,
        }
    }

    pub fn add_drink(&mut self, selected_date: NaiveDate, water_amount: &str, hydration_percent: &str) {
        self.water_history.entry(selected_date).or_default().insert(0, Drink::new(chrono::Local::now().time(), Beverage::new(
            BeverageCategory::Other(String::from("Drink")),
            water_amount.trim().parse::<u32>().unwrap_or(0),
            Some(hydration_percent.trim().parse::<u32>().unwrap_or(0)).clamp(Some(0), Some(100)),
        )));
    }
}  

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Drink {
    pub date: NaiveTime,
    pub beverage: Beverage,
}

impl Drink {
    pub fn new(date: NaiveTime, beverage: Beverage) -> Self {
        Self {
            date,
            beverage,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BeverageCategory {
    Water,
    Coffee,
    Tea,
    Juice,
    Soda,
    EnergyDrink,
    Alcohol,
    Other(String),
}

impl Default for BeverageCategory {
    fn default() -> Self {
        BeverageCategory::Water
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Beverage {
    pub category: BeverageCategory,
    pub name: String,
    pub amount: u32,
    pub hydration_amount: u32,
}

impl Beverage {
    pub fn new(category: BeverageCategory, amount: u32, hydration_percent: Option<u32>) -> Self {
        let (name, hydration_percent) = match &category {
            BeverageCategory::Water => ("Water".to_string(), 100),
            BeverageCategory::Coffee => ("Coffee".to_string(), 80),
            BeverageCategory::Tea => ("Tea".to_string(), 90),
            BeverageCategory::Juice => ("Juice".to_string(), 90),
            BeverageCategory::Soda => ("Soda".to_string(), 60),
            BeverageCategory::EnergyDrink => ("Energy Drink".to_string(), 50),
            BeverageCategory::Alcohol => ("Alcohol".to_string(), 20),
            BeverageCategory::Other(name) => (name.clone(), hydration_percent.unwrap()),
        };

        let hydration_amount = (amount / 100) * hydration_percent;

        Self {
            category,
            name,
            amount,
            hydration_amount,
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
    pub macro_add_clicked: bool,
    pub calory_add_value: String,
    pub protein_add_value: String,
    pub carb_add_value: String,
    pub fat_add_value: String,
    pub alert_modal: bool,
    pub delete_was_positive: bool,
    pub water_add_clicked: bool,
    pub water_add_value: String,
    pub hydration_percent: String,
    pub templates_window: bool,
    pub show_templates: bool,
    pub create_template: bool,
    pub current_template: String,
    pub editable: bool,
    pub exercises_window: bool,
    pub new_template_name: String,
    pub new_template_exercises: Vec<Exercises>,
    // pub scroll_offset: f32,
    // pub velocity: f32,
    // pub dragging: bool,
    // pub calories:u32,
    // pub proteins: u32,
    // pub carbs: u32,
    // pub fats: u32,
}

impl States {
    pub fn default() -> Self {
        Self {
            calory_add_modal: false,
            selected_tab: 2,
            skip_days: 0,
            // selected_day: OffsetDateTime::now_local().unwrap(),
            selected_day: Local::now().date_naive(),
            // selected_day_workouts_ui: Local::now().date_naive(),
            // selected_day_calory_ui: Local::now().date_naive(),
            strip_size: 150.0,
            calendar_mode_calory_ui: true,
            macro_add_clicked: false,
            calory_add_value: String::from("0"), 
            protein_add_value: String::from("0"),
            carb_add_value: String::from("0"),
            fat_add_value: String::from("0"),
            alert_modal: false,
            delete_was_positive: false,
            water_add_clicked: false,
            water_add_value: String::from("0"),
            hydration_percent: String::from("0"),
            templates_window: false,
            show_templates: true,
            create_template: false,
            current_template: String::new(),
            editable: false,
            exercises_window: false,
            new_template_name: String::from("workout name"),
            new_template_exercises: Vec::new(),

            // scroll_offset: 0.0,
            // velocity: 0.0,
            // dragging: false,
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

    pub fn reset_water(&mut self) {
        self.water_add_value = String::from("0");
        self.hydration_percent = String::from("0");
    }

    pub fn reset_template_window(&mut self) {
        self.templates_window = false;
        self.show_templates = true;
        self.create_template = false;
        self.current_template = String::new();
        self.editable = false;
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
        }
    }
}

impl Summary for WaterData {
    fn summarize(&mut self, selected_day: Option<NaiveDate>) {
        if let Some(drinks) = self.water_history.get(&selected_day.unwrap()) {
            let mut drinked_amount: u32 = 0;
            let mut hydrolized_amount: u32 = 0;
            for drink in drinks{
                drinked_amount += drink.beverage.amount;
                hydrolized_amount += drink.beverage.hydration_amount;
            }

            self.water_registered = drinked_amount;
            self.hydrolized = hydrolized_amount;
        } else {
            self.water_registered = 0;
            self.hydrolized = 0;
        }
    }
}
