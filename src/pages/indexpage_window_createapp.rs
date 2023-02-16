use yew::prelude::*;

pub enum Msg {}

pub struct AppCreate {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
}

impl Component for AppCreate {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {

        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="window-overlay">
                <div class="window-index" id="create-app"> 

                    <div class="top-row-index-window">
                        <h1>{"CREATE NEW APPLICATION"}</h1>
                        
                        <button 
                            type="button" 
                            class="window-index-closebutton">
                                <img src="images/close.png" alt="close window" style="width: 32px"/> 
                        </button>
                    </div> 

                    <h5>{"INSERT APPLICATION NAME"}</h5>

                    <form class="createindex-text-input" id="submit-createapp">
                        <input 
                            type="text" 
                            class="form-control" 
                            id="create-app-text" 
                            aria-describedby="emailHelp"
                            placeholder="Application name here..."/>
            
                    // <div class="window-confirm-button">
                    // </div>
                    </form>   

                    <button 
                        type="submit"
                        class="window-confirm-button"
                        form="submit-createapp"
                    >
                    { "CREATE APPLICATION" }
                    </button>
                </div>

            </div>
        }
    }
}