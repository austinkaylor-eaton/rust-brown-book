use std::collections::HashMap;

/// Challenges offered by the book for chapter 8
/// https://rust-book.cs.brown.edu/ch08-03-hash-maps.html#summary

/// Challenge 1
/// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.
pub fn challenge_1(vec: Vec<i32>) -> Challenge1Result
{
    // Calculate the median and the mode
    // median is 1st in tuple, mode is 2nd
    let result = Challenge1Result {
        median: calculate_median(vec.clone()),
        mode: calculate_mode(vec.clone()),
    };
    result
}

// Enum to represent the result of Challenge 1
pub struct Challenge1Result {
    pub median: f32,
    pub mode: Option<i32>,
}

/// Calculate the median of a vector of integers
fn calculate_median(vec: Vec<i32>) -> f32
{
    let mut sorted_vec = vec;
    sorted_vec.sort();
    let middle_index = sorted_vec.len() / 2;
    println!("The middle index is: {middle_index}");
    let median: f32 = match sorted_vec.len() % 2 {
        0 => {
            (sorted_vec[middle_index] + sorted_vec[middle_index - 1]) as f32 / 2.0
        },
        1 => sorted_vec[middle_index] as f32,
        _ => 0f32
    };
    println!("The median is: {median}");
    median 
}

/// Calculate the mode of a vector of integers
/// The mode is the value that occurs most often
/// A list can have more than one mode if multiple values occur the same number of times
/// If no value occurs more than once, the mode is 0
/// 
/// Steps to calculate the mode:
///     - Create a frequency dictionary to count the occurrences of each number.
///     - Identify the number(s) with the highest frequency.
fn calculate_mode(vec: Vec<i32>) -> Option<i32>
{
    let mut frequency_dict: HashMap<i32, i32> = HashMap::new();
    for num in vec.iter()
    {
        let count = frequency_dict.entry(*num).or_insert(0);
        *count += 1;
    }
    
    let mut mode = 0;
    let mut max_frequency = 0;
    for (num, frequency) in frequency_dict.iter()
    {
        if *frequency > max_frequency
        {
            mode = *num;
            max_frequency = *frequency;
        }
    }
    
    // If no value occurs more than once, the mode does not exist
    // So, we can convert this in rust to None
    if max_frequency == 1
    {
        None
    }
    else
    {
        Some(mode)
    }
}

/// Challenge 2
/// Convert strings to pig latin. 
/// The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay. 
/// Words that start with a vowel have hay added to the end instead (apple becomes apple-hay). 
/// Keep in mind the details about UTF-8 encoding!
pub fn challenge_2(words: String) -> String
{
    let mut pig_latin_words: Vec<String> = Vec::new();
    for word in words.split_whitespace()
    {
        let mut chars = word.chars();
        let first_char = chars.next().unwrap();
        let pig_latin_word = match first_char
        {
            'a' | 'e' | 'i' | 'o' | 'u' => format!("{word}-hay"),
            _ => {
                let rest_of_word: String = chars.collect();
                format!("{rest_of_word}-{first_char}ay")
            },
        };
        pig_latin_words.push(pig_latin_word);
    }
    pig_latin_words.join(" ")
}

/// Challenge 3
/// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company; 
/// for example, “Add Sally to Engineering” or “Add Amir to Sales.” 
/// Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
mod challenge_3
{
    use std::collections::HashMap;

    enum Command
    {
        Add(String, String),
        Retrieve(String),
    }
    
    #[derive(PartialEq, Eq, Hash)]
    enum Department
    {
        Engineering,
        Sales,
        Marketing,
        HumanResources,
    }
    
    struct Company
    {
        departments: HashMap<Department, Vec<String>>,
    }

    impl Company {
        pub fn new() -> Company
        {
            Company {
                departments: HashMap::new(),
            }
        }
        
