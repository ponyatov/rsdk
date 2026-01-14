Sys.command("ln -fs ../rc rc")|>ignore;
Sys.command("ln -fs ~/em/legas legas")|>ignore;
Sys.command("cp legas/.ocaml* ./")|>ignore;
Sys.command("mkdir lib")|>ignore;
Sys.command("cp ~/em/legas/meta.ml lib/meta.ml")|>ignore;
Sys.command("code legas/legas.ml lib/meta.ml")|>ignore;

#use "lib/meta.ml"

#use "legas/files.ml"
files();;
#use "legas/appfiles.ml"
appfiles();;

#use "legas/vscode.ml"
vscode()

#use "legas/git.ml";;
git();;

#use "legas/ref.ml";;
ref();

#use "legas/dev01.ml"
dev01();

#use "legas/ocaml.ml"
ocamldots();
(* ocaml(); *)

#use "legas/doc.ml"
doc();

#use "legas/mk.ml"
mk();

#use "legas/cpp.ml"
hpp();cpp();init();

#use "legas/cmake.ml"
cmake();

#use "legas/rust.ml"
rust();

(* #use "legas/sync.ml" *)
#use "legas/etc.ml"
etc();
