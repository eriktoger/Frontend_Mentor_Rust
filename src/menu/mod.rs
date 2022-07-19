use super::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, Clone, PartialEq)]
struct CardProps {
    title: String,
    difficulty: String,
    image_url: String,
    route: Route,
}

struct Card;

impl Component for Card {
    type Message = ();
    type Properties = CardProps;
    fn create(ctx: &Context<Self>) -> Self {
        ctx.props();
        Self
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let history = ctx.link().history().unwrap();
        let route = ctx.props().route.clone();
        let onclick = Callback::once(move |_| history.push(route));
        html! {
            <div onclick={onclick} class="card">
                <div class="text">
                    <span class="title">{&ctx.props().title}</span>
                    <span>{&ctx.props().difficulty}</span>
                </div>
                <img src={ctx.props().image_url.clone()} alt={ctx.props().title.clone()}/>
            </div>
        }
    }
}

pub struct Menu;

impl Component for Menu {
    type Message = ();
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <main class="menu">
                <div class="container">
                <Card title={"Stats Preview Card"}
                      difficulty="Newbie"
                      image_url="images/previews/stats_preview_card.jpg"
                      route={Route::StatsPreviewCard} />
                </div>
            </main>
        }
    }
}
