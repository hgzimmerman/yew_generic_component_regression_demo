use yew::{html, Component, ComponentLink, Html, ShouldRender};
use std::marker::PhantomData;



struct Model;

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <GenericComponent<String>> </GenericComponent>
        }
    }
}

struct GenericComponent<T: 'static> {
    link: ComponentLink<Self>,
    phantom: PhantomData<T>
}

enum Msg {
    DoIt,
}

impl <T: 'static> Component for GenericComponent<T> {
    // Some details omitted. Explore the examples to see more.

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        GenericComponent { link, phantom: PhantomData }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DoIt => {
                true
            }
        }
    }

    fn view(&self) -> Html {
        let onclick = self.link.callback(|_| Msg::DoIt);
        html! {
            <button onclick=onclick>{ "Click me!" }</button>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}