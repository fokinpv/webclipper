use actix_web::{HttpRequest, HttpResponse, State};
use super::*;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NewItem {
    content: String
}

pub struct Clips;
impl Clips {
    pub fn get((req, state): (HttpRequest<AppState>, State<AppState>)) -> HttpResponse {
        HttpResponse::Ok().json(state.db.lock().unwrap().all())
    }
    pub fn get_one() {}
    pub fn post((new_item, state): (Json<NewItem>, State<AppState>)) -> HttpResponse {
        let item = Item {
            id: state.db.lock().unwrap().next_pk(),
            content: new_item.content.clone(),
        };

        state.db.lock().unwrap().insert(item.clone());
        HttpResponse::Ok().json(item)
    }
}
