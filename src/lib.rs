mod functions;
mod semdata;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use web_sys::HtmlInputElement;

#[derive(Debug)]
pub enum Msg {
    SelectCalcType(String),
    SelectSemester(String),
    SelectSemOption(String),
    UpdateGrade(usize, String),
    UpdateCpiOp1Var1(String),
    UpdateCpiOp2Var1(String),
    UpdateCpiOp2Var2(String),
    Calculate,
    Reset,
    ToggleGradeTable,
    SetDone,
    SelectCpiCalcOption(String),
}

pub struct App {
    calc_type: Option<i32>, // 0 for SPI, 1 for CPI
    sem_no: Option<i32>,
    sem_option: Option<i32>, // For sem 7/8
    sem_no_f32: Option<f32>, // Combined sem_no and option
    done_1: bool, // Tracks if initial semester selection is done
    sem_info: Option<semdata::Semester>,
    grades: Vec<String>, // Store grades as strings for input binding
    calculated_spi: Option<f32>,
    calculated_cpi: Option<f32>,
    show_grade_table: bool,
    calc_cpi_option: Option<i32>, // 1 or 2
    cpi_op1_var1_str: String,
    cpi_op2_var1_str: String,
    cpi_op2_var2_str: String,
    error_message: Option<String>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            calc_type: None,
            sem_no: None,
            sem_option: None,
            sem_no_f32: None,
            done_1: false,
            sem_info: None,
            grades: Vec::new(),
            calculated_spi: None,
            calculated_cpi: None,
            show_grade_table: false,
            calc_cpi_option: None,
            cpi_op1_var1_str: "".to_string(),
            cpi_op2_var1_str: "".to_string(),
            cpi_op2_var2_str: "".to_string(),
            error_message: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        // Clear previous results and errors on most interactions (except Reset and ToggleGradeTable)
        if !matches!(msg, Msg::Reset | Msg::ToggleGradeTable) {
             self.calculated_spi = None;
             self.calculated_cpi = None;
             self.error_message = None;
        }


