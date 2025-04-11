use serde::Deserialize;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::time::Duration;
use std::thread;

#[derive(Debug)]
enum ApiResult<T>{ //enum stolen from dog image example with added generic to allow each struct to call it.
    Success(T), 
    ApiError(String), //API error or network error will output a string
    NetworkError(String),
}

#[derive(Deserialize, Debug)] //Second struct to get the dollar data as the API uses "coin": { "usd": # } instead of separate lines like dog image example.
struct Price {
    usd: f64,
}

pub trait Pricing { //Pricing trait so each struct has the same functions
    fn fetch_price(&self) -> ApiResult<Self>
    where 
    Self: Sized;

    fn save_to_file(&self);
}

#[derive(Debug, Deserialize)]
pub struct Bitcoin{
    bitcoin: Price,
}
impl Pricing for Bitcoin{ //Implementing the trait 
    fn fetch_price(&self) -> ApiResult<Self> { //Referencing itself and returning the result with the generic also declared as self.
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd"; //This API works but will throw errors for requesting too fast. 
        
        match ureq::get(url).call() { //Nested match stolen from dog image example to check for errors
            Ok(response) => {
                if response.status() == 200 {
                    match response.into_json::<Bitcoin>() {
                        Ok(price) => ApiResult::Success(price), //If no error, will return API result.
                        Err(e) => ApiResult::ApiError(format!("Failed to parse JSON: {}", e)),
                    }
                } else {
                    ApiResult::ApiError(format!("HTTP error: {}", response.status()))
                }
            },
            Err(e) => {
                let error_details = format!("Request failed: {}", e);
                ApiResult::NetworkError(error_details)
            },
        }
    }
    fn save_to_file(&self){
        println!("Saving Bit Price"); //Terminal output so I know if the code is running 
        match Self::fetch_price(&self){ //Calling the function to check API
        ApiResult::Success(price) => { //If no error, output to Struct specific file. 
           let mut file = OpenOptions::new() 
                .append(true)
                .open("bitcoin.txt")
                .unwrap();

                writeln!(file, "{}", price.bitcoin.usd);
                println!("Saved Bit");
        },
        ApiResult::ApiError(e) => println!("❌ API Error: {}", e),
        ApiResult::NetworkError(e) => println!("❌ Network Error: {}", e),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Ethereum{
    ethereum: Price,
}

impl Pricing for Ethereum{
    fn fetch_price(&self) -> ApiResult<Self> {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd"; //This API works but will throw errors for requesting too fast. 
        
        match ureq::get(url).call() {
            Ok(response) => {
                if response.status() == 200 {
                    match response.into_json::<Ethereum>() {
                        Ok(price) => ApiResult::Success(price),
                        Err(e) => ApiResult::ApiError(format!("Failed to parse JSON: {}", e)),
                    }
                } else {
                    ApiResult::ApiError(format!("HTTP error: {}", response.status()))
                }
            },
            Err(e) => {
                let error_details = format!("Request failed: {}", e);
                ApiResult::NetworkError(error_details)
            },
        }
    }
    fn save_to_file(&self){
        println!("Saving Eth Price");
        match Self::fetch_price(&self){
            ApiResult::Success(price) => {
                let mut file = OpenOptions::new()
                .append(true)
                .open("ethereum.txt")
                .unwrap();

                writeln!(file, "{}", price.ethereum.usd);
                println!("Saved Eth");
            },
            ApiResult::ApiError(e) => println!("❌ API Error: {}", e),
            ApiResult::NetworkError(e) => println!("❌ Network Error: {}", e),
        }
    }
}

/*#[derive(Debug, Deserialize)]
pub struct SP500{
    sp500: Price,
}

impl Pricing for SP500;
    fn fetch_price(&self) -> ApiResult<Self> {
        let url = "";
        
        match ureq::get(url).call() {
            Ok(response) => {
                if response.status() == 200 {
                    match response.into_json::<SP500>() {
                        Ok(price) => ApiResult::Success(price),
                        Err(e) => ApiResult::ApiError(format!("Failed to parse JSON: {}", e)),
                    }
                } else {
                    ApiResult::ApiError(format!("HTTP error: {}", response.status()))
                }
            },
            Err(e) => {
                let error_details = format!("Request failed: {}", e);
                ApiResult::NetworkError(error_details)
            },
        }
    }
    fn save_to_file(&self){
        println!("Saving SP5 Price");
        match Self::fetch_price(&self){
            ApiResult::Success(price) => {
                let mut file = OpenOptions::new()
                .append(true)
                .open("sp500.txt")
                .unwrap();

                writeln!(file, "{}", price.sp500.usd);
                println!("Saved SP5");
            },
            ApiResult::ApiError(e) => println!("❌ API Error: {}", e),
            ApiResult::NetworkError(e) => println!("❌ Network Error: {}", e),
        }
    }
}
*/
////////////////////////////////////// I couldn't find an API for the S&P 500 without having to pay. 

fn main() {
    let mut file = File::create("bitcoin.txt").unwrap(); //Make new files for the data.
    let mut file = File::create("ethereum.txt").unwrap();
    //let mut file = File::create("sp500.txt").unwrap();

    let bit = Bitcoin {bitcoin: Price{ usd: 0 as f64}}; //Make structs for the data
    let eth = Ethereum {ethereum: Price{ usd: 0 as f64}};
    //let sp5 = SP500 {sp500: Price{ usd: 0 as f64}};
    
    let data: Vec<Box<dyn Pricing>> = vec![Box::new(bit), Box::new(eth)];

    for i in 1..5{ //For loop so it doesn't run forever.
        for i in &data{ 
        i.save_to_file(); //Call the save functions, don't have to call fetch as the save function calls it anyways.
        //sp5.save_to_file();
        }
        thread::sleep(Duration::from_secs(10)); //Sleep for 10 seconds. API will whine with errors beacuse the requests are too fast.
   }
}