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
}

pub struct IndexPage {
    link: ComponentLink<Self>,
    //DISPLAY WINDOWS / MODAL (STATE)
    display_create_app: bool,
    display_create_index: bool,
    display_insert_record: bool,
    display_edit_record: bool,
    display_delete_record: bool,
}

impl Component for IndexPage {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            //DISPLAY WINDOWS / MODAL (STATE)
            display_create_index: false,
            display_create_app: false,
            display_insert_record: false,
            display_edit_record: false,
            display_delete_record: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            //EVENT BUAT OPEN MODAL
            Msg::ToggleCreateIndex => {
                self.display_create_index = !self.display_create_index;
                ConsoleService::info(&format!("DEBUG : display_create_index:{:?}", self.display_create_index));
                true
            }
            Msg::ToggleCreateApp => {
                self.display_create_app = !self.display_create_app;
                ConsoleService::info(&format!("DEBUG : display_create_app:{:?}", self.display_create_app));
                true
            }
            Msg::ToggleInsertRecord => {
                self.display_insert_record = !self.display_insert_record;
                ConsoleService::info(&format!("DEBUG : display_insert_record:{:?}", self.display_insert_record));
                true
            }
            Msg::ToggleEditRecord => {
                self.display_edit_record = !self.display_edit_record;
                ConsoleService::info(&format!("DEBUG : display_edit_record:{:?}", self.display_edit_record));
                true
            }
            Msg::ToggleDeleteRecord => {
                self.display_delete_record = !self.display_delete_record;
                ConsoleService::info(&format!("DEBUG : display_delete_record:{:?}", self.display_delete_record));
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

        let ToggleCreateApp = self.display_create_app;
        let ToggleCreateIndex = self.display_create_index;
        let ToggleInsertRecord = self.display_insert_record;
        let ToggleEditRecord = self.display_edit_record;
        let ToggleDeleteRecord = self.display_delete_record;
        
        //CONDITIONAL KALAU BUKA CREATE APP
        if ToggleCreateApp { 
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
                                    <a href="#">{ "Link 1" }</a>
                                    <a href="#">{ "Link 2" }</a>
                                    <a href="#">{ "Link 3" }</a>
                                </div>
                            </div>

                            <div class="dropdownRecord">
                                <button class="mainmenubtnRecord">{ "Manage Index \u{00a0} \u{00a0} \u{00a0} \u{23F7}"}</button>
                                <div class="dropdown-childRecord">
                                    <a href="#">{ "Link 1" }</a>
                                    <a href="#">{ "Link 2" }</a>
                                    <a href="#">{ "Link 3" }</a>
                                </div>
                            </div>

                            <img class="copyIcon" src="images/Copy Icon.png"/>
                            <img class="copyIcon" src="images/Refresh.png"/>

                        </div>
                    </div>

                    //DISPLAY WINDOW DISINI         
                    <AppCreate 
                        display_create_app=self.display_create_app.clone()
                        on_toggle = self.link.callback(|_| Msg::ToggleCreateApp) />

                </div>
                
            }
        //CONDITIONAL BUKA MODAL CREATE INDEX
        } else if ToggleCreateIndex {
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
                                    <a href="#">{ "Link 1" }</a>
                                    <a href="#">{ "Link 2" }</a>
                                    <a href="#">{ "Link 3" }</a>
                                </div>
                            </div>

                            <div class="dropdownRecord">
                                <button class="mainmenubtnRecord">{ "Manage Index \u{00a0} \u{00a0} \u{00a0} \u{23F7}"}</button>
                                <div class="dropdown-childRecord">
                                    <a href="#">{ "Link 1" }</a>
                                    <a href="#">{ "Link 2" }</a>
                                    <a href="#">{ "Link 3" }</a>
                                </div>
                            </div>

                            <img class="copyIcon" src="images/Copy Icon.png"/>
                            <img class="copyIcon" src="images/Refresh.png"/>

                        </div>
                    </div>

                    //DISPLAY WINDOW DISINI      
                    <IndexCreate 
                        display_create_index=self.display_create_index.clone()
                        on_toggle = self.link.callback(|_| Msg::ToggleCreateIndex) />

                </div>
                
            }
        //CONDITIONAL BUKA MODAL INSERT RECORD
        } else if ToggleInsertRecord {
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
                                    <a href="#">{ "Link 1" }</a>
                                    <a href="#">{ "Link 2" }</a>
                                    <a href="#">{ "Link 3" }</a>
                                </div>
                            </div>

                            <div class="dropdownRecord">
                                <button class="mainmenubtnRecord">{ "Manage Index \u{00a0} \u{00a0} \u{00a0} \u{23F7}"}</button>
                                <div class="dropdown-childRecord">
                                    <a href="#">{ "Link 1" }</a>
                                    <a href="#">{ "Link 2" }</a>
                                    <a href="#">{ "Link 3" }</a>
                                </div>
                            </div>

                            <img class="copyIcon" src="images/Copy Icon.png"/>
                            <img class="copyIcon" src="images/Refresh.png"/>

                        </div>
                    </div>

                    //DISPLAY WINDOW DISINI         
                    <InsertRecord
                        display_insert_record=self.display_insert_record.clone()
                        on_toggle = self.link.callback(|_| Msg::ToggleInsertRecord) />

                </div>
            }
        //CONDITIONAL BUKA MODAL EDIT RECORD
        } else if ToggleEditRecord {
            html!{  
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
                                    <a href="#">{ "Link 1" }</a>
                                    <a href="#">{ "Link 2" }</a>
                                    <a href="#">{ "Link 3" }</a>
                                </div>
                            </div>

                            <div class="dropdownRecord">
                                <button class="mainmenubtnRecord">{ "Manage Index \u{00a0} \u{00a0} \u{00a0} \u{23F7}"}</button>
                                <div class="dropdown-childRecord">
                                    <a href="#">{ "Link 1" }</a>
                                    <a href="#">{ "Link 2" }</a>
                                    <a href="#">{ "Link 3" }</a>
                                </div>
                            </div>

                            <img class="copyIcon" src="images/Copy Icon.png"/>
                            <img class="copyIcon" src="images/Refresh.png"/>

                        </div>
                    </div>

                    //DISPLAY WINDOW DISINI         
                    <EditRecord
                        display_edit_record=self.display_edit_record.clone()
                        on_toggle = self.link.callback(|_| Msg::ToggleEditRecord) />

                </div>

            }
        //CONDITIONAL BUKA MODAL DELETE RECORD
        } else if ToggleDeleteRecord {
            html!{
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
                                    <a href="#">{ "Link 1" }</a>
                                    <a href="#">{ "Link 2" }</a>
                                    <a href="#">{ "Link 3" }</a>
                                </div>
                            </div>

                            <div class="dropdownRecord">
                                <button class="mainmenubtnRecord">{ "Manage Index \u{00a0} \u{00a0} \u{00a0} \u{23F7}"}</button>
                                <div class="dropdown-childRecord">
                                    <a href="#">{ "Link 1" }</a>
                                    <a href="#">{ "Link 2" }</a>
                                    <a href="#">{ "Link 3" }</a>
                                </div>
                            </div>

                            <img class="copyIcon" src="images/Copy Icon.png"/>
                            <img class="copyIcon" src="images/Refresh.png"/>

                        </div>
                    </div>

                    //DISPLAY WINDOW DISINI         
                    <DeleteRecord
                        display_delete_record=self.display_delete_record.clone()
                        on_toggle = self.link.callback(|_| Msg::ToggleDeleteRecord) />

                </div>
            }
        //CONDITIONAL DEFAULT CASE (NO MODAL)
        } else {
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
                                        <a href="#">{ "Link 1" }</a>
                                        <a href="#">{ "Link 2" }</a>
                                        <a href="#">{ "Link 3" }</a>
                                    </div>
                                </div>

                                <div class="dropdownRecord">
                                    <button class="mainmenubtnRecord">{ "Manage Index \u{00a0} \u{00a0} \u{00a0} \u{23F7}"}</button>
                                    <div class="dropdown-childRecord">
                                        <a href="#">{ "Link 1" }</a>
                                        <a href="#">{ "Link 2" }</a>
                                        <a href="#">{ "Link 3" }</a>
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
}
