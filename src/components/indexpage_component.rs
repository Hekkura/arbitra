use yew::{prelude::*, services::ConsoleService};

use crate::components::{
    indexpage_window_createapp::AppCreate,
    indexpage_window_createindex::IndexCreate,
    indexpage_window_deleterecord::DeleteRecord,
    indexpage_window_editrecord::EditRecord,
    indexpage_window_insertrecord::InsertRecord,
};

pub enum Msg {
    //EVENT TOGGLE (MERGE CLOSE DAN OPEN)
    ToggleCreateApp,
    ToggleCreateIndex,
    ToggleInsertRecord,
    ToggleEditRecord,
    ToggleDeleteRecord,

    Ignore,
}


#[derive(Properties, Clone, Debug, PartialEq)]
pub struct IndexPageCompProps {
    // #[prop_or(String::from("this is value"))]
    #[prop_or(false)]
    pub display_create_app: bool,
    #[prop_or(false)]
    pub display_create_index: bool,
    #[prop_or(false)]
    pub display_insert_record: bool,
    #[prop_or(false)]
    pub display_edit_record: bool,
    #[prop_or(false)]
    pub display_delete_record: bool,

    pub on_toggle_createapp:Callback<Msg>,
    pub on_toggle_createindex:Callback<Msg>,
    pub on_toggle_insertrecord:Callback<Msg>,
    pub on_toggle_editrecord:Callback<Msg>,
    pub on_toggle_deleterecord:Callback<Msg>,
    
}


pub struct IndexPageComp {
    link: ComponentLink<Self>,
    //DISPLAY WINDOWS / MODAL (STATE)
    // display_create_app: bool,
    // display_create_index: bool,
    // display_insert_record: bool,
    // display_edit_record: bool,
    // display_delete_record: bool,

    props: IndexPageCompProps,
    callback_toggle_createapp: Callback<Msg>,
    callback_toggle_createindex: Callback<Msg>,
    callback_toggle_insertrecord: Callback<Msg>,
    callback_toggle_editrecord: Callback<Msg>,
    callback_toggle_deleterecord: Callback<Msg>,
}

