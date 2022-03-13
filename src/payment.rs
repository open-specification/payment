pub fn luhn_method(credit_number:&str) -> bool {

    // Initialize General Variables
    let all_digits:Vec<char> = credit_number.chars().collect();
    let digit_count = all_digits.len();
    let last_digit = all_digits[digit_count - 1];
    let other_digits = &all_digits[0..(digit_count - 1)];

    // Initialize Looping Variables
    let mut index = 0;
    let mut sum = 0;

    // Loop Through Every Digit
    for digit in other_digits {

        // Convert it to U8
        let mut real_digit = (*digit as u8) - ('0' as u8);

        // Check If It's Even
        if index % 2 == 0 {

            // If So, Double It
            real_digit = real_digit * 2;

            // If New Number has Two Digits, Sum Them
            if real_digit > 9 {

                // Get Each Digit
                let first = real_digit % 10;
                let second = real_digit / 10;

                // Get the Sum of the Two Digits
                real_digit = first + second;

            } 

        }

        // Increase Index and Add the Sum to Total
        index = index + 1;
        sum = sum + real_digit;

    }

    // Convert Sum to Char, and Check if Equal to Last Digit
    let sum_digit = ((sum % 10) + ('0' as u8)) as char;
    return sum_digit == last_digit;

}