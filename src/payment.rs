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

pub fn get_issuer(credit_number:&str) -> &str {
    
    if credit_number.starts_with("6759") { return "Maestro UK"; }
    if credit_number.starts_with("676770") { return "Maestro UK"; }
    if credit_number.starts_with("676774") { return "Maestro UK"; }

    if credit_number.starts_with("357111") { return "LankaPay"; }
    
    if credit_number.starts_with("417500") { return "Visa Electron"; }
    if credit_number.starts_with("4026") { return "Visa Electron"; }
    if credit_number.starts_with("4508") { return "Visa Electron"; }
    if credit_number.starts_with("4844") { return "Visa Electron"; }
    if credit_number.starts_with("4913") { return "Visa Electron"; }
    if credit_number.starts_with("4917") { return "Visa Electron"; }

    if credit_number.starts_with("8600") { return "UzCard"; }

    if credit_number.starts_with("9860") { return "Humo"; }

    if credit_number.starts_with("6304") { return "Laser"; }
    if credit_number.starts_with("6706") { return "Laser"; }
    if credit_number.starts_with("6771") { return "Laser"; }
    if credit_number.starts_with("6709") { return "Laser"; }

    if credit_number.starts_with("5018") { return "Maestro"; }
    if credit_number.starts_with("5020") { return "Maestro"; }
    if credit_number.starts_with("5038") { return "Maestro"; }
    if credit_number.starts_with("5893") { return "Maestro"; }
    if credit_number.starts_with("6304") { return "Maestro"; }
    if credit_number.starts_with("6759") { return "Maestro"; }
    if credit_number.starts_with("6761") { return "Maestro"; }
    if credit_number.starts_with("6762") { return "Maestro"; }
    if credit_number.starts_with("6763") { return "Maestro"; }

    if credit_number.starts_with("5019") { return "Dankort"; }
    if credit_number.starts_with("4571") { return "Dankort-Visa"; }

    if credit_number.starts_with("2200") { return "Mir"; }
    if credit_number.starts_with("2201") { return "Mir"; }
    if credit_number.starts_with("2202") { return "Mir"; }
    if credit_number.starts_with("2203") { return "Mir"; }
    if credit_number.starts_with("2204") { return "Mir"; }

    if credit_number.starts_with("9792") { return "Troy"; }
    
    if credit_number.starts_with("637") { return "InstaPayment"; }
    if credit_number.starts_with("638") { return "InstaPayment"; }
    if credit_number.starts_with("639") { return "InstaPayment"; }

    if credit_number.starts_with("636") { return "InterPayment"; }
    
    if credit_number.starts_with("353") { return "RuPay-JCB"; }
    if credit_number.starts_with("356") { return "RuPay-JCB"; }

    if credit_number.starts_with("6011") { return "Discover Card"; }
    if credit_number.starts_with("644") { return "Discover Card"; }
    if credit_number.starts_with("645") { return "Discover Card"; }
    if credit_number.starts_with("646") { return "Discover Card"; }
    if credit_number.starts_with("647") { return "Discover Card"; }
    if credit_number.starts_with("648") { return "Discover Card"; }
    if credit_number.starts_with("649") { return "Discover Card"; }
    if credit_number.starts_with("65") { return "Discover Card"; }

    if credit_number.starts_with("508") { return "RuPay"; }
    if credit_number.starts_with("60") { return "RuPay"; }
    if credit_number.starts_with("65") { return "RuPay"; }
    if credit_number.starts_with("81") { return "RuPay"; }
    if credit_number.starts_with("82") { return "RuPay"; }

    if credit_number.starts_with("65") { return "Troy-Discover"; }

    if credit_number.starts_with("51") { return "Mastercard"; }
    if credit_number.starts_with("52") { return "Mastercard"; }
    if credit_number.starts_with("53") { return "Mastercard"; }
    if credit_number.starts_with("55") { return "Mastercard"; }

    if credit_number.starts_with("34") { return "American Express"; }
    if credit_number.starts_with("37") { return "American Express"; }
    
    if credit_number.starts_with("31") { return "China T-Union"; }
    
    if credit_number.starts_with("62") { return "China UnionPay"; }
    
    if credit_number.starts_with("36") { return "Diners Club International"; }
    
    if credit_number.starts_with("54") { return "Diners Club United States & Canada"; }

    if credit_number.starts_with("1") { return "UATP"; }
    
    if credit_number.starts_with("2") { return "GPN"; }
    if credit_number.starts_with("6") { return "GPN"; }
    if credit_number.starts_with("7") { return "GPN"; }
    if credit_number.starts_with("8") { return "GPN"; }
    if credit_number.starts_with("9") { return "GPN"; }

    if credit_number.starts_with("4") { return "Visa"; }

    // 622126–622925: China UnionPay co-branded
    // 60400100–60420099: UkrCard
    
    // 3528–3589: JCB

    // 6054740–6054744: NPS Pridnestrovie

    // 2221–2720: Mastercard

    // 650002–650027: Verve
    // 506099–506198: Verve

    return "";

}