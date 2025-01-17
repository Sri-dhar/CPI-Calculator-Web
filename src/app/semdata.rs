#[derive(serde::Deserialize, serde::Serialize)]
pub struct Semester {
    pub sem_no: f32,
    pub course_code: Vec<String>,
    pub course_name: Vec<String>,
    pub course_credit: Vec<f32>,
    pub total_credit: f32,
    pub total_credit_till_sem: f32,
}

impl Default for Semester {
    fn default() -> Self {
        Self {
            sem_no: 0.0,
            course_code: Vec::new(),
            course_name: Vec::new(),
            course_credit: Vec::new(),
            total_credit: 0.0,
            total_credit_till_sem: 0.0,
        }
    }
}

pub fn get_semesters(sem_no:f32) -> Option<Semester> {
    let vector_of_semester = vec![
        Semester {
            sem_no: 1.0,
            course_code: vec![
                "MA101".to_string(),
                "CS101".to_string(),
                "CS110".to_string(),
                "EC101".to_string(),
                "EC110".to_string(),
                "EC102".to_string(),
                "HS101".to_string(),
                "GE101".to_string(),
            ],
            course_name: vec![
                "Mathematics I".to_string(),
                "Computer Programming".to_string(),
                "Computer Programming Lab".to_string(),
                "Digital Design".to_string(),
                "Digital Design Lab".to_string(),
                "Electrical Circuit Analysis".to_string(),
                "English".to_string(),
                "Induction Program".to_string(),
            ],
            course_credit: vec![8.0, 6.0, 3.0, 8.0, 3.0, 8.0, 4.0, 6.0],
            total_credit: 46.0,
            total_credit_till_sem: 46.0,
        },
        Semester {
            sem_no: 2.0,
            course_code: vec![
                "MA102".to_string(),
                "CS103".to_string(),
                "CS111".to_string(),
                "CS104".to_string(),
                "EC103".to_string(),
                "EC111".to_string(),
                "HSXXX".to_string(),
            ],
            course_name: vec![
                "Mathematics II".to_string(),
                "Data Structures".to_string(),
                "Data Structures Lab".to_string(),
                "Computer Organization".to_string(),
                "Basic Electronic Circuits".to_string(),
                "Basic Electronics Lab".to_string(),
                "HSS Elective".to_string(),
            ],
            course_credit: vec![8.0, 8.0, 3.0, 8.0, 8.0, 3.0, 6.0],
            total_credit: 44.0,
            total_credit_till_sem: 90.0,
        },
        Semester {
            sem_no: 3.0,
            course_code: vec![
                "MA203".to_string(),
                "MA205".to_string(),
                "CS201".to_string(),
                "CS210".to_string(),
                "CS202".to_string(),
                "SC201".to_string(),
                "HSXXX".to_string(),
            ],
            course_name: vec![
                "Mathematics III".to_string(),
                "Discrete Mathematics".to_string(),
                "Algorithms".to_string(),
                "Algorithm Lab".to_string(),
                "IT Workshop I".to_string(),
                "Physics I".to_string(),
                "HSS Elective".to_string(),
            ],
            course_credit: vec![6.0, 6.0, 6.0, 3.0, 7.0, 6.0, 6.0],
            total_credit: 40.0,
            total_credit_till_sem: 130.0,
        },
        Semester {
            sem_no: 4.0,
            course_code: vec![
                "CS205".to_string(),
                "CS231".to_string(),
                "CS232".to_string(),
                "CS235".to_string(),
                "CS236".to_string(),
                "CS240".to_string(),
                "CS241".to_string(),
                "SC202".to_string(),
                "HSXXX".to_string(),
            ],
            course_name: vec![
                "Optimization Techniques".to_string(),
                "Operating Systems".to_string(),
                "Operating Systems Lab".to_string(),
                "Artificial Intelligence".to_string(),
                "Artificial Intelligence Lab".to_string(),
                "Database Management Systems".to_string(),
                "DBMS Lab".to_string(),
                "Chemistry".to_string(),
                "HSS Elective".to_string(),
            ],
            course_credit: vec![6.0, 6.0, 3.0, 6.0, 3.0, 6.0, 4.0, 6.0, 6.0],
            total_credit: 46.0,
            total_credit_till_sem: 176.0,
        },
        Semester {
            sem_no: 5.0,
            course_code: vec![
                "CS301".to_string(),
                "CS352".to_string(),
                "CS353".to_string(),
                "CS306".to_string(),
                "CS360".to_string(),
                "CS351".to_string(),
                "SC301".to_string(),
                "HSXXX".to_string(),
            ],
            course_name: vec![
                "Theory of Computation".to_string(),
                "Computer Networks".to_string(),
                "Computer Networks Lab".to_string(),
                "Machine Learning".to_string(),
                "Machine Learning Lab".to_string(),
                "IT Workshop III : Cloud Computing".to_string(),
                "Biology".to_string(),
                "HSS Elective".to_string(),
            ],
            course_credit: vec![6.0, 6.0, 4.0, 6.0, 3.0, 7.0, 6.0, 6.0],
            total_credit: 44.0,
            total_credit_till_sem: 220.0,
        },
        Semester {
            sem_no: 6.0,
            course_code: vec![
                "CS330".to_string(),
                "CS331".to_string(),
                "CS320".to_string(),
                "CS321".to_string(),
                "CS361".to_string(),
                "SC302".to_string(),
                "CS300".to_string(),
                "HSXXX".to_string(),
            ],
            course_name: vec![
                "Software Engineering".to_string(),
                "Software Engineering Lab".to_string(),
                "Compilers".to_string(),
                "Compilers Lab".to_string(),
                "Computer Security".to_string(),
                "Physics II".to_string(),
                "Project-I / Elective - I".to_string(),
                "HSS Elective".to_string(),
            ],
            course_credit: vec![6.0, 3.0, 6.0, 3.0, 6.0, 6.0, 6.0, 6.0],
            total_credit: 42.0,
            total_credit_till_sem: 262.0,
        },
        Semester {
            sem_no: 7.1,
            course_code: vec![
                "CS401".to_string(),
                "CS4XX".to_string(),
                "CS4XX".to_string(),
                "CS4XX".to_string(),
                "HSXXX".to_string(),
            ],
            course_name: vec![
                "Data Analytics".to_string(),
                "Open Elective".to_string(),
                "Elective I".to_string(),
                "Elective II".to_string(),
                "HSS Elective".to_string(),
            ],
            course_credit: vec![6.0, 6.0, 6.0, 6.0, 6.0],
            total_credit: 30.0,
            total_credit_till_sem: 292.0,
        },
        Semester {
            sem_no: 7.2,
            course_code: vec![
                "CS401".to_string(),
                "CS4XX".to_string(),
                "CS4XX".to_string(),
                "CS400".to_string(),
                "HSXXX".to_string(),
            ],
            course_name: vec![
                "Data Analytics".to_string(),
                "Open Elective".to_string(),
                "Elective I".to_string(),
                "Project II".to_string(),
                "HSS Elective".to_string(),
            ],
            course_credit: vec![6.0, 6.0, 6.0, 6.0, 6.0],
            total_credit: 30.0,
            total_credit_till_sem: 292.0,

        },
        Semester {
            sem_no: 8.1,
            course_code: vec![
                "CS4XX".to_string(),
                "CS4XX".to_string(),
                "CS4XX".to_string(),
                "CS4XX".to_string(),
                "HS4XX".to_string(),
            ],
            course_name: vec![
                "Elective".to_string(),
                "Elective".to_string(),
                "Elective".to_string(),
                "Elective".to_string(),
                "Elective (HSS)".to_string(),
            ],
            course_credit: vec![6.0, 6.0, 6.0, 6.0, 6.0],
            total_credit: 30.0,
            total_credit_till_sem: 322.0,

        },
        Semester {
            sem_no: 8.2,
            course_code: vec![
                "CS4XX".to_string(),
                "HS4XX".to_string(),
                "CS410".to_string(),
            ],
            course_name: vec![
                "Elective".to_string(),
                "Elective (HSS)".to_string(),
                "Project III".to_string(),
            ],
            course_credit: vec![6.0, 6.0, 18.0],
            total_credit: 30.0,
            total_credit_till_sem: 322.0,

        },
        Semester {
            sem_no: 8.3,
            course_code: vec![
                "CS4XX".to_string(),
                "HS4XX".to_string(),
                "CS411".to_string(),
            ],
            course_name: vec![
                "Elective".to_string(),
                "Elective (HSS)".to_string(),
                "Internship".to_string(),
            ],
            course_credit: vec![6.0, 6.0, 18.0],
            total_credit: 30.0,
            total_credit_till_sem: 322.0,

        },
    ];

    for sem in vector_of_semester {
        if sem.sem_no == sem_no {
            return Some(sem);
        }
    }

    None
}
