use yew::{function_component, html, use_state};
mod faq_card;
use faq_card::get_items;
use faq_card::FAQCard;

use std::rc::Rc;
#[function_component(FaqAccorcionCard)]
pub fn faqAccorcionCard() -> Html {
    let selected_item = use_state(|| -1);
    let selected_item_ptr = Rc::new(selected_item);
    let items = get_items();
    html! {
        <main class="faq_accordion_card">
            <img src="images/faq_accordion_card/box.svg" alt="box" />
            <div class="container">
                <div class="woman" />
                <div class="accordion" >
                    <h1>{"FAQ"}</h1>
                    {for items.into_iter().map(|item|{
                        html!{<FAQCard selected_item_ptr={selected_item_ptr.clone()} {item}/> }
                        })
                    }
                </div>
            </div>
        </main>
    }
}
