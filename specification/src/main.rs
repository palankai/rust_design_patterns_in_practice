// Let's imagine you have a big pool of candidates for a job opening.
// You want to filter out the candidates that don't meet your criteria.
// Disclaimer: This is a fictional example, demonstrating the use of the specification pattern.

use specification::Specification;

#[derive(Debug, Clone)]
struct JobCandidate {
    pub name: String,
    pub years_of_experience: f64,
    pub github_contributions: i64,
    pub languages_worked_with: Vec<String>,
    pub desired_salary: i64,
    pub science_degree: bool,
}

// You can construct a specification for each of your criteria.

#[derive(Debug)]
struct MinimumYearsOfExperience {
    min_years: f64,
}

impl Specification<JobCandidate> for MinimumYearsOfExperience {
    fn is_satisfied_by(&self, candidate: &JobCandidate) -> bool {
        candidate.years_of_experience >= self.min_years
    }
}

#[derive(Debug)]
struct MinimumGithubContributions {
    min_contributions: i64,
}

impl Specification<JobCandidate> for MinimumGithubContributions {
    fn is_satisfied_by(&self, candidate: &JobCandidate) -> bool {
        candidate.github_contributions >= self.min_contributions
    }
}

#[derive(Debug)]
struct WorkedWithLanguage {
    language: String,
}

impl Specification<JobCandidate> for WorkedWithLanguage {
    fn is_satisfied_by(&self, candidate: &JobCandidate) -> bool {
        candidate.languages_worked_with.contains(&self.language)
    }
}

#[derive(Debug)]
struct MaxDesiredSalary {
    max_salary: i64,
}

impl Specification<JobCandidate> for MaxDesiredSalary {
    fn is_satisfied_by(&self, candidate: &JobCandidate) -> bool {
        candidate.desired_salary <= self.max_salary
    }
}

#[derive(Debug)]
struct HasScienceDegree {}

impl Specification<JobCandidate> for HasScienceDegree {
    fn is_satisfied_by(&self, candidate: &JobCandidate) -> bool {
        candidate.science_degree
    }
}

const fn yes_or_no(b: bool) -> &'static str {
    if b {
        "Yes"
    } else {
        "No"
    }
}

fn main() {
    // Let's define our criteria, as our Boss said:
    // "We need someone with at least 10 years of experience,
    // who has contributed to at least 5 open source projects,
    // and has worked with C++, or Python.
    // If they have a science degree, then 5 years of experience is enough.
    // If they know Rust we can pay them 130k, otherwise 90k. :D"

    let worked_with_rust = WorkedWithLanguage {
        language: "Rust".to_string(),
    }
    .composite(); //Little trick to make it cloneable
    let worked_with_c_plus_plus = WorkedWithLanguage {
        language: "C++".to_string(),
    };
    let worked_with_python = WorkedWithLanguage {
        language: "Python".to_string(),
    };
    let ten_years_of_experience = MinimumYearsOfExperience { min_years: 10.0 };
    let five_years_of_experience = MinimumYearsOfExperience { min_years: 5.0 };
    let five_github_contributions = MinimumGithubContributions {
        min_contributions: 5,
    };
    let have_science_degree = HasScienceDegree {};
    let desire_no_more_than_90k = MaxDesiredSalary { max_salary: 90_000 };
    let desire_no_more_than_130k = MaxDesiredSalary {
        max_salary: 130_000,
    };

    let satisfies_minimum_requirement =
        five_github_contributions.and(worked_with_c_plus_plus.or(worked_with_python));
    let desires_rust_programmer_salary = worked_with_rust.clone().and(desire_no_more_than_130k);
    let desires_non_rust_programmer_salary = worked_with_rust.invert().and(desire_no_more_than_90k);
    let satisfies_salary_requirement =
        desires_rust_programmer_salary.or(desires_non_rust_programmer_salary);
    let satisfies_experience_requirement =
        ten_years_of_experience.or(five_years_of_experience.and(have_science_degree));

    let good_for_interview = satisfies_minimum_requirement
        .and(satisfies_salary_requirement)
        .and(satisfies_experience_requirement);

    // ^^^ I think that's pretty readable given the complexity of the requirements.
    // Ok, that invert is a bit ugly, but wouln't take long to have a nice `not` function,
    // and have something like this: `let desires_non_rust_programmer_salary = not(worked_with_rust).and(desire_no_more_than_90k);`

    let candidate_a: JobCandidate = {
        let languages_worked_with = vec![
            "Rust".to_string(),
            "C++".to_string(),
            "Python".to_string(),
            "Go".to_string(),
        ];

        JobCandidate {
            name: "John".to_string(),
            years_of_experience: 5.0,
            github_contributions: 10,
            languages_worked_with,
            desired_salary: 100_000,
            science_degree: true,
        }
    };
    println!(
        "Candidate A {}, is good for interview: {}",
        &candidate_a.name,
        yes_or_no(good_for_interview.is_satisfied_by(&candidate_a))
    );

    let candidate_b: JobCandidate = {
        let languages_worked_with = vec!["C++".to_string(), "Python".to_string(), "Go".to_string()];

        JobCandidate {
            name: "Mike".to_string(),
            years_of_experience: 5.0,
            github_contributions: 10,
            languages_worked_with,
            desired_salary: 100_000,
            science_degree: true,
        }
    };
    println!(
        "Candidate B {}, is good for interview: {}",
        &candidate_b.name,
        yes_or_no(good_for_interview.is_satisfied_by(&candidate_b))
    );
    println!(
        "Candidate B is not good for interview because {:?}",
        good_for_interview.reminder_unsatisfied_by(&candidate_b)
    );
    // I admit this isn't necessary the best output, but it is a good example.
}