        match msg {
            Msg::SelectCalcType(val) => {
                self.calc_type = val.parse().ok();
                // Reset dependent state
                self.reset_semester_dependent_state();
                self.done_1 = false; // Need to re-confirm semester
                true // Re-render needed
            }
            Msg::SelectSemester(val) => {
                self.sem_no = val.parse().ok();
                // Reset options if semester changes
                self.sem_option = None;
                self.sem_no_f32 = self.sem_no.map(|n| n as f32);
                self.done_1 = false; // Need to re-confirm semester
                self.reset_semester_dependent_state();
                true
            }
            Msg::SelectSemOption(val) => {
                self.sem_option = val.parse().ok();
                if let (Some(sem), Some(opt)) = (self.sem_no, self.sem_option) {
                     if sem == 7 || sem == 8 {
                        self.sem_no_f32 = Some(sem as f32 + 0.1 * opt as f32);
                     } else {
                         self.sem_no_f32 = Some(sem as f32); // Fallback if option selected for wrong sem
                     }
                }
                self.done_1 = false; // Need to re-confirm semester
                self.reset_semester_dependent_state();
                true
            }
             Msg::SetDone => {
                if self.sem_no.is_some() {
                    // Check if options are needed and selected
                    let sem = self.sem_no.unwrap();
                    if (sem == 7 || sem == 8) && self.sem_option.is_none() {
                         self.error_message = Some("Please select an option for Semester 7/8.".to_string());
                         self.done_1 = false;
                    } else {
                        self.done_1 = true;
                        self.load_semester_data(); // Load data when 'Done' is clicked
                    }
                } else {
                    self.error_message = Some("Please select a semester first.".to_string());
                    self.done_1 = false;
                }
                true
            }
            Msg::UpdateGrade(index, grade_str) => {
                if let Some(_sem_info) = &self.sem_info { // Use _sem_info if not needed inside
                    if index < self.grades.len() {
                         // Basic validation: Allow empty or numbers between 0 and 10
                        if grade_str.is_empty() || (grade_str.parse::<f32>().map_or(false, |g| g >= 0.0 && g <= 10.0)) {
                            self.grades[index] = grade_str;
                            self.error_message = None; // Clear error on valid input
                        } else {
                             // Provide specific feedback but don't overwrite the input value immediately
                             self.error_message = Some(format!("Invalid grade '{}'. Please enter a number between 0 and 10.", grade_str));
                        }
                    }
                }
                true // Re-render to show updated value or error
            }
            Msg::SelectCpiCalcOption(val) => {
                self.calc_cpi_option = val.parse().ok();
                // Reset CPI specific inputs when option changes
                self.cpi_op1_var1_str = "".to_string();
                self.cpi_op2_var1_str = "".to_string();
                self.cpi_op2_var2_str = "".to_string();
                self.grades = vec![]; // Reset grades if switching CPI option
                if let Some(sem_info) = &self.sem_info {
                     self.grades = vec!["".to_string(); sem_info.course_code.len()];
                }
                true
            }
             Msg::UpdateCpiOp1Var1(val) => {
                self.cpi_op1_var1_str = val;
                true
            }
            Msg::UpdateCpiOp2Var1(val) => {
                self.cpi_op2_var1_str = val;
                true
            }
            Msg::UpdateCpiOp2Var2(val) => {
                self.cpi_op2_var2_str = val;
                true
            }
            Msg::Calculate => {
                self.calculate_results();
                true // Re-render to show results or errors
            }
            Msg::Reset => {
                 // Reset state fields directly to defaults
                self.calc_type = None;
                self.sem_no = None;
                self.sem_option = None;
                self.sem_no_f32 = None;
                self.done_1 = false;
                self.sem_info = None;
                self.grades = Vec::new();
                self.calculated_spi = None;
                self.calculated_cpi = None;
                self.show_grade_table = false;
                self.calc_cpi_option = None;
                self.cpi_op1_var1_str = "".to_string();
                self.cpi_op2_var1_str = "".to_string();
                self.cpi_op2_var2_str = "".to_string();
                self.error_message = None;
                true // Re-render needed
            }
            Msg::ToggleGradeTable => {
                self.show_grade_table = !self.show_grade_table;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            // Added wrapper div with ID for CSS styling
            <div id="app-container">
                <h1>{ "CPI / SPI Calculator" }</h1>
                <hr/>

                // --- Calculation Type Selection ---
                <div>
                    <label for="calc_type">{ "Select Calculation:" }</label>
                    <select id="calc_type" onchange={link.callback(|e: Event| Msg::SelectCalcType(e.target_unchecked_into::<HtmlInputElement>().value()))}>
                        <option value="" selected={self.calc_type.is_none()} disabled=true>{ "Select" }</option>
                        <option value="0" selected={self.calc_type == Some(0)}>{ "Calculate SPI" }</option>
                        <option value="1" selected={self.calc_type == Some(1)}>{ "Calculate CPI" }</option>
                    </select>
                </div>

                // --- Semester Selection (Common for SPI/CPI) ---
                { if self.calc_type.is_some() { html! {
                    <div>
                        <label for="semester">{ "Select Semester:" }</label>
                        <select id="semester" onchange={link.callback(|e: Event| Msg::SelectSemester(e.target_unchecked_into::<HtmlInputElement>().value()))}>
                            <option value="" selected={self.sem_no.is_none()} disabled=true>{ "Select" }</option>
                            { for (1..=8).map(|i| html! { <option value={i.to_string()} selected={self.sem_no == Some(i)}>{ format!("Semester {}", i) }</option> }) }
                        </select>

                        // --- Semester Options (for Sem 7/8) ---
                        { if let Some(sem) = self.sem_no {
                            if sem == 7 { html! {
                                <>
                                <label for="sem_option">{ "Select Option (Sem 7):" }</label>
                                <select id="sem_option" onchange={link.callback(|e: Event| Msg::SelectSemOption(e.target_unchecked_into::<HtmlInputElement>().value()))}>
                                    <option value="" selected={self.sem_option.is_none()} disabled=true>{ "Select" }</option>
                                    <option value="1" selected={self.sem_option == Some(1)}>{ "Option 1" }</option>
                                    <option value="2" selected={self.sem_option == Some(2)}>{ "Option 2" }</option>
                                </select>
                                </>
                            }} else if sem == 8 { html! {
                                <>
                                <label for="sem_option">{ "Select Option (Sem 8):" }</label>
                                <select id="sem_option" onchange={link.callback(|e: Event| Msg::SelectSemOption(e.target_unchecked_into::<HtmlInputElement>().value()))}>
                                    <option value="" selected={self.sem_option.is_none()} disabled=true>{ "Select" }</option>
                                    <option value="1" selected={self.sem_option == Some(1)}>{ "Option 1" }</option>
                                    <option value="2" selected={self.sem_option == Some(2)}>{ "Option 2" }</option>
                                    <option value="3" selected={self.sem_option == Some(3)}>{ "Option 3" }</option>
                                </select>
                                </>
                            }} else { html!{} } // No options for other semesters
                        } else { html!{} }}

                        // --- Done Button ---
                        <button onclick={link.callback(|_| Msg::SetDone)}>{ "Done" }</button>

                        // --- Display Selected Semester/Option ---
                        { if let Some(sem_f32) = self.sem_no_f32 {
                             if self.done_1 { // Only show after 'Done' is clicked
                                html!{ <p>{ format!("Selected: Semester {:.1}", sem_f32) }</p> }
                             } else { html!{} }
                        } else { html!{} }}
                    </div>
                }} else { html!{} }} // End of calc_type check

                <hr/>

                // --- Grade Input Section (SPI or CPI Option 1) ---
                { if self.done_1 && self.should_show_grade_input() {
                    self.view_grade_input(link)
                } else { html!{} }}

                // --- CPI Calculation Options (Only for CPI and Sem > 1) ---
                 { if self.done_1 && self.calc_type == Some(1) && self.sem_no.map_or(false, |s| s > 1) {
                    self.view_cpi_options(link)
                 } else { html!{} }}


                // --- Calculate Button ---
                 { if self.done_1 && (self.should_show_grade_input() || self.calc_cpi_option == Some(2)) { html! {
                    <button onclick={link.callback(|_| Msg::Calculate)}>{ "Calculate" }</button>
                 }} else { html!{} }}


                // --- Display Results ---
                { self.view_results() }

                // --- Display Errors ---
                { self.view_error() }

                <hr/>

                // --- Action Buttons ---
                <div class="action-buttons"> // Added class for centering
                    <button onclick={link.callback(|_| Msg::ToggleGradeTable)}>
                        { if self.show_grade_table { "Hide Grade Table" } else { "Show Grade Table" } }
                    </button>
                    <button onclick={link.callback(|_| Msg::Reset)} class="reset-button">{ "Reset" }</button> // Added class for potential specific styling
                </div>

                // --- Grade Table ---
                { self.view_grade_table() }

            </div> // End of #app-container
        }
    }
}

