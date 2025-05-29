{
    stdenv,
    fetchgit,
}:

stdenv.mkDerivation {
    pname = "web-dump-rs";
    version = "v0.3.1";

    src = fetchgit {
        url = "https://github.com/lilith-roth/web-dump-rs";
        sha256 = "yweiUXX6BBiPpafoeFgRhGRAiuy17R+LBnB+OAwlV5";
I=
    };
}
