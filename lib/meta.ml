let app = Sys.getcwd () |> String.split_on_char '/' |> List.rev |> List.hd
let title = "Rust bindings for DPDK/SPDK"
let about = "\
- raw networking
- raw storage
- high-load I/O
"

(* *)
let user = "dponyatov"
let devuser = user
let devserver = "10.110.1.110"

(* *)
let author = "Dmitry Ponyatov"
let email = "dponyatov@gmail.com"
let year = 2026
let version = "0.0.1"
let license = "MIT"
let github = [%string "https://github.com/ponyatov/%{app}"]
let gitflic = [%string "https://gitflic.ru/project/%{user}/%{app}"]
