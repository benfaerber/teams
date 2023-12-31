mod api;
mod schema;
pub use schema::{Message, CardAttachment, CardContent, CardBlock, TextBlock, Color};
pub use api::Teams;



#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn it_works() {
        let key = env::var("TEAMS_WEBHOOK_API_KEY")
            .expect(&format!("Missing Enviroment Variable"));
        let teams = Teams::new(&key);
        

    }
}
