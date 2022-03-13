use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

mod response;
mod request;

mod payment;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn bad_format() -> response::Response {

    return response::Response {
        response_code: 400,
        body: ("Bad Format.".to_string()),
        headers: HashMap::from([("Content-Length".to_string(), "Bad Format.".len().to_string()), ("Content-Type".to_string(), "text/html".to_string())])
    };

}

fn get_luhn(request_data:request::Request) -> response::Response {

    // Get the Credit Card Number from the Path
    let request_parts: Vec<&str> = request_data.path.split('/').collect();
    if request_parts.len() < 3 { return bad_format(); }
    let credit_number:&str = request_parts[2];
    if credit_number.len() == 0 { return bad_format(); }

    // Return Valid or Invalid
    if payment::luhn_method(credit_number) {

        return response::Response {
            response_code: 200,
            body: ("Valid.".to_string()),
            headers: HashMap::from([("Content-Length".to_string(), "Valid.".len().to_string()), ("Content-Type".to_string(), "text/html".to_string())])
        };

    } else {
        
        return response::Response {
            response_code: 400,
            body: ("Invalid.".to_string()),
            headers: HashMap::from([("Content-Length".to_string(), "Invalid.".len().to_string()), ("Content-Type".to_string(), "text/html".to_string())])
        };

    }

}

