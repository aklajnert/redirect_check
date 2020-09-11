# Redirect check

Simple application that checks if redirects are set up properly. It takes an input in form of a CSV
file with 2 or 3 columns - `Name` (*optional*), `Source`, `Target`. Example input:  
```csv
Name,Source,Target
Github,http://github.com,https://github.com
Match only end,https://bitly.is/2YX2mnI,/pages/privacy
Wrong match,http://github.com,https://github.com/redirect
```

Running the program with example input should result a following output:  
```
Provide path to the redirect definitions file:
example-input.csv
Loaded 3 records. Checking...
OK: Github (http://github.com)
OK: Match only end (https://bitly.is/2YX2mnI)
Fail: Wrong match (http://github.com)

1/3 tests failed.

Failures:
---------

Wrong match (http://github.com)
	Expected: http://github.com
	Got: https://github.com

Press ENTER to exit...

```

## Installation

As I don't have a certificate to sign the binary, the only way to install it is via [cargo](https://doc.rust-lang.org/cargo/):  
```shell script
cargo install --git https://github.com/aklajnert/redirect_check/
```
