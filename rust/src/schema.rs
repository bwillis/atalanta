use models::user::User;
use models::workout::Workout;
use models::workout_plan::WorkoutPlan;
use models::database::Database;
use juniper::FieldResult;

pub struct QueryRoot;

graphql_object!(QueryRoot: Database as "Query" |&self| {
    field user(&mut executor, id: String) -> Option<&User> {
        executor.context().users.get(&id)
    }

    field workout_plan(&mut executor, id: String) -> Option<&WorkoutPlan> {
        executor.context().workout_plans.get(&id)
    }

    field workout(&mut executor, id: String) -> Option<&Workout> {
        executor.context().workouts.get(&id)
    }
});

graphql_object!(User: Database as "User" |&self| {
    field id() -> &String {
        &self.id
    }

    field name() -> &String {
        &self.name
    }

    field secret() -> FieldResult<&String> {
        Err("Can't touch this".to_owned())
    }

    field friends(&mut executor) -> Vec<&User> {
        self.friend_ids.iter()
            .filter_map(|id| executor.context().users.get(id))
            .collect()
    }

    field workout_plans(&mut executor) -> Vec<&WorkoutPlan> {
        self.workout_plan_ids.iter()
            .filter_map(|id| executor.context().workout_plans.get(id))
            .collect()
    }
});

graphql_object!(WorkoutPlan: Database as "WorkoutPlan" |&self| {
    field id() -> &String {
        &self.id
    }

    field name() -> &String {
        &self.name
    }

    field start_date() -> &String {
        &self.start_date
    }

    field end_date() -> &String {
        &self.end_date
    }

    field workouts(&mut executor) -> Vec<&Workout> {
        self.workout_ids.iter()
            .filter_map(|id| executor.context().workouts.get(id))
            .collect()
    }
});

graphql_object!(Workout: Database as "Workout" |&self| {
    field id() -> &String {
        &self.id
    }

    field name() -> &String {
        &self.name
    }

    field description() -> &String {
        &self.description
    }

    field date() -> &String {
        &self.date
    }
});
