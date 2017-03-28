Neues Spiel
-----------
POST /rest/new/<white>/<black>

Response:
{
  "board": {
    "white": [[x, y, type], ... ],
    "black": [[x, y, type], ... ]
  }
  "next_move":
  "valid_pieces_to_move":
  "uid": "uid"
  "winner":
}

Spielstein auswählen
--------------------

POST /rest/select/:uid:/:x:/:y:

Response:
{
  "valid_moves": [[x, y], ...]
}

Spielstein bewegen
------------------

POST /rest/move/:uid:/:src_x:/:src_y:/:dst_x:/:dst_y:

Reponse:
see "Neues Spiel"
but without "uid"


Status
------
GET / ....

https://preloaders.net/en/circular/filled-fading-balls/
https://github.com/JohnPostlethwait/fixme

TODO
* bessere evaluierungsfunktion
* zeitbasierten abbruch
* im hintergrund nachdenken
* parallelisieren: dazu erst breitensuche und dann mit mehreren threds
  tiefensuche

* run tests
cargo test --release --lib -- --nocapture
build shared library + executable
* run main
cargo build --release
* run an example
cargo run --example ai_vs_ai

TODO
  Load game via ./rest-py --load t

  // Cargo.toml
  // # compile release code with debug information
  // [profile.release]
  // debug = true
  // cargo build --release
  // valgrind --tool=callgrind ./target/release/engine
  // kcachegrind callgrind.out.30282

./rest.py --load q
Die AI berechnet für alle drei "normalen" schwarzen Steine 0.25. Aber der eine Zug
hat ein sofortiges Schlagen des schwarzen Steins zur Folge. Wieso dann 0.25?
Das ist der Pfad zum berechneten Score:
A f4-e3    s:3n+1d  w:5n+1d   0*10 + -2*1/12 + 0*3 + 1*0.5 = 0.33
H f2-d4    s:2n+1d  w:5n+1d
A g7-h6
H e1-f2
A g5-h4
H d8-e7    s:2n+1d  w:5n+1d   0*10 + -3*1/12 + 0*3 + 1*0.5 = 0.25
