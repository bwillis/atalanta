use models::user::User;
use models::workout::Workout;
use models::workout_plan::WorkoutPlan;
use models::database::Database;
use juniper::FieldResult;

pub struct QueryRoot;
graphql_object!(User: Database as "User" |&self| {

    // Expose a simple field as a GraphQL string.
    field id() -> &String {
        &self.id
    }

    field name() -> &String {
        &self.name
    }

    // FieldResult<T> is an alias for Result<T, String> - simply return
    // a string from this method and it will be correctly inserted into
    // the execution response.
    field secret() -> FieldResult<&String> {
        Err("Can't touch this".to_owned())
    }

    // Field accessors can optionally take an "executor" as their first
    // argument. This object can help guide query execution and provides
    // access to the context instance.
    //
    // In this example, the context is used to convert the friend_ids array
    // into actual User objects.
    field friends(&mut executor) -> Vec<&User> {
        self.friend_ids.iter()
            .filter_map(|id| executor.context().users.get(id))
            .collect()
    }

    field workout_plan_ids(&mut executor) -> Vec<&WorkoutPlan> {
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



// The context object is passed down to all referenced types - all your exposed
// types need to have the same context type.
graphql_object!(QueryRoot: Database as "Query" |&self| {

    // Arguments work just like they do on functions.
    field user(&mut executor, id: String) -> Option<&User> {
        executor.context().users.get(&id)
    }
});
