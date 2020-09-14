use crate::month::Month;
use crate::weekday::Weekday;

//  DateInfo generated by Intdate::get_date_info()
#[derive(Debug)]
pub struct DateInfo {
    pub year: u16,
    pub month: Month,
    pub weekday: Weekday,
    pub day: u8,
    pub day_of_year: u16,
}
impl DateInfo {
    fn build_fdate(&self, input: &str) -> String {
        let mut output = String::new();

        let mut escape_next = false;
        let mut is_expr = false;

        let mut last_char: char = '0';

        for c in input.chars() {

            // interpret as-is (escape)
            if escape_next {
                output.push(c);
                escape_next = false;
                continue;
            }
            escape_next = c == '\\';
            if c == '%' { is_expr = true; }

            // interpret as expression
            if is_expr {
                match c {
                    '%' => { last_char = c; continue; },
                    '0' | '-' | '.' => { last_char = c; continue; },

                    'Y' => {
                        output.push_str(&self.year.to_string());
                        is_expr = false;
                    },
                    'y' => {
                        output.push_str(&self.year.to_string()[2..]);
                        is_expr = false;
                    },

                    'M' => {
                        output.push_str(self.month.name());
                        is_expr = false;
                    },
                    'm' => {
                        match last_char {
                            '%' => output.push_str(&self.month.name()[..3]),
                            '0' => output.push_str(
                                &format!("{:0>2}", self.month.number())),
                            '-' => output.push_str(
                                &self.month.number().to_string()),
                            _ => output.push('m'),
                        }
                        is_expr = false;
                    },

                    'D' => {
                        output.push_str(self.weekday.name());
                        is_expr = false;
                    },
                    'd' => {
                        match last_char {
                            '%' => {
                                if &self.weekday.name() == &"Thursday" {
                                    output.push_str(&self.weekday.name()[..5])
                                } else {
                                    output.push_str(&self.weekday.name()[..3])
                                }
                            },
                            '0' => output.push_str(
                                &format!("{:0>2}", self.day)),
                            '-' => output.push_str(&self.day.to_string()),
                            '.' => {
                                output.push_str(
                                    &format!("{}{}", self.day,
                                        match self.day
                                            .to_string()
                                            .chars()
                                            .last()
                                            .unwrap() {
                                            '1' => "st",
                                            '2' => "nd",
                                            '3' => "rd",
                                            _ => "th",
                                        }));
                            },
                            _ => output.push('d'),
                        }
                    },

                    'j' => {
                        match last_char {
                            '%' => output.push_str(
                                &self.day_of_year.to_string()),
                            '0' => output.push_str(
                                &format!("{:0>3}", self.day_of_year)),
                            _ => output.push('j'),
                        }
                    },

                    _ => {
                        output.push(c);
                        is_expr = false;
                    },
                }
            }

            // interpret as text
            else if !escape_next {
                output.push(c);
            }

            last_char = c;
        }

        output
    }

    pub fn format(&self, input: &str) -> String {
        self.build_fdate(input)
    }
}
