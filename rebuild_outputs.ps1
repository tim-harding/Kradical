cargo run --release -- krad rust --inputs .\assets\edrdg_files\kradfile .\assets\edrdg_files\kradfile2 --output .\kradical_static\src\decompositions.rs;
cargo run --release -- radk rust --inputs .\assets\edrdg_files\radkfile .\assets\edrdg_files\radkfile2 --output .\kradical_static\src\memberships.rs;
cargo run --release -- radk unicode --inputs .\assets\edrdg_files\radkfile .\assets\edrdg_files\radkfile2 --output .\assets\outputs\radk_utf8.txt;
cargo run --release -- krad unicode --inputs .\assets\edrdg_files\kradfile .\assets\edrdg_files\kradfile2 --output .\assets\outputs\krad_utf8.txt;