impl App {
    // Helper function to reset state dependent on semester selection
    fn reset_semester_dependent_state(&mut self) {
        self.sem_info = None;
        self.grades = Vec::new();
        self.calculated_spi = None;
        self.calculated_cpi = None;
        self.calc_cpi_option = None; // Reset CPI option as well
        self.cpi_op1_var1_str = "".to_string();
        self.cpi_op2_var1_str = "".to_string();
        self.cpi_op2_var2_str = "".to_string();
        self.error_message = None;
    }

    // Helper function to load semester data
    fn load_semester_data(&mut self) {
        if let Some(sem_f32) = self.sem_no_f32 {
            match semdata::get_semesters(sem_f32) {
                Some(info) => {
                    self.grades = vec!["".to_string(); info.course_code.len()]; // Initialize grades vector
                    self.sem_info = Some(info);
                    self.error_message = None; // Clear previous errors
                }
                None => {
                    self.sem_info = None;
                    self.grades = Vec::new();
                    self.error_message = Some(format!("Could not find data for semester {:.1}", sem_f32));
                }
            }
        }
    }

     // Determine if the grade input section should be shown
    fn should_show_grade_input(&self) -> bool {
        if !self.done_1 || self.sem_info.is_none() {
            return false;
        }
        match self.calc_type {
            Some(0) => true, // Always show for SPI
            Some(1) => self.sem_no == Some(1) || self.calc_cpi_option == Some(1), // Show for CPI Sem 1 or CPI Option 1
            _ => false,
        }
    }

