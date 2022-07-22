mod constants;
mod context;
mod user_container;
use context::{Context, ModalAction, ModalState};
use stylist::Style;
use user_container::UserContainer;
use yew::{function_component, html, use_reducer, Callback, ContextProvider};

#[function_component(ArticlePreciewComponent)]
pub fn faqAccorcionCard() -> Html {
    let style_sheet = Style::new(include_str!("style.css")).expect("Css failed to load");
    let show_modal_reducer = use_reducer(ModalState::default);
    let show_modal_reducer_clone = show_modal_reducer.clone();
    let reset_onclick = Callback::from(move |_| {
        show_modal_reducer.dispatch(ModalAction::Hide);
    });
    html! {
        <ContextProvider<Context> context={show_modal_reducer_clone}>
            <main class={style_sheet}  >
                <div class="outer" onclick={reset_onclick}>
                    <div class="container">
                        <img class="mainImg" src="images/article_preview_component/drawers-desktop.jpg" alt="drawers"/>
                        <div class="text">
                            <h1>{"Shift the overall look and feel by adding these wonderful touches to furniture in your home"} </h1>
                            <h2>{"Ever been in a room and felt like something was missing? Perhaps it felt slightly bare and uninviting. I’ve got some simple tips to help you make any room feel complete."}</h2>
                            <UserContainer />
                        </div>
                    </div>
                </div>
            </main>
        </ContextProvider<Context> >
    }
}
