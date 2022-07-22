use super::constants::{HIDE, ICONS, PREFIX, SHOW};
use super::context::{ModalAction, ModalState};
use yew::{function_component, html, use_context, Callback, MouseEvent, UseReducerHandle};

#[function_component(LeftSide)]
fn leftSide() -> Html {
    html! {<div class="leftSide">
        <img class="avatarImg" src="images/article_preview_component/avatar-michelle.jpg" alt="avatar"/>
        <div class="userText" >
            <h3>{"Michelle Appleton"}</h3>
            <h4>{"28 Jun 2020"} </h4>
        </div>
    </div>}
}

#[function_component(RightSide)]
fn rightSide() -> Html {
    let ctx = use_context::<UseReducerHandle<ModalState>>().expect("no ctx found");
    let ctx_clone = ctx.clone();
    let (share_modal_class, share_button_class, share_button_src) =
        if ctx.show { HIDE } else { SHOW };
    let handle_share = Callback::from(move |event: MouseEvent| {
        event.stop_propagation();
    });
    let handle_show_modal = Callback::from(move |event: MouseEvent| {
        event.stop_propagation();
        ctx_clone.dispatch(ModalAction::Toggle);
    });
    html! {<div class="shareContainer ">
        <div class={share_modal_class}>
            <span>{"Share"}</span>
            {for ICONS.map(|icon|
                html!{<img
                    src={format!("{}{}{}",PREFIX,icon,".svg")}
                    alt={icon}
                    onclick={handle_share.clone()}/>
                })}
        </div>
        <img onclick={handle_show_modal} class={share_button_class} src={share_button_src} alt="share"/>
    </div>}
}

#[function_component(UserContainer)]
pub fn userContainer() -> Html {
    html! {
    <div class="userContainer">
        <LeftSide/>
        <RightSide />
    </div>}
}