    // --- View Helper Functions ---

    fn view_grade_input(&self, link: &html::Scope<Self>) -> Html {
        match &self.sem_info {
            Some(info) => html! {
                <div>
                    <h3>{ format!("Enter Grades for Semester {:.1}", self.sem_no_f32.unwrap_or(0.0)) }</h3>
                    { for info.course_code.iter().enumerate().map(|(i, code)| self.view_course_grade_input(link, i, code, &info.course_name[i], info.course_credit[i])) }
                </div>
            },
            None => html! { <p>{ "Select semester and click 'Done' to load courses." }</p> },
        }
    }

     fn view_course_grade_input(&self, link: &html::Scope<Self>, index: usize, code: &str, name: &str, credit: f32) -> Html {
        let grade_value = self.grades.get(index).cloned().unwrap_or_default();
        html! {
            <div class="course-item">
                <div class="course-details">{ format!("{} - {} (Credits: {})", code, name, credit) }</div>
                <label for={format!("grade-{}", index)}>{ "Enter Grade (0-10):" }</label>
                <input
                    id={format!("grade-{}", index)}
                    type="number"
                    min="0"
                    max="10"
                    step="1" // Or "0.1" if decimal grades are allowed by logic
                    placeholder="e.g., 8"
                    value={grade_value}
                    oninput={link.callback(move |e: InputEvent| Msg::UpdateGrade(index, e.target_unchecked_into::<HtmlInputElement>().value()))}
                />
            </div>
        }
    }

