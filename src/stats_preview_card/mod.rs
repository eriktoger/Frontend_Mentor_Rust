use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
struct Props {
    title: String,
    name: String,
}

struct OneStat;

impl Component for OneStat {
    type Message = ();
    type Properties = Props;
    fn create(ctx: &Context<Self>) -> Self {
        ctx.props();
        Self
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="oneStat">
                <span>{&ctx.props().title}</span>
                <span>{&ctx.props().name}</span>
            </div>
        }
    }
}

pub struct StatsPreviewCard;

impl Component for StatsPreviewCard {
    type Message = ();
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="stats_preview_card">
                <div class="container">
                    <div class="text">
                        <h1>{"Get "} <span class="addColor"> {"insights"}</span> {" that help your business grow."}</h1>
                        <h2>{"Discover the benefits of data analytics
                        and make better decisions regarding revenue, 
                        customer experience, and overall efficiency."} </h2>
                        <div class="stats">
                            <OneStat name="companies" title="10k+"/>
                            <OneStat name="templates" title="314"/>
                            <OneStat name="queries" title="12M+"/>
                        </div>
                    </div>
                    <div class="image" />
                </div>
            </div>
        }
    }
}
