use std::fmt;

use crate::month::Month;
use crate::weekday::Weekday;


pub struct FormattedDate<'a, 'b> {
    date: &'a DateInfo,
    format: &'b str,
}
impl<'a, 'b> fmt::Display for FormattedDate<'a, 'b> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let year = self.date.year;
        let month = self.date.month;
        let weekday = self.date.weekday;
        let day = self.date.day;
        let day_of_year = self.date.day_of_year;

        let input = self.format.chars();

        let mut escape_next = false;
        let mut is_expr = false;

        let mut last_c = '0';

        for c in input {
            if escape_next {
                write!(f, "{}", c);
                escape_next = false;
                continue;
            }
            escape_next = c == '\\';
            if c == '%' { is_expr = true; }

            if is_expr {
                match c {
                    '%' | '0' | '-' | '.' => { last_c = c; continue; },

                    'Y' => {
                        fmt::Display::fmt(&year, f)?;
                        is_expr = false;
                    },
                    'y' => {
                        fmt::Display::fmt(&(year % 100), f);
                        is_expr = false;
                    },

                    'M' => {
                        f.write_str(month.name());
                        is_expr = false;
                    },
                    'm' => {
                        match last_c {
                            '%' => f.write_str(&month.name()[..3]),
                            '0' => write!(f, "{:0>2}", month.number()),
                            '-' => fmt::Display::fmt(&month.number(), f),
                            _ => f.write_str("m")
                        };
                        is_expr = false
                    },

                    'D' => {
                        f.write_str(weekday.name());
                        is_expr = false;
                    },
                    'd' => {
                        match last_c {
                            '%' => { f.write_str(&weekday.name()[..3]); },
                            '0' => { write!(f, "{:0>2}", day); },
                            '-' => { fmt::Display::fmt(&day, f); },
                            '.' => {
                                write!(f, "{}{}", day,
                                    match day
                                        .to_string()
                                        .chars()
                                        .last()
                                        .unwrap() {
                                        '1' => "st",
                                        '2' => "nd",
                                        '3' => "rd",
                                        _ => "th",
                                    });
                            },
                            _ => { f.write_str("d"); },
                        };
                        is_expr = false;
                    },

                    'j' => {
                        match last_c {
                            '%' => fmt::Display::fmt(&day_of_year, f),
                            '0' => write!(f, "{:0>3}", day_of_year),
                            _ => f.write_str("j"),
                        };
                        is_expr = false;
                    },
                    
                    _ => { write!(f, "{}", c); },
                }
            } else { write!(f, "{}", c); }
        }
        Ok(())
    }
}

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
    pub fn format<'b>(&self, input: &'b str) -> FormattedDate<'_, 'b> {
        FormattedDate { date: self, format: input }
    }
}