        /// Executes either the Add or Retrieve command
        pub fn execute_command(&mut self, command: Command)
        {
            match command
            {
                Command::Add(employee, department) => self.add_employee(employee, department),
                Command::Retrieve(department) => {
                    let employees_in_department = self.retrieve_employees(department);
                    
                }
            }
        }
        
        /// Adds an employee to a department
        pub fn add_employee(&mut self, employee: String, department: String)
        {
            let department = match department.as_str()
            {
                "Engineering" => Department::Engineering,
                "Sales" => Department::Sales,
                "Marketing" => Department::Marketing,
                "HumanResources" => Department::HumanResources,
                _ => panic!("Invalid department"),
            };
            let employees = self.departments.entry(department).or_insert(Vec::new());
            employees.push(employee);
        }
        
        /// Retrieves a list of all people in a department or all people in the company by department, sorted alphabetically
        pub fn retrieve_employees(&self, department: String) -> Vec<String>
        {
            let department = match department.as_str()
            {
                "Engineering" => Department::Engineering,
                "Sales" => Department::Sales,
                "Marketing" => Department::Marketing,
                "HumanResources" => Department::HumanResources,
                _ => panic!("Invalid department"),
            };
            let employees = self.departments.get(&department).unwrap();
            let mut sorted_employees = employees.clone();
            sorted_employees.sort().into()
        }
    }
    
    #[cfg(test)]
    #[test]
    fn test_challenge_3()
    {
        let mut company = Company::new();
        company.execute_command(Command::Add(String::from("Sally"), String::from("Engineering")));
        company.execute_command(Command::Add(String::from("Amir"), String::from("Sales")));
        company.execute_command(Command::Add(String::from("John"), String::from("Engineering")));
        
        // Expected output:
        assert_eq!(company.retrieve_employees(String::from("Engineering")), vec!["John", "Sally"]);
        assert_eq!(company.retrieve_employees(String::from("Sales")), vec!["Amir"]);
    }
}

#[cfg(test)] 
mod tests_challenge_1 {
    use super::*;
    
    #[test]
    fn returns_correct_median_and_mode_for_odd_numbered_vector_length_with_repeating_numbers()
    {
        let vec: Vec<i32> = vec![1, 1, 2, 3, 4, 5, 6];
        let result = challenge_1(vec);
        assert_eq!(result.median, 3.0);
        assert_eq!(result.mode, Some(1));
    }
    
    #[test]
    fn returns_correct_median_and_mode_for_odd_numbered_vector_length_with_non_repeating_numbers()
    {
        let vec: Vec<i32> = vec![1, 2, 3, 4, 5];
        let result = challenge_1(vec);
        assert_eq!(result.median, 3.0);
        assert_eq!(result.mode, None);
    }

    #[test]
    fn returns_correct_median_and_mode_for_even_numbered_vector_length_with_repeating_numbers()
    {
        let vec: Vec<i32> = vec![1, 1, 2, 3, 4, 5, 6, 7];
        let result = challenge_1(vec);
        assert_eq!(result.median, 3.5);
        assert_eq!(result.mode, Some(1));
    }

    #[test]
    fn returns_correct_median_and_mode_for_even_numbered_vector_length_with_non_repeating_numbers()
    {
        let vec: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
        let result = challenge_1(vec);
        assert_eq!(result.median, 3.5);
        assert_eq!(result.mode, None);
    }
}

#[cfg(test)]
mod tests_challenge_2 {
    use super::*;

    #[test]
    fn returns_correct_pig_latin_for_words_starting_with_vowels()
    {
        let words = String::from("apple orange elephant");
        let result = challenge_2(words);
        assert_eq!(result, "apple-hay orange-hay elephant-hay");
    }

    #[test]
    fn returns_correct_pig_latin_for_words_starting_with_consonants()
    {
        let words = String::from("first second third");
        let result = challenge_2(words);
        assert_eq!(result, "irst-fay econd-say hird-tay");
    }
}