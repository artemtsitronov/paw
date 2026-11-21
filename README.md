 https://github.com/user-attachments/assets/79e51273-6bfa-46e2-a8bc-4051c550e9c8
 
 # paw
 
 Paw is a simple URL fetcher, written in Rust by Artem Tsitronov.
 
 ## description
 
 `paw` takes a URL as an argument and downloads the file.
 It has a pretty CLI interface (see the video for details).
 It's (obviously) less powerful than wget or curl, but it's a simple, useful tool.
 Made for the [Hack Club](https://hackclub.com) "Help Grandma Out" challenge.
 
 ## Installation
 
 Download a binary for Linux, Windows or macOS from the [Releases page](https://github.com/artemtsitronov/paw/releases), or install with cargo:
 
 ```bash
 cargo install --git https://github.com/artemtsitronov/paw
 ```
 
 ## Usage
 
 ```bash
 paw <url>
 paw <url> -o name.ext   # choose the file name
 paw <url> -q            # quiet mode
 ```
 
 Paw never overwrites an existing file. If the name is taken, it exits with an error.
 