impl Component for IndexPageComp {
    type Message = Msg;
    type Properties = IndexPageCompProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            callback_toggle_createapp: props.on_toggle_createapp.clone(),
            callback_toggle_createindex: props.on_toggle_createindex.clone(),
            callback_toggle_insertrecord: props.on_toggle_insertrecord.clone(),
            callback_toggle_editrecord: props.on_toggle_editrecord.clone(),
            callback_toggle_deleterecord: props.on_toggle_deleterecord.clone(),
            props,
            // DISPLAY WINDOWS / MODAL (STATE)

        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            //EVENT BUAT OPEN MODAL
            Msg::ToggleCreateIndex => {
                self.callback_toggle_createindex.emit(Msg::ToggleCreateIndex);
                ConsoleService::info(&format!("DEBUG : display_create_index:{:?}", self.props.display_create_index));
                true
            }
            Msg::ToggleCreateApp => {
                self.callback_toggle_createapp.emit(Msg::ToggleCreateApp);
                ConsoleService::info(&format!("DEBUG : display_create_app:{:?}", self.props.display_create_app));
                true
            }
            Msg::ToggleInsertRecord => {
                self.callback_toggle_insertrecord.emit(Msg::ToggleInsertRecord);
                ConsoleService::info(&format!("DEBUG : display_insert_record:{:?}", self.props.display_insert_record));
                true
            }
            Msg::ToggleEditRecord => {
                self.callback_toggle_editrecord.emit(Msg::ToggleEditRecord);
                ConsoleService::info(&format!("DEBUG : display_edit_record:{:?}", self.props.display_edit_record));
                true
            }
            Msg::ToggleDeleteRecord => {
                self.callback_toggle_deleterecord.emit(Msg::ToggleDeleteRecord);
                ConsoleService::info(&format!("DEBUG : display_delete_record:{:?}", self.props.display_delete_record));
                true
            }
            Msg::Ignore => {
                ConsoleService::info(&format!("DEBUG : Event Ignore", ));
                true
            }
            
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        //CONDITIONAL DEFAULT CASE (NO MODAL)
            html! {
                <div> 
                        <div>
                            <div class="leftbox index-sidebar-small">
                                <img class="index-logo" src="images/Arbitra_LogoOnly.png"/> 
                            </div>

                            <div class="rightSideBar">
                                <p style="color: #bd3143; font-size: 2rem">{"S E A R C H"}</p>
                                <p style="margin-top: -8px">{ "Application" }</p>

                                <div class="dropdown">
                                    <button class="mainmenubtn"><img class="applicationIcon" src="images/APP.png"/>{ "Scara \u{00a0} \u{23F7}"}</button>
                                    <div class="dropdown-child">
                                        <a 
                                            href="#" 
                                            onclick=self.link.callback(|_| Msg::ToggleCreateApp)>
                                            { "Create New Application" }
                                        </a>
                                        // <a href="#">{ "Link 2" }</a>
                                        // <a href="#">{ "Link 3" }</a>
                                    </div>
                                </div>
                                
                                <br/><br/>

                                <p class="index-directry">{ "\u{007C}\u{00a0} Index" }</p>
                                <p class="index-directry">{ "\u{007C}\u{00a0} Dictionary" }</p>
                                <p class="index-directry">{ "\u{007C}\u{00a0} Lorem Ipsum" }</p>
                                <p class="index-directry">{ "\u{007C}\u{00a0} Lorem Ipsum" }</p>
                            </div>
                        </div>

                        <div>
                            <div class="top-index-dashboard">

                                <div class="dropdownIndex">
                                    <button class="mainmenubtnIndex">{ "INDEX NAME \u{00a0} \u{00a0} \u{00a0} \u{00a0} \u{00a0} \u{00a0} \u{00a0} \u{00a0} \u{00a0} \u{23F7}"}</button>
                                    <div class="dropdown-childIndex">
                                        <a 
                                            href="#"
                                            onclick=self.link.callback(|_| Msg::ToggleCreateIndex)>
                                            { "Create New Index" }
                                        </a>
                                        // <a href="#">{ "Link 2" }</a>
                                        // <a href="#">{ "Link 3" }</a>
                                    </div>
                                </div>

                                <div class="recordData">
                                    <p class="recordNum">{ "No. of Records \u{00a0} \u{00a0} \u{00a0} \u{00a0} 1.000.000.000" }</p>
                                    <p style="float: left;">{ "\u{00a0} \u{00a0} \u{00a0}" }</p>
                                    <p class="recordSize">{ "Average Record Size\u{00a0} \u{00a0} \u{00a0} \u{00a0} 1.000.000.000B" }</p>
                                </div>

                                <br/><br/><br/>

                                <div class="dropdownRecord">
                                    <button class="mainmenubtnRecord">{ "New Record \u{00a0} \u{00a0} \u{00a0} \u{00a0} \u{23F7}"}</button>
                                    <div class="dropdown-childRecord">
                                        <a href="#" onclick=self.link.callback(|_| Msg::ToggleInsertRecord)>{ "Insert New Record" }</a>
                                        <a href="#" onclick=self.link.callback(|_| Msg::ToggleEditRecord)>{ "Edit Record" }</a>
                                        <a href="#" onclick=self.link.callback(|_| Msg::ToggleDeleteRecord)>{ "Delete Record" }</a>
                                    </div>
                                </div>

                                <div class="dropdownRecord">
                                    <button class="mainmenubtnRecord">{ "Add Records \u{00a0} \u{00a0} \u{00a0} \u{23F7}"}</button>
                                    <div class="dropdown-childRecord">
                                        <a href="#">{ "Upload File" }</a>
                                        <a href="#">{ "Use the API" }</a>
                                        <a href="#">{ "Add Manually" }</a>
                                    </div>
                                </div>

                                <div class="dropdownRecord">
                                    <button class="mainmenubtnRecord">{ "Manage Index \u{00a0} \u{00a0} \u{00a0} \u{23F7}"}</button>
                                    <div class="dropdown-childRecord">
                                        <a href="#">{ "Rename" }</a>
                                        <a href="#">{ "Duplicate" }</a>
                                        <a href="#">{ "Copy Settings" }</a>
                                        <a href="#">{ "Clear" }</a>
                                        <a href="#">{ "Delete" }</a>
                                    </div>
                                </div>

                                <img class="copyIcon" src="images/Copy Icon.png"/>
                                <img class="copyIcon" src="images/Refresh.png"/>

                            </div>
                        </div>
                                        
                    </div>
               
        }
            //BODY END
        }
    }
