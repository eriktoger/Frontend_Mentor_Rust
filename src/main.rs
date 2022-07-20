mod faq_accordion_card;
mod menu;
mod routes;
mod stats_preview_card;

use faq_accordion_card::FaqAccorcionCard;
use menu::Menu;
use routes::Route;
use stats_preview_card::StatsPreviewCard;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {<Menu/>},
        Route::StatsPreviewCard => html! { <StatsPreviewCard/>},
        Route::FaqAccorcionCard => html! {<FaqAccorcionCard/>},
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}
