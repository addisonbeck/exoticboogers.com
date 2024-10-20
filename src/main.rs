use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
      <>
        <h1>
          {"exoticboogers 👃🦠"}
        </h1>
        <img src="img/1.png" width="500" height="auto"/>
      </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
