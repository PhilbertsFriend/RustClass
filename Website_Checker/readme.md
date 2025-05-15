// Rust Website Checker Instructions

Build Instructions:
1. In the directory of the program, run "cargo build --release" 
2. This will generate the program at "./target/release/Website_Checker" where "Website_Checker" is the name of your program

Usage:
1. Text File 
If you want to check urls from a text file, it has to be in the format of 1 url per line. Any blank line or line starting with # will be automatically skipped.
The command is "./target/release/Website_Checker --file websites.txt" where "websites.txt" can be any text file in the format described above.

2. Direct URLs
If you want to check a direct link without using a text file, you can type it as an argument after "--file" with a space before and after the link in a format like:
"./target/release/Website_Checker https://example.com"

3. Combined
Both methods above can be combined like:
" ./target/release/Website_Checker --file websites.txt https://example.com"

Output:
All checked websites will output to "status.json" with the format 
{
    "url": "https://reddit.com",
    "status": 403,
    "response_time_ms": 28,
    "timestamp": "SystemTime { tv_sec: 1747264466, tv_nsec: 791717156 }"
},

Example outputs:
- No provided links or text file = "No URLs provided."

- Working Website = 
  {
    "url": "https://google.com",
    "status": 200,
    "response_time_ms": 152,
    "timestamp": "SystemTime { tv_sec: 1747264466, tv_nsec: 902081019 }"
  },

- Nonresponding Website = 
  {
    "url": "https://microsoftonline.com",
    "status": "Error: error sending request for url (https://microsoftonline.com/)",
    "response_time_ms": 22,
    "timestamp": "SystemTime { tv_sec: 1747264467, tv_nsec: 61286752 }"
  },