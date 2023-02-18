use yew::prelude::*;

pub enum Msg {}

pub struct EditRecord {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
}

impl Component for EditRecord {
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
                <div class="window-index" id="create-index"> 

                    <div class="top-row-index-window-insert">
                        <h1>{"EDIT RECORD #"}{""}</h1>
                        
                        <button 
                            type="button" 
                            class="window-index-closebutton">
                                <img src="images/close.png" alt="close window" style="width: 32px"/> 
                        </button>
                    </div> 

                    <h6>{"Edit this record's JSON values"}
                     </h6>

                    <div class="window-submit-form">
                        <form class="record-text-input" id="submit-editrecord">
                            <textarea 
                                type="text" 
                                class="insert-record" 
                                >
                            {""}  
                            </textarea>
                        </form>   
                    </div> 

                    <button 
                        type="submit"
                        form="submit-editrecord"
                        class="window-confirm-button"
                    >
                        { "EDIT RECORD" }
                    </button>
                    
                </div>

            </div>
        }
    }
}