#include <cstdlib>
#include <iostream>
#include <string>

int main(int argc, char* argv[]) {
    std::string bin = "rustps", out = "submit.rs";
    if (argc > 1) bin = argv[1];
    if (argc > 2) out = argv[2];

    std::string cmd =
        "cargo equip --bin " + bin +
        " --remove docs"
        " --remove comments"
        " --minify all"
        " --no-rustfmt"
        " > " + out;

    int ret = std::system(cmd.c_str());
    if (ret) {
        std::cerr << "Failed : " << ret << '\n';
        return 1;
    }

    std::cout << "Gen : " << out << '\n';
    return 0;
}