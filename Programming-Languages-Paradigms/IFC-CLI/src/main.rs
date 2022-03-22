use std::io::stdin;
// Convert a Gregorian date to a International Fixed Calendar
// https://en.wikipedia.org/wiki/International_Fixed_Calendar

fn main() {
    loop {
        println!("Please enter three positive integers, separated by spaces:");
        let mut input = String::new();

        // if the user enters "quit" terminate the program
        if input.contains("quit") {
            break;
        }

        // Read the input
        stdin().read_line(&mut input).unwrap();

        // Remove the whitespace
        let mut iter = input.split_whitespace();

        // first value is year
        let year = iter.next().unwrap().parse::<i32>().unwrap();

        // second value is month
        let month = iter.next().unwrap().parse::<i32>().unwrap();

        // third value is day
        let day = iter.next().unwrap().parse::<i32>().unwrap();

        // Check if the input is valid date
        if validate_date(year, month, day) {
            // Convert the date to I.F.D.

            let day_of_year = day_of_year(year, month, day);
            let message = convert_idf_to_humanreadable_date(day_of_year, year);
            println!("{}", message);
        } else {
            println!("Invalid Input, Please try again!");
        }
    }
}

fn validate_date(year: i32, month: i32, day: i32) -> bool {
    if year < 1 || month < 1 || day < 1 {
        return false;
    }

    let mut days_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
        days_in_month[1] = 29;
    }

    let month_minus_1 = month - 1;
    if month > 12 || day > days_in_month[month_minus_1 as usize] {
        return false;
    }

    true
}
// find the day of the year and whether or not leap year. Then convert to I.F.D.
fn day_of_year(year: i32, month: i32, day: i32) -> i32 {
    let mut days_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    // Leap year calculations
    if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
        days_in_month[1] = 29;
    }

    // find the day of the year
    let mut day_of_year = 0;
    for i in 0..month - 1 {
        day_of_year += days_in_month[i as usize];
    }

    // convert to I.F.D.
    day_of_year += day;

    day_of_year
}

fn convert_idf_to_humanreadable_date(day_of_year: i32, year: i32) -> String {
    // find the month in the International Fixed Calendar
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "Sol",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let mut month = "January";
    let mut date = day_of_year;
    let mut day = String::from("(Saturday)");

    let mut leap_year = false;
    if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
        leap_year = true
    }

    if leap_year == true {
        if day_of_year == 169 {
            return String::from("Leap Day");
        }
        if day_of_year == 365 {
            return String::from("2020 December 28 (Saturday)");
        }
    }

    //  For loop wasn't working properly so I had to use gross if statements
    // for i in 0..=12 {
    //     if day_of_year < 28 {
    //         month = months[0];
    //         date = day_of_year;
    //         day = find_day_of_the_week(date);

    //         return format!("{} {} {} {}", year, month, date, day);
    //     }
    //     if day_of_year > (28 * i) + 1 && day_of_year < 28 * (i + 1) {
    //         month = months[i as usize];
    //         date = day_of_year - 28 * i;
    //         day = find_day_of_the_week(date);

    //         return format!("{} {} {} {}", year, month, date, day);

    //     } else if day_of_year > 364 {
    //         return String::from("Year Day");
    // }

    let jan = 1..29;
    let feb = 28 * 1..2 * 28;
    let mar = 28 * 2 + 1..28 * 3;
    let apr = 28 * 3 + 1..28 * 4;
    let may = 28 * 4 + 1..28 * 5;
    let jun = 28 * 5 + 1..28 * 6;
    let sol = 28 * 6 + 1..28 * 7;
    let jul = 28 * 7 + 1..28 * 8;
    let aug = 28 * 8 + 1..28 * 9;
    let sep = 28 * 9 + 1..28 * 10;
    let oct = 28 * 10 + 1..28 * 11;
    let nov = 28 * 11 + 1..28 * 12;
    let dec = 28 * 12 + 1..28 * 13 + 1;

    if jan.contains(&day_of_year) {
        month = months[0 as usize];
        date = day_of_year;
        day = find_day_of_the_week(date);
    } else if feb.contains(&day_of_year) {
        month = months[1 as usize];
        date = day_of_year - 28;
        day = find_day_of_the_week(date);
    } else if mar.contains(&day_of_year) {
        month = months[2 as usize];
        date = day_of_year - 28 * 2;
        day = find_day_of_the_week(date);
    } else if apr.contains(&day_of_year) {
        month = months[3 as usize];
        date = day_of_year - 28 * 3;
        day = find_day_of_the_week(date);
    } else if may.contains(&day_of_year) {
        month = months[4 as usize];
        date = day_of_year - 28 * 4;
        day = find_day_of_the_week(date);
    } else if jun.contains(&day_of_year) {
        month = months[5 as usize];
        date = day_of_year - 28 * 5;
        day = find_day_of_the_week(date);
    } else if sol.contains(&day_of_year) {
        month = months[6 as usize];
        date = day_of_year - 28 * 6;
        day = find_day_of_the_week(date);
    } else if jul.contains(&day_of_year) {
        month = months[7 as usize];
        date = day_of_year - 28 * 7;
        day = find_day_of_the_week(date);
    } else if aug.contains(&day_of_year) {
        month = months[8 as usize];
        date = day_of_year - 28 * 8;
        day = find_day_of_the_week(date);
    } else if sep.contains(&day_of_year) {
        month = months[9 as usize];
        date = day_of_year - 28 * 9;
        day = find_day_of_the_week(date);
    } else if oct.contains(&day_of_year) {
        month = months[10 as usize];
        date = day_of_year - 28 * 10;
        day = find_day_of_the_week(date);
    } else if nov.contains(&day_of_year) {
        month = months[11 as usize];
        date = day_of_year - 28 * 11;
        day = find_day_of_the_week(date);
    } else if dec.contains(&day_of_year) {
        month = months[12 as usize];
        date = day_of_year - 28 * 12;
        day = find_day_of_the_week(date);
    } else if day_of_year > 364 {
        return String::from("Year Day");
    }

    return format!("{} {} {} {}", year, month, date, day);
}

fn find_day_of_the_week(day_of_month: i32) -> String {
    struct DayOfIfd {
        name: String,
        dates: [i32; 4],
    }

    let days_of_ifd = [
        DayOfIfd {
            name: "(Sunday)".to_string(),
            dates: [1, 8, 15, 22],
        },
        DayOfIfd {
            name: "(Monday)".to_string(),
            dates: [2, 9, 16, 23],
        },
        DayOfIfd {
            name: "(Tuesday)".to_string(),
            dates: [3, 10, 17, 24],
        },
        DayOfIfd {
            name: "(Wednesday)".to_string(),
            dates: [4, 11, 18, 25],
        },
        DayOfIfd {
            name: "(Thursday)".to_string(),
            dates: [5, 12, 19, 26],
        },
        DayOfIfd {
            name: "(Friday)".to_string(),
            dates: [6, 13, 20, 27],
        },
        DayOfIfd {
            name: "(Saturday)".to_string(),
            dates: [7, 14, 21, 28],
        },
    ];

    let mut day = String::from("Sunday");

    // Match day to dates in DayOfIfd
    for i in 0..days_of_ifd.len() {
        for j in 0..days_of_ifd[i as usize].dates.len() {
            if days_of_ifd[i as usize].dates[j as usize] == day_of_month {
                day = days_of_ifd[i as usize].name.clone();
            }
        }
    }

    day
}