     fn view_cpi_options(&self, link: &html::Scope<Self>) -> Html {
        let sem = self.sem_no.unwrap_or(0);
        html! {
            <div>
                <h3>{ "Calculate CPI Options" }</h3>
                <div>
                    <input
                        type="radio"
                        id="cpi_opt_1"
                        name="cpi_option"
                        value="1"
                        checked={self.calc_cpi_option == Some(1)}
                        onclick={link.callback(|_| Msg::SelectCpiCalcOption("1".to_string()))}
                    />
                    <label for="cpi_opt_1">{ format!("Calculate CPI for Sem {} using previous CPI (Sem {}) and current semester grades", sem, sem - 1) }</label>
                </div>
                 <div>
                    <input
                        type="radio"
                        id="cpi_opt_2"
                        name="cpi_option"
                        value="2"
                        checked={self.calc_cpi_option == Some(2)}
                        onclick={link.callback(|_| Msg::SelectCpiCalcOption("2".to_string()))}
                    />
                    <label for="cpi_opt_2">{ format!("Calculate CPI for Sem {} using previous CPI (Sem {}) and current semester SPI (Sem {})", sem, sem - 1, sem) }</label>
                </div>

                // --- Inputs for CPI Option 1 ---
                { if self.calc_cpi_option == Some(1) { html! {
                    <div>
                        <label for="cpi_op1_var1">{ format!("Enter CPI up to Semester {}:", sem - 1) }</label>
                        <input
                            id="cpi_op1_var1"
                            type="number"
                            step="any"
                            placeholder="e.g., 8.5"
                            value={self.cpi_op1_var1_str.clone()}
                            oninput={link.callback(|e: InputEvent| Msg::UpdateCpiOp1Var1(e.target_unchecked_into::<HtmlInputElement>().value()))}
                        />
                        // Grade inputs are handled by view_grade_input, shown conditionally
                    </div>
                }} else { html!{} }}

                 // --- Inputs for CPI Option 2 ---
                { if self.calc_cpi_option == Some(2) { html! {
                    <div>
                        <div>
                            <label for="cpi_op2_var1">{ format!("Enter CPI up to Semester {}:", sem - 1) }</label>
                            <input
                                id="cpi_op2_var1"
                                type="number"
                                step="any"
                                placeholder="e.g., 8.5"
                                value={self.cpi_op2_var1_str.clone()}
                                oninput={link.callback(|e: InputEvent| Msg::UpdateCpiOp2Var1(e.target_unchecked_into::<HtmlInputElement>().value()))}
                            />
                        </div>
                        <div>
                            <label for="cpi_op2_var2">{ format!("Enter SPI of Semester {}:", sem) }</label>
                            <input
                                id="cpi_op2_var2"
                                type="number"
                                step="any"
                                placeholder="e.g., 9.0"
                                value={self.cpi_op2_var2_str.clone()}
                                oninput={link.callback(|e: InputEvent| Msg::UpdateCpiOp2Var2(e.target_unchecked_into::<HtmlInputElement>().value()))}
                            />
                        </div>
                    </div>
                }} else { html!{} }}
            </div>
        }
    }

    fn view_results(&self) -> Html {
        html! {
            <>
                { if let Some(spi) = self.calculated_spi {
                    html! { <div class="result">{ format!("Calculated SPI: {:.3}", spi) }</div> }
                } else { html!{} }}
                { if let Some(cpi) = self.calculated_cpi {
                     html! { <div class="result">{ format!("Calculated CPI: {:.3}", cpi) }</div> }
                } else { html!{} }}
            </>
        }
    }

     fn view_error(&self) -> Html {
        match &self.error_message {
            Some(err) => html! { <p class="error">{ err }</p> },
            None => html! {},
        }
    }

    fn view_grade_table(&self) -> Html {
        if self.show_grade_table {
            html! {
                <div class="grade-table-container">
                    <h3>{ "Grade Point Table" }</h3>
                    <table class="grade-table">
                        <thead>
                            <tr><th>{ "Grade" }</th><th>{ "Points" }</th></tr>
                        </thead>
                        <tbody>
                            <tr><td>{ "AA" }</td><td>{ "10" }</td></tr>
                            <tr><td>{ "AB" }</td><td>{ "9" }</td></tr>
                            <tr><td>{ "BB" }</td><td>{ "8" }</td></tr>
                            <tr><td>{ "BC" }</td><td>{ "7" }</td></tr>
                            <tr><td>{ "CC" }</td><td>{ "6" }</td></tr>
                            <tr><td>{ "CD" }</td><td>{ "5" }</td></tr>
                            <tr><td>{ "DD" }</td><td>{ "4" }</td></tr>
                            <tr><td>{ "FF" }</td><td>{ "0" }</td></tr>
                        </tbody>
                    </table>
                </div>
            }
        } else {
            html! {}
        }
    }

