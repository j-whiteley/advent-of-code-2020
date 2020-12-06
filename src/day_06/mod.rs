use std::fs;
use regex::Regex;
use std::collections::HashMap;

pub fn day_six(args: &Vec<String>) {

    if args.len() == 3
    {
        let filename = &args[2];
        println!("file name, {}\n", filename);

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");
    
        let split = contents.split("\n");
        let mut current_survey_results = HashMap::new();
        let mut all_surveys = Vec::new();
        let survey_regex = Regex::new(r"[a-z]{1}").unwrap();
        let mut person_count = 0;
        let mut group_concensus = 0;
        
        for line in split {
            if line.len() > 0 {
                for matched in survey_regex.find_iter(line) {
                    let match_key = matched.as_str();
                    if current_survey_results.contains_key(match_key) {
                        current_survey_results.insert(match_key, current_survey_results.get(match_key).unwrap()+1);
                    } else {
                        current_survey_results.insert(match_key, 1);
                    }
                } 
                person_count += 1;
            } else {
                //empty line start of new survey
                for (_key, value) in current_survey_results.iter() {
                    if value.clone() == person_count {
                        group_concensus += 1;
                    }
                }
                all_surveys.push(current_survey_results);
                current_survey_results = HashMap::new();
                person_count = 0;
            }
        }

        for (_key, value) in current_survey_results.iter() {
            if value.clone() == person_count {
                group_concensus += 1;
            }
        }

        all_surveys.push(current_survey_results);

        println!("Surveys :{} \n", all_surveys.len());
        let mut unique_count = 0;

        for survey in all_surveys.iter() {
            unique_count = unique_count + survey.keys().len();
        }

        println!("Unique answers : {}\n", unique_count);
        println!("Group conscense : {}\n", group_concensus);
    }
    else
    {
        println!("day_six needs 2 args\n");
    }
}