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

struct CardInfo<'a> {
    title: &'a str,
    image_url: &'a str,
    difficulty: &'a str,
    route: Route,
}

const CARD_INFO: [CardInfo; 3] = [
    CardInfo {
        title: "Stats Preview Card",
        image_url: "images/previews/stats_preview_card.jpg",
        difficulty: "Newbie",
        route: Route::StatsPreviewCard,
    },
    CardInfo {
        title: "FAQ Accordion Card",
        image_url: "images/previews/faq_accordion_card.jpg",
        difficulty: "Newbie",
        route: Route::FaqAccorcionCard,
    },
    CardInfo {
        title: "Article Preview Component",
        image_url: "images/previews/article_preview_component.jpg",
        difficulty: "Newbie",
        route: Route::ArticlePreciewComponent,
    },
];

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
                {for CARD_INFO.map(|item|{
                    html!{<Card title={item.title}
                      difficulty={item.difficulty}
                      image_url={item.image_url}
                      route={item.route.clone()} />}
                })}
                </div>
            </main>
        }
    }
}
