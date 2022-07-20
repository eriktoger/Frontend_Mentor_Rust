use yew::{function_component, html, Callback, Properties};

pub struct FAQ {
    id: i32,
    question: String,
    answer: String,
}

impl PartialEq for FAQ {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Properties, PartialEq)]
pub struct FAQProps {
    pub item: FAQ,
    pub selected_item_ptr: std::rc::Rc<yew::UseStateHandle<i32>>,
}

#[function_component(FAQCard)]
pub fn faqCard(props: &FAQProps) -> Html {
    let props_se_ptr = props.selected_item_ptr.clone();
    let props_id = props.item.id.clone();
    let onclick = Callback::from(move |_| {
        props_se_ptr.set(props_id);
    });
    let selected = if props.item.id == **props.selected_item_ptr {
        "selected"
    } else {
        "not-selected"
    };
    html! {
        <div class={"item"}>
            <div onclick={onclick} class="title">
                <h2 class={selected}> {&props.item.question}</h2>
                <img class={selected} src="images/faq_accordion_card/arrow.svg" alt="arrow"/>
            </div>
            <h3 class={selected}> {&props.item.answer} </h3>
        </div>

    }
}

pub fn get_items() -> [FAQ; 5] {
    [
        FAQ{
            id:0,
            question: String::from("How many team members can I invite?"),
            answer:String::from("You can invite up to 2 additional users on the Free plan. There is no limit on team members for the Premium plan."),
        },
        FAQ{
            id:1,
            question: String::from("What is the maximum file upload size?"),
            answer:String::from("No more than 2GB. All files in your account must fit your allotted storage space."),
        },
        FAQ{
            id:2,
            question:String::from("How do I reset my password?"),
            answer:String::from(" Click “Forgot password” from the login page or “Change password” from your profile page. A reset link will be emailed to you.")
        },
        FAQ{
            id:3,
            question:String::from("Can I cancel my subscription?"),
            answer:String::from("Yes! Send us a message and we’ll process your request no questions asked.")
        },
        FAQ{
            id:4,
            question:String::from("Do you provide additional support?"),
            answer:String::from("Chat and email support is available 24/7. Phone lines are open during normal business hours.")
        }
    ]
}
