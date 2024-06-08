#[macro_export]
macro_rules! get_function_string {
    ($func:ident) => {{
        stringify!($func)
    }};
}

#[macro_use]
mod models;
mod ai_functions;
mod apis;
mod helpers;

use helpers::command_line::get_user_response;
use models::agents_manager::managing_agent::ManagingAgent;

#[tokio::main]
async fn main() {
    // Obtain user goal
    let usr_req: String = get_user_response("What are we building today?");

    // Create Gippity Managing Agent
    let managing_agent: ManagingAgent = ManagingAgent::new(usr_req)
        .await
        .expect("Error creating agent");

    dbg!(managing_agent);
}
