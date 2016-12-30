pub use models::user::User;
pub use models::workout::Workout;
pub use models::workout_plan::WorkoutPlan;
use std::collections::HashMap;
use iron::prelude::*;

pub struct Database {
    pub users: HashMap<String, User>,
    pub workout_plans: HashMap<String, WorkoutPlan>,
    pub workouts: HashMap<String, Workout>
}

// This function is executed for every request. Here, we would realistically
// provide a database connection or similar. For this example, we'll be
// creating the database from scratch.
pub fn context_factory(_: &mut Request) -> self::Database {
    self::Database {
        users: vec![
            (
                "1000".to_owned(),
                User {
                    id: "1000".to_owned(),
                    name: "Robin".to_owned(),
                    friend_ids: vec!["1001".to_owned()],
                    workout_plan_ids: vec!["1000".to_owned()],
                }
            ),
            (
                "1001".to_owned(),
                User {
                    id: "1001".to_owned(),
                    name: "Max".to_owned(),
                    friend_ids: vec!["1000".to_owned()],
                    workout_plan_ids: vec!["1001".to_owned()],
                }
            ),
        ].into_iter().collect(),
        workout_plans: vec![
            (
                "1000".to_owned(),
                WorkoutPlan {
                    id: "1000".to_owned(),
                    name: "Road to 100k".to_owned(),
                    start_date: "".to_owned(),
                    end_date: "".to_owned(),
                    workout_ids:  vec!["1001".to_owned()],
                }
            ),
            (
                "1001".to_owned(),
                WorkoutPlan {
                    id: "1001".to_owned(),
                    name: "2016 Season".to_owned(),
                    start_date: "".to_owned(),
                    end_date: "".to_owned(),
                    workout_ids:  vec!["1000".to_owned()],
                }
            ),
        ].into_iter().collect(),
        workouts: vec![
            (
                "1000".to_owned(),
                Workout {
                    id: "1000".to_owned(),
                    name: "Long Run".to_owned(),
                    description: "10-15 miler".to_owned(),
                    date: "".to_owned(),
                }
            ),
            (
                "1001".to_owned(),
                Workout {
                    id: "1001".to_owned(),
                    name: "Fartlek".to_owned(),
                    description: "3x2 mile repeats".to_owned(),
                    date: "".to_owned(),
                }
            ),
        ].into_iter().collect(),
    }
}
