use yew::prelude::*;

pub enum Msg {}

pub struct InsertRecord {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
}

impl Component for InsertRecord {
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
                        <h1>{"INSERT NEW RECORD"}</h1>
                        
                        <button 
                            type="button" 
                            class="window-index-closebutton">
                                <img src="images/close.png" alt="close window" style="width: 32px"/> 
                        </button>
                    </div> 

                    <h6>{"Add record with the JSON Format, containing a single object or an array of
                     objects, for example : "}
                     </h6>

// FORM INPUT TEXT UNTUK EXAMPLE JSON 
                    <div>
                        <form class="record-text-input" id="example-no-submit">
                            <textarea 
                                readonly=true
                                type="text" 
                                class="insert-record" 
                                
                                >{"[
        {
            \"firstname\": \"Jimmie\",
            \"lastname\": \"Barninger\",
            \"zip_code\": 12345
        },
        {
            \"firstname\": \"John\",
            \"lastname\": \"Doe\",
            \"zip_code\": null
        }
    ]"}                     </textarea>
                        </form>   
                    </div> 
// FORM INPUT EXAMPLE END

                    <h6>{"Add your records here"}
                    </h6>


// FORM INPUT TEXT UNTUK EXAMPLE JSON 
                    <div class="window-submit-form">
                        <form class="record-text-input" id="submit-insertrecord">
                            <textarea 
                                type="text" 
                                class="insert-record" 
                                >
                            {""}  
                            </textarea>
                //BUTTON SUBMIT (HARUS DI FORM YANG SAMA)
                        </form>   
                    </div> 
// FORM INPUT EXAMPLE END
                    <button 
                        type="submit"
                        form="submit-insertrecord"
                        class="window-confirm-button"
                    >
                        { "INSERT NEW RECORD" }
                    </button>
                    
                </div>

            </div>
        }
    }
}