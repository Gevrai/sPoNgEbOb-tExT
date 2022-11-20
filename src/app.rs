use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::use_clipboard;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <SpongebobText/>
    }
}

#[function_component(SpongebobText)]
pub fn spongebob_text() -> Html {
    let text = use_state(|| "".to_owned());
    let copied_state = use_state(|| false);

    let oninput = {
        let text = text.clone();
        let copied_state = copied_state.clone();
        Callback::from(move |e: InputEvent| {
            let elem = e.target().unwrap().unchecked_into::<HtmlInputElement>();
            text.set(spongebobify(elem.value()));
            copied_state.set(false);
        })
    };

    let copy_clipboard = {
        let text = text.clone();
        let copied_state = copied_state.clone();
        let clipboard = use_clipboard();
        Callback::from(move |_| {
            clipboard.write_text((*text).clone());
            copied_state.set(true);
        })
    };

    html! {
        <div id="spongebob">
            <textarea
                id="input"
                autofocus=true
                value={ (*text).clone() }
                placeholder={ spongebobify("enter text here".to_string()) }
                oninput={oninput}
            ></textarea>
            <div id="shirt"/>
            <button id="copy-button" onclick={copy_clipboard}>
                { if *copied_state {
                    spongebobify("Copied!".to_string())
                } else {
                    spongebobify("Copy to clipboard".to_string())
                } }
            </button>
        </div>
    }
}

fn spongebobify(s: String) -> String {
    s.chars()
        .enumerate()
        .map(|(i, c)| {
            if i % 2 == 0 {
                c.to_lowercase().to_string()
            } else {
                c.to_uppercase().to_string()
            }
        })
        .collect()
}