fn get_issuer(request_data:request::Request) -> response::Response {

    let request_parts: Vec<&str> = request_data.path.split('/').collect();

    if request_parts.len() < 3 { return bad_format(); }

    let credit_number:&str = request_parts[2];
    let mut issuer_name:&str = "Invalid.";

    if credit_number.starts_with("1") { issuer_name = "UATP"; }
    
    if credit_number.starts_with("2") { issuer_name = "GPN"; }
    if credit_number.starts_with("6") { issuer_name = "GPN"; }
    if credit_number.starts_with("7") { issuer_name = "GPN"; }
    if credit_number.starts_with("8") { issuer_name = "GPN"; }
    if credit_number.starts_with("9") { issuer_name = "GPN"; }

    if credit_number.starts_with("4") { issuer_name = "Visa"; }

    if credit_number.starts_with("34") { issuer_name = "American Express"; }
    if credit_number.starts_with("37") { issuer_name = "American Express"; }
    
    if credit_number.starts_with("31") { issuer_name = "China T-Union"; }
    
    if credit_number.starts_with("62") { issuer_name = "China UnionPay"; }
    
    if credit_number.starts_with("36") { issuer_name = "Diners Club International"; }
    
    if credit_number.starts_with("54") { issuer_name = "Diners Club United States & Canada"; }
    
    if credit_number.starts_with("60") { issuer_name = "RuPay"; }
    if credit_number.starts_with("65") { issuer_name = "RuPay"; }
    if credit_number.starts_with("81") { issuer_name = "RuPay"; }
    if credit_number.starts_with("82") { issuer_name = "RuPay"; }
    if credit_number.starts_with("508") { issuer_name = "RuPay"; }

    if credit_number.starts_with("51") { issuer_name = "Mastercard"; }
    if credit_number.starts_with("52") { issuer_name = "Mastercard"; }
    if credit_number.starts_with("53") { issuer_name = "Mastercard"; }
    // if credit_number.starts_with("54") { issuer_name = "Mastercard"; }
    if credit_number.starts_with("55") { issuer_name = "Mastercard"; }

    if credit_number.starts_with("65") { issuer_name = "Troy-Discover"; }

    if credit_number.starts_with("65") { issuer_name = "Discover Card"; }
    if credit_number.starts_with("6011") { issuer_name = "Discover Card"; }
    if credit_number.starts_with("644") { issuer_name = "Discover Card"; }
    if credit_number.starts_with("645") { issuer_name = "Discover Card"; }
    if credit_number.starts_with("646") { issuer_name = "Discover Card"; }
    if credit_number.starts_with("647") { issuer_name = "Discover Card"; }
    if credit_number.starts_with("648") { issuer_name = "Discover Card"; }
    if credit_number.starts_with("649") { issuer_name = "Discover Card"; }
    
    // 622126–622925: China UnionPay co-branded
    // 60400100–60420099: UkrCard
    
    if credit_number.starts_with("353") { issuer_name = "RuPay-JCB"; }
    if credit_number.starts_with("356") { issuer_name = "RuPay-JCB"; }
    
    if credit_number.starts_with("636") { issuer_name = "InterPayment"; }

    if credit_number.starts_with("637") { issuer_name = "InstaPayment"; }
    if credit_number.starts_with("638") { issuer_name = "InstaPayment"; }
    if credit_number.starts_with("639") { issuer_name = "InstaPayment"; }

    // 3528–3589: JCB

    if credit_number.starts_with("8600") { issuer_name = "UzCard"; }

    if credit_number.starts_with("9860") { issuer_name = "Humo"; }

    if credit_number.starts_with("6304") { issuer_name = "Laser"; }
    if credit_number.starts_with("6706") { issuer_name = "Laser"; }
    if credit_number.starts_with("6771") { issuer_name = "Laser"; }
    if credit_number.starts_with("6709") { issuer_name = "Laser"; }

    if credit_number.starts_with("5018") { issuer_name = "Maestro"; }
    if credit_number.starts_with("5020") { issuer_name = "Maestro"; }
    if credit_number.starts_with("5038") { issuer_name = "Maestro"; }
    if credit_number.starts_with("5893") { issuer_name = "Maestro"; }
    if credit_number.starts_with("6304") { issuer_name = "Maestro"; }
    if credit_number.starts_with("6759") { issuer_name = "Maestro"; }
    if credit_number.starts_with("6761") { issuer_name = "Maestro"; }
    if credit_number.starts_with("6762") { issuer_name = "Maestro"; }
    if credit_number.starts_with("6763") { issuer_name = "Maestro"; }

    if credit_number.starts_with("5019") { issuer_name = "Dankort"; }
    if credit_number.starts_with("4571") { issuer_name = "Dankort-Visa"; }

    if credit_number.starts_with("2200") { issuer_name = "Mir"; }
    if credit_number.starts_with("2201") { issuer_name = "Mir"; }
    if credit_number.starts_with("2202") { issuer_name = "Mir"; }
    if credit_number.starts_with("2203") { issuer_name = "Mir"; }
    if credit_number.starts_with("2204") { issuer_name = "Mir"; }

    // 6054740–6054744: NPS Pridnestrovie

    // 2221–2720: Mastercard

    if credit_number.starts_with("9792") { issuer_name = "Troy"; }

    if credit_number.starts_with("4026") { issuer_name = "Visa Electron"; }
    if credit_number.starts_with("417500") { issuer_name = "Visa Electron"; }
    if credit_number.starts_with("4508") { issuer_name = "Visa Electron"; }
    if credit_number.starts_with("4844") { issuer_name = "Visa Electron"; }
    if credit_number.starts_with("4913") { issuer_name = "Visa Electron"; }
    if credit_number.starts_with("4917") { issuer_name = "Visa Electron"; }
    
    // 650002–650027: Verve
    // 506099–506198: Verve

    if credit_number.starts_with("357111") { issuer_name = "LankaPay"; }

    if credit_number.starts_with("6759") { issuer_name = "Maestro UK"; }
    if credit_number.starts_with("676770") { issuer_name = "Maestro UK"; }
    if credit_number.starts_with("676774") { issuer_name = "Maestro UK"; }

    return response::Response {
        response_code: 200,
        body: (issuer_name.to_string()),
        headers: HashMap::from([("Content-Length".to_string(), issuer_name.len().to_string()), ("Content-Type".to_string(), "text/html".to_string())])
    };

}

fn handle_connection(mut stream: TcpStream) {

    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let request_data = request::parse_request(buffer);
    let request_parts: Vec<&str> = request_data.path.split('/').collect();
    let request_intro = request_parts[1];

    let response_data:response::Response = match request_intro {

        "issuer" => get_issuer(request_data),
        "luhn" => get_luhn(request_data),
        _ => response::Response {
            response_code: 404,
            body: ("Endpoint Not Found".to_string()),
            headers: HashMap::from([("Content-Length".to_string(), "Endpoint Not Found".len().to_string()), ("Content-Type".to_string(), "text/html".to_string())])
        }

    };

    let response = response_data.response_full();

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}