pub mod types {

    use std::{ error::Error, fmt };
    use chrono:: {DateTime, FixedOffset };

    // Summary report of application run
    pub struct Report {
        pub examined: i32,
        pub updated: i32,
        pub failed: i32,
        pub errors: Vec<String>
    }
    impl Default for Report {
        fn default() -> Self {
            return Report {
                examined: 0,
                updated: 0,
                failed: 0,
                errors: vec![]
            }
        }
    }

    // Holds any datetimes retrieved from metedata
    pub struct DateTimes {
        pub created_date: Option<DateTime<FixedOffset>>,
        pub modified_date: Option<DateTime<FixedOffset>>
    }
    impl Default for DateTimes {
        fn default() -> Self {
            return DateTimes {
                created_date: None,
                modified_date: None
            }
        }
    }

    // Errors for failure to read or parse dateimes
    #[derive(Debug)]
   pub struct MissingCreateDateError {
        pub file_path: String
    }
    impl fmt::Display for MissingCreateDateError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "No 'Created' date in metadata in '{}'", self.file_path)
        }
    }
    impl Error for MissingCreateDateError {}

    #[derive(Debug)]
    pub struct BadCreateDateError {
        pub file_path: String
    }
    impl fmt::Display for BadCreateDateError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "'Created' date malformed in '{}'", self.file_path)
        }
    }
    impl Error for BadCreateDateError {}

    #[derive(Debug)]
   pub struct MissingModifyDateError {
        pub file_path: String
    }
    impl fmt::Display for MissingModifyDateError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "No 'Modified' date in metadata in '{}'", self.file_path)
        }
    }
    impl Error for MissingModifyDateError {}

    #[derive(Debug)]
    pub struct BadModifyDateError {
        pub file_path: String
    }
    impl fmt::Display for BadModifyDateError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "'Modifed' date malformed in '{}'", self.file_path)
        }
    }
    impl Error for BadModifyDateError {}
}