    // --- Calculation Logic ---
    fn calculate_results(&mut self) {
        // Clear previous results before new calculation
        self.calculated_spi = None;
        self.calculated_cpi = None;
        self.error_message = None;

        let sem_f32 = match self.sem_no_f32 {
            Some(s) => s,
            None => {
                self.error_message = Some("Semester not selected.".to_string());
                return;
            }
        };

        // --- SPI Calculation (Needed for SPI mode, CPI Sem 1, CPI Option 1) ---
        let spi_result: Option<f32> = if self.calc_type == Some(0) || (self.calc_type == Some(1) && (self.sem_no == Some(1) || self.calc_cpi_option == Some(1))) {
             if self.grades.iter().any(|g| g.is_empty()) {
                self.error_message = Some("Please enter all grades.".to_string());
                None // Indicate error by returning None
            } else {
                 match self.grades.iter().map(|g| g.parse::<f32>()).collect::<Result<Vec<f32>, _>>() {
                    Ok(_parsed_grades) => { // Use _parsed_grades if calculate_spi_n is used later
                        // Using calc_spi which takes Vec<String> as per original logic
                        let spi = functions::calc_spi(sem_f32, self.grades.clone());
                        Some(spi)
                    }
                    Err(_) => {
                        self.error_message = Some("Invalid grade(s) entered. Please use numbers between 0 and 10.".to_string());
                        None
                    }
                }
            }
        } else {
            None // SPI not needed for CPI Option 2 directly here
        };

        // --- Set SPI result if calculated ---
        if let Some(spi) = spi_result {
            self.calculated_spi = Some(spi);
        }


        // --- CPI Calculation ---
        if self.calc_type == Some(1) { // Only calculate CPI if CPI mode is selected
            if self.sem_no == Some(1) {
                 if let Some(spi) = spi_result {
                    self.calculated_cpi = Some(spi); // For Sem 1, CPI = SPI
                 } // Error handled by SPI calculation part
            } else if self.calc_cpi_option == Some(1) {
                 if let Some(spi) = spi_result { // CPI Option 1 depends on successful SPI calculation
                     match self.cpi_op1_var1_str.parse::<f32>() {
                        Ok(prev_cpi) => {
                            let cpi = functions::calculate_cpi_option3(sem_f32, prev_cpi, spi);
                            self.calculated_cpi = Some(cpi);
                        }
                        Err(_) => {
                            self.error_message = Some("Invalid previous CPI value entered for Option 1.".to_string());
                            self.calculated_spi = None; // Clear SPI if CPI calc fails due to this input
                        }
                    }
                 } // Error handled by SPI calculation part
            } else if self.calc_cpi_option == Some(2) {
                 match (self.cpi_op2_var1_str.parse::<f32>(), self.cpi_op2_var2_str.parse::<f32>()) {
                    (Ok(prev_cpi), Ok(current_spi)) => {
                        let cpi = functions::calculate_cpi_option3(sem_f32, prev_cpi, current_spi);
                        self.calculated_cpi = Some(cpi);
                        // Display the entered SPI for clarity as well
                        self.calculated_spi = Some(current_spi);
                    }
                    (Err(_), _) => {
                         self.error_message = Some("Invalid previous CPI value entered for Option 2.".to_string());
                    }
                     (_, Err(_)) => {
                         self.error_message = Some("Invalid current SPI value entered for Option 2.".to_string());
                    }
                }
            } else if self.sem_no.map_or(false, |s| s > 1) {
                 // If CPI is selected, semester > 1, but no CPI option chosen
                 self.error_message = Some("Please select a CPI calculation option.".to_string());
            }
        } else if self.calc_type.is_none() {
             self.error_message = Some("Please select calculation type (SPI or CPI).".to_string());
        }

        // Final check if no results were calculated and no specific error was set yet
        if self.calculated_spi.is_none() && self.calculated_cpi.is_none() && self.error_message.is_none() {
             // This case might occur if Calculate is pressed too early, e.g., before 'Done'
             if !self.done_1 {
                 self.error_message = Some("Please select semester/options and click 'Done' first.".to_string());
             } else {
                 // Default error if something unexpected happened
                 self.error_message = Some("Calculation could not be performed. Check inputs.".to_string());
             }
        }
    }
}


// Entry point for WASM
#[wasm_bindgen(start)]
pub fn run_app() {
    // Mount the Yew app to the document body
    yew::Renderer::<App>::new().render();
}
