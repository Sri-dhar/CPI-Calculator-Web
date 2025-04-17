use crate::semdata;

//create write data and read data function
// Note: These functions interact with the filesystem and won't work directly in WASM/browser.
// They might need to be removed or adapted if scale persistence is needed in the web version.
// For now, they are left here but won't be called by the lib.rs code.
pub fn write_data(scale: f32) {
    // This will likely fail in a browser environment due to sandbox restrictions.
    // Consider using localStorage or other web storage APIs if needed.
    // std::fs::write("configs/scale.txt", scale.to_string()).expect("Unable to write file");
    let _ = scale; // Avoid unused variable warning
    // Using web_sys::console::log_1 might be better for logging in WASM
    println!("Note: write_data called, but filesystem access is restricted in WASM.");
}
pub fn read_data() -> f32 {
    // This will likely fail in a browser environment. Defaulting to a fixed scale.
    // let data = std::fs::read_to_string("configs/scale.txt").expect("Unable to read file");
    // let scale = data.parse::<f32>().unwrap();
    // scale
    println!("Note: read_data called, returning default scale (1.5) as filesystem access is restricted.");
    1.5 // Default scale for web version
}


pub fn calculate_spi_n(sem_no: f32, grades: Vec<f32>) -> f32 {
    let semesters = match semdata::get_semesters(sem_no) {
        Some(s) => s,
        None => return 0.0, // Or handle error appropriately
    };
    let mut spi = 0.0;
    let total_credit = semesters.total_credit;
    if total_credit == 0.0 { return 0.0; } // Avoid division by zero

    for i in 0..semesters.course_name.len() {
        // Ensure grades vector matches course vector length
        if let Some(grade) = grades.get(i) {
             if let Some(credit) = semesters.course_credit.get(i) {
                spi += grade * credit;
             }
        }
    }
    spi / total_credit
}

// This function now directly returns the calculated SPI
// It no longer modifies external state like SHOW_SPI
pub fn calc_spi(sem_no: f32, grades: Vec<String>) -> f32 {
    let mut grades_recieved_by_student: Vec<f32> = Vec::new();
    for grade_str in grades {
        // Use Result for parsing and handle potential errors gracefully
        match grade_str.parse::<f32>() {
            Ok(grade) if grade >= 0.0 && grade <= 10.0 => {
                grades_recieved_by_student.push(grade);
            }
            _ => {
                // Handle invalid grade input - perhaps return an error or a specific value?
                // For now, returning 0.0 SPI if any grade is invalid.
                 println!("Invalid grade found: {}", grade_str); // Log error
                return 0.0;
            }
        }
    }

    // Ensure the number of grades matches the expected number for the semester
    if let Some(sem_info) = semdata::get_semesters(sem_no) {
        if grades_recieved_by_student.len() != sem_info.course_code.len() {
             println!("Mismatch in number of grades provided and courses for semester {}", sem_no);
             return 0.0; // Return 0.0 if grade count is wrong
        }
    } else {
        return 0.0; // Semester data not found
    }


    let spi = calculate_spi_n(sem_no, grades_recieved_by_student);
    spi
}

pub fn calculate_cpi_option3(x: f32, cpi_of_xminus1: f32, spi_of_x: f32) -> f32 {
    // Add checks for valid semester numbers and data existence
    let semester_x = match semdata::get_semesters(x) {
        Some(s) => s,
        None => return 0.0, // Or handle error
    };
     // Handle semester 1 case where x-1.0 would be 0.0
    let prev_sem_no = x - 1.0;
    if prev_sem_no < 1.0 {
        // This function shouldn't be called for Sem 1 CPI calculation directly.
        // CPI for Sem 1 is just SPI of Sem 1.
        // However, if called, return the current SPI as CPI.
        return spi_of_x;
    }

    let semester_x_minus_1 = match semdata::get_semesters(prev_sem_no) {
         Some(s) => s,
         None => return 0.0, // Or handle error
    };


    let credit_of_sem_x :f32 = semester_x.total_credit;
    let cum_sum_of_credit_till_sem_x_minus_1 :f32 = semester_x_minus_1.total_credit_till_sem;

    let total_cumulative_credits = cum_sum_of_credit_till_sem_x_minus_1 + credit_of_sem_x;

    // Avoid division by zero
    if total_cumulative_credits == 0.0 {
        return 0.0;
    }

    let cpi_of_sem_x = (cpi_of_xminus1 * cum_sum_of_credit_till_sem_x_minus_1 + spi_of_x * credit_of_sem_x) / total_cumulative_credits;

    cpi_of_sem_x
}
