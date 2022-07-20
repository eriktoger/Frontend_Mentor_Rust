use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/statspreviewcard")]
    StatsPreviewCard,
    #[at("/faqaccordioncard")]
    FaqAccorcionCard,
}
