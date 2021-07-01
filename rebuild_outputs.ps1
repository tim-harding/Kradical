cargo build --release;
.\target\release\kradical_converter.exe krad rust --inputs .\assets\edrdg_files\kradfile .\assets\edrdg_files\kradfile2 --output .\kradical_static\src\decompositions.rs;
.\target\release\kradical_converter.exe radk rust --inputs .\assets\edrdg_files\radkfile .\assets\edrdg_files\radkfile2 --output .\kradical_static\src\memberships.rs;
.\target\release\kradical_converter.exe radk unicode --inputs .\assets\edrdg_files\radkfile .\assets\edrdg_files\radkfile2 --output .\assets\outputs\radk_utf8.txt;
.\target\release\kradical_converter.exe krad unicode --inputs .\assets\edrdg_files\kradfile .\assets\edrdg_files\kradfile2 --output .\assets\outputs\krad_utf8.txt